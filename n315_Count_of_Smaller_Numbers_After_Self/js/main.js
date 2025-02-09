class BinaryTreeNode {
	constructor(key, left, right) {
		this.key = key;
		this.left = left;
		this.right = right;
	}

	get_left() {
		return (this.left);
	}

	get_right() {
		return (this.right);
	}

}

function display(node) {
	if (this.get_left == null) {
		console.log("|")
		console.log(this.left, "Leaf")
	} else {
		console.log("|-")
		display(this.get_left);
	}

	if (this.get_right == null) {
		console.log("|")
		console.log(this.right, "Leaf")
	} else {
		console.log("|-")
		display(this.get_right);
	}
}

function create_binary_tree_from_vector (v) {
	let len = v.length;

	let mid_rounded_up = Math.ceil((len / 2)); 

	let left = v.slice(0, mid_rounded_up -1);
	let mid = v[mid_rounded_up -1];
	let right = v.slice(mid_rounded_up, len);

	// console.log(left, mid, right);

	if (len > 2) {
		return (new BinaryTreeNode (mid, create_binary_tree_from_vector(left), create_binary_tree_from_vector(right)))
	} else if (len == 2) {
		return (new BinaryTreeNode (mid, null, create_binary_tree_from_vector(right)))
	} else {
		return (new BinaryTreeNode (mid, null, null))
	}
}

let c = create_binary_tree_from_vector([1, 1, 4, 5, 9, 9, 1]);

display(c);
console.log(c);
