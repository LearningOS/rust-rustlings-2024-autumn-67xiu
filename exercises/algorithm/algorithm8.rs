/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]  
pub struct Queue<T> {  
    elements: Vec<T>,  
}  

impl<T> Queue<T> {  
    pub fn new() -> Queue<T> {  
        Queue { elements: Vec::new() }  
    }  

    pub fn enqueue(&mut self, value: T) {  
        self.elements.push(value);  
    }  

    pub fn dequeue(&mut self) -> Result<T, &str> {  
        if !self.elements.is_empty() {  
            Ok(self.elements.remove(0))  
        } else {  
            Err("Queue is empty")  
        }  
    }  

    pub fn is_empty(&self) -> bool {  
        self.elements.is_empty()  
    }  
}  

pub struct myStack<T> {  
    q1: Queue<T>,  
}  

impl<T> myStack<T> {  
    pub fn new() -> Self {  
        Self { q1: Queue::new() }  
    }  

    pub fn push(&mut self, elem: T) {  
        // Enqueue the new element to the stack  
        // Invert the order of elements to maintain LIFO behavior  
        let size = self.q1.elements.len();  
        self.q1.enqueue(elem);  
        for _ in 0..size {  
            let value = self.q1.dequeue().unwrap();  
            self.q1.enqueue(value);  
        }  
    }  

    pub fn pop(&mut self) -> Result<T, &str> {  
        self.q1.dequeue().map_err(|_| "Stack is empty")  
    }  

    pub fn is_empty(&self) -> bool {  
        self.q1.is_empty()  
    }  
}  

#[cfg(test)]  
mod tests {  
    use super::*;  
    
    #[test]  
    fn test_queue() {  
        let mut s = myStack::<i32>::new();  
        assert_eq!(s.pop(), Err("Stack is empty"));  
        s.push(1);  
        s.push(2);  
        s.push(3);  
        assert_eq!(s.pop(), Ok(3));  
        assert_eq!(s.pop(), Ok(2));  
        s.push(4);  
        s.push(5);  
        assert_eq!(s.is_empty(), false);  
        assert_eq!(s.pop(), Ok(5));  
        assert_eq!(s.pop(), Ok(4));  
        assert_eq!(s.pop(), Ok(1));  
        assert_eq!(s.pop(), Err("Stack is empty"));  
        assert_eq!(s.is_empty(), true);  
    }  
}