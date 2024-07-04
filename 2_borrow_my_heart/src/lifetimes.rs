pub fn lifetimes() {
    // Lifetimes are the mechanism by with Rust ensures that references are always valid
    // A reference to a value may never outlive the value it references
    // Usually, the compiler can infer the lifetimes of references
    // Sometimes it is necessary to specify the lifetimes of references

    // This function will not compile because the compiler doesn't know what
    // lifetime the returned reference will have

    // fn longest_bad(s1: &str, s2: &str) -> &str {
    //     if s1.len() > s2.len() {
    //         s1
    //     } else {
    //         s2
    //     }
    // }

    // Therefore we need to specify the lifetimes of the references
    // This is done using the 'a syntax

    // This is a function that takes two references and returns the longest one
    // The longest one could be either s1 or s2
    // The compiler can't tell, what the lifetime of the returned reference will be
    // So we need to specify that the returned reference may not outlive s1 and s2
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    // This will work because both strings have the same lifetime
    let s1 = String::from("hello");
    let s2 = String::from("world but longer");
    let result = longest(&s1, &s2); // Should return s2
    println!("The longest string is: {}", result);

    // This will not work because the strings have different lifetimes
    // s2 is created inside the block and will be dropped at the end of the block
    // And despite that s1 would be returned, the lifetimes for s1 and s2 are equivalent
    // due to the function definition.
    // The compiler will always take the shorter lifetime

    // let s1 = String::from("hello but way longer than the other string"); // s1 valid
    // let longest = {
    //     let s2 = String::from("world but shorter");  //s1 & s2 valid
    //     longest(&s1, &s2)                            // s1 & s2 valid
    //     // s2 is dropped here                        // s2 dropped
    // };
    // println!("The longest string is: {}", longest); // longest could reference s2, therefore invalid



    // Lifetime annotations are required when returning a reference from a function
    // It should make intuitive sense, that this requires at least one reference to be passed in.
    // But in obvious cases, the compiler can elide the annotations
    fn find_in_list(list: &Vec<i32>, target: i32) -> Option<&i32> {
        for item in list {
            if *item == target {
                return Some(item);
            }
        }
        None
    }

    // The following function signature is equivalent to the one above (minus the name)
    fn find_in_list_annotated<'a>(list: &'a Vec<i32>, target: i32) -> Option<&'a i32> {
        for item in list {
            if *item == target {
                return Some(item);
            }
        }
        None
    }

    // When a struct holds a reference, the struct must have a lifetime annotation
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // A function can have multiple lifetime annotations
    fn find_by_keyword<'a, 'b>(
        text: &'a str,
        // If we change the lifetime to 'a, the call will not compile
        keyword: &'b str,
    ) -> Result<ImportantExcerpt<'a>, ()> {
        let part = text.split(keyword).next().ok_or(())?;
        Ok(ImportantExcerpt { part })
    }

    // Since the lifetime of the returned struct is independent of 'b, this will compile
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let excerpt = {
        let keyword = String::from("fox");
        find_by_keyword(&text, &keyword).unwrap()
    };

    println!("The excerpt is: {}", excerpt.part);

    // The reason we have been using String::from is because string literals have the 'static lifetime
    // This means that they will be available for the duration of the program, so they have
    // the longest possible lifetime

    // This will compile even if both lifetime annotations are 'a
    // Since both have a lifetime of 'static, the references will always be valid
    let text = "The quick brown fox jumps over the lazy dog";
    let excerpt = {
        let keyword = "fox";
        find_by_keyword(text, keyword).unwrap()
    };

    println!("The excerpt is: {}", excerpt.part);
}
