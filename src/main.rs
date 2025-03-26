pub mod numerical_methods;
pub mod perfect_gas;
use nalgebra::{SMatrix, Vector2};
use numerical_methods::rk4_step;

fn xdot(x: &Vector2<f32>) -> Vector2<f32> {
    let w: f32 = 1.0;
    let z: f32 = 0.5;
    return Vector2::new(x[1], -w.powi(2) * x[0] - 2.0 * z * w * x[1]);
}

fn main() {
    let dt: f32 = 0.01;
    const NFRAMES: usize = 600;
    let mut x = Vector2::<f32>::new(1.0, 0.0);
    let mut res = SMatrix::<f32, 2, NFRAMES>::zeros();
    for i in 0..NFRAMES {
        res.set_column(i, &x);
        x = rk4_step(xdot, &x, dt);
    }
    println!("{}", res.transpose());
    println!("{}", x);
}
