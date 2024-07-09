use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Dll {
    // Only stores a pointer to the head element
    head: Option<Rc<RefCell<DllElement>>>,
}

impl Dll {
    pub fn new(elements: &[i32]) -> Self {
        let mut head = None;
        let mut prev: Option<Rc<RefCell<DllElement>>> = None;

        for element in elements {
            let new_element = Rc::new(RefCell::new(DllElement {
                value: *element,
                next: None,
                prev: None,
            }));

            match prev {
                Some(prev) => {
                    prev.borrow_mut().next = Some(new_element.clone());
                    new_element.borrow_mut().prev = Some(Rc::downgrade(&prev));
                }
                None => {
                    head = Some(new_element.clone());
                }
            }

            prev = Some(new_element);
        }

        Dll { head }
    }

    pub fn ref_at_index(&self, index: usize) -> Option<Weak<RefCell<DllElement>>> {
        let mut current = self.head.clone();
        let mut current_index = 0;

        while let Some(current_element) = current {
            if current_index == index {
                return Some(Rc::downgrade(&current_element));
            }

            current = current_element.borrow().next.clone();
            current_index += 1;
        }

        None
    }

    pub fn drop_after_index(&mut self, index: usize) {
        let mut current = self.head.clone();
        let mut current_index = 0;

        while let Some(current_element) = current {
            if current_index == index {
                let prev = current_element.borrow().prev.clone();

                match prev {
                    None => {
                        // No previous element, this is the head
                        self.head = None;
                    }
                    Some(prev) => {
                        // There is a previous element, make it forget
                        prev.upgrade().unwrap().borrow_mut().next = None;
                    }
                }
            }

            current = current_element.borrow().next.clone();
            current_index += 1;
        }
    }
}

#[derive(Debug)]
pub struct DllElement {
    value: i32,
    // We have a strong reference to the next element
    next: Option<Rc<RefCell<DllElement>>>,
    // We have a weak reference to the previous element
    prev: Option<Weak<RefCell<DllElement>>>,
}
