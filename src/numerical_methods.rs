use nalgebra::SVector;

pub fn forward_euler_step<const N: usize>(
    xdot: fn(&SVector<f32, N>) -> SVector<f32, N>,
    x: &SVector<f32, N>,
    dt: f32,
) -> SVector<f32, N> {
    return x + xdot(x) * dt;
}

pub fn rk4_step<const N: usize>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector2;

    fn xdot(x: &Vector2<f32>) -> Vector2<f32> {
        let w: f32 = 1.0;
        let z: f32 = 0.5;
        return Vector2::new(x[1], -w.powi(2) * x[0] - 2.0 * z * w * x[1]);
    }

    #[test]
    fn rk4_step_smoketest() {
        let dt: f32 = 0.1;
        let x = Vector2::<f32>::new(1.0, 0.0);
        let x1 = rk4_step(xdot, &x, dt);
        assert!(x1[0] < 1.0);
        assert!(x1[1] < 0.0);
    }

    #[test]
    fn forward_euler_step_smoketest() {
        let dt: f32 = 0.1;
        let mut x = Vector2::<f32>::new(1.0, 0.0);
        x = forward_euler_step(xdot, &x, dt);
        let x2 = forward_euler_step(xdot, &x, dt);
        assert!(x2[0] < 1.0);
        assert!(x2[1] < 0.0);
    }
}
