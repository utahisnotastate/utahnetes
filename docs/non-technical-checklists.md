# Non-technical checklists (print-friendly)

These checklists are for **hosts**, **facilitators**, and **team leads**. They assume a technical helper exists for build issues.

---

## Checklist A — Before the demo starts

- [ ] Everyone agrees this is a **trusted LAN** experiment.
- [ ] We have a **trusted WASM** file and know what it does when run.
- [ ] Laptops are plugged in; sleep is disabled for demo machines.
- [ ] We confirmed **AP isolation** is off (or we are on wired Ethernet).
- [ ] We have a **rollback plan** (stop terminals, disconnect Wi‑Fi).

---

## Checklist B — Right before `pour`

- [ ] At least **two** `ignite` terminals are running and look healthy.
- [ ] We waited **10+ seconds** after the last `ignite` started.
- [ ] We know the **exact path** to the `.wasm` file.
- [ ] We verified the file is **under the size limit** (see [Tutorial: pour](tutorial-pour.md)).

---

## Checklist C — After the demo (5 minutes)

- [ ] Capture **screenshots** or saved logs (redact sensitive info).
- [ ] Write **three bullets**: what worked / what didn’t / what to try next.
- [ ] Stop processes with **Ctrl+C**.

---

## Checklist D — “Should we use this for real work?”

Use with your team:

- [ ] Do we have **authentication** needs Utahnetes does not provide?
- [ ] Do we have **data sensitivity** needs Utahnetes does not provide?
- [ ] Do we need **SLAs** and on-call playbooks?
- [ ] If yes to any: Utahnetes is at best a **learning spike**, not the production backbone.

See [When to use Utahnetes](when-to-use.md).

---

## Related

- [Non-technical guide](non-technical-guide.md)
- [Troubleshooting](troubleshooting.md)
