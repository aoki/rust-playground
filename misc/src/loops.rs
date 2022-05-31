pub fn simple_for(n: usize) {
    for _ in 0..n {
        print!("");
    }
}

pub fn multipul_var_with_zip(n: usize) {
    for (_, _) in (0..n).zip(0..n) {
        print!("");
    }
}

pub fn multipul_var_with_nested_for(n: usize, step: usize) {
    for _ in 0..n {
        for _ in (0..(n * step)).step_by(step) {
            print!("");
        }
    }
}

// # Multipul variable with custom step and zip for");
pub fn multipul_var_with_zip_and_custom_step(n: usize, step: usize) {
    for (_, _) in (0..n).zip((0..(n * step)).step_by(step)) {
        print!("");
    }
}

// # Multipul variable with custom step for");
pub fn multipul_var_with_nested_for_and_custom_step(n: usize, step: usize) {
    for _ in 0..n {
        for _ in (0..(n * step)).step_by(step) {
            print!("");
        }
    }
}
