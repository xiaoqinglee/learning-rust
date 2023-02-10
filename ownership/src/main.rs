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
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。

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

fn main() {
    no_dangle();
}
