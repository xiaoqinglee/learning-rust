//指针 （pointer）是一个包含内存地址的变量的通用概念。
// 这个地址引用，或 “指向”（points at）一些其他数据。
// Rust 中最常见的指针是 引用（reference）。
// 引用以 & 符号为标志并借用了他们所指向的值。
// 除了引用数据没有任何其他特殊功能，也没有额外开销。
//
// 智能指针（smart pointers）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。
// Rust 标准库中定义了多种不同的智能指针，它们提供了多于引用的额外功能。
//
// 引用计数 （reference counting）智能指针类型允许数据有多个所有者，它会记录所有者的数量，当没有所有者时清理数据。
// String 和 Vec<T> 这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们。
// 它们也拥有元数据和额外的功能或保证。
// 例如 String 存储了其容量作为元数据，并拥有额外的能力确保其数据总是有效的 UTF-8 编码。
//
// 智能指针不同于结构体的地方在于其实现了 Deref 和 Drop trait。
// Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码。
// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。

// Box<T>，用于在堆上分配值
// Rc<T>，一个引用计数类型，其数据可以有多个所有者
// Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问。（ RefCell<T> 是一个在运行时而不是在编译时执行借用规则的类型）。

use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn use_box() {
    let list = List::Cons(
        4,
        Box::new(List::Cons(
            3,
            Box::new(List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))),
        )),
    );
    println!("{list:?}");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn fn_require_ref(elem: &i32) {
    println!("{elem}");
}

fn use_deref() {
    let my_box = MyBox::new(42);

    //实现了 Deref trait 后, 该类型就可以当成 immutable ref 使用了, 可以使用 * 来解引用.
    //具体解引用后指向的对象是谁由 Deref trait 的实现决定.
    assert_eq!(42, *my_box); //原理如下行
    assert_eq!(42, *(my_box.deref()));

    // //如果不使用 * , rust 编译器不会自己尝试调用 deref() .
    // //mismatched types [E0308] expected `&i32`, found `MyBox<i32>` ,
    // fn_require_ref(my_box);
    fn_require_ref(my_box.deref());

    //Deref 强制转换（deref coercions）将实现了 Deref trait 的类型的引用转换为另一种类型的引用。
    // 例如，Deref 强制转换可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以返回 &str。
    // Deref 强制转换是 Rust 在函数或方法传参上的一种便利操作，并且只能作用于实现了 Deref trait 的类型。
    // 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时将自动进行。
    // 这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。
    //
    // Deref 强制转换的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。
    // 这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。

    // [注意]: Deref 强制转换（deref coercions） 和 自动引用和解引用（automatic referencing and dereferencing）是两码事, 后者见use_struct.

    fn_require_ref(&my_box); //发生了 deref coercions, 原理如下两行
    fn_require_ref(&(*(my_box.deref())));
    fn_require_ref(my_box.deref());

    //类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
    //
    //Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    //
    //     当 T: Deref<Target=U>    时从 &T 到 &U。
    //     当 T: Deref<Target=U>    时从 &mut T 到 &U。
    //     当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn use_drop_trait() {
    //对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。
    // 可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
    //
    // 在 Rust 中，指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。
    // 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。
    // Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("Two CustomSmartPointers created.");

    //栈顺序调用 Drop trait 的 drop 方法
}

fn use_drop_fn() {
    //有时你可能需要提早清理某个值。
    // 一个例子是当使用智能指针管理锁时；你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁。
    // Rust 并不允许我们主动调用 Drop trait 的 drop 方法；
    // 当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop (core::mem::drop)。

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    //Disposes of a value.
    //
    // This does so by calling the argument's implementation of Drop.
    // This effectively does nothing for types which implement Copy, e.g. integers.
    // Such values are copied and then moved into the function, so the value persists after this function call.
    //
    // This function is not magic; it is literally defined as
    // pub fn drop<T>(_x: T) { }
    //
    // Because _x is moved into the function, it is automatically dropped before the function returns.
    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("Two CustomSmartPointers created.");
}

enum ListV2 {
    Cons(i32, Rc<ListV2>),
    Nil,
}

use crate::ListV2::{Cons, Nil};

fn use_rc() {
    let a = Rc::new(Cons(
        4,
        Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil))))))),
    ));

    // Rc<T> 允许通过 immutable ref 在程序的多个部分之间只读地共享数据。
    //也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    //下面两种方式使用的都是 Rc<T> 实现的 Clone trait 的 fn clone(&self) -> Rc<T> 方法
    let b = Rc::new(Cons(11, Rc::clone(&a)));
    let c = Rc::new(Cons(12, a.clone()));

    {
        let d = Cons(13, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 4);
    }
    assert_eq!(Rc::strong_count(&a), 3);
}

fn main() {
    // use_box();
    // use_deref();
    // use_drop_trait();
    // use_drop_fn();
    use_rc();
}
