#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// 返回该时间单位的复数名词。
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
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
        RoughTime::InThePast(TimeUnit::Hours, 1) => format!("an hour ago"),
        RoughTime::InThePast(unit, 1) => format!("a {} ago", unit.singular()),
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
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

fn main() {
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Hours, 1));
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Days, 1));
    // arms_order_matters(RoughTime::InThePast(TimeUnit::Days, 2));
    // c_language_switch();
    // ref_and_deref();
    // default_binding_modes();
    // wildcard_pattern();
    where_pattern_matching_occurs();
}
