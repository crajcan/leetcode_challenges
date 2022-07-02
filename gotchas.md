# Gotchas
    ## Stuff that modern tooling would catch but we have to remember

1. Indexing into a `String`:

```
let c = s.as_bytes(get(i));
```

### Note that `s.chars()` returns a `Chars` iterator and cannot be used for indexing the same way

1. Appending a `char` onto a `String`:
   
```
let char = 'c';
s.push(c);
```

1. Unwrapping a None value gracefully

```
let o_i = None;
o_i.unwrap_or_else(|| {  println!("no value found!"); 45});
```

1. Reversing a range

```
for end in (0..=nums_sub.len()).rev() {
```

1. Creating and using an `Rc<T>`:

```
    let a = Rc::new(ListNode { val: 3, next: None });
    let b = Rc::clone(&a);
    println!("b.val: {}", b.val); 

    rc_helper(Some(b));
```

1. Passing an `Rc<T>`:

```
fn play_with_rc() {
    let a = Rc::new(ListNode { val: 3, next: None });
    let b = Rc::clone(&a);
    rc_helper(Some(b));
}

fn rc_helper(head: Option<Rc<ListNode>>) {
    if let Some(ref node) = head {
        println!("{}", node.val);
    }
}
```

1. Passing an `Rc<T>` when it is wrapped

```
    // these work the same way, in the first, the call to @clone cascades
    // to the rc
    let mut right = Self::inorder_traversal(node.right.clone());
    let mut left = Self::inorder_traversal(match &node.left {
        Some(rc) => Some(Rc::clone(&rc)),
        None => None 
    });
```

2. Creating and using a `RefCell<T>`

```
    use std::cell::Ref;

    let mut a = "foo".to_string();
    let b: RefCell<String> = RefCell::new(a);
    let c: Ref<String> = b.borrow();

    // "foo"
    println!("c: {:?}", c);

    // RefCell { value: "foo" }
    println!("b: {:?}", b);
```

1. Passing a `RefCell<T>`

```
fn play_with_ref_cell() {
    let mut a = "foo".to_string();
    let b = RefCell::new(a);
    refcell_helper(Some(b));
}

fn refcell_helper(head: Option<RefCell<String>>) {
    if let Some(ref node) = head {
        println!("{}", node.borrow());
    }
}
```

1. Mutating a value held in `RefCell<T>`

```
    let mut a = "foo".to_string();
    let b = RefCell::new(a);

    *b.borrow_mut() = "baz".to_string();

    // baz
    println!("b: {:?}", b);
```

1. Max of two numbers

let max = std::cmp::max(4, 5);

1. Max of two or more numbers 

let Some(max) = [1,2,3].iter().max();