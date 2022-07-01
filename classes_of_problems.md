# Classes of Problems

1. String
1. Other Collection
  1. Linked List
  2. Tree
     1. brute search
        1. left to right traversal (DFS)
        2. tree equality
  3. vector/array
     1. one ptr
     2. two ptrs
        1. finding
           1. two sum
        2. moving
           1. rotate
           2. find zeroes

## When to use recurision

When it is useful for the 'loop' (recursive case) to return information about itself (and the later cases).
If we used imperative programming in these cases we would need some (relatively) global data structure to modify as iterating.

Ex: src/problems/easy/max_depth.rs

## When to use iteration

When we can't 'collapse the problem', likely because each step in the process requires knowledge about the original state of things. eg. When we are mutating a structure in place.

Ex: src/problems/easy/move_zeroes.rs