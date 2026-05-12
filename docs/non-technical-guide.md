# Utahnetes for humans who do not think in code

This guide is intentionally long. You can skim the headings, stop when something clicks, and come back later.

If you only read one section, read: **[The one-minute story](#the-one-minute-story)** and then **[Things you can do without writing a single line of code](#things-you-can-do-without-writing-a-single-line-of-code)**.

---

## The one-minute story

Imagine a neighborhood potluck:

- Some neighbors have **big counters** (empty space).
- Some neighbors are **already juggling five dishes** (busy).
- You have a **recipe card** (a small program) you want someone to execute.

Utahnetes is like a **very simple community bulletin board** on your local Wi‑Fi:

- Each computer that runs `ignite` pins a note that says, roughly: **“Here is how full my counter is.”**
- When someone runs `pour`, they look at the notes, pick a neighbor whose counter looks emptier, and post a **“please run this recipe”** message.

The “recipe” in this project is a **WebAssembly** file (`.wasm`). That is a technical format, but you can treat it like a **prepared meal kit**: someone else prepares it; your job can be to **host the table** and **press the right buttons** at the right time.

---

## Why this exists (without buzzwords)

Many systems that run software across lots of machines feel like **corporate headquarters**:

- huge binders of configuration,
- special rituals to deploy anything,
- a strong “do not touch” vibe unless you are a specialist.

Utahnetes is a **counterexample in miniature**: it tries to show that you can coordinate machines with:

- **local discovery** (“who is around me on this network?”),
- **simple signals** (“how busy am I?”),
- **small executable packages** (WASM),

…even if the real world still needs a lot more machinery for serious production.

---

## Things you can do **without** writing a single line of code

These are real, practical roles. They are phrased so you can be useful even if you never open an IDE.

### 1) Be the “lab host” for a demo

**What you do**

- Reserve a room (or a table at home) with power strips.
- Put 2–3 laptops on the same Wi‑Fi (or wired LAN).
- Ask a technical teammate to install Utahnetes on each machine.
- You run the schedule: “We start in two minutes; everyone open a terminal window.”

**Why it matters**

Demos fail for human reasons more often than software reasons. Hosting removes chaos.

**Cool factor**

You become the person who makes “impossible-sounding” distributed demos feel calm and professional.

---

### 2) Run the “cup” machine (the steady presence)

**What you do**

- Pick one computer that stays on and plugged in.
- Ask your technical teammate to start:

  `utahnetes ignite`

- Put a sticky note on it: **“Utahnetes cup — do not sleep.”**

**Why it matters**

In this prototype, other machines discover work opportunities partly by who is present on the network. A steady machine makes experiments repeatable.

**Productivity angle**

This is the same skill as keeping a shared drive online, or keeping a Zoom host machine awake: **reliability beats cleverness**.

---

### 3) Be the “pilot” who pours work at the right moment

**What you do**

- Your teammate hands you a `.wasm` file (or a path to it).
- They tell you: “Wait until two cups are ignited.”
- You run:

  `utahnetes pour --file path\to\thing.wasm`

**Why it matters**

You are practicing **release timing**: coordinating humans and machines so an action happens when the system is ready.

**Cool factor**

You are the person pressing “go” on a distributed system, which is a legitimate leadership skill in tech organizations.

---

### 4) Create a “demo script” your team can repeat

**What you do**

Write a half-page checklist (Google Doc is fine) with:

- prerequisites (same network, Rust installed, repo cloned),
- exact commands (copy/paste),
- expected log lines (“Broadcast heat index…”, “Node listening on…”),
- what “success” looks like (“we saw mDNS events”, “WASM executed”).

**Why it matters**

Repetition turns a novelty into a teaching tool.

---

### 5) Translate between experts and everyone else

**What you do**

In meetings, you use metaphors from this doc:

- “**Heat**” means “how busy the machine feels in this toy model.”
- “**Pour**” means “send a runnable package into the pool.”
- “**Ignite**” means “raise your hand and join the pool.”

**Why it matters**

Most organizational failures are **translation** failures. You do not need to know Rust to prevent confusion.

---

### 6) Organize a “family LAN science night” (safe, bounded)

**What you do**

- Use a **guest Wi‑Fi** network, not your bank laptop.
- Two machines only.
- A technical family member runs `ignite` on both; they pour a harmless WASM demo.

**Why it matters**

It teaches “computers coordinate by messages” without exposing the whole internet.

**Safety note**

Treat this like plugging in a new gadget: **trusted people, trusted network, bounded experiment**.

---

### 7) Be the documentation champion

**What you do**

- Read [When to use Utahnetes](when-to-use.md).
- Write three bullets for your org: “We should / should not explore this because…”

**Why it matters**

Good decisions beat good hype.

---

### 8) Run a “post-incident storytime” (even if nothing broke)

**What you do**

After a demo, ask:

- What surprised us?
- What felt fragile?
- What would we need for real workloads?

Capture answers. That is how prototypes graduate into products—or get retired cleanly.

---

### 9) Pair with a student: you narrate, they operate

**What you do**

- You read logs aloud like sports commentary.
- The student runs commands.
- You ask questions: “What changed in the log right after pour?”

**Why it matters**

Teaching deepens your understanding faster than passive reading.

---

### 10) Build a “press kit” for your hackathon table

**What you do**

Prepare:

- a one-sentence pitch (“local computers discover each other and share tiny runnable modules”),
- a QR code to this repo’s README,
- a printed QR to `docs/homelab-scenarios.md`.

**Cool factor**

Judges and visitors can self-serve context while you talk to someone else.

---

## Things you can do **with a little help** from a technical friend

These tasks are “non-technical friendly,” but someone needs to produce the WASM or verify security for your environment.

### A) Ship a “micro tool” inside your team

Example: a tiny internal utility compiled to WASM (your teammate builds it once).

You coordinate:

- which machine is always-on (`ignite`),
- who is allowed to `pour`,
- what “done” means (logs, output files, etc.).

### B) Create a “demo WASM menu”

Ask your teammate for three `.wasm` files labeled:

- **Green:** always safe (prints hello),
- **Yellow:** touches the network (if any),
- **Red:** lab-only (never run on important machines)

You keep the menu. They keep the builds.

### C) Turn Utahnetes into a teaching prop

Use it in a workshop module titled:

> “Why modern systems obsess over packaging and discovery.”

Utahnetes is small enough that the whole pipeline fits in a short lesson.

---

## Tasks that make you look extremely competent (even if you are new)

### The five-minute “calm expert” routine

1. Confirm everyone is on the **same network segment**.
2. Confirm nobody is on a **captive portal** Wi‑Fi that isolates clients.
3. Confirm **firewall** rules (if any) allow local peer traffic (your teammate handles details).
4. Start `ignite` on the steady machine first.
5. Only then `pour`.

### The “debugging without touching code” routine

Ask your teammate for logs with `RUST_LOG=info` (or `debug`) and look for patterns:

- Did the cup **listen** on an address?
- Did **mDNS** print discovery events?
- Did pour say **“No remote heat samples yet”**? (means timing or network isolation)

Then open [Troubleshooting](troubleshooting.md) and search for the phrase you saw.

---

## What you should **not** expect Utahnetes to do (yet)

Being clear is part of being helpful.

- **Not** a secure multi-tenant cloud by default.
- **Not** a complete replacement for Kubernetes-style operations.
- **Not** guaranteed to move giant applications without extra transport design (WASM pour size is intentionally limited in this prototype).
- **Not** a consumer app your non-technical relatives should install blindly.

---

## How to talk about Utahnetes at a dinner party (without lying)

Good line:

> “It’s a tiny experimental tool that lets computers on the same Wi‑Fi find each other and share small programs, using metaphors from physics as a teaching device.”

Great follow-up:

> “It’s not ‘the future of the cloud’ in a box—it’s a classroom-sized model of ideas the cloud actually uses, like discovery and pub/sub messaging.”

---

## Where to go next

- If you want **capabilities** in plain language: [What is Utahnetes?](what-is-utahnetes.md)
- If you want **decision guidance**: [When to use Utahnetes](when-to-use.md)
- If you want **hands-on steps**: [Installation](installation.md) then [Tutorial: ignite](tutorial-ignite.md)
- If you want **printable host checklists**: [Non-technical checklists](non-technical-checklists.md)

You are not “behind” if you do not read the Rust source. This project is as much about **communication** and **systems intuition** as it is about code.
