use std::fmt::{Debug, Display, Formatter};

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

// //通过 + 指定多个 trait bound
// //item 需要同时实现两个不同的 trait：Display 和 Summary
// fn notify(item: impl Summary_V1 + Display) {}

// //通过 where 简化 trait bound
//
// fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     42
// }
//
// fn some_function_v2<T, U>(t: T, u: U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     42
// }

fn returns_summarizable() -> impl Summary_V1 {
    Novel {
        author: String::from("欧亨利"),
        content: String::from("麦琪的礼物"),
    }
}

fn use_impl_trait_as_return() {
    dbg!(returns_summarizable().summarize());
}

//使用 trait bound 有条件地实现方法
//所有的Pair<T>都实现了new(), 只有<T: Display + PartialOrd>这个子集里的Pair<T>实现了cmp_display()
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn conditionally_impl_methods() {
    let pair1 = Pair::new(1, 2);
    pair1.cmp_display();
    let a = Novel {
        author: String::from("欧亨利"),
        content: String::from("麦琪的礼物"),
    };
    let b = Novel {
        author: String::from("欧亨利"),
        content: String::from("警察与赞美诗"),
    };
    let pair2 = Pair::new(a, b);
    // //the method `cmp_display` exists for struct `Pair<Novel>`,
    // // but its trait bounds were not satisfied [E0599]
    // pair2.cmp_display();
}

//使用 trait bound 有条件地实现trait
// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds
// are called blanket implementations and are extensively used in the Rust standard library.
// For example, the standard library implements the ToString trait on any type
// that implements the Display trait.
//
// impl<T: Display> ToString for T {
//     // --snip--
// }

trait PrintV1 {
    fn print_v1(&self) -> String;
}

trait PrintV2 {
    fn print_v2(&self) -> String;
}

// //尝试为所有实现了 PrintV1 trait 的类型实现 Debug trait, 结果 Debug 处报错
// //Only traits defined in the current crate can be implemented for arbitrary types [E0117]
// impl<T: PrintV1> Debug for T {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.print_v1())
//     }
// }

// //尝试为所有实现了 Debug trait 的类型实现 PrintV2 trait //没有问题
// impl<T: Debug> PrintV2 for T {
//     fn print_v2(&self) -> String {
//         String::from(format!("{:?}", *self))
//     }
// }

// //尝试为所有实现了 PrintV1 trait 的类型实现 PrintV2 trait //没有问题
// impl<T: PrintV1> PrintV2 for T {
//     fn print_v2(&self) -> String {
//         String::from(self.print_v1())
//     }
// }

//生命周期的存在旨在避免悬垂引用
//
//longest 函数签名中的生命周期标注的实际含义是
// longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
// 这就是我们告诉 Rust 需要其保证的约束条件。
// 记住通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，
// 而是指出任何不满足这个约束条件的值都将被借用检查器（borrow checker）拒绝。
//
// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，
// 所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//结构体定义中的每一个引用类型的字段都必须添加生命周期标注.
//这个标注意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    // //lifetime may not live long enough
    // // associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
    // // Help: consider introducing a named lifetime parameter and update trait if needed
    // fn return_announcement_wrong(&self, announcement: &str) -> &str {
    //     announcement
    // }
    fn return_announcement_right<'b>(&self, announcement: &'b str) -> &'b str {
        announcement
    }
}

//借用检查器在这些情况下就能推断出生命周期而不再强制开发者显式的增加标注。
//
// 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。
// 这并不是需要开发者遵守的规则；
// 这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。
//
// 省略规则并不提供完整的推断：
// 如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，
// 它不会猜测剩余引用的生命周期应该是什么。
// 在这种情况，编译器会给出一个错误，这时需要程序员通过添加对应引用之间相联系的生命周期标注来解决。
//
// [另外, 如果针对函数签名做出的生命周期推断结果与函数体内部的逻辑不符时,
// 编译器也会给出错误, 这时同样需要程序员通过添加正确的生命周期标注来解决. 见 return_announcement_wrong]
//
// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。
//
// 编译器采用三条规则来判断引用何时不需要明确的标注。
// 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
// 这些规则适用于 fn 定义，以及 impl 块。
//
// 第一条规则是，每一个是引用的参数都有它自己的生命周期参数。
// 换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
// 有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
//
// 第二条规则是，如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
// [rust可以通过返回tuple的方式返回多个返回值]
//
// 第三条规则是，如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method),
// 那么所有输出生命周期参数被赋予 self 的生命周期。
//
// 第三条规则使得方法更容易读写，因为只需更少的符号。
//
// [一个可以编译通过的rust函数, 如果它的返回值类型里有引用, 那么该引用必定和输入参数中的引用相关,
// 因为永远无法在函数体内产生一个变量然后返回指向这个变量的引用.]

fn use_lifetimed_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

//这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
// 所有的字符串字面量都拥有 'static 生命周期。
// 字符串字面量被直接储存在程序的二进制文件中而这个文件总是可用的。
// 因此所有的字符串字面量都是 'static 的。
// 你可能在错误信息的帮助文本中见过使用 'static 生命周期的建议，
// 不过将引用指定为 'static 之前，思考一下这个引用是否真的在整个程序的生命周期里都有效。
// 你也许要考虑是否希望它存在得这么久，即使技术上是可行的。
// 大部分情况，代码中的问题是尝试创建一个悬垂引用或者可用的生命周期不匹配，请解决这些问题而不是指定一个 'static 的生命周期。

// 生命周期标注和泛型类型参数一起使用
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//In general, fully qualified syntax is defined as follows:
//
// <Type as Trait>::function(receiver_if_method, next_arg, ...);

trait Flyable {
    fn live(&self) {
        println!("我会飞");
    }
}
trait Mammal {
    fn live(&self) {
        println!("我哺乳");
    }
}

struct Bat;
impl Bat {
    fn live(&self) {
        println!("我喜欢黑夜");
    }
}
impl Flyable for Bat {}
impl Mammal for Bat {}

trait Animal {
    fn name();
}
impl Animal for Bat {
    fn name() {
        println!("动物之蝙蝠");
    }
}
impl Bat {
    fn name() {
        println!("蝙蝠");
    }
}

fn test_call_trait_method() {
    let a = Bat;

    a.live();
    Flyable::live(&a);
    Mammal::live(&a);
    <Bat as Flyable>::live(&a);
    <Bat as Mammal>::live(&a);

    Bat::name();
    <Bat as Animal>::name();
}

fn main() {
    // use_impl_trait();
    // use_trait_bound();
    // use_impl_trait_as_return();
    // conditionally_impl_methods();
    test_call_trait_method()
}
