fn main() {
    println!("Hello, world!");
    println!("Hello, Ade!");
    println!("Hello, Zaqia!");
}

#[test]
fn hello_test() {
    println!("Hello test");
}

#[test]
fn test_variable() {
    let name = "Kurniawan Ade Putra";
    println!("Name: {}", name);
}

#[test]
fn test_mutable_variable() {
    let mut name = "Kurniawan Ade Putra";
    println!("Hello {}", name);

    name = "Rahdatul Zaqia";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Kurniawan Ade Putra";
    println!("Hello {}", name);

    // name = 10;

    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Kurniawan Ade Putra";
    println!("Hello {}", name);

    // name = 10;
    let name = 10;
    println!("Hello {}", name);
}

/*
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
*/
#[test]
fn comment() {
    // ini komentar satu baris
    println!("Hello Ade"); // ini komentar 1 baris
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8; // akan overflow
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}

#[test]
fn augmanted_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let result: bool = 10 > 20;
    println!("{}", result);
}
