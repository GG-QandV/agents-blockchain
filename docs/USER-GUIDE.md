# μ + Δ-Composer — User Guide

## What is it (30 seconds)

**μ** is a digital wallet-guard for your AI agent. The agent can pay for services
(USDC — stablecoin on Base), but **never sees the wallet keys**. What and whom
it can pay is decided by you — through rules.

**Δ-Composer** is the app where you set those rules: daily limit, list of approved
recipients, threshold above which a payment requires your biometric confirmation.

Every decision is recorded in a tamper-evident append-only journal.

---

## First-time setup (do once)

1. Start the μ-daemon (background process) — it creates a wallet; keys go into
   your device's secure chip (Secure Enclave / StrongBox / TPM).
2. Fund the wallet address with a small amount of USDC on Base.
   ⚠ Keep only pocket-money amounts on μ — like cash in a wallet, not a bank account.
3. Open Δ-Composer and set three rules:

| Rule | Meaning | Advice |
|------|---------|--------|
| Daily limit | Maximum spend per 24 hours (including network fees) | Weekly agent budget ÷ 7 |
| Whitelist | Addresses the agent is allowed to pay | Only services the agent actually uses |
| Confirmation threshold | Above this amount, payment waits for your approval | Set LOW: crypto payments are irreversible |

4. **Adding an address**: paste the address → an identicon and enlarged first/last
   characters appear → cross-check them against the address source → confirm.
   The identicon is always the same for the same address on any device.

5. Activate the ruleset. That's it — the agent can now spend USDC while μ
   enforces your rules.

---

## Day-to-day use

- **Agent wants to pay** → μ checks allowed addresses, daily budget remaining,
  and whether the amount exceeds the confirmation threshold.
- If the amount is below the threshold → payment goes through automatically.
  The agent gets a response within ~2 seconds.
- If the amount is above the threshold → μ shows you a confirmation dialog:
  who is being paid, what amount, what for (label from the whitelist).
  Approve or reject — no other options.

---

## How to check what happened

The journal is an append-only hash chain that records every decision:
- Allowed payments (amount, recipient, time)
- Denials (reason code — wrong address, over budget, owner rejected)
- Owner confirmations

You can view the journal in Δ-Composer or export it as a file for your accountant.

No entry can be silently modified or deleted — each new entry chains to the previous one.

---

## Safety

- **Keys never leave the secure chip.** μ signs transactions internally; the
  agent sends a payment intent, never a raw transaction.
- **Zero gas fees for you** — payments use x402 (EIP-3009) where the facilitator
  pays gas.
- **Rules are local.** μ runs as a local daemon on your machine. No cloud, no
  third-party server.
