#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl TimeUnit {
    /// 返回该时间单位的复数名词。
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Second => "seconds",
            TimeUnit::Minute => "minutes",
            TimeUnit::Hour => "hours",
            TimeUnit::Day => "days",
            TimeUnit::Month => "months",
            TimeUnit::Year => "years",
        }
    }
    /// 返回该时间单位的单数名词。
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn arms_order_matters(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(TimeUnit::Hour, 1) => format!("an hour ago"),
        RoughTime::InThePast(unit, 1) => format!("a {} ago", unit.singular()),
        RoughTime::InThePast(unit, count) => format!("{} {} ago", count, unit.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hour, 1) => format!("an hour from now"),
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(unit, count) => {
            format!("{} {} from now", count, unit.plural())
        }
    }
}

fn c_language_switch() {
    let count_rabbits = || 42;
    match count_rabbits() {
        0 => {} // 什么都不输出
        1 => println!("A rabbit is nosing around in the clover."),
        n => println!("There are {} rabbits hopping about in the meadow", n),
    }
}

fn ref_and_deref() {
    let i32_instance = 5;
    println!("i32_instance: {:?}", i32_instance);
    if let i32_instance @ 3..=7 = 5 {
        println!("i32_instance: {:?}", i32_instance);
    }
    let ref ref_of_i32_instance = 5;
    println!("ref_of_i32_instance: {:?}", ref_of_i32_instance);
    if let ref ref_of_i32_instance @ 3..=7 = 5 {
        println!("ref_of_i32_instance: {:?}", ref_of_i32_instance);
    }

    let &i32_five = &5;
    println!("i32_five: {:?}", i32_five);
    // let s = "".to_string();
    // //cannot move out of a shared reference [E0507]
    // if let &string_instance = &s {
    //     println!("string_instance: {:?}", string_instance);
    // }
}

fn default_binding_modes() {
    // // Better ergonomics for pattern-matching on references.
    // // Currently, matching on references requires a bit of a dance using ref and & patterns:
    //
    //     let x: &Option<_> = &Some(0);
    //
    //     match x {
    //         &Some(ref y) => { ... },
    //         &None => { ... },
    //     }
    //
    // // or using `*`:
    //
    //     match *x {
    //         Some(ref x) => { ... },
    //         None => { ... },
    //     }
    //
    // // After this RFC, the above form still works, but now we also allow a simpler form:
    //
    //     let x: &Option<_> = &Some(0);
    //
    //     match x {
    //         Some(y) => { ... }, // `y` is a reference to `0`
    //         None => { ... },
    //     }
    //
    // // This is accomplished through automatic dereferencing and the introduction of default binding modes.

    // // Detailed design
    // //
    // // This RFC is a refinement of the match ergonomics RFC. Rather than using auto-deref and auto-referencing, this RFC introduces default binding modes used when a reference value is matched by a non-reference pattern.
    // //
    // // In other words, we allow auto-dereferencing values during pattern-matching. When an auto-dereference occurs, the compiler will automatically treat the inner bindings as ref or ref mut bindings.
    // //
    // // Example:
    //
    //     let x = Some(3);
    //     let y: &Option<i32> = &x;
    //     match y {
    //       Some(a) => {
    //         // `y` is dereferenced, and `a` is bound like `ref a`.
    //       }
    //       None => {}
    //     }
    //
    // // Note that this RFC applies to all instances of pattern-matching, not just match expressions:
    //
    //     struct Foo(i32);
    //
    //     let foo = Foo(6);
    //     let foo_ref = &foo;
    //     // `foo_ref` is dereferenced, and `x` is bound like `ref x`.
    //     let Foo(x) = foo_ref;

    // // Examples
    // //
    // // No new behavior:
    //
    //     match &Some(3) {
    //         p => {
    //             // `p` is a variable binding. Hence, this is **not** a ref-defaulting
    //             // match, and `p` is bound with `move` semantics
    //             // (and has type `&Option<i32>`).
    //         },
    //     }
    //
    // // One match arm with new behavior:
    //
    //     match &Some(3) {
    //         Some(p) => {
    //             // This pattern is not a `const` reference, `_`, or `&`-pattern,
    //             // so this is a "non-reference pattern."
    //             // We dereference the `&` and shift the
    //             // default binding mode to `ref`. `p` is read as `ref p` and given
    //             // type `&i32`.
    //         },
    //         x => {
    //             // In this arm, we are still in `move`-mode by default, so `x` has type
    //             // `&Option<i32>`
    //         },
    //     }
    //
    //     // Desugared:
    //     match &Some(3) {
    //         &Some(ref p) => {
    //             ...
    //         },
    //         x => {
    //             ...
    //         },
    //     }

    // 我们没有办法只经由一个引用将被指向的对象的一个 component "move" 出来,  除非该 component 实现了Copy trait.

    // 下面的 component 可以被 "move" 出来
    let x: &Option<i32> = &Some(42);
    match x {
        &Some(ref y) => {} //y: &i32
        &None => {}
    }
    match x {
        &Some(y) => {} //y: i32
        &None => {}
    }
    match *x {
        Some(ref y) => {} //y: &i32
        None => {}
    }
    match *x {
        Some(y) => {} //y: i32
        None => {}
    }

    // 下面的 component 不可以被 move 出来
    let x: &Option<String> = &Some("".to_string());
    match x {
        &Some(ref y) => {} //y: &String
        &None => {}
    }
    // //cannot move out of `x` as enum variant `Some` which is behind a shared reference [E0507]
    // match x {
    //     &Some(y) => {  },
    //     &None => {  },
    // }
    match *x {
        Some(ref y) => {} //y: &String
        None => {}
    }
    // //cannot move out of `x` as enum variant `Some` which is behind a shared reference [E0507]
    // match *x {
    //     Some(y) => {  },
    //     None => {  },
    // }

    // 综上, 当匹配一个引用的时候, 通用操作都是将被指向对象的某个 component 的引用取出来.
    // 这是 default binding modes 产生的意图.
    // default binding modes 仅仅发生在 when a reference is matched with a non-reference pattern.

    println!("====================");
    match &Some(3) {
        p => {
            // `p` is a variable binding. Hence, this is **not** a ref-defaulting
            // match, and `p` is bound with `move` semantics
            // (and has type `&Option<i32>`).
            dbg!(p);
        }
    }
    println!("====================");
    match &Some(3) {
        Some(p) => {
            // This pattern is not a `const` reference, `_`, or `&`-pattern,
            // so this is a "non-reference pattern."
            // We dereference the `&` and shift the
            // default binding mode to `ref`. `p` is read as `ref p` and given
            // type `&i32`.
            dbg!(p);
        }
        x => {
            // In this arm, we are still in `move`-mode by default, so `x` has type
            // `&Option<i32>`
            dbg!(x);
        }
    }
    println!("====================");
    match &Option::None::<i32> {
        Some(p) => {
            dbg!(p);
        }
        x => {
            dbg!(x);
        }
    }

    //see more examples:
    //https://stackoverflow.com/questions/36590549/what-is-the-syntax-to-match-on-a-reference-to-an-enum
    //https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
}

fn wildcard_pattern() {
    // go func
    // func goFunc() {
    // 	_, y := 42, 43
    // 	////Cannot use '_' as a value
    // 	//fmt.Println(_)
    // 	fmt.Println(y)
    // }

    if let x = 42 {
        println!("{}", x);
    }
    if let _ = 42 {
        // //in expressions, `_` can only be used on the left-hand side of an assignment `_` not allowed here
        // println!("{}", _);
    }
}

fn where_pattern_matching_occurs() {
    //所有 pattern matching 发生的地方:
    // let, if-let, while-let, let-else, for, match branch, 函数参数

    // //Examples
    // let a = ['a', 'b', 'c'];
    // let mut iter = a.iter().enumerate();
    // assert_eq!(iter.next(), Some((0, &'a')));
    // assert_eq!(iter.next(), Some((1, &'b')));
    // assert_eq!(iter.next(), Some((2, &'c')));
    // assert_eq!(iter.next(), None);

    let v = vec![Some(1), Some(2), Some(3), Some(4)];
    for pair in v.iter().enumerate() {
        println!("{:?}", pair); //pair: (usize, &Option<i32>)
    }
    for (index, value) in v.iter().enumerate() {
        println!("{:?} is at index {}", value, index); //value: &Option<i32>
    }
    for (index, &value) in v.iter().enumerate() {
        println!("{:?} is at index {}", value, index); //value: Option<i32>, i32 has implemented Copy trait
    }

    fn foo(pair: (usize, &Option<i32>)) {
        println!("{:?}", pair); //pair: (usize, &Option<i32>)
    }
    fn bar((index, value): (usize, &Option<i32>)) {
        println!("{:?} is at index {}", value, index); //value: &Option<i32>
    }
    fn baz((index, &value): (usize, &Option<i32>)) {
        println!("{:?} is at index {}", value, index); //value: Option<i32>, i32 has implemented Copy trait
    }
}

fn named_variables() {
    //Named variables are irrefutable patterns that match any value.

    // Matched, y = 5
    // at the end: x = Some(5), y = 10

    let x = Some(5);
    let y = 10;

    match x {
        // 这里创造了一个词法块, 块内的 y 屏蔽了块外的 y
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn or_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_pattern() {
    // The compiler checks that the range isn’t empty at compile time,
    // and because the only types for which Rust can tell if a range is empty or not are char and numeric values,
    // ranges are only allowed with numeric or char values.

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// We can also use patterns to destructure structs, enums, and tuples to use different parts of these values.

struct Point {
    x: i32,
    y: i32,
}

fn destructure_struct() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enum() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructure_nested() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

fn destructure_nested2() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    assert_eq!(feet, 3);
    assert_eq!(inches, 10);
    assert_eq!(x, 3);
    assert_eq!(y, -10);
}

fn ignore_entire_or_part_of_value_using_wildcard() {
    //下划线可作为匹配但不绑定任何值的通配符模式。虽然这作为 match 表达式最后的分支特别有用，也可以将其用于任意模式，包括函数参数中。
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    //单独使用下划线不会绑定值, 所以没有实现 Copy trait 的 String 变量不会 move 进去.
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); // 可以编译

    // 可以在一个模式内部使用 _ 忽略部分值. 例如, 当只需要测试部分值, 在运行的代码中没有用到其他被忽略部分的值时。
    // 当不需要 Some 中的值时在模式内使用下划线来匹配 Some 成员
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

fn ignore_part_of_value_using_two_dots() {
    // Ignoring Remaining Parts of a Value with ..

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // // 一个带有歧义的 .. 例子, 不能编译
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }
}

fn match_guard() {
    // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须同时被满足才能选择此分支。
    // 缺点: 编译器不会尝试为包含匹配守卫的模式检查穷尽性

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // match guard 提供了这样一个能力: 在测试的时候使用 match 词法块外部的某个变量的值作为 input.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // match guard 作用于整个 match arm
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // 等价于 (4 | 5 | 6) if y => ...
        _ => println!("no"),
    }
}

fn at_binding() {
    // The at operator @ lets us create a variable that holds a value
    // at the same time as we’re testing that value for a pattern match.
    // 使用 @ 可以在一个模式中同时测试和保存变量值。

    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    // 上例会打印出 Found an id in range: 5。通过在 3..=7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时测试其值匹配这个范围模式。
    //
    // 第二个分支只在模式中指定了一个范围，分支相关代码没有一个包含 id 字段实际值的变量。
    // id 字段的值可以是 10、11 或 12，不过这个模式的代码并不知情也不能使用 id 字段中的值，因为没有将 id 值保存进一个变量。
    //
    // 最后一个分支指定了一个没有范围的变量，此时确实拥有可以用于分支代码的变量 id，因为这里使用了结构体字段简写语法。
    // 不过此分支中没有像前两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。
}

fn main() {
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Hours, 1));
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Days, 1));
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Days, 2));
    // c_language_switch();
    // ref_and_deref();
    // default_binding_modes();
    // wildcard_pattern();
    // where_pattern_matching_occurs();
    // named_variables();
    // or_pattern();
    // range_pattern();
    // destructure_struct();
    // destructure_enum();
    // destructure_nested();
    // destructure_nested2();
    ignore_entire_or_part_of_value_using_wildcard();
    ignore_part_of_value_using_two_dots();
    // match_guard();
    // at_binding();
}
