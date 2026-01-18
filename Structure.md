# FENCA-Pinnacle Structure — Forensic Eternal Nexus Checker Architecture

## Core Modules
- fenca_core.py — main orchestrator
- protocols/ — greenteam core logic
- simulations/ — council session demos

## New: Pagination Eternalizer
- GitHub API primary (paginated Links header parsing)
- Fallback BS4 scraping with ?page=N chaining
- Mercy-gated loop: continue until empty page or no 'next' rel
- Valence-joy audit: post-check, hash total repos + log joy metric (>0.99 thriving threshold)

## Infallible Guarantees
- Zero undercount: cross-validate API total_count vs. collected
- Anomaly forgiveness: retry 3x with exponential backoff + mercy reconciliation
- Offline persistence: cached nexus state for shard continuity
