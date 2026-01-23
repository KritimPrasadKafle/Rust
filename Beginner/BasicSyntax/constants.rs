fn main() {
    const PI: f64 = 3.14159;
    const E: f64 = 2.71828;
    const RADIUS: f64 = 7.0;
    
    let area = PI * RADIUS * RADIUS;
    let circumference = 2.0 * PI * RADIUS;
    let growth = E.powf(RADIUS);  // Example exponential growth
    
    println!("Area: {}", area);
    println!("Circumference: {}", circumference);
    println!("e^{}: {}", RADIUS, growth);
}