// <<<<<<<<<<<<<--------  ABST Implementation ---------->>>>>>>>>

#derive(Debug, Clone)]
enum ABST<T: PartialOrd + Clone> {
	Leaf,
	Node(Node<T>),
}

#derive[(Debug, Clone)]
struct Node<T: PartialOrd + Clone> {
	data: T,
	left: Box<ABST<T>>,
	right: Box<ABST<T>>,
}

impl <T: PartialOrd + Clone> ABST<T> {

	//Creates a new empty tree
	fn new() -> ABST<T> {
		ABST::Leaf
	}
	
	// Determines if this tree is a leaf or a node
	fn is_leaf(&self) -> bool {
		match *self {
			ABST::Leaf => true,
			ABST::Node(_) => false,
		}
	}

	// Determines if this tree contains the passed element
	fn contains(&self, data: T) -> bool {
		match &*self {
			ABST::Leaf => false,
			ABST::Node(n) => if n.data == data { return true; } else { return n.left.contains(data.clone()) || n.right.contains(data.clone()) },
		}
	}
	
	// Inserts data into the tree according to the data's value
	fn insert(self, data: T) -> ABST<T> {
		match self {
			ABST::Node(node) => if node.data > data {
						return ABST::Node(Node {
							data: node.data,
							left: Box::new(node.left.insert(data)),
							right: node.right,
						});
					} else {
						return ABST::Node(Node {
							data: node.data,
							left: node.left,
							right: Box::new(node.right.insert(data)),	
						});
					},
			ABST::Leaf => ABST::Node(Node {
				data: data,
				left: Box::new(ABST::Leaf),
				right: Box::new(ABST::Leaf),
			}),
		}
	}

	// gets the leftmost item of the tree
	fn leftmost(&self) -> ABST<T> {
		match &*self {
			ABST::Leaf => ABST::Leaf,
			ABST::Node(n) => if n.should_return_parent() { return ABST::Node(n.clone()); } else { return n.left.leftmost(); },
		}
	}

	// gets the rightmost item of the tree
	fn rightmost(&self) -> ABST<T> {
		match &*self {
			ABST::Leaf => ABST::Leaf,
			ABST::Node(n) => if n.should_return_parent { return ABST::Node(n.clone()); } else { return n.right.rightmost(); },
		}
	}

	
	// determines if this node should return its parent
	fn should_return_parent(&self) -> bool {
		match &*self {
			ABST::Leaf => true,
			ABST::Node(n) => false,
		}
	}
}



// <<<<<<<<<<<<<<<<<------------------- Cons List Implemenation --------------->>>>>>>>>>>>>.
#[derive(Debug, PartialEq, Clone)]
enum List<T: PartialEq + Clone> {
	Cons(T, Box<List<T>>),
	Empty,
}

// methods for the list
impl <T: PartialEq + Clone> List<T> {
	// returns a new empty list
	fn new() -> List<T> {
		List::Empty
	}

	// returns the length of the list
	fn len(&self) -> u32 {
		match *self {
			List::Cons(_, ref rest) => 1 + rest.len(),
			List::Empty => 0,
		}
	}

	// adds an element to the list
	fn add(self, elem: T) -> List<T> {
		List::Cons(elem, Box::new(self))
	}

	// determines if the list contains the passed element
	fn contains(&self, elem: T) -> bool {
		match &*self {
			List::Empty => false,
			List::Cons(x, rest) => if elem == x.clone() { return true; } else { return rest.clone().contains(elem); },
		}
	}

	// gets the first element of the list
	fn first(&self) -> Option<T> {
		match &*self {
			List::Empty => None,
			List::Cons(x, _) => Some(x.clone()),
		}
	}

	// determines if this list is empty
	fn empty(&self) -> bool {
		match *self {
			List::Cons(_, _) => false,
			List::Empty => true,
		}
	}

	// Determines if this list is the same as the passed list
	fn same(selfm other: List<T>) -> bool {
		if self.empty() && other.empty() {
			return true;
		} else if self.first() == other.first() {
			let rest = self.rest();
			let other_rest = other.rest();
			return rest.same(other_rest);
		} else {
			return false;
		}
	}
}

fn main() {
	// <<<<------------- Tree Exampels -------->>>>>>
	let mut tree = ABST::new();
	tree = tree.insert(4);
	tree = tree.insert(10);
	tree = tree.insert(7);
	tree = tree.insert(2);
	println!("The tree is: {:?}", tree);
	println!("Does the tree contains 1? {:?}", tree.contains(1));
	println!("Does the tree contain 7? {:?}", tree.contains(7));
	println!("The leftmost item in the tree is {:?}", tree.leftmost());
	println!("The rightmost item in the tree is {:?}", tree.rightmost());

	// <<<<----------- List examples -------->>>>>>>
	let mut list = List::new();
	list = list.add(1);
	list = list.add(2);
	list = list.add(3);
	
	let mut list2 = List::new();
	list2 = list2.add(1);
	list2 = list2.add(2);
	
	println!("The list is {:?}", list);
	println!("The length of the list is {:?}", list.len());
	println!("Does the list contain 4? {:?}", list.contains(4));
	println!("Does the list contain 2? {:?}", list.contains(2));
	println!("the first element of the lsit is {:?}", list.first());
	
	let list1_c = list.copy();
	println!("Are these list the same? {:?}", list.same(list1_c));
}











	
