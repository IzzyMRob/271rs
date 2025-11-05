#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}


pub trait Push<T> {
    fn push(self, val: T) -> Self;
}

pub trait Pop<T> {
    fn pop(self) -> Self;
}


#[derive(Debug)]
pub struct Stack<T> {
    vals: Option<Node<T>>,
}

impl<T> Push<T> for Stack<T> {
    fn push(mut self, val: T) -> Stack<T> {
        // add another node with all previous things inside it?
        let newNode = Node {
            data: val,
            next: Some(Box::new(self.vals.unwrap()))
        };
        return Stack {
            vals: Some(newNode),
        };
    }
}

impl<T> Pop<T> for Stack<T> {
    fn pop(mut self) -> Stack<T> {
        let mut values = (None,None);
         self.vals.unwrap().data;
        return values;
    }
}

pub fn stack<T>() -> Stack<T> {
    return Stack {
        vals: None,
    };
}


#[derive(Debug)]
pub struct Queue<T> {
    vals: Option<Node<T>>,
}

impl<T> Push<T> for Queue<T> {
    fn push(mut self, val: T) -> Queue<T> {
        let newNode = Node {
            data: val,
            next: Some(Box::new(self.vals.unwrap()))
        };
        return Queue {
            vals: Some(newNode),
        };
    }
}

pub fn queue<T>() -> Queue<T> {
    return Queue {
        vals: None,
    };
}
