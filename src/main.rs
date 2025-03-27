pub mod numerical_methods;
pub mod perfect_gas;
use nalgebra::{SMatrix, Vector2, Vector4};
use numerical_methods::rk4_step;
use perfect_gas as pg;

fn xdot(x: &Vector2<f32>) -> Vector2<f32> {
    let w: f32 = 1.0;
    let z: f32 = 0.5;
    return Vector2::new(x[1], -w.powi(2) * x[0] - 2.0 * z * w * x[1]);
}

struct Model {
    vols: [pg::PerfectGasVol],
    oris: [pg::CompOrifice],
}
impl Model {
    fn xdot(x: )
}

fn main() {
    let dt: f32 = 0.01;
    const NFRAMES: usize = 600;
    let ch4 = pg::PerfectGas {
        r_specific: 518.28,
        gamma: 1.32,
    };
    let vols = [
        pg::PerfectGasVol::new(ch4, 100.0, 100e5, 300.0),
        pg::PerfectGasVol::new(ch4, 100.0, 1e5, 300.0),
    ];
    let oris = [pg::CompOrifice::new(0, 1, 1e-5)];
    let mut x = Vector4::<f32>::new(vols[0].rho, vols[0].u, vols[1].rho, vols[1].u);
    for i in 0..NFRAMES {
        x = rk4_step(, x, dt)
    }
    // let mut x = Vector2::<f32>::new(1.0, 0.0);
    // let mut res = SMatrix::<f32, 2, NFRAMES>::zeros();
    // for i in 0..NFRAMES {
    //     res.set_column(i, &x);
    //     x = rk4_step(xdot, &x, dt);
    // }
    // println!("{}", res.transpose());
    // println!("{}", x);
}
