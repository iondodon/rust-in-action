struct MyList<T> {
    elements: [T],
}

impl Iterator for MyList<i32> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
