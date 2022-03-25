pub struct List {
    head: Option<Box<Node>>,
}

struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}


impl List {
	pub fn new() -> Self {
		List {head: Option::None}
	}

	pub fn push(&mut self, elem: i32) {
		let new_node: Box<Node> = Box::new(Node {
			elem,
			next: std::mem::replace(&mut self.head, Option::None)
		});
		self.head = Option::Some(new_node)
	}

	pub fn pop(&mut self) -> Option<i32> {
		match std::mem::replace(&mut self.head, Option::None) {
			Option::None => Option::None,
			Option::Some(node) => {
				self.head = node.next;
				Option::Some(node.elem)
			},
		}
	}
}

impl Drop for List {
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
		let mut list = List::new();
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
}