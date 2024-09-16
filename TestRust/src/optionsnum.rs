pub fn test() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = Some(5);
    let z: Option<i32> = None;

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    let sum = x.unwrap_or(0) + y.unwrap_or(0) + z.unwrap_or(0);

    println!("sum is {:?}", sum);
}
