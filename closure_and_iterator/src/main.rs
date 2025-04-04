use std::thread;

fn concat() {
    let left = String::from("hello");
    let right = " world";
    let new = left + right;
    // //use of moved value: `left` [E0382]
    // dbg!(left);
    dbg!(new);
}

//https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn fn_vs_closure() {
    fn add_one1(i: i64) -> i64 {
        i + 1
    }
    let add_one2 = |i: i64| -> i64 { return i + 1 };
    let add_one3 = |i: i64| -> i64 { i + 1 };
    let add_one4 = |i: i64| i + 1;
    // In Rust, the type of closure is inferred based on the types of its arguments and the return value.
    // 闭包的类型可以被 inferred，所以闭包的参数可以被 inferred。这一点和函数 fn 不同，fn 参数的类型必须被显式声明。
    let add_one5 = |i| i + 1;
    dbg!(add_one5(42 as i64));

    //get_type(&add_one1) = "closure_and_iterator::test_closure::add_one1"
    // get_type(&add_one2) = "closure_and_iterator::test_closure::{{closure}}"
    // get_type(&add_one3) = "closure_and_iterator::test_closure::{{closure}}"
    // get_type(&add_one4) = "closure_and_iterator::test_closure::{{closure}}"
    // get_type(&add_one5) = "closure_and_iterator::test_closure::{{closure}}"
    dbg!(get_type(&add_one1));
    dbg!(get_type(&add_one2));
    dbg!(get_type(&add_one3));
    dbg!(get_type(&add_one4));
    dbg!(get_type(&add_one5));
}

//A closure expression produces a closure value with a unique, anonymous type that cannot be written out.
// A closure type is approximately equivalent to a struct which contains the captured variables.

//All closure types implement Sized.
// Additionally, closure types implement the following traits
// if allowed to do so by the types of the captures it stores:
//
// Clone
// Copy
// Sync
// Send
//
// The rules for Send and Sync match those for normal struct types,
// while Clone and Copy behave as if derived.
// For Clone, the order of cloning of the captured variables is left unspecified.
//
// Because captures are often by reference, the following general rules arise:
//
//
// A closure is Sync
// if all captured variables are Sync.
//
// A closure is Send
// if all variables captured by non-unique immutable reference are Sync,
// and all values captured by unique immutable or mutable reference, copy, or move are Send.
//
// A closure is Clone or Copy
// if it does not capture any values by unique immutable or mutable reference,
// and if all values it captures by copy or move are Clone or Copy, respectively.

fn test_closure() {
    fn add_one(x: usize) -> usize {
        x + 1
    }
    let ptr: fn(usize) -> usize = add_one;
    assert_eq!(ptr(5), 6);
    let clos: fn(usize) -> usize = |x| x + 5;
    assert_eq!(clos(5), 10);

    fn fn_instance(i: i64) -> i64 {
        i + 1
    }

    let Fn_instance1 = |arg: i64| arg + 1;

    let vec = vec![1, 2, 3];
    let Fn_instance2 = |arg: i64| {
        vec.contains(&arg);
        arg + 1
    };

    let mut vec = vec![1, 2, 3];
    let FnMut_instance = |arg: i64| {
        vec.push(arg);
        arg + 1
    };

    let vec = vec![1, 2, 3];
    let FnOnce_instance = |arg: i64| {
        drop(vec);
        arg + 1
    };

    fn test_fn(f: fn(i64) -> i64) {
        f(42);
    }
    fn test_Fn<F: Fn(i64) -> i64>(f: F) {
        f(42);
    }
    fn test_FnMut<F: FnMut(i64) -> i64>(mut f: F) {
        f(42);
    }
    fn test_FnOnce<F: FnOnce(i64) -> i64>(f: F) {
        f(42);
    }

    //fn_instance 实现了 Copy trait
    //
    test_fn(fn_instance);
    test_Fn(fn_instance);
    test_FnMut(fn_instance);
    test_FnOnce(fn_instance);

    //Fn_instance1 实现了 Copy trait
    //
    test_fn(Fn_instance1); // 没有捕获任何上下文变量的闭包实例可以传递给 function pointer 变量.
    test_Fn(Fn_instance1);
    test_FnMut(Fn_instance1);
    test_FnOnce(Fn_instance1);

    //Fn_instance2 实现了 Copy trait
    //
    // test_fn(Fn_instance2); //mismatched types [E0308] expected fn pointer, found closure
    test_Fn(Fn_instance2);
    test_FnMut(Fn_instance2);
    test_FnOnce(Fn_instance2);

    //FnMut_instance 没有实现 Copy trait
    //
    // test_fn(FnMut_instance); //mismatched types [E0308] expected fn pointer, found closure
    // test_Fn(FnMut_instance); //expected a closure that implements the `Fn` trait, but this closure only implements `FnMut` [E0525]
    test_FnMut(FnMut_instance);
    // test_FnMut(FnMut_instance); //use of moved value: `FnMut_instance` [E0382] value used here after move Note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `vec` out of its environment Help: consider mutably borrowing `FnMut_instance
    // test_FnOnce(FnMut_instance); //use of moved value: `FnMut_instance` [E0382] value used here after move Note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `vec` out of its environment Help: consider mutably borrowing `FnMut_instance

    //FnOnce_instance 没有实现 Copy trait
    //
    // test_fn(FnOnce_instance); //mismatched types [E0308] expected fn pointer, found closure
    // test_Fn(FnOnce_instance); //expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce` [E0525]
    // test_FnMut(FnOnce_instance); //expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce` [E0525]
    test_FnOnce(FnOnce_instance);
    // test_FnOnce(FnOnce_instance); //use of moved value: `FnOnce_instance` [E0382] value used here after move Note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `vec` out of its environment
}

//https://doc.rust-lang.org/reference/types/closure.html
//
// The compiler prefers to capture a closed-over variable by immutable borrow,
// followed by unique immutable borrow (see below), by mutable borrow, and finally by move.
// It will pick the first choice of these
// that is compatible with how the captured variable is used inside the closure body.
//
// The compiler does not take surrounding code into account,
// such as the lifetimes of involved variables, or of the closure itself.
//
//If the move keyword is used,
// then all captures are by move or, for Copy types, by copy, regardless of whether a borrow would work.
// The move keyword is usually used to allow the closure to outlive the captured values,
// such as if the closure is being returned or used to spawn a new thread.
//
//Composite types such as structs, tuples, and enums are always captured entirely, not by individual fields.
// It may be necessary to borrow into a local variable in order to capture a single field

// Unique immutable borrows in captures
//
// Captures can occur by a special kind of borrow called a unique immutable borrow,
// which cannot be used anywhere else in the language and cannot be written out explicitly.
// It occurs when modifying the referent of a mutable reference, as in the following example:

fn unique_immutable_borrow() {
    let mut b = false;
    let x = &mut b;
    {
        let mut c = || {
            *x = true;
        };
        // // The following line is an error:
        // // cannot borrow `x` as immutable because previous closure requires unique access [E0501]
        // let y = &x;
        c();
    }
    let z = &x;
}

// In this case, borrowing x mutably is not possible, because x is not mut.
// But at the same time, borrowing x immutably would make the assignment illegal,
// because a & &mut reference might not be unique, so it cannot safely be used to modify a value.
// So a unique immutable borrow is used: it borrows x immutably, but like a mutable borrow, it must be unique.
// In the above example, uncommenting the declaration of y will produce an error
// because it would violate the uniqueness of the closure's borrow of x;
// the declaration of z is valid
// because the closure's lifetime has expired at the end of the block, releasing the borrow.

// 如果闭包体不要求获取它用到的环境中变量的所有权，但我们仍希望闭包获取所有权，可以在参数列表前使用 move 关键字。
// move captures a closure’s environment by value.
//
// move converts any variables captured by reference or mutable reference to variables captured by value.
// Note: move closures may still implement Fn or FnMut, even though they capture variables by move.
// This is because the traits implemented by a closure type are determined by
// what the closure does with captured values, not how it captures them.
fn test_move() {
    let v = vec![1, 2, 3];
    println!("{v:?}");
    thread::spawn(move || println!("{v:?}")).join().unwrap();
}

//在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
fn implicit_iter() {
    let a = [10, 20, 30, 40, 50];
    //在底层它隐式地创建并接着消费了一个迭代器.
    for element in a {
        println!("the value is: {element}");
    }
}

fn intro_iter() {
    let v1 = vec![1, 2, 3];
    //注意 v1_iter 需要是 mut 的：
    // 在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    // 换句话说，代码 消费（consume）了，或使用了迭代器。
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn adaptors() {
    //Iterator trait 有一系列不同的由标准库提供默认实现的方法；
    // 你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。
    // 一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。
    //
    // 这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），
    // 因为调用他们会消耗迭代器。一个消费适配器的例子是 sum 方法。
    // 这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
    //
    // Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），
    // 他们允许我们将当前迭代器变为不同类型的迭代器。
    // 可以链式调用多个迭代器适配器。
    // 不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。
}

fn three_iters() {
    //https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
    //The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
    //The iterator returned by iter will yield &T, by convention.
    //The iterator returned by iter_mut will yield &mut T, by convention.

    //impl<T> [T]
    // pub fn iter(&self) -> Iter<'_, T>
    // Returns an iterator of slices.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //impl<T> [T]
    // pub fn iter_mut(&mut self) -> IterMut<'_, T>
    // Returns an iterator that allows modifying each value.
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter_mut = v1.iter_mut();

    assert_eq!(v1_iter_mut.next(), Some(&mut 1));
    assert_eq!(v1_iter_mut.next(), Some(&mut 2));
    assert_eq!(v1_iter_mut.next(), Some(&mut 3));
    assert_eq!(v1_iter_mut.next(), None);

    //impl<T, A: Allocator> IntoIterator for Vec<T, A>
    // fn into_iter(self) -> Self::IntoIter
    // Creates a consuming iterator,
    // that is, one that moves each value out of the vector (from start to end).
    // The vector cannot be used after calling this.
    let v1 = vec![1, 2, 3];
    let mut v1_into_iter = v1.into_iter();

    assert_eq!(v1_into_iter.next(), Some(1));
    assert_eq!(v1_into_iter.next(), Some(2));
    assert_eq!(v1_into_iter.next(), Some(3));
    assert_eq!(v1_into_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let mut v1_into_iter = (&v1).into_iter();
    assert_eq!(v1_into_iter.next(), Some(&1));
    assert_eq!(v1_into_iter.next(), Some(&2));
    assert_eq!(v1_into_iter.next(), Some(&3));
    assert_eq!(v1_into_iter.next(), None);

    let mut v1 = vec![1, 2, 3];
    let mut v1_into_iter = (&mut v1).into_iter();
    assert_eq!(v1_into_iter.next(), Some(&mut 1));
    assert_eq!(v1_into_iter.next(), Some(&mut 2));
    assert_eq!(v1_into_iter.next(), Some(&mut 3));
    assert_eq!(v1_into_iter.next(), None);
}

fn test_trait_object() {
    //https://doc.rust-lang.org/reference/types/closure.html
    //https://users.rust-lang.org/t/cannot-move-a-value-of-type-dyn-for-r-fnonce-r-mut-u8-the-size-of-dyn-for-r-fnonce-r-mut-u8-cannot-be-statically-determined/57364/2
    //https://stackoverflow.com/questions/30411594/cannot-move-a-value-of-type-fnonce-when-moving-a-boxed-function

    //A closure type is approximately equivalent to a struct which contains the captured variables.
    // For instance, the following closure:
    //
    // fn f<F : FnOnce() -> String> (g: F) {
    //     println!("{}", g());
    // }
    //
    // let mut s = String::from("foo");
    // let t = String::from("bar");
    //
    // f(|| {
    //     s += &t;
    //     s
    // });
    // // Prints "foobar".
    //
    // generates a closure type roughly like the following:
    //
    // struct Closure<'a> {
    //     s : String,
    //     t : &'a String,
    // }
    //
    // impl<'a> FnOnce<()> for Closure<'a> {
    //     type Output = String;
    //     fn call_once(self) -> String {
    //         self.s += &*self.t;
    //         self.s
    //     }
    // }
    //
    // so that the call to f works as if it were:
    //
    // f(Closure{s: s, t: &t});

    // pub trait Fn<Args: Tuple>: FnMut<Args> {
    //     /// Performs the call operation.
    //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // }
    // pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    //     /// Performs the call operation.
    //     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    // }
    // pub trait FnOnce<Args: Tuple> {
    //     /// The returned type after the call operator is used.
    //     type Output;
    //
    //     /// Performs the call operation.
    //     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    // }

    fn test_1<F: Fn()>(f: F) {
        f()
    }
    fn test_2<F: FnMut()>(mut f: F) {
        f()
    }
    fn test_3<F: FnOnce()>(f: F) {
        f()
    }

    fn test_4(f: impl Fn()) {
        f()
    }
    fn test_5(mut f: impl FnMut()) {
        f()
    }
    fn test_6(f: impl FnOnce()) {
        f()
    }

    fn test_7(f: Box<dyn Fn()>) {
        f()
    }
    fn test_8(mut f: Box<dyn FnMut()>) {
        f()
    }
    fn test_9(f: Box<dyn FnOnce()>) {
        f()
    }

    fn test_10(f: &dyn Fn()) {
        f()
    }
    fn test_11(f: &mut dyn FnMut()) {
        f()
    }
    fn test_12(f: &dyn FnOnce()) {
        // // cannot move a value of type `dyn FnOnce()` [E0161]
        // // the size of `dyn FnOnce()` cannot be statically determined
        // //
        // // cannot move out of `*f` which is behind a shared reference [E0507]
        // // move occurs because `*f` has type `dyn FnOnce()`, which does not implement the `Copy` trait
        // // Note: this value implements `FnOnce`, which causes it to be moved when called
        // f()
    }
}
fn main() {
    fn_vs_closure();
    test_closure();
    test_move();
    adaptors();
    three_iters();
    test_trait_object();
}
