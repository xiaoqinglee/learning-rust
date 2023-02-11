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

#[derive(Debug)]
struct AlwaysEqual;

fn use_unit_like_struct() {
    //类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    let a = AlwaysEqual;
    println!("a: {:?}", a);
}

fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}

fn use_debug_macro() {
    //println! 获得参数的 shared ref, 返回 ()
    assert_eq!(println!(), ());

    //dbg! 获得参数的 ownership, 并返回该 ownership
    dbg!(factorial(4));

    let a = AlwaysEqual;
    dbg!(a);

    let a = AlwaysEqual;
    dbg!(&a);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 可以分成多个impl块
impl Rectangle {
    // 允许method和field同名, 实践上常常将同名field设计成私有的, 将同名method设计成pub的, 后者是前者的getter
    fn width(&self) -> u32 {
        self.width
    }

    //&self 实际上是 self: &Self 的缩写
    fn get_width(&self) -> u32 {
        //
        self.width
    }

    //&mut self 实际上是 self: &mut Self 的缩写
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }

    //通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；
    // 这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。

    //Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
    // 方法调用是 Rust 中少数几个拥有这种行为的地方。
    // 它是这样工作的：
    // 当使用 object.something() 调用方法时，
    // Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
}

impl Rectangle {
    //所有在 impl 块中定义的函数被称为关联函数（associated function），
    // 因为它们与 impl 后面命名的类型相关。
    //
    // 我们可以定义不以 self 为第一参数的关联函数（因此不是方法），
    // 因为它们并不作用于一个结构体的实例。
    //
    // 关联函数经常被用作返回一个结构体新实例的构造函数。

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn use_associated_func() {
    //::语法用于关联函数和模块创建的命名空间
    let mut a = Rectangle::new(2, 3);
    println!("a {:?}", a);
    println!("a.width {:?}", a.width);
    println!("a.width() {:?}", a.width());
    println!("a.get_width() {:?}", a.get_width());
    a.set_width(1);
    println!("a.get_width() {:?}", a.get_width());

    let a = Rectangle::new_square(2);
    println!("a {:?}", a)
}

fn main() {
    use_associated_func();
}
