# Glossary

| Term | Plain language | Technical note (short) |
|------|----------------|------------------------|
| **Cup** | A computer participating as a worker node in the metaphor | A machine running `ignite` |
| **Heat** | A simple “how busy am I?” dial | `NodeThermodynamics` heat index in \([0,1]\) |
| **Ignite** | “Join the pool and advertise readiness” | CLI subcommand starting the reactor loop |
| **Pour** | “Send a WASM package into the pool” | CLI subcommand publishing pour frames |
| **WASM / WebAssembly** | A portable binary format for programs | Executed by Wasmtime via `WasmCell` |
| **Cell** | A tiny packaged program (metaphor) | WASM module bytes |
| **Swarm** | The group of cooperating peers | libp2p `Swarm` |
| **mDNS** | Local “who is here?” discovery | multicast DNS style discovery |
| **gossipsub** | Many-to-many messaging over topics | pub/sub gossip protocol |
| **Topic** | A named channel for messages | e.g. `utahnetes-heat` |
| **Noise** | Encryption layer for connections | libp2p Noise protocol |
| **Yamux** | Multiplexing streams on one connection | Yamux muxer |
| **libp2p** | A toolkit for peer-to-peer networking | Rust crate ecosystem |
| **LAN** | Local area network: home/office lab segment | not the whole internet by default |
| **Prototype** | Learning software, not full production platform | explicit scope boundary |

---

## Related

- [What is Utahnetes?](what-is-utahnetes.md)
- [Non-technical guide](non-technical-guide.md)
