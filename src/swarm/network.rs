// [MODULE: UTAHNETES-FABRIC]
// libp2p mesh: mDNS discovery + gossipsub for heat diffusion messages.

use libp2p::{gossipsub, identity::Keypair, mdns, swarm::NetworkBehaviour, PeerId};

/// Unified swarm event for the CLI reactor (`ignite` / `pour`).
#[derive(Debug)]
pub enum UtahnetesSwarmEvent {
    LocalDiscovery(mdns::Event),
    FluidGossip(gossipsub::Event),
}

impl From<mdns::Event> for UtahnetesSwarmEvent {
    fn from(e: mdns::Event) -> Self {
        UtahnetesSwarmEvent::LocalDiscovery(e)
    }
}

impl From<gossipsub::Event> for UtahnetesSwarmEvent {
    fn from(e: gossipsub::Event) -> Self {
        UtahnetesSwarmEvent::FluidGossip(e)
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "UtahnetesSwarmEvent")]
pub struct UtahnetesBehavior {
    /// Local peer discovery (LAN / data center) without static peer lists.
    pub local_discovery: mdns::tokio::Behaviour,
    /// Epidemic broadcast channel for thermodynamic signals (heat, intent hints).
    pub fluid_gossip: gossipsub::Behaviour,
}

impl UtahnetesBehavior {
    /// Default LAN-friendly stack: mDNS discovery + permissive gossipsub (prototype / lab use).
    pub fn new(local_key: Keypair) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let local_peer_id = PeerId::from(local_key.public());
        let local_discovery =
            mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .validation_mode(gossipsub::ValidationMode::Permissive)
            // Default is 64 KiB; pour carries raw WASM on the LAN prototype channel.
            .max_transmit_size(512 * 1024)
            .build()
            .map_err(|e| format!("gossipsub config: {e}"))?;
        let fluid_gossip = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key),
            gossipsub_config,
        )
        .map_err(|e| format!("gossipsub behaviour: {e}"))?;
        Ok(Self {
            local_discovery,
            fluid_gossip,
        })
    }

    /// Publish a scalar heat reading to a gossipsub topic (UTF-8 payload `HEAT:<f32>`).
    pub fn broadcast_heat_signature(
        &mut self,
        topic: &gossipsub::IdentTopic,
        heat: f32,
    ) -> Result<gossipsub::MessageId, gossipsub::PublishError> {
        let msg = format!("HEAT:{heat}");
        self.fluid_gossip.publish(topic.clone(), msg.into_bytes())
    }
}
