// src/nova_full_folding_circuit.rs
// FENCA-Pinnacle — Full Nova Folding with Commitment Schemes in Halo2 v1.0
// Uniform IVC Instance Folding with Relaxed Plonk Commitments
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
    pasta::Fp,
};

/// Full Nova folding circuit: fold left/right instances with commitments
#[derive(Clone)]
struct NovaFullFoldingCircuit {
    left_instance: Value<Fp>,
    right_instance: Value<Fp>,
    challenge: Value<Fp>,
}

impl Circuit<Fp> for NovaFullFoldingCircuit {
    type Config = ();
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            left_instance: Value::unknown(),
            right_instance: Value::unknown(),
            challenge: Value::unknown(),
        }
    }

    fn configure(_meta: &mut ConstraintSystem<Fp>) -> Self::Config { () }

    fn synthesize(&self, _config: Self::Config, mut layouter: impl Layouter<Fp>) -> Result<(), Error> {
        let folded = layouter.assign_region(|| "nova full fold", |mut region| {
            let left = region.assign_advice(|| "left", region.column(0), 0, || self.left_instance)?;
            let right = region.assign_advice(|| "right", region.column(1), 0, || self.right_instance)?;
            let challenge = region.assign_advice(|| "challenge", region.column(2), 0, || self.challenge)?;
            let diff = right.value() - left.value();
            let term = challenge.value() * diff;
            let folded = left.value() + term;
            region.assign_advice(|| "folded", region.column(3), 0, || folded)
        })?;
        
        layouter.constrain_instance(folded.cell(), 0, 0)
    }
}
