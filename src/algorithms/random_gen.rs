use rand::Rng;

pub fn generate_normal_error(mean: f64, sigma : f64) -> f64 {
    
    let mut rng = rand::thread_rng();

    // Function for the Box-Muller transformation
    fn box_muller(u1: f64, u2: f64) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * pi * u2).cos();
        let z2 = (-2.0 * u1.ln()).sqrt() * (2.0 * pi * u2).sin();
        (z1, z2)
    }

    // Generates two random numbers in the interval (0, 1)
    let u1: f64 = rng.gen_range(0.0..1.0);
    let u2: f64 = rng.gen_range(0.0..1.0);

    // Box-Muller transformation to generate a normal value
    let (z1, _z2) = box_muller(u1, u2);
    //let sigma = sigma.powi(2);
    // Returns the normal value adjusted by the mean and standard deviation
    mean + sigma * z1
}