// Project III: TV Depreciation Calculator
// This program finds the value of a TV after it loses value (depreciates) over time.

fn main() {
    // Step 1: Set up our known values
    let principal: f64 = 210000.0; // P = original cost of the TV (N210,000)
    let rate: f64 = 5.0;           // R = depreciation rate per year (5%)
    let years: i32 = 3;            // n = number of years

    // Step 2: Calculate the value year by year using a loop
    // Depreciation means the value goes DOWN each year, so we SUBTRACT
    // instead of adding (unlike compound interest, which adds)

    let mut value: f64 = principal; // start with the original price

    let mut year = 1;
    while year <= years {
        value = value - (value * rate / 100.0); // subtract 5% each year
        println!("After year {}: N{:.2}", year, value);
        year = year + 1;
    }

    // Step 3: Print the final result
    println!("");
    println!("Original Price (P): N{:.2}", principal);
    println!("Value of TV after {} years: N{:.2}", years, value);
}