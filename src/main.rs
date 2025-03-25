use nalgebra::{SMatrix, SVector, Vector2};

fn xdot(x: &Vector2<f32>) -> Vector2<f32> {
    let w: f32 = 1.0;
    let z: f32 = 0.5;
    return Vector2::new(x[1], -w.powi(2) * x[0] - 2.0 * z * w * x[1]);
}

// fn forward_euler_step(
//     xdot: fn(&Vector2<f32>) -> Vector2<f32>,
//     x: &Vector2<f32>,
//     dt: f32,
// ) -> Vector2<f32> {
//     return x + xdot(x) * dt;
// }

fn rk4_step<const N: usize>(
    xdot: fn(&SVector<f32, N>) -> SVector<f32, N>,
    x: &SVector<f32, N>,
    dt: f32,
) -> SVector<f32, N> {
    let k1 = xdot(x);
    let x2 = x + 0.5 * k1 * dt;
    let k2 = xdot(&x2);
    let x3 = x + 0.5 * k2 * dt;
    let k3 = xdot(&x3);
    let x4 = x + k3 * dt;
    let k4 = xdot(&x4);
    return x + (k1 + 2. * k2 + 2. * k3 + k4) * dt / 6.0;
}

// fn heun_step()

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
