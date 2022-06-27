# Gotchas
## Stuff that modern tooling would catch but we have to remember

1. Indexing into a `String`:

```
let c = s.as_bytes(get(i));
```

### Note that `s.chars()` returns a `Chars` iterator and cannot be used for indexing the same way

2. Appending a `char` onto a `String`:
   
```
let char = 'c';
s.push(c);
```

3. Unwrapping a None value gracefully

```
let o_i = None;
o_i.unwrap_or_else(|| {  println!("no value found!"); 45});
```

4. Reversing a range

```
for end in (0..=nums_sub.len()).rev() {
```
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    