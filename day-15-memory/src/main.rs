// use std::cell::RefCell;
// use std::rc::Rc;

fn main() {
    // **The Three Rules: Ownership**
    // 1. Each value has exactly one owner
    // 2. When the owner goes out of scope, the value is dropped (freed)
    // 3. Ownership can be transferred (moved)

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s2: {}", s2);

    let s3 = String::from("world");
    borrow_demo(&s3);
    print!("After borrow_demo: {}", s3);

    // let s3 = String::from("world");
    // borrow_demo(s3.clone());
    // print!("After borrow_demo: {}", s3);

    let mut s4 = String::from("hello");
    mutate_demo(&mut s4);
    println!("s4: {}", s4);

    // let mut s4 = String::from("hello");
    // let mut s5 = s4.clone();
    // mutate_demo(&mut s5);
    // println!("s4: {}", s4);

    // // Lifetimes
    // let result;
    // let a = String::from("abcd");
    // {
    //     let b = String::from("xyz");
    //     result = longest(&a, &b);
    //     println!("⏳ Longest string: {}", result);
    // }

    // // Box (heap allocation)
    // let boxed = Box::new(42);
    // println!("📦 Boxed value: {}", boxed);

    // // Rc (reference-counted pointer)
    // let rc_val = Rc::new(String::from("Shared"));
    // let rc_clone = Rc::clone(&rc_val);
    // println!("📚 Rc values: {}, {}", rc_val, rc_clone);
    // println!("Ref count: {}", Rc::strong_count(&rc_val));

    // // RefCell (interior mutability)
    // let cell = RefCell::new(100);
    // *cell.borrow_mut() += 50;
    // println!("🧪 RefCell value: {}", cell.borrow());
}

fn borrow_demo(datum: &String) {
    println!("Borrowed {}", datum);
}

fn mutate_demo(datum: &mut String) {
    datum.push_str(", world!");
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// ### Resources
// - Smart pointers in the book https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
// - copy and clone everything - https://youtu.be/2hXNd6x9sZs?t=557
// - structs, vectors, iteratio, result and option - https://youtu.be/sDtQaO5_SOw?t=162
