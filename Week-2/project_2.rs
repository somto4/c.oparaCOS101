// Project II: Sales Record Calculator
// This program calculates the SUM and the AVERAGE of a sales record.

fn main() {
    // Step 1: Store the amount for each item in an array
    let amounts: [f64; 5] = [
        450000.00,  
        1500000.00,  
        750000.00,   
        2850000.00,  
        250000.00,   
    ];

    // Step 2: Add up all the amounts using a loop
    let mut total: f64 = 0.0;
    for amount in amounts.iter() {
        total = total + amount;
    }

    // Step 3: Find the average
    let number_of_items: f64 = amounts.len() as f64;
    let average: f64 = total / number_of_items;

    // Step 4: Print the results
    println!("Total Sum: N{:.2}", total);
    println!("Average: N{:.2}", average);
}