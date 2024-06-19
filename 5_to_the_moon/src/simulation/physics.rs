/// Height in meters
/// Size in m^2
/// Velocity in m/s
/// Returns force in Newtons
fn estimate_drag_force(h: f64, size: f64, velocity: f64) -> f64 {
    let density = estimate_atmosphere_density(h);
    let k = (density * size) / 2.0;
    k * (velocity.powf(2.0))
}

/// Height in meters
/// Returns density in kg/m^3
fn estimate_atmosphere_density(h: f64) -> f64 {
    1.3 * (-h / 7000.0).exp()
}

/// Height in meters
/// Mass in kg
/// Returns force in Newtons
fn estimate_gravitational_force(h: f64, mass: f64) -> f64 {
    let earth_mass = 5.972e24;
    let g = 6.674e-11;
    let sea_level_height = 6_371_146_f64;
    let r = sea_level_height + h;

    (g * earth_mass * mass) / (r.powf(2.0))
}
