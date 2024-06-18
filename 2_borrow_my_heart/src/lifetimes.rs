pub fn lifetimes() {
    // Sometimes it is necessary to specify the lifetimes of references

    // fn longest_bad(s1: &str, s2: &str) -> &str {
    //     if s1.len() > s2.len() {
    //         s1
    //     } else {
    //         s2
    //     }
    // }

    // This is done using the 'a syntax

    // This is a function that takes two references and returns the longest one
    // The longest one could be either s1 or s2
    // The compiler isn't doesn't know how long the references will live
    // So we need to specify that the references will live for the same amount of time
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
    // due to the function definition. In such a case, the compiler takes the shortest lifetime

    // let s1 = String::from("hello but way longer than the other string");
    // let longest = {
    //     let s2 = String::from("world but shorter");
    //     longest(&s1, &s2)
    // };

    // Lifetime annotations are always required when using references in signatures
    // But in obvious cases, the compiler can elide the annotations
    // The following function signature is equivalent to the one above
    fn find_in_list(list: &Vec<i32>, target: i32) -> Option<&i32> {
        for item in list {
            if *item == target {
                return Some(item);
            }
        }
        None
    }

    // The following function signature is equivalent to the one above
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
    // This means that they will always be available for the duration of the program, so they have
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
