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

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 800;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;

    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    /*
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
     */

    let (a, b, c) = data; // destructuring
    println!("{} {} {}", a, b, c);

    data.0 = 20; // mutable
    data.1 = 20.5; // mutable
    data.2 = false; //mutable   
    println!("{:?}", data);
}

fn unit() {
    println!("hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b: i32 = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length: usize = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    // ARRAY DI DALAM ARRAY
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [3, 4, 5]];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    // ARRAY DI DALAM ARRAY
    const MINIMUM: i32 = 10; // untuk nama variabel constant biasanya pakai huruf besar semua
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    let ade = 1;

    {
        println!("ade di dalam scope: {}", ade);

        let kurniawan = 2;
        println!("kurniawan di dalam scope: {}", kurniawan);
    }

    // println!("kurniawan di luar scope: {}", kurniawan); // error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("kurniawan");
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Ade");
    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name: &str = "  Kurniawan Ade Putra    ";
    let trim: &str = name.trim();

    println!("'{}'", name);
    println!("'{}'", trim);

    // let mut username: &str = "Ade";
    // username = "Zaqia";
    // println!("{}", username);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Kurniawan Ade");
    println!("{}", name);

    name.push_str(" Putra");
    println!("{}", name);

    let budi = name.replace("Ade", "budi");
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    // println!("{}", b); // error karena b sudah tidak valid di sini
    println!("{}", a);
}
#[test]
fn data_copy() {
    let a = 10;
    let mut b = a; // data di copy

    b = 20;
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Ade");
    println!("{}", name1);

    let name2: String = name1; // ownership berpindah dari name1 ke name2
    println!("{}", name2);
    // println!("{}", name1); // error karena name1 sudah tidak valid di sini
}

#[test]
fn clone() {
    let name1 = String::from("Ade");
    let name2 = name1.clone(); // data di clone
    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 10 {
        println!("lulus!");
    } else if value >= 6 {
        println!("Not bad!");
    } else if value >= 3 {
        println!("bad!");
    } else {
        println!("Very Bad!");
    }
}
