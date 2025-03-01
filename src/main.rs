fn main() {
    println!("Hello, world!");
    let ans = sum(1, 2);
    println!("{}", ans);
    println!("{}", is_even(123));

    let name = String::from("suryakant");
    println!("{}", name);
    let mut v = vec![1, 23, 34, 54];
    println!("{:?}", v);
    println!("{:?}", v.len()); //length of vector
    v.push(234);
    println!("{:?}", v);
    println!("{:?}", v.len())
}
fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
fn is_even(no: u32) -> bool {
    return no % 2 == 0;
}
