fn main() {
    println!("Hello, world!");

    another_function(88);
    print_labeled_measurement(22, 'g');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
