// [MODULE: UTAHNETES-CLI-REACTOR]
// CLI entry: `ignite` (thermodynamic reactor) and `pour` (WASM dispatch).

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::{Parser, Subcommand};
use libp2p::futures::StreamExt;
use libp2p::gossipsub::{self, IdentTopic};
use libp2p::identity::Keypair;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{Multiaddr, PeerId};
use log::{error, info, warn};
use utahnetes::{NodeThermodynamics, UtahnetesBehavior, UtahnetesSwarmEvent, WasmCell};

const HEAT_TOPIC: &str = "utahnetes-heat";
const POUR_TOPIC: &str = "utahnetes-pour";
/// Max WASM bytes embedded in a gossipsub pour frame (512 KiB mesh limit minus framing).
const MAX_POUR_WASM: usize = 480 * 1024;

#[derive(Parser)]
#[command(name = "utahnetes")]
#[command(about = "Liquid compute orchestrator (post-Kubernetes prototype)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Join the LAN swarm (mDNS), gossip heat, and execute pour payloads destined for this cup.
    Ignite {
        #[arg(short, long, default_value_t = 10_000)]
        capacity: usize,
    },
    /// Read a WASM cell, observe peer heat on the mesh, then pour into the coldest cup (or any cold cup).
    Pour {
        /// Path to the `.wasm` file
        #[arg(short, long)]
        file: PathBuf,
        /// Seconds to listen for `HEAT:` gossip before choosing a target
        #[arg(long, default_value_t = 5)]
        observe_secs: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ignite { capacity } => {
            info!("Igniting Utahnetes reactor (capacity: {})", capacity);
            run_reactor(*capacity).await?;
        }
        Commands::Pour {
            file,
            observe_secs,
        } => {
            info!("Pouring {:?} into the swarm", file);
            execute_pour(file, Duration::from_secs(*observe_secs)).await?;
        }
    }

    Ok(())
}

fn heat_topic() -> IdentTopic {
    IdentTopic::new(HEAT_TOPIC)
}

fn pour_topic() -> IdentTopic {
    IdentTopic::new(POUR_TOPIC)
}

fn build_swarm(keypair: Keypair) -> Result<Swarm<UtahnetesBehavior>, Box<dyn Error + Send + Sync>> {
    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| UtahnetesBehavior::new(key.clone()))?
        .build();
    Ok(swarm)
}

async fn run_reactor(capacity: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    let thermo = NodeThermodynamics::new(capacity);
    let keypair = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(keypair.public());

    let mut swarm = build_swarm(keypair)?;
    let heat = heat_topic();
    let pour = pour_topic();
    let _ = swarm.behaviour_mut().fluid_gossip.subscribe(&heat);
    let _ = swarm.behaviour_mut().fluid_gossip.subscribe(&pour);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    let mut tick = tokio::time::interval(Duration::from_secs(5));
    tick.tick().await;

    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("Node listening on {}", address);
                    }
                    SwarmEvent::Behaviour(ev) => {
                        handle_swarm_event(local_peer_id, &thermo, &heat, &pour, ev);
                    }
                    _ => {}
                }
            }
            _ = tick.tick() => {
                let heat_idx = thermo.current_heat_index();
                if let Err(e) = swarm.behaviour_mut().broadcast_heat_signature(&heat, heat_idx) {
                    warn!("Heat gossip failed: {:?}", e);
                } else {
                    info!("Broadcast heat index: {:.1}%", heat_idx * 100.0);
                }
            }
        }
    }
}

fn handle_swarm_event(
    local_peer_id: PeerId,
    thermo: &NodeThermodynamics,
    heat_topic: &IdentTopic,
    pour_topic: &IdentTopic,
    ev: UtahnetesSwarmEvent,
) {
    match ev {
        UtahnetesSwarmEvent::LocalDiscovery(e) => {
            info!("mDNS: {:?}", e);
        }
        UtahnetesSwarmEvent::FluidGossip(gossipsub::Event::Message {
            propagation_source,
            message,
            ..
        }) => {
            if message.topic == heat_topic.hash() {
                if let Ok(s) = std::str::from_utf8(&message.data) {
                    if let Some(rest) = s.strip_prefix("HEAT:") {
                        if let Ok(h) = rest.trim().parse::<f32>() {
                            let origin = message
                                .source
                                .unwrap_or(propagation_source);
                            if origin != local_peer_id {
                                info!("Peer {} heat (approx): {:.1}%", origin, h * 100.0);
                            }
                        }
                    }
                }
                return;
            }
            if message.topic == pour_topic.hash() {
                if let Some((mode, wasm)) = decode_pour_frame(&message.data) {
                    let run = match mode {
                        PourMode::Any => thermo.can_accept_liquid(),
                        PourMode::Target(pid) => pid == local_peer_id && thermo.can_accept_liquid(),
                    };
                    if !run {
                        return;
                    }
                    info!("Accepting liquid pour ({} bytes)", wasm.len());
                    match WasmCell::ingest_code(&wasm) {
                        Ok(cell) => {
                            thermo.add_load(1_000);
                            if let Err(e) = cell.execute_intent() {
                                error!("WASM run failed: {e}");
                                thermo.remove_load(1_000);
                            } else {
                                info!("WASM cell executed; thermodynamic load applied");
                            }
                        }
                        Err(e) => error!("ingest_code failed: {e}"),
                    }
                }
            }
        }
        UtahnetesSwarmEvent::FluidGossip(other) => {
            log::debug!("Gossipsub: {:?}", other);
        }
    }
}

#[derive(Clone, Debug)]
enum PourMode {
    Any,
    Target(PeerId),
}

/// `UTPA` + u32 LE wasm len + wasm — any cold node.
/// `UTPO` + u16 BE peer string len + peer UTF-8 + u32 LE wasm len + wasm — targeted pour.
fn encode_pour_frame(mode: &PourMode, wasm: &[u8]) -> Result<Vec<u8>, &'static str> {
    if wasm.len() > MAX_POUR_WASM {
        return Err("WASM exceeds pour frame limit; shrink module or add request/response transport");
    }
    let mut out = Vec::new();
    match mode {
        PourMode::Any => {
            out.extend_from_slice(b"UTPA");
            out.extend_from_slice(&(wasm.len() as u32).to_le_bytes());
            out.extend_from_slice(wasm);
        }
        PourMode::Target(peer) => {
            let pid = peer.to_string();
            let pid_bytes = pid.as_bytes();
            if pid_bytes.len() > u16::MAX as usize {
                return Err("PeerId string too long");
            }
            out.extend_from_slice(b"UTPO");
            out.extend_from_slice(&(pid_bytes.len() as u16).to_be_bytes());
            out.extend_from_slice(pid_bytes);
            out.extend_from_slice(&(wasm.len() as u32).to_le_bytes());
            out.extend_from_slice(wasm);
        }
    }
    Ok(out)
}

fn decode_pour_frame(data: &[u8]) -> Option<(PourMode, Vec<u8>)> {
    if data.starts_with(b"UTPA") && data.len() >= 8 {
        let len = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
        let rest = data.get(8..)?;
        if rest.len() != len {
            return None;
        }
        return Some((PourMode::Any, rest.to_vec()));
    }
    if data.starts_with(b"UTPO") && data.len() >= 6 {
        let pid_len = u16::from_be_bytes(data[4..6].try_into().ok()?) as usize;
        let pid_end = 6 + pid_len;
        let pid_str = std::str::from_utf8(data.get(6..pid_end)?).ok()?;
        let peer: PeerId = pid_str.parse().ok()?;
        if data.len() < pid_end + 4 {
            return None;
        }
        let wasm_len = u32::from_le_bytes(data[pid_end..pid_end + 4].try_into().ok()?) as usize;
        let wasm = data.get(pid_end + 4..pid_end + 4 + wasm_len)?;
        if wasm.len() != wasm_len {
            return None;
        }
        return Some((PourMode::Target(peer), wasm.to_vec()));
    }
    None
}

async fn execute_pour(
    file: &PathBuf,
    observe: Duration,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let wasm = std::fs::read(file)?;
    if wasm.len() > MAX_POUR_WASM {
        return Err(format!(
            "WASM size {} exceeds pour limit {} bytes",
            wasm.len(),
            MAX_POUR_WASM
        )
        .into());
    }

    let keypair = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(keypair.public());
    let mut swarm = build_swarm(keypair)?;
    let heat = heat_topic();
    let pour = pour_topic();
    let _ = swarm.behaviour_mut().fluid_gossip.subscribe(&heat);
    let _ = swarm.behaviour_mut().fluid_gossip.subscribe(&pour);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    let deadline = Instant::now() + observe;
    let mut heat_by_peer: HashMap<PeerId, f32> = HashMap::new();

    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        match tokio::time::timeout(
            remaining.min(Duration::from_millis(500)),
            swarm.select_next_some(),
        )
        .await
        {
            Ok(SwarmEvent::Behaviour(UtahnetesSwarmEvent::FluidGossip(
                gossipsub::Event::Message {
                    propagation_source,
                    message,
                    ..
                },
            ))) => {
                if message.topic != heat.hash() {
                    continue;
                }
                let origin = message.source.unwrap_or(propagation_source);
                if origin == local_peer_id {
                    continue;
                }
                if let Ok(s) = std::str::from_utf8(&message.data) {
                    if let Some(rest) = s.strip_prefix("HEAT:") {
                        if let Ok(h) = rest.trim().parse::<f32>() {
                            heat_by_peer.insert(origin, h);
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }

    let mode = if heat_by_peer.is_empty() {
        warn!("No remote heat samples yet; pouring in ANY-cold mode");
        PourMode::Any
    } else {
        let (coldest, h) = heat_by_peer
            .iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .expect("non-empty");
        info!(
            "Coldest observed peer: {} (heat index {:.1}%)",
            coldest,
            *h * 100.0
        );
        PourMode::Target(*coldest)
    };

    let frame = encode_pour_frame(&mode, &wasm).map_err(|e| e.to_string())?;
    let frame_len = frame.len();
    swarm
        .behaviour_mut()
        .fluid_gossip
        .publish(pour.clone(), frame)
        .map_err(|e| format!("pour publish failed: {e:?}"))?;

    info!(
        "Pour frame published ({} bytes); waiting for mesh propagation",
        frame_len
    );
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(())
}
