# Approach

Create a hashmap containing the locations of all symbols.  
Locations are written as coordinate strings that can be hashed into a set eg:

```py
symbols = {"1,0"}
```

Then, for each number we can check in linear time (with respect to the digit length) for symbols around the number.  
With the hashing technique, we don't lookup directly in the original map.  
Direct lookups would require defensive bound checking which is quite a pain (eg. row 0, col 0 shouldn't be checked above or to the left).

# Rust Issues

Parsing numbers in text is quite hard.  
Originally, I tried manually finding all consecutive digit characters and then converting them manually to a base-10 number.

```rs
let map = "...123";
let digits = vec!['1', '2', '3'];
let answer = 1 * 100 + 2 * 10 + 3 * 1;
```

. This is because there is no in-built support for regex in Rust.

# Why Python is Better

Python's `re` module is perfect for finding numbers, and also in identifying where matches occur through the `Match` object.
