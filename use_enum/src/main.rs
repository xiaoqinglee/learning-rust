#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//enum上可以定义关联函数
impl IpAddr {
    fn new_v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        Self::V4(a, b, c, d)
    }
    fn new_v6(addr: &str) -> IpAddr {
        Self::V6(String::from(addr))
    }
    fn print(&self) {
        println!("{:?}", *self)
    }
}

// //Option的定义
// enum Option<T> {
//     Some(T),
//     None,
// }

fn use_enum_() {
    let a: IpAddrKind = IpAddrKind::V4;
    dbg!(&a);

    let a: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let b: IpAddr = IpAddr::V6(String::from("::1"));
    dbg!(&a);
    dbg!(&b);

    let a = IpAddr::new_v4(127, 0, 0, 1);
    let b = IpAddr::new_v6("::1");
    a.print();
    b.print();

    let a: Option<u8> = Some(5);
    let b: Option<u8> = None;
    dbg!(&a);
    dbg!(&b);
}

#[derive(Debug)]
struct ReleaseYear(String); //tuple struct

enum Coin {
    OneFen,
    FiveFen,
    OneJiao,
    FiveJiao,
    OneYuan(ReleaseYear),
}

fn match_coin(coin: Coin) -> u8 {
    match coin {
        Coin::OneFen => 1,
        Coin::FiveFen => 5,
        Coin::OneJiao => 10,
        Coin::FiveJiao => {
            println!("金色幸运币");
            50
        }
        Coin::OneYuan(release_year) => {
            println!("发行时间: {:?}", release_year);
            100
        }
    }
}

fn plus_one_or_do_nothing(optional: Option<i32>) -> Option<i32> {
    match optional {
        Some(inner) => Some(inner + 1),
        None => None,
    }
}

#[derive(Debug)]
enum Language {
    Lisp,
    C,
    Java,
    Go,
    Python,
    Erlang,
    Rust,
}

fn match_language_v1(lang: Language) -> String {
    match lang {
        Language::C => String::from("c"),
        Language::Rust => String::from("rust"),
        other => format!("other: {:?}", other),
    }
}

fn match_language_v2(lang: Language) -> &'static str {
    match lang {
        Language::C => "c",
        Language::Rust => "rust",
        _ => "others",
    }
}

fn irrefutable_or_refutable() {
    //irrefutable
    let s = String::from("s");

    let optional = Some(42);

    //refutable: let-else
    let Some(value1) = optional else{
        panic!("match failed.")
    };
    //The scope of name bindings is the main thing that makes this different from match or if let-else expressions.
    println!("value1: {}", value1);

    //refutable: if-let
    if let Some(value2) = optional {
        println!("match succeeded: value2 {}", value2);
    } else {
        panic!("match failed.")
    }
    // //cannot find value `value2` in this scope [E0425]
    // println!("value: {}", value2);

    let mut optional: Option<i32> = Some(1);
    //refutable: while-let
    while let Some(value3) = optional {
        if value3 > 10 {
            optional = None
        } else {
            dbg!(value3);
            optional = Some(value3 + 1)
        }
    }
    // //cannot find value `value3` in this scope [E0425]
    // println!("value: {}", value3);
}

fn use_pattern_matching() {
    dbg!(match_coin(Coin::OneFen));
    dbg!(match_coin(Coin::FiveJiao));
    dbg!(match_coin(Coin::OneYuan(ReleaseYear(String::from("2008")))));

    let a = Option::None::<i32>;
    let b: Option<u8> = Option::None;
    let c: Option<i32> = Option::None;
    // //mismatched types [E0308] expected `i32`, found `u8` Note: expected enum `Option<i32>` found enum `Option<u8>`
    // println!("a == b: {}", a == b)
    println!("a == c: {}", a == c); //a == c: true

    dbg!(match_language_v1(Language::C));
    dbg!(match_language_v1(Language::Rust));
    dbg!(match_language_v1(Language::Go));

    dbg!(match_language_v2(Language::C));
    dbg!(match_language_v2(Language::Rust));
    dbg!(match_language_v2(Language::Go));
}

fn main() {
    use_pattern_matching();
}
