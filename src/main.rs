use std::io;

fn main() {
    println!("Enter your Weight on The Earth(kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f64 = input.trim().parse().unwrap();
    println!("Your Weight on The Earth: {}kg", input);
    
    let mars_weight = calculate_weight_on_mars(weight);

    println!("Your Weight on Mars: {:.2}kg", mars_weight);
    
}

fn calculate_weight_on_mars( _weight: f64)-> f64{
    (_weight / 9.81) * 3.711
     


    
}