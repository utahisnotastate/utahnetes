# Utahnetes documentation

Welcome. This folder is designed so you can **start anywhere** and still end up with a coherent picture.

## Learning paths

### Path A — “Explain it like I’m not an engineer”

1. [Non-technical guide](non-technical-guide.md) (tasks, metaphors, collaboration patterns)
2. [Non-technical checklists](non-technical-checklists.md) (printable)
3. [What is Utahnetes?](what-is-utahnetes.md) (capabilities in plain language)
4. [When to use Utahnetes](when-to-use.md) (fit / non-fit)
5. [Glossary](glossary.md)

### Path B — “I want to run it today”

1. [Installation](installation.md)
2. [First demo end-to-end](tutorial-first-demo-end-to-end.md)
3. [Tutorial: `ignite`](tutorial-ignite.md)
4. [Tutorial: `pour`](tutorial-pour.md)
5. [CLI reference](cli-reference.md)
6. [Troubleshooting](troubleshooting.md)

### Path C — “I’m building or integrating”

1. [Architecture overview](architecture-overview.md)
2. [Networking concepts](networking-concepts.md)
3. [Tutorial: WASM cells](tutorial-wasm-cell.md)
4. [Security and privacy](security-and-privacy.md)

### Path D — “Ideas, demos, teaching”

1. [Homelab and scenario cookbook](homelab-scenarios.md)
2. [For managers and educators](for-managers-and-educators.md)
3. [Comparing to Kubernetes (teaching)](tutorial-comparing-to-kubernetes.md)
4. [FAQ](faq.md)

## All documents

| Document | Purpose |
|----------|---------|
| [non-technical-guide.md](non-technical-guide.md) | Non-engineer tasks, metaphors, how to work with a builder |
| [what-is-utahnetes.md](what-is-utahnetes.md) | Conceptual model and what the software literally does |
| [when-to-use.md](when-to-use.md) | Decision guide: good fits vs bad fits |
| [installation.md](installation.md) | Build, run, environment variables, platform notes |
| [tutorial-ignite.md](tutorial-ignite.md) | Deep dive on `utahnetes ignite` |
| [tutorial-pour.md](tutorial-pour.md) | Deep dive on `utahnetes pour` |
| [tutorial-wasm-cell.md](tutorial-wasm-cell.md) | WASM expectations, limits, examples |
| [networking-concepts.md](networking-concepts.md) | mDNS, gossipsub, topics, LAN scope |
| [security-and-privacy.md](security-and-privacy.md) | Threat model for a prototype LAN tool |
| [homelab-scenarios.md](homelab-scenarios.md) | Exhaustive scenario list for demos and teaching |
| [troubleshooting.md](troubleshooting.md) | Common failures and fixes |
| [glossary.md](glossary.md) | Vocabulary |
| [faq.md](faq.md) | Short answers to common questions |
| [architecture-overview.md](architecture-overview.md) | Module map for Rust readers |
| [contributing-and-roadmap.md](contributing-and-roadmap.md) | How to extend the project |
| [tutorial-first-demo-end-to-end.md](tutorial-first-demo-end-to-end.md) | Copy/paste first LAN demo |
| [cli-reference.md](cli-reference.md) | Flags, env vars, topics |
| [for-managers-and-educators.md](for-managers-and-educators.md) | Workshops, learning outcomes, inclusion |
| [non-technical-checklists.md](non-technical-checklists.md) | Printable preflight / debrief lists |
| [tutorial-comparing-to-kubernetes.md](tutorial-comparing-to-kubernetes.md) | Teaching comparison, not migration |

## Conventions used in these docs

- **Command examples** use Unix-style `RUST_LOG=...` where relevant; Windows equivalents are in [Installation](installation.md).
- **“LAN”** means a local network (home, office lab, classroom VLAN), not the public internet by default.
- **“Prototype”** means: exciting, educational, not automatically production-hardened.
