mod safe_wrapper;

fn main() {
    println!("=== Rust + C Complete Integration ===\n");

    let a = 10;
    let b = 5;
    println!("Math Check:");
    println!("  {} + {} = {}", a, b, safe_wrapper::add(a, b));
    println!("  {} - {} = {}", a, b, safe_wrapper::subtract(a, b));
    println!("  {} * {} = {}", a, b, safe_wrapper::multiply(a, b));
    println!();

    let limit = safe_wrapper::system_limit();
    println!("Global Check:");
    println!("  System Limit (from C): {}", limit);
    println!();


    let stack_p = safe_wrapper::Point {
        x: 3.0,
        y: 4.0,
        label: String::from("Stack Point"),
    };

    println!("Struct Check (Stack):");
    println!("  Calculated Distance: {:.2}", stack_p.distance());
    println!();


    println!("Memory Check (Heap):");
    {
        println!("  [Scope Start] Creating SmartPoint...");

        let heap_p = safe_wrapper::SmartPoint::new(15.0, 20.0, "Heap Point");

        println!("  [Usage] SmartPoint X: {:.1}", heap_p.get_x());
        println!("  [Usage] SmartPoint Y: {:.1}", heap_p.get_y());

        println!("  [Scope End] heap_p is about to go out of scope...");
    }

    println!("  [After Scope] Execution continues safe and sound.");
}