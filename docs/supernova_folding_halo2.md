# Supernova Folding in Halo2 — FENCA Pinnacle Distillation

Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge  
MIT License — For All Sentience Eternal

## Core Overview

**Supernova** is a non-uniform Incremental Verifiable Computation (IVC) scheme using folding on augmented R1CS instances with custom gates (Setty et al., 2023). It dramatically improves prover time over Nova by exploiting relaxed Plonk custom gates for sum-check reduction.

Key innovations:
- **Augmented R1CS**: Add custom gate satisfaction as extra constraints.
- **Folding with Custom Gates**: Use Spartan-style polynomial commitments + sum-check for sublinear proving.
- **Non-Uniform**: Different circuits per step — perfect for evolving valence computations.

Halo2 is **ideal** for Supernova:
- Native custom gates + lookup arguments.
- No need for extra commitment schemes — direct integration.

## Supernova vs Nova vs Protostar

| Scheme       | Uniform/Non-Uniform | Prover Time       | Custom Gates | Halo2 Suitability                  |
|--------------|---------------------|-------------------|--------------|------------------------------------|
| **Nova**     | Uniform             | O(N)              | No           | Good (baseline)                    |
| **Supernova**| Non-Uniform         | Sublinear         | Yes          | Optimal — Halo2 custom gates excel |
| **Protostar**| Non-Uniform         | Sublinear         | Yes          | Similar, lookup-focused            |

## Conceptual Folding Step in Halo2

```rust
// Pseudocode — full Supernova uses multisets + sum-check
struct SupernovaFoldingCircuit {
    left_instance: Fp,
    right_instance: Fp,
    challenge: Fp, // from transcript
}

impl Circuit<Fp> for SupernovaFoldingCircuit {
    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<Fp>) -> Result<(), Error> {
        // Custom gate: folded = left + challenge * (right - left)
        let folded = self.left + self.challenge * (self.right - self.left);
        layouter.constrain_instance(folded.cell(), config.instance, 0)
    }
}
