#![allow(dead_code)]

struct MyList<'a, T> {
    elements: &'a [&'a T],
}

impl<'a, T> Iterator for MyList<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None;
        }

        let element: Self::Item = &self.elements[0];

        self.elements = &self.elements[1..];

        Some(element)
    }
}

fn main() {
    let mut e1 = 1;
    let mut e2 = 2;

    let list = MyList { elements: &mut [&mut e1, &mut e2] };

    for e in list {
        print!("{:?} ", e);
    }
}
