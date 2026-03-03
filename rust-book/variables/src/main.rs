fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Tuple
    let tup = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    // Array types

    // Array of zeros
    let zeros = [0; 10];
    println!("Array of zeroes {:?}", zeros);

    let inc = [1, 2, 3];
    println!("inc={:?} inc[0]={1}", inc, inc[0]);
}
