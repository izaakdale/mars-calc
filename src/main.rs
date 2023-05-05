
fn main() {
    let mut input = String::new();
    let _result = std::io::stdin().read_line(&mut input);

    let mut mars_weight = calculate_weight_on_mars(80.0);
    mars_weight = mars_weight * 1000.0;
    println!("Your weight on Mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
