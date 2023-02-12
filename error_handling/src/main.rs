use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn open_or_create_file_v1() {
    let input = "./input.txt";
    let f = match File::open(input) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(input) {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            any_other_kind => panic!(
                "Problem opening the file: [kind:{}, value:{}]",
                any_other_kind, e
            ),
        },
    };
    dbg!(f);
}

fn open_or_create_file_v2() {
    let input = "./input.txt";
    let f = File::open(input).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(input)
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!(
                "Problem opening the file: [kind:{}, value:{}]",
                error.kind(),
                error
            )
        }
    });
    dbg!(f);
}

fn unwrap_or_expect() {
    let not_existing_file_path = "/not_existing";

    //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
    File::open(not_existing_file_path).unwrap();

    //thread 'main' panicked at 'Ops..: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
    File::open(not_existing_file_path).expect("Ops..");
}

fn read_string_from_file_v1() -> Result<String, io::Error> {
    let mut f = match File::open("./input.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    //If successful, this function returns the number of bytes which were read and appended to buf.
    match f.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(s)
}

fn read_string_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("./input.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_string_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("./input.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

#[derive(Debug)]
struct MyErrorFoo(String);
#[derive(Debug)]
struct MyErrorBar(String);
#[derive(Debug)]
struct MyErrorBaz(String);

// //conflicting implementations of trait `From<MyErrorFoo>` for type `MyErrorFoo` [E0119]
// // Note: conflicting implementation in crate `core`:,
// // impl<T> From<T> for T;
// impl From<MyErrorFoo> for MyErrorFoo {
//     fn from(value: MyErrorFoo) -> Self {
//         println!("MyErrorFoo::from(value: MyErrorFoo) called()");
//         MyErrorFoo(value.0)
//     }
// }

impl From<MyErrorBar> for MyErrorFoo {
    fn from(value: MyErrorBar) -> Self {
        println!("MyErrorFoo::from(value: MyErrorBar) called()");
        MyErrorFoo(value.0)
    }
}

impl From<MyErrorBaz> for MyErrorFoo {
    fn from(value: MyErrorBaz) -> Self {
        println!("MyErrorFoo::from(value: MyErrorBaz) called()");
        MyErrorFoo(value.0)
    }
}

fn use_From_trait() -> Result<String, MyErrorFoo> {
    let secret_number = rand::thread_rng().gen_range(1..=5);

    if secret_number == 1 {
        Result::Ok(String::from("Ok string"))
    } else if secret_number == 2 {
        Result::Err(MyErrorFoo(String::from("MyErrorFoo string")))
    } else if secret_number == 3 {
        Result::Err(MyErrorFoo(String::from("MyErrorFoo string")))?
    } else if secret_number == 4 {
        Result::Err(MyErrorBar(String::from("MyErrorBar string")))?
    } else {
        Result::Err(MyErrorBaz(String::from("MyErrorBaz string")))?
    }
}

fn use_unwrapped_ok_value() -> Result<String, MyErrorFoo> {
    let secret_number = rand::thread_rng().gen_range(1..=2);

    //ok_value is String
    let ok_value = if secret_number == 1 {
        Result::Ok::<String, MyErrorFoo>(String::from("Ok string"))?
    } else {
        Result::Err(MyErrorBar(String::from("MyErrorBar string")))?
    };
    println!("didn't return half way.");
    Result::Ok(ok_value)
}

// fn main() {
//     // dbg!(read_string_from_file_v1());
//     // dbg!(read_string_from_file_v2());
//     // dbg!(read_string_from_file_v3());
//     // dbg!(use_From_trait());
//     dbg!(use_unwrapped_ok_value());
// }

//main 函数是特殊的，其必须返回什么类型是有限制的。
// main 函数的一个有效的返回值是 ()，同时出于方便，另一个有效的返回值是 Result<T, E>。
fn main() -> Result<(), Box<dyn Error>> {
    File::open("./input.txt")?;
    println!("didn't return half way.");
    Ok(())
}
