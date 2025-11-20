#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    items: Vec<i32>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut stack = vec![];
        let mut items = vec![];

        for item in nested_list.into_iter().rev() {
            stack.push(item);
        }

        while stack.len() > 0 {
            let item = stack.pop();
            match item {
                None => break,
                Some(NestedInteger::Int(val)) => items.push(val),
                Some(NestedInteger::List(list)) => {
                    for item in list.into_iter().rev() {
                        stack.push(item);
                    }
                }
            }
        }

        Self { items, index: 0 }
    }

    fn next(&mut self) -> i32 {
        self.index += 1;
        self.items[self.index - 1]
    }

    fn has_next(&self) -> bool {
        self.index < self.items.len()
    }
}
