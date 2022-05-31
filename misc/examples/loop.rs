fn main() {
    println!("# Simple for");
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("\n");

    println!("# Multipul variable and zip for");
    for (i, j) in (0..10).zip(100..110) {
        print!("{}:{} ", i, j);
    }
    println!("\n");

    println!("# Multipul variable with custom step and zip for");
    for (i, j) in (0..10).zip((100..2000).step_by(100)) {
        print!("{}:{} ", i, j);
    }
    println!("\n");

    println!("# Multipul variable with custom step for");
    for i in 0..10 {
        for j in (100..1100).step_by(100) {
            print!("{}:{} ", i, j);
        }
        println!("");
    }
    println!("\n");
}
