# Tutorial: comparing Utahnetes to Kubernetes (mental model only)

This is not a migration guide. It is a **translation table** so you can place Utahnetes in your mental map of the ecosystem.

---

## One-sentence each

- **Kubernetes:** a production-grade container orchestrator with a control plane, declarative APIs, and a large operational surface area.
- **Utahnetes:** a tiny LAN prototype about discovery, gossip, toy “heat,” and WASM execution hooks.

---

## Concept mapping (loose analogies)

| Kubernetes idea | Utahnetes prototype analogue | Important caveat |
|-----------------|------------------------------|------------------|
| Node | machine running `ignite` | no kubelet/agent model |
| Pod / workload | WASM bytes + `run` export | not containers |
| Scheduler | “coldest peer” heuristic in `pour` | extremely simplified |
| Service discovery | mDNS + gossipsub | LAN-oriented |
| Config manifests | none in CLI path | everything is code/constants |
| etcd | none | no cluster state DB |

---

## What Kubernetes does that Utahnetes does not (by design)

- declarative desired state reconciliation
- rolling updates and rollbacks (built-in)
- network policies
- persistent volumes
- cloud integrations

---

## What Utahnetes highlights that Kubernetes courses sometimes bury under YAML

- **membership dynamics** on local networks (discovery)
- **epidemic messaging** patterns (gossip)
- **packaging compute** as WASM modules

---

## Classroom prompt

Ask students:

> “Which parts of Kubernetes exist because distributed systems are genuinely hard, and which parts exist because organizations want uniform control?”

Utahnetes is a toy lens, not an answer key.

---

## Related

- [What is Utahnetes?](what-is-utahnetes.md)
- [When to use Utahnetes](when-to-use.md)
