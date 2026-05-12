# For managers, teachers, and curriculum designers

Utahnetes is a **teaching object** that happens to run. This page helps you decide how to use it responsibly in classrooms, meetups, and internal training.

---

## Learning outcomes you can honestly claim

After a well-run session, participants should be able to:

- explain **local discovery** in one sentence (“find peers nearby without a central directory”),
- explain why **pub/sub topics** help decouple senders and receivers,
- describe WASM as a **packaging format** for portable programs,
- articulate at least **one** reason why production systems add controls Utahnetes does not include.

---

## What not to promise

Avoid promising:

- “secure multi-tenant cloud”
- “automatic scaling to global traffic”
- “no-code app platform for everyone”

Those are separate products and disciplines.

---

## Suggested 60-minute workshop agenda

1. **Metaphor (10 min):** cups, heat, pour — use [Non-technical guide](non-technical-guide.md).
2. **Safety framing (5 min):** trusted LAN, trusted WASM — [Security and privacy](security-and-privacy.md).
3. **Live demo (20 min):** follow [First demo end-to-end](tutorial-first-demo-end-to-end.md).
4. **Debrief (15 min):** what broke, what surprised, what production needs.
5. **Optional homework:** draw your LAN; mark multicast paths.

---

## Assessment ideas (no coding required)

- **Written:** “List three risks of running unknown WASM.”
- **Oral:** “Explain why guest Wi‑Fi might break the demo.”
- **Team:** create a demo checklist with ten items.

---

## Inclusion notes

Pair non-technical participants with operators:

- one person reads logs aloud,
- one person runs commands,
- one person time-keeps.

This prevents a single “keyboard wizard” from dominating.

---

## Accessibility

- Prefer large fonts in terminals and screen magnification.
- Provide printed copies of command lines for participants who struggle with typing under pressure.

---

## Related

- [Homelab scenarios](homelab-scenarios.md)
- [When to use Utahnetes](when-to-use.md)
