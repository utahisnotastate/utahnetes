# Homelab and scenario cookbook

This page lists **many** ways to use Utahnetes as a prop, tool, or conversation starter. Not every scenario requires code: several are purely organizational.

---

## A. Solo scenarios (one human)

1. **“I want to read logs until it clicks.”**  
   Run `ignite`, watch mDNS and heat logs, narrate what you think is happening.

2. **“I want a weekend systems rabbit hole.”**  
   Read [Networking concepts](networking-concepts.md), draw a diagram of your Wi‑Fi, mark where multicast passes.

3. **“I want to benchmark my patience.”**  
   Try `pour` with `--observe-secs 1` vs `20` and notice how targeting changes.

---

## B. Two-person scenarios

4. **Pilot + engineer:** engineer prepares WASM; pilot runs `pour` on cue.

5. **Two cups + one observer:** two laptops `ignite`; third person only reads logs and reports anomalies.

6. **“Can we break it?” (consensual chaos engineering):**  
   One person toggles Wi‑Fi mid-demo; the other takes notes for a retrospective.

---

## C. Small team / meetup scenarios (3–12 people)

7. **Lightning talk demo (5 minutes):**  
   Two ignites, one pour, one slide with metaphors from [Non-technical guide](non-technical-guide.md).

8. **Hands-on station at a booth:**  
   QR codes to docs; a printed “expected log lines” cheat sheet.

9. **Interview simulation:**  
   Candidate explains what happens when `pour` runs; interviewer asks clarifying questions about trust boundaries.

10. **Onboarding day for interns:**  
    A guided hour: install → ignite → observe logs → discuss why production is harder.

---

## D. Classroom scenarios

11. **Middle school / high school “how computers talk” module:**  
    Emphasize discovery + messages; skip WASM compilation entirely; show prebuilt `.wasm`.

12. **College distributed systems week 0 lab:**  
    Utahnetes as a “small enough to read” counterpart to industrial systems.

13. **Ethics discussion prompt:**  
    “When is it okay to run a neighbor’s code?” Tie to consent, trust, and law.

---

## E. Homelab “projects that sound cool”

14. **LAN-only “compute club” night:** members bring machines; you run ignites; one member pours demos.

15. **Raspberry Pi + laptop asymmetry demo:** show heterogeneous hardware participating.

16. **Wired vs Wi‑Fi A/B test:** document discovery reliability differences.

---

## F. Product / engineering team scenarios

17. **Architecture reading group:** compare Utahnetes to your real orchestrator; list what is missing.

18. **“WASM distribution spike”:** evaluate whether tiny WASM tools could fit your internal platform story.

19. **SRE discussion prop:** talk about health signals, backoff, saturation—using heat as a toy dial.

---

## G. Creative / storytelling scenarios

20. **Sci-fi club:** Utahnetes as a prop for “liquid compute” narratives—explicitly labeled fiction + real protocols.

21. **Podcast B-roll:** typing commands while a guest explains P2P basics.

22. **Museum of obsolete complexity:** place Utahnetes next to a printed YAML manifest as a contrast artifact.

---

## H. Non-technical “productivity” scenarios (serious)

23. **Run a repeatable demo checklist** for executives (see [Non-technical guide](non-technical-guide.md)).

24. **Translate engineering updates** into metaphors for PMs weekly.

25. **Create a risk register** entry: “P2P demos require LAN constraints,” with mitigations.

---

## I. Developer-only scenarios

26. **Embed the library** in a larger Rust service and reuse `UtahnetesBehavior`.

27. **Replace pour transport** with chunked uploads (project idea).

28. **Add metrics export** (Prometheus, etc.) around heat and execution counts.

---

## J. “Cool but responsible” challenges

29. **Smallest useful WASM:** smallest module that still demonstrates `run`.

30. **Largest safe WASM under cap:** push toward the 480 KiB limit intentionally to learn performance.

31. **Write a postmortem template** for any failed demo (network, policy, build).

---

## When to stop using scenarios and read warnings

If any scenario involves:

- strangers,
- sensitive data,
- production SLAs,

…pause and read [Security and privacy](security-and-privacy.md) and [When to use Utahnetes](when-to-use.md).

---

## Next

- [FAQ](faq.md)
- [Troubleshooting](troubleshooting.md)
