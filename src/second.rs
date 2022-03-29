pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
	    self.next.map(|node| {
		    self.next = node.next.as_deref();
		    &node.elem
	    })
    }
}

pub struct IterMut<'a, T> {
	next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
		self.next = node.next.as_deref_mut();
		&mut node.elem
	})
    }
}


impl<T> List<T> {
	pub fn iter(&self) -> Iter<T> {
		Iter{next: self.head.as_deref()}
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut { next: self.head.as_deref_mut() }
	}

	pub fn new() -> Self {
		List {head: Option::None}
	}

	pub fn push(&mut self, elem: T) {
		let new_node= Box::new(Node {
			elem,
			next: std::mem::replace(&mut self.head, Option::None)
		});
		self.head = Option::Some(new_node)
	}

	pub fn pop(&mut self) -> Option<T> {
		match std::mem::replace(&mut self.head, Option::None) {
			Option::None => Option::None,
			Option::Some(node) => {
				self.head = node.next;
				Option::Some(node.elem)
			},
		}
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| &node.elem)
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| &mut node.elem)
	}
}

impl <T> Drop for List<T> {
	fn drop(&mut self) {
		let mut current = std::mem::replace(&mut self.head, Option::None);
			while let Option::Some(mut node) = current {
			current = std::mem::replace(&mut node.next, Option::None);
		}
	}
}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn is_empty() {
		let mut list: List<i32> = List::new();
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn basics() {
		let mut list = List::new();

		list.push(1);
		list.push(2);
		list.push(3);

		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2));
		
		list.push(50);

		assert_eq!(list.pop(), Some(50));
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn peek() {
		let mut list = List::new();
		assert_eq!(list.peek(), None);
		list.push(1); list.push(2); list.push(3);

		assert_eq!(list.peek(), Some(&3));
		assert_eq!(list.pop(), Some(3));
	}

	#[test]
	fn peek_mut() {
		let mut list = List::new();
		assert_eq!(list.peek_mut(), None);
		list.push(1); list.push(2); list.push(3);

		assert_eq!(list.peek_mut(), Some(&mut 3));

		list.peek_mut().map(|value| {
		*value = 42
		});

		assert_eq!(list.pop(), Some(42));
	}

	#[test]
	fn into_iter() {
		let mut list = List::new();
		list.push(1); list.push(2); list.push(3);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iter() {
		let mut list = List::new();
		list.push(1); list.push(2); list.push(3);

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&1));
	}

	#[test]
	fn iter_mut() {
		let mut list = List::new();
		list.push(1); list.push(2); list.push(3);

		let mut iter = list.iter_mut();
		assert_eq!(iter.next(), Some(&mut 3));
		assert_eq!(iter.next(), Some(&mut 2));
		assert_eq!(iter.next(), Some(&mut 1));
	}

}
