fn shallow_transfer_vs_deep_clone() {
    let s1 = String::from("s1");
    //shallow, 栈上数据被move
    let s2 = s1;

    // //borrow of moved value: `s1` [E0382]
    // println!("s1: {}", s1);

    let s1 = String::from("s1");
    //deep, 栈上数据和堆上数据都被clone
    let s2 = s1.clone();

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn shallow_copy_vs_shallow_transfer() {
    //如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型实现 Copy trait。

    //任何一组简单标量值的组合都可以实现 Copy.
    //四种标量类型(integer, bool, float, char)和仅由四种标量类型作为元素的tuple和array实现了Copy trait

    let a = [11, 22, 33, 44];
    let b = a;
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    let a = (10, 42.0, '中');
    let b = a;
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

fn shared_ref_vs_mutable_ref() {
    let s1 = String::from("s1");
    let p1_s1 = &s1; // p1_s1 is &String
    println!("s1: {:?}", s1);
    println!("p1_s1: {:?}", p1_s1);
    println!("==============================");

    let mut s1 = String::from("s1");

    let p1_s1 = &mut s1; // p1_s1 is &mut String

    // //cannot borrow `s1` as mutable more than once at a time [E0499]
    // let p2_s1 = &mut s1;

    let p2_s1 = p1_s1; // move occurs here. p2_s1 is &mut String

    // //cannot borrow `s1` as immutable because it is also borrowed as mutable [E0502]
    // println!("s1: {:?}", s1);

    // //borrow of moved value: `p1_s1` [E0382]
    // println!("p1_s1: {:?}", p1_s1);

    println!("p2_s1: {:?}", p2_s1);
    println!("==============================");

    let s1 = String::from("s1");
    let s2 = String::from("s2");

    let mut p = &s1; // p is &String and p itself is mutable
    println!("p: {:?}", p);

    p = &s2;
    println!("p: {:?}", p);
    println!("==============================");
}

// //missing lifetime specifier [E0106]
// fn dangle() -> &String{
//     let s = String::from("s");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("s");
    s
}

//另一个没有所有权的数据类型是 slice.
//ref 是 ordinary pointer;
//slice 是 two-word pointer.

fn use_shared_slice() {
    let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = &a[5..]; //b is &[i32]

    //以下两行保持此顺序可以编译
    println!("b: {:?}", b);
    a[3] = 100;

    // //以下两行保持此顺序无法编译
    // //cannot borrow `a` as mutable because it is also borrowed as immutable
    // a[3] = 100;
    // println!("b: {:?}", b);
}

fn use_mutable_slice() {
    let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = &mut a[5..]; //b is &mut [i32]

    // //以下两行保持此顺序无法编译
    // //cannot borrow `a` as immutable because it is also borrowed as mutable [E0502]
    // println!("a: {:?}", a);
    // b[3] = 100;

    //以下两行保持此顺序可以编译
    b[3] = 100;
    println!("a: {:?}", a);
}

fn str_() {
    //&str 底层是 &[u8].
    // 但是字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
    // 如果尝试从一个多字节字符的中间位置创建字符串 slice，
    // 则程序将会panic。

    let s = String::from("中国");
    //Returns the length of this String, in bytes,
    let len_ = s.len();
    println!("len_ {}", len_); //6
    println!("&s[..] {}", &s[..]);
    // //thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside '中' (bytes 0..3) of `中国`'
    // println!("&s[..1]{}", &s[..1]);
}

fn bad_signature(s: &String) {
    println!("s: {}", s);
}

fn good_signature(s: &str) {
    println!("s: {}", s);
}

fn str_vs_String() {
    //如果有一个字符串 slice，可以直接传递它。
    // 如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
    // 这种灵活性利用了 deref coercions 的优势。
    // 定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API 更加通用并且不会丢失任何功能。

    let s = String::from("hello");
    bad_signature(&s);
    good_signature(&s);

    let slice: &str = &s;
}

fn what_mutable_mean() {
    //shared or mutable 都是编译期要处理的逻辑, 被编译好的二进制可执行文件中是不包含shared or mutable的信息的

    //1. let 与变量名之间的 mut 标识:
    // 1.1如果变量类型是非ref非slice, 那么mut的有无代表该变量own的数据是否可以被更改.
    //    (有两个方面: 1.1.1: 被own的数据发生in-place更改. 1.1.2 放弃原数据, 改own其他数据.)
    // 1.2如果变量类型是ref或slice, 那么mut的有无代表该ref或slice是否可以指向其他的数据.

    //2. 创建 mut ref 和 mut slice 实例时的 mut 标识:
    // mut的有无代表是否允许通过该实例修改被指向的数据.

    let mut a = vec![11, 22, 33, 44, 55];
    //1.1.1
    a.clear();
    //1.1.2
    a = vec![111, 222, 333, 444, 555];

    let a = vec![11, 22, 33, 44, 55];
    // //1.1.1
    // //Cannot borrow immutable local variable `a` as mutable
    // a.clear();
    // //1.1.2
    // //Cannot assign twice to immutable variable [E0384]
    // a = vec![111,222,333,444,555];

    let vec_1 = vec![11, 22, 33, 44, 55];
    let vec_2 = vec![111, 222, 333, 444, 555];
    //1.2
    let mut a = &vec_1;
    a = &vec_2;
    let mut a = &vec_1[..2];
    a = &vec_2[..2];
    //1.2
    let a = &vec_1;
    // //Cannot assign twice to immutable variable [E0384]
    // a = &vec_2;
    let a = &vec_1[..2];
    // //Cannot assign twice to immutable variable [E0384]
    // a = &vec_2[..2];

    //2
    let mut vec_1 = vec![11, 22, 33, 44, 55];
    //a 是 immutable 的, 代表它不能再指向别的数据. 它的类型是&mut[i32], 我们可以经由它改变它所指向的数据.
    let a = &mut vec_1[..2];
    a[0] = 42;
    println!("a: {:?}", a); //[42, 22].
}

#[derive(Debug)]
struct Droppable {
    int_field: i32,
    string_field1: String,
    string_field2: String,
}

fn partial_move() {
    let a = Droppable {
        int_field: 0,
        string_field1: "".to_string(),
        string_field2: "".to_string(),
    };

    let i = a.int_field;
    println!("{a:?}");

    let s = a.string_field1;

    // // borrow of partially moved value: `a` [E0382]
    // // Note: partial move occurs because `a.string_field1` has type `String`, which does not implement the `Copy` trait
    // println!("{a:?}");

    println!("{:?}", a.int_field); //ok

    println!("{:?}", a.string_field2); //ok
}

// &'static and T: 'static
// https://practice.rs/lifetime/static.html
//
//
// 1. &'static
//
// As a reference lifetime, &'static indicates the data pointed to by the reference lives as long as the running program.
//
// 2. T: 'static
//
// As a trait bound, it means the type does not contain any non-static references.

// &'static only indicates that the data can live forever, not the reference.
// The latter one will be constrained by its scope.
fn static_in_ref() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    // //cannot find value `static_string` in this scope [E0425]
    // println!("static_string reference remains alive: {}", static_string);
}

fn main() {
    // what_mutable_mean();
    partial_move();
}
