use nalgebra::SVector;

#[derive(Clone, Copy, Debug)]
pub struct PerfectGas {
    pub r_specific: f32,
    pub gamma: f32,
}

#[derive(Debug)]
pub struct PerfectGasVol {
    gas: PerfectGas,
    v: f32,
    p: f32,
    t: f32,
    pub rho: f32,
    pub u: f32,
    h: f32,
    mdot_net: f32,
    qdot_net: f32,
}

impl PerfectGasVol {
    pub fn new(gas: PerfectGas, v: f32, p: f32, t: f32) -> PerfectGasVol {
        let mut vol = PerfectGasVol {
            gas,
            v,
            p,
            t,
            rho: 0.0,
            u: 0.0,
            h: 0.0,
            mdot_net: 0.0,
            qdot_net: 0.0,
        };
        vol.pt_update(p, t);
        return vol;
    }
    fn pt_update(&mut self, p: f32, t: f32) {
        let cp = self.gas.r_specific / (1.0 - 1.0 / self.gas.gamma);
        self.p = p;
        self.t = t;
        self.rho = p / self.gas.r_specific / t;
        self.u = t * cp / self.gas.gamma;
        self.h = cp * t;
    }
    fn ru_update(&mut self, rho: f32, u: f32) {
        let cp = self.gas.r_specific / (1.0 - 1.0 / self.gas.gamma);
        self.rho = rho;
        self.u = u;
        self.t = u / (cp / self.gas.gamma);
        self.p = rho * self.gas.r_specific * self.t;
        self.h = cp * self.t;
    }
}

#[derive(Debug)]
pub struct CompOrifice {
    inlet_index: usize,
    outlet_index: usize,
    cda: f32,
    mdot: f32,
}

impl CompOrifice {
    pub fn new(inlet_index: usize, outlet_index: usize, cda: f32) -> CompOrifice {
        return CompOrifice {
            inlet_index,
            outlet_index,
            cda,
            mdot: 0.0,
        };
    }
    fn interact(&mut self, vols: &mut [PerfectGasVol]) {
        self.mdot = self.cda * (vols[self.inlet_index].p - vols[self.outlet_index].p);
        vols[self.inlet_index].mdot_net -= self.mdot;
        vols[self.outlet_index].mdot_net += self.mdot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_gas_smoke() {
        let ch4 = PerfectGas {
            r_specific: 518.28,
            gamma: 1.32,
        };
        let mut v1 = PerfectGasVol::new(ch4, 1.0, 1e5, 300.0);
        println!("{:?}", v1);
        v1.ru_update(0.5, v1.u);
        println!("{:?}", v1);

        let v2 = PerfectGasVol::new(ch4, 1.0, 10e5, 300.0);
        let mut vols = [v1, v2];
        let mut ori = CompOrifice::new(0, 1, 1e-4);
        ori.interact(&mut vols);
        println!("{:?}", vols);
        println!("{:?}", ori);
    }
}
