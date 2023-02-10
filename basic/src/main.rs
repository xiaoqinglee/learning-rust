fn main() {
    test_fun()
}

//https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn const_() {
    const THREE_HOURS_IN_SECONDS: u64 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);
}

fn scalar() {
    //标量（scalar）类型表示单个值。Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符。

    //那么该使用哪种类型的整型呢？
    // 如果不确定，Rust 的默认形式通常是个不错的选择，整型默认是 i32。
    // isize 和 usize 的主要应用场景是用作某些集合的索引。

    //Rust 的整型字面量
    //下面前4个都是i32
    let a = 98_222; //等于98222
    let a = 0xff;
    let a = 0o77;
    let a = 0b1111_0000;
    let a = b'A'; //u8

    //认浮点类型是 f64，因为在现代的 CPU 中它的速度与 f32 的几乎相同，但精度更高。
    let b = 2.0; //f64
    let b: f32 = 2.0;

    // //cannot add a float to an integer [E0277] no implementation for `{integer} + {float}`
    // let c = 5 + 1.0;

    let c = 10.0 / 3.0; //f64
    println!("c: {}", c);

    let c = 5 / 2; //i32
    println!("c: {}", c);
    let c = 5 % 2; //i32
    println!("c: {}", c);

    let d = true;
    let d: bool = false;
    println!("d: {}", d);

    //Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值，这意味着它可以表示的远远不止是 ASCII。
    let e = 'A'; //char
    println!("e: {}", e);
    let e = '中';
    println!("e: {}", e);
    let e = '😻';
    println!("e: {}", e);
}

fn tuple() {
    let a = (42, 42., true); //(i32, f64, bool)
    let b: (i32, f64, bool) = (42, 3.14, false);

    //模式匹配
    let (x, y, z) = a;
    println!("The value of y is: {}", y);
    println!("type of x: {}", get_type(&x));
    println!("type of y: {}", get_type(&y));
    println!("type of z: {}", get_type(&z));
    println!("b.1: {}", b.0);
    println!("b.2: {}", b.1);
    println!("b.3: {}", b.2);
}

fn array() {
    let a = [11, 22, 33, 44, 55]; //[i32; 5]
    let b = [3; 5]; //[3, 3, 3, 3, 3]

    // //mismatched types [E0308] expected `[i32; 5]`, found `[i32; 2]`
    // let c: [i32; 5] = [3, 5];

    println!("array a: {:?}", a);
    println!("array b: {:?}", b);
    println!("a[0]: {}", a[0]);
    println!("a[4]: {}", a[4]);

    // //thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5',
    // println!("a[5]: {}", a[b.len()]);
}

//cast 在语义上就是有损失的类型转换
fn cast() {
    let mut big_int: i32 = 255;
    println!("255 as u8: {}", big_int as u8); // 255
    println!("256 as u8: {}", (big_int + 1) as u8); //0

    let big_int: i32 = i32::MAX;
    assert_eq!(big_int, (big_int as f32) as i32);

    let big_int = i64::MAX - 1;
    assert_ne!(big_int, (big_int as f32) as i64);
}

// 函数的参数必须标明类型, 编译器不提供参数类型的推断, 这是rust语言的策略.
// 如果返回值不为(), 那么必须标明类型.

fn test_fun() {
    assert_eq!((), fun1(42, 42.0))
}

// 下面几个函数是一样的
fn fun1(var1: i32, var2: f64) {
    println!("var1: {}", var1);
    println!("var2: {}", var2);
}

fn fun2(var1: i32, var2: f64) -> () {
    println!("var1: {}", var1);
    println!("var2: {}", var2);
}

fn fun3(var1: i32, var2: f64) {
    println!("var1: {}", var1);
    println!("var2: {}", var2);
    ()
}

fn fun4(var1: i32, var2: f64) {
    {
        println!("var1: {}", var1);
        println!("var2: {}", var2);
        ()
    }
}

fn fun5(var1: i32, var2: f64) {
    println!("var1: {}", var1);
    println!("var2: {}", var2);
    {}
}

fn fun6(var1: i32, var2: f64) {
    println!("var1: {}", var1);
    println!("var2: {}", var2);
    {
        ()
    }
}
