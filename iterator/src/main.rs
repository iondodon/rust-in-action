#[warn(dead_code)]
struct MyList<T> {
    index: usize,
    size: usize,
    elements: [T; 5],
}

impl Default for MyList<i32> {
    fn default() -> Self {
        MyList {
            index: 0,
            size: 5,
            elements: [1, 2, 3, 4, 5],
        }
    }
}

impl Iterator for MyList<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.size {
            return None;
        }

        let element = Some(self.elements[self.index]);
        self.index += 1;

        element
    }
}

fn main() {
    let list = MyList::default();

    for e in list {
        println!("{:?}", e);
    }
}
