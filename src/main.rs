fn main() {
    println!("Hello, world!");

    // number
    let ans = sum(1, 2);
    println!("{}", ans);
    println!("{}", is_even(123));
    // String
    let name = String::from("suryakant");
    println!("{}", name);

    // vector
    let mut v = vec![1, 23, 34, 54];
    println!("{:?}", v);
    println!("{:?}", v.len()); //length of vector
    v.push(234);
    println!("{:?}", v);
    println!("{:?}", v.len());

    //array

    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{:?}", arr.len());

    // constional statement
    let x = 90;
    let is_odd = is_odd(x);
    if is_odd {
        println!("{} is odd", x)
    } else {
        println!("{} is even", x)
    }

    //loop
    for i in 0..10 {
        println!("no is {} ", i)
    }

    //program to find the first name from a full name

    let fullname = String::from("Suryakant das");
    let firstname = find_first_name(fullname);
    println!("firstname is {}", firstname);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
fn is_even(no: u32) -> bool {
    return no % 2 == 0;
}

fn is_odd(no: u32) -> bool {
    return no % 2 != 0;
}

fn find_first_name(name: String) -> String {
    let mut fname = String::from("");

    for c in name.chars() {
        if c == ' ' {
            break;
        } else {
            fname.push(c);
        }
    }
    return fname;
}
