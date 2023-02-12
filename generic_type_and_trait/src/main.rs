#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32, i32> {
    fn only_for_this_concrete_type(&self) -> i32 {
        self.x * self.y
    }
}

fn use_generic_type() {
    dbg!(Point { x: 1, y: 2 });
    dbg!(Point { x: 1.0, y: 2.0 });
    dbg!(Point { x: 1, y: 2.0 });

    // //no method named `only_for_this_concrete_type` found
    // // for struct `Point<{float}, {float}>` in the current scope [E0599]
    // // method not found in `Point<{float}, {float}>
    // let a = Point { x: 1.0, y: 2.0 };
    // dbg!(a.only_for_this_concrete_type());

    let a = Point { x: 1, y: 2 };
    dbg!(a.only_for_this_concrete_type());
}

trait Summary_V1 {
    fn summarize(&self) -> String;
}

trait Summary_V2 {
    //trait方法可以有默认实现, 供实现者直接使用或覆盖写.
    //一个trait方法可以调用同trait下另一个trait方法.
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

struct Novel {
    author: String,
    content: String,
}

impl Summary_V1 for Novel {
    fn summarize(&self) -> String {
        format!("Novel from {}", self.author)
    }
}

struct Review {
    author: String,
    content: String,
}

impl Summary_V2 for Review {
    //实现所有没有默认实现的方法和想要覆盖的有默认实现的方法
    //这个集合可以使空集
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

fn use_trait() {
    let a = Novel {
        author: String::from("欧亨利"),
        content: String::from("麦琪的礼物"),
    };
    dbg!(a.summarize());

    let a = Review {
        author: String::from("列宁"),
        content: String::from("该怎么办"),
    };
    dbg!(a.summarize());
}

// impl Trait 可以作为函数参数和返回值.
//
// //It’s important to note
// // that Rust doesn’t allow trait methods to use impl Trait return values.
// // Supporting this will require some improvements in the languages’s type system.
// // Until that work is done,
// // only free functions and functions associated with specific types can use impl Trait returns.
//
//
// 作为参数:
// fn print<T: Display>(val: T) {
//     println!("{}", val);
// }
// 它和下面的使用 impl Trait 的版本相同：
// fn print(val: impl Display) {
//     println!("{}", val);
// }
//
// 作为返回值:
// fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item=u8>> {
//     Box::new(v.into_iter().chain(u.into_iter()).cycle())
// }
// 改造成:
// fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item=u8> {
//     v.into_iter().chain(u.into_iter()).cycle()
// }
//
// impl Trait 是一种静态分发的版本，
// 因此编译器需要在编译期知道函数内返回的实际类型，
// 这样才能在栈上分配正确数量的空间并调用正确的字段和方法。
//
// dyn Trait 是动态分发。
//
// fn say_hello<W: Write>(out: &mut W) //generic function 静态分发
// fn say_hello(out: &mut dyn Write) //plain function 动态分发

fn print_summary_v1(sth_has_summary: impl Summary_V1) {
    println!("summary: {}", sth_has_summary.summarize())
}

fn use_impl_trait() {
    let a = Novel {
        author: String::from("欧亨利"),
        content: String::from("麦琪的礼物"),
    };
    print_summary_v1(a);
}

//impl Trait 很方便，适用于短小的例子。
// trait bound 则适用于更复杂的场景。
// 例如，可以获取两个实现了 Summary 的参数。
// 使用 impl Trait 的语法看起来像这样：
// pub fn notify(item1: impl Summary, item2: impl Summary) {}

fn print_summary_v2<T: Summary_V1>(sth: T) {
    println!("summary: {}", sth.summarize())
}

fn use_trait_bound() {
    let a = Novel {
        author: String::from("欧亨利"),
        content: String::from("麦琪的礼物"),
    };
    print_summary_v2(a);
}

fn main() {
    use_impl_trait();
    use_trait_bound();
}
