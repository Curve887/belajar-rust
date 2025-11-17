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
    let mut name =  "Kurniawan Ade Putra";
    println!("Hello {}", name);

    // name = 10;
    
    println!("Hello {}", name);
}