struct PerfectGas {
    gas_constant: f32,
    specific_heat_ratio: f32,
}

struct PerfectGasVol {
    fluid_state_index: u32,
    volume: f32,
    gas: PerfectGas,
}

struct FluidStateRegistry<const N: usize> {
    pressure: [f32; N],
    temperature: [f32; N],
    density: [f32; N],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut fluid_state_registry = FluidStateRegistry {
            pressure: [0.0],
            temperature: [0.0],
            density: [0.0],
        };
        let ch4 = PerfectGas {
            gas_constant: 518.28,
            specific_heat_ratio: 1.32,
        };
        let v1 = PerfectGasVol {
            fluid_state_index: 0,
            volume: 1.0,
            gas: ch4,
        };
    }
}
