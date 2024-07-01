#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {

    println!("Hello world!");

    let n = 1024;
    let m = 512;

    let x;

    x = n + m;
    // Print x
    println!("x = {}", x);

    let k = 10;
    // loop from 0 to k
    for i in 0..k {
        // Print i
        println!("i = {}", i);
    }

    let matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    // Print matrix
    println!("matrix = {:?}", matrix);

    let m1 = matrix.0;
    // Print m1
    println!("m1 = {}", m1);
}