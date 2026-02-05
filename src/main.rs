// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("=== Rust Bindgen Example ===");
    println!("Calling C functions from Rust using bindgen\n");

    // Call the C functions through the bindings
    unsafe {
        let a = 10;
        let b = 5;

        let sum = add(a, b);
        println!("add({}, {}) = {}", a, b, sum);

        let diff = subtract(a, b);
        println!("subtract({}, {}) = {}", a, b, diff);

        let product = multiply(a, b);
        println!("multiply({}, {}) = {}", a, b, product);
    }

    println!("\nSuccessfully called C functions from Rust!");
}
