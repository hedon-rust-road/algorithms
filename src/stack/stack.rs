#[derive(Debug)]
pub struct Stack<T> {
    /// 数据
    data: Vec<T>,
    /// 栈大小
    size: usize,
}

impl<T> Stack<T> {
    /// 初始化一个空栈
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 获取栈大小
    pub fn len(&self) -> usize {
        self.size
    }

    /// 从栈顶弹出一个元素
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    /// 将一个元素压入栈顶
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    /// 获取栈顶元素，不从栈中弹出
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }

    /// 获取栈顶元素的可变引用，不从栈中弹出
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data.get_mut(self.size - 1)
        }
    }

    /// 清空栈
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_and_pop() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek_and_peek_mut() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.peek(), Some(&20));
        if let Some(top) = stack.peek_mut() {
            *top = 99;
        }
        assert_eq!(stack.peek(), Some(&99));
    }

    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.clear();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let collected: Vec<_> = stack.iter().cloned().collect();
        assert_eq!(collected, vec![3, 2, 1]);
    }

    #[test]
    fn test_iter_mut() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        for v in stack.iter_mut() {
            *v *= 2;
        }
        let collected: Vec<_> = stack.iter().cloned().collect();
        assert_eq!(collected, vec![6, 4, 2]);
    }
}
