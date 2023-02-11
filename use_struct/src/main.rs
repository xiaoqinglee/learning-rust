#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn use_struct() {
    let u1 = User {
        username: String::from("u1"),
        email: String::from("u1@example.com"),
        active: true,
        sign_in_count: 42,
    };
    println!("u1: {:?}", u1);

    let username = String::from("u2");
    let email = String::from("u2@example.com");
    let u2 = User {
        username,
        email,
        active: false,
        sign_in_count: 43,
    };
    println!("u2: {:?}", u2);

    let u3 = User {
        username: String::from("u3"),
        sign_in_count: 44,
        ..u1
    };
    println!("u3: {:?}", u3);

    // // u1.email 没有实现Copy trait, 已经被transfer了
    // //borrow of partially moved value: `u1` [E0382]
    // println!("u1: {:?}", u1);

    // //borrow of moved value: `u1.email` [E0382]
    // println!("u1.email: {:?}", u1.email);

    //bool是标量类型, 实现了Copy trait, 所以没有被transfer, 而是被copy了
    println!("u1.active: {:?}", u1.active);

    // //如果整个结构体变量不是mutable的, 那么所有字段都不是mutable的, 不支持在字段级别设置是否mutable.
    // //Cannot assign to field of immutable binding [E0594]
    // u1.sign_in_count = 142;

    // //use of partially moved value: `u1` [E0382]
    // let mut u4 = u1;

    let mut u5 = u2;
    u5.username = String::from("u5");
    println!("u5: {:?}", u5);
}

struct Color(f64, u32, u32);
struct Point(f64, u32, u32);

fn use_tuple_struct() {
    let black = Color(3.33, 10, 5);
    let origin = Point(0.00, 0, 0);

    // //mismatched types [E0308] expected struct `Color`, found struct `Point`
    // let white: Color = origin;

    // //mismatched types [E0308] expected struct `Point`, found tuple
    // let (x, y, z) = origin;

    let Color(x, y, z) = black;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}

fn main() {
    use_tuple_struct()
}
