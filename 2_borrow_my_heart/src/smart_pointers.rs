// Here we will be looking at how smart pointers work in Rust

// Usually, pointers in Rust are denoted using the & symbol
// This means that the value is a reference to another value (a pointer)

// Sometimes this is not enough, and we need more advanced features

// Smart pointers are data structures that act like pointers but may have additional metadata and
// or capabilities
// By implementing the Deref trait, they can act like regular references

use std::{
    cell::RefCell
    ,
    rc::Rc,
};
use std::rc::Weak;
use crate::smart_pointers::dll::{Dll, DllElement};

pub fn smart_pointers() {
    // A box is a smart pointer that allows us to store data on the heap
    // Side note: the way that Rusts initializes a Box is by allocating the
    // memory on the stack and then moving the value into the heap. This is
    // usually optimized away by the compiler, but can lead to stack overflow.
    // There are workarounds for this, but they are not in the scope for now.
    let boxed = Box::new(5);

    // Despite the box owning the i32, we can dereference it and get the contained value
    println!("The value of the box is: {}", *boxed);

    // We can also pass the box to a function
    // As you can see, the compiler is smart enough to dereference the box for us
    takes_reference(&boxed);

    // A very common use case for boxes is when we have a recursive data structure
    // This struct won't compile because it is infinitely sized
    // struct BadListElement {
    //    value: i32,
    //    next: Option<BadListElement>,
    // }

    // We can get it to work using a box
    // Now the size of the struct is known at compile time
    struct ListElement {
        value: i32,
        next: Option<Box<ListElement>>,
    }

    // Box is a fairly simple smart pointer, but there are more advanced ones
    // like Rc, Arc, Mutex, and RefCell.

    // Rc is a reference counted smart pointer

    // We can create a new Rc
    let rc = Rc::new(5);

    // We can clone the Rc
    // The underlying value is not cloned
    let rc2 = rc.clone();
    let rc3 = rc.clone();

    // We can give away a clone of the Rc
    takes_ownership(*rc2);
    // And we can still use the original Rc
    println!("I can still access the Rc content: {}", *rc);

    // However, we can't mutate the Rc
    // *rc3 = 7;

    // To be able to mutate the Rc we can use the RefCell smart pointer
    // RefCell is a smart pointer that allows us to mutate the value inside it
    // even if the pointer is immutable. This is called interior mutability
    let value = Rc::new(RefCell::new(5));
    let value2 = value.clone();
    let value3 = value.clone();

    // We can now mutate the value inside the Rc
    *value.borrow_mut() += 1;

    // We can alo pass the Rc to a function
    takes_mutable_reference(&mut *value2.borrow_mut());

    // And all these changes are reflected in the original Rc
    println!("Content of the RefCell: {}", *value3.borrow());

    // However, this still doesn't allow us to have multiple mutable references
    // let mut1 = value.borrow_mut();
    // let mut2 = value.borrow_mut();
    // This will compile (scary, right?)
    // println!("The value of the RefCell is: {} and: {}", *mut1, *mut2);
    // But it will panic at runtime
    // Meaning that we have to be very careful when using RefCell (or just not use it)

    // Finally, lets look at weak pointers
    // When doing a cyclic reference with Rc we can run into a problem
    // The reference count will never reach zero and the memory will never be freed

    // We can solve this by using a weak pointer
    // A weak pointer is a smart pointer that doesn't increment the reference count

    // Look at the implementation of the Dll and the DllElement
    let mut list = Dll::new(&vec![1, 2, 3, 4]);

    let third = list.ref_at_index(2).unwrap();
    let fourth = list.ref_at_index(3).unwrap();

    match third.upgrade() {
        Some(value) => println!("The value of the third element is: {:?}", value.borrow()),
        None => println!("The third element has been removed"),
    }

    list.drop_after_index(1);

    match fourth.upgrade() {
        Some(value) => println!("The value of the fourth element is: {:?}", value.borrow()),
        None => println!("The fourth element has been dropped"),
    }

    match third.upgrade() {
        Some(value) => println!("The value of the third element is: {:?}", value.borrow()),
        None => println!("The third element has been dropped"),
    }
}

fn takes_reference(s: &i32) {
    println!("I have taken a reference: {}", s);
}

fn takes_ownership(s: i32) {
    println!("I have taken ownership: {}", s);
}

fn takes_mutable_reference(s: &mut i32) {
    *s += 1;
}


pub mod dll;