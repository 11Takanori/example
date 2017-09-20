use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
     pub fn new() -> Self {
         List { head: None, tail: ptr::null_mut() }
     }

     pub fn push(&'a mut self, elem: T) {
         let new_tail = Box::new(Node {
             elem: elem,
             next: None,
         });

         let new_tail = match self.tail.take() {
             Some(old_tail) => {
                 old_tail.next = Some(new_tail);
                 old_tail.next.as_mut().map(|node| &mut **node)
             }
             None => {
                 self.head = Some(new_tail);
                 self.head.as_mut().map(|node| &mut **node)
             }
         };

         self.tail = new_tail;
     }

     pub fn pop(&mut self) -> Option<T> {
         self.head.take().map(|head| {
             let head = *head;
             self.head = head.next;

             if self.head.is_none() {
                 self.tail = None;
             }

             head.elem
         })
     }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}