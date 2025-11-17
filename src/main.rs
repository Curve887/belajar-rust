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
fn test_variable(){
    let name= "Kurniawan Ade Putra";
    println!("Name: {}", name);
}

#[test]
fn test_mutable_variable(){
    let mut name =  "Kurniawan Ade Putra";
    println!("Hello {}", name);

    name = "Rahdatul Zaqia";
    println!("Hello {}", name);
}

#[test]
fn static_typing(){
    let name =  "Kurniawan Ade Putra";
    println!("Hello {}", name);

    // name = 10;

    println!("Hello {}", name);
}

#[test]
fn shadowing(){
    let name =  "Kurniawan Ade Putra";
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
fn comment(){
    // ini komentar satu baris
    println!("Hello Ade"); // ini komentar 1 baris
}

#[test]
fn explicit () {
    let age: i32 = 20;
    println!("{}", age);
}