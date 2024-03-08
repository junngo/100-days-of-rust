fn main() {

    // 1.mut keyword to make the mutable variable
    let mut x = 100;
    println!("{}", x);
    x = 200;
    println!("{}", x);

    // 2.shadowing
    let y = 300;
    println!("{}", y);
    let y = 400;
    println!("{}", y);

    // 3.Makes the error to test the immutable.
    // let a = 500;
    // a = 600;

    // 4.When the variable type is different,
    //   It is an error in the mut.
    // let mut b = "   ";
    // println!("{}", b);
    // b = b.len();
    // println!("{}", b);

    // 5.When the variable type is different,
    //   It is not an error in the shadowing
    // let c = " ";
    // println!("{}", c);
    // let c = 1000;
    // println!("{}", c);
}
