#[warn(dead_code)]
struct MyList<'a, T> {
    elements: &'a [T],
}

impl<'a> Default for MyList<'a, i32> {
    fn default() -> Self {
        MyList {
            elements: &[1, 2, 3, 4, 5],
        }
    }
}

impl<'a> Iterator for MyList<'a, i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None;
        }

        let element = Some(self.elements[0]);

        self.elements = &self.elements[1..];

        element
    }
}

fn main() {
    for e in MyList::default() {
        print!("{:?} ", e);
    }
}
