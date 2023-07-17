use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }   
    }
    
    fn push(&mut self, x: i32) {
        self.q2.push_front(x);
        if self.q2.len() > 1 {
            self.q1.push_front(self.q2.pop_back().unwrap());
        }
    }
    
    fn pop(&mut self) -> i32 {
        let res = self.q2.pop_back().unwrap();
        while self.q1.len() > 1 {
            self.q2.push_front(self.q1.pop_back().unwrap());
        }
        std::mem::swap(&mut self.q1, &mut self.q2) ;
        res 
    }
    
    fn top(&mut self) -> i32 {
        *self.q2.back().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_my_stack() {
        let mut stack = super::MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.empty(), false);
    }
}