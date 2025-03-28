#[derive(Clone, Copy, Debug)]
pub struct PerfectGas {
    pub r_specific: f32,
    pub gamma: f32,
}

#[derive(Debug)]
pub struct PerfectGasVol {
    gas: PerfectGas,
    pub v: f32,
    pub p: f32,
    t: f32,
    pub rho: f32,
    pub u: f32,
    pub h: f32,
    pub mdot_net: f32,
    pub qdot_net: f32,
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
    pub fn pt_update(&mut self, p: f32, t: f32) {
        let cp = self.gas.r_specific / (1.0 - 1.0 / self.gas.gamma);
        self.p = p;
        self.t = t;
        self.rho = p / self.gas.r_specific / t;
        self.u = t * cp / self.gas.gamma;
        self.h = cp * t;
    }
    pub fn ru_update(&mut self, rho: f32, u: f32) {
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
    pub mdot: f32,
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
    pub fn interact(&mut self, vols: &mut [PerfectGasVol]) {
        let i_up: usize;
        let i_dn: usize;
        if vols[self.inlet_index].p > vols[self.outlet_index].p {
            i_up = self.inlet_index;
            i_dn = self.outlet_index;
        } else {
            i_up = self.outlet_index;
            i_dn = self.inlet_index;
        }
        let y = vols[i_up].gas.gamma;
        let pr_crit = f32::powf(2.0 / (y + 1.0), y / (y - 1.0));
        let pr = f32::max(vols[i_dn].p / vols[i_up].p, pr_crit);
        let rho = vols[i_up].rho;
        self.mdot = self.cda
            * f32::sqrt(
                2.0 * y / (y - 1.0)
                    * rho
                    * vols[i_up].p
                    * (pr.powf(2.0 / y) - pr.powf((y + 1.0) / y)),
            );
        vols[i_up].mdot_net -= self.mdot;
        vols[i_up].qdot_net -= self.mdot * vols[i_up].h;
        vols[i_dn].mdot_net += self.mdot;
        vols[i_dn].qdot_net += self.mdot * vols[i_up].h;
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
