pub mod numerical_methods;
pub mod perfect_gas;
use nalgebra::Vector4;
use perfect_gas as pg;

fn main() {
    let dt: f32 = 0.1;
    const NFRAMES: usize = 100;
    let ch4 = pg::PerfectGas {
        r_specific: 518.28,
        gamma: 1.32,
    };
    let mut vols = [
        pg::PerfectGasVol::new(ch4, 1.0, 100e5, 300.0),
        pg::PerfectGasVol::new(ch4, 1.0, 1e5, 300.0),
    ];
    let mut oris = [pg::CompOrifice::new(0, 1, 1e-4)];
    let mut x = Vector4::<f32>::new(vols[0].rho, vols[0].u, vols[1].rho, vols[1].u);
    for _ in 0..NFRAMES {
        vols[0].ru_update(x[0], x[1]);
        vols[0].mdot_net = 0.0;
        vols[0].qdot_net = 0.0;
        vols[1].ru_update(x[2], x[3]);
        vols[1].mdot_net = 0.0;
        vols[1].qdot_net = 0.0;

        oris[0].interact(&mut vols);
        let xdot = Vector4::<f32>::new(
            vols[0].mdot_net / vols[0].v,
            (vols[0].qdot_net - vols[0].mdot_net) / vols[0].rho / vols[0].v,
            vols[1].mdot_net / vols[1].v,
            (vols[1].qdot_net - vols[1].mdot_net) / vols[1].rho / vols[1].v,
        );
        x += xdot * dt;
        println!(
            "p1:{} p2:{} mdot:{}",
            vols[0].p / 1e5,
            vols[1].p / 1e5,
            oris[0].mdot,
        );
    }
}
