use nalgebra::SVector;

fn main() {
    let v1 = SVector::<f32, 3>::new(10.0, 0.0, 0.0);
    let v2 = SVector::<f32, 3>::new(0.5, 0.5, 0.0);
    println!("Hello, world!");
    println!("{}", v1.norm());
    println!("{}", v2.norm());
    println!("{}", v1.dot(&v2) / v1.norm());
}
