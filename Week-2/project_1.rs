fn main() {
    let p: f64 = 520_000_000.0; // Principal
    let r: f64 = 10.0;          // Rate per annum
    let n: i32 = 5;             // Number of years

   
    let a: f64 = p * (1.0 + (r / 100.0)).powi(n);


    let ci: f64 = a - p;

    println!("Principal (P): N{:.2}", p);
    println!("Rate (R): {}%", r);
    println!("Time (n): {} years", n);
    println!("Amount (A): N{:.2}", a);
    println!("Compound Interest (CI): N{:.2}", ci);
}