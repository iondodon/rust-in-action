#![allow(dead_code)]

struct MyList<'a, T> {
    elements: &'a mut [&'a mut T],
}

impl<'a, T> Iterator for MyList<'a, T> {
    type Item = &'a mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None;
        }

        let element: Self::Item = self.elements[0];

        Some(element)
    }
}

fn main() {
    let mut e1 = 1;
    let mut e2 = 2;

    let list = MyList { elements: &mut [&mut e1, &mut e2] };

    for e in list {
        *e = *e + 1;
        print!("{:?} ", e);
    }
}
