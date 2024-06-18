pub fn trait_fun() {
    // Lets look at some traits

    // Traits are conceptually similar to interfaces in other languages
    // They define behavior of a type
    // Traits and enums are the two ways to get polymorphism in Rust
    // While with enums every member is always known, traits allow any type
    // that implements the trait to be used in its place (conditions apply)

    // The standart library has a trait called Default
    // It is used to define a default value for a type

    struct MyType {
        value: i32,
    }

    impl Default for MyType {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    // We can now create a new instance of MyType with the default value
    let my_type = MyType::default();

    // Default is useful, not because it is very complicated, but because it is used in many places

    let maybe: Option<MyType> = None;

    // We can use the unwrap_or method to get the value of the Option or a default value
    let my_type = maybe.unwrap_or_default();

    // In rust you will often have types that contain the same/similar data but
    // are still distinct types

    // Lets say we are working with colors
    struct Rgb {
        r: u8,
        g: u8,
        b: u8,
    }

    struct Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    // We can use the From trait to convert between types
    impl From<Rgb> for Rgba {
        fn from(rgb: Rgb) -> Self {
            Self {
                r: rgb.r,
                g: rgb.g,
                b: rgb.b,
                a: 255,
            }
        }
    }

    impl From<Rgba> for Rgb {
        fn from(rgba: Rgba) -> Self {
            Self {
                r: rgba.r,
                g: rgba.g,
                b: rgba.b,
            }
        }
    }

    // We can now convert between the two types
    let rgb = Rgb { r: 0, g: 0, b: 0 };
    let rgba = Rgba::from(rgb);

    // Furthermore, we can use the Into trait to convert between types
    let rgb: Rgb = rgba.into();
    // Into gets automatically implemented for us if we implement From

    // Some traits do not need to be implemented manually
    // For example the Debug and Clone trait

    #[derive(Debug, Clone)]
    struct MyType2 {
        value: i32,
    }

    // We can now print the value of MyType2
    let my_type = MyType2 { value: 0 };
    println!("{:?}", my_type);
    // And we can clone it
    let my_type2 = my_type.clone();

    // Some more types from the standard library are
    // Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug, Clone, and many more

    // Of course we can also define our own traits
    trait MyTrait {
        // We define a function signature
        fn my_function(&self) -> u8;

        // And we can also define a default implementation
        fn my_function2(&self) -> u8 {
            0
        }
    }

    // We can now implement the trait for a type
    impl MyTrait for MyType {
        fn my_function(&self) -> u8 {
            // As is a cast from i32 to u8
            self.value as u8
        }
    }

    // We can now call the function on an instance of MyType
    let my_type = MyType { value: 0 };
    let result = my_type.my_function();
    println!("The result of my_function is: {}", result);
    // We can also call the default implementation
    let result = my_type.my_function2();
    println!("The result of my_function2 is: {}", result);

    // We can use traits as a parameter to a function
    fn print(content: impl MyTrait) {
        // In this scope we can use the functions of the trait
        // But nothing else
        println!(
            "Calling my_function in a function: {}",
            content.my_function()
        );
    }

    // We can now call the function with any type that implements ToString
    print(my_type);

    // Now, there is another keyword besides impl that we can use in function signatures
    // It is dyn

    // dyn is "complicated"
    // It is used to define a trait object, meaning that the compiler no longer knows the type
    // of the object at compile time.

    // This can happen when you store different types in a collection

    impl MyTrait for Rgb {
        fn my_function(&self) -> u8 {
            self.r
        }
    }

    let rgb = Rgb { r: 0, g: 0, b: 0 };
    let my_type = MyType { value: 0 };

    // Do not be confused by the Box, it is a smart pointer that we will talk about later
    let mut vec: Vec<Box<dyn MyTrait>> = Vec::new();
    for element in vec {
        // We can no longer use our function
        // print(*element);

        print_dyn(element);
    }

    fn print_dyn(content: Box<dyn MyTrait>) {
        // In this scope we can use the functions of the trait
        // But nothing else
        println!(
            "Calling my_function in a function: {}",
            content.my_function()
        );
    }

    // Traits can also have blanket implementations
    // This means that we can implement a trait for all types that implement another trait
    trait ToCoolString {
        fn to_cool_string(&self) -> String;
    }

    // We don't need to implement the trait for every type that implements ToString
    impl<T> ToCoolString for T
    where
        T: ToString,
    {
        fn to_cool_string(&self) -> String {
            format!("Cool {}", self.to_string())
        }
    }

    let hey = "Hey";
    println!("{}", hey.to_cool_string());
    // i32 implements Display, and therefore also ToString
    // Meaning that our blanket implementation works for i32
    let number = 42;
    println!("{}", number.to_cool_string());
}
