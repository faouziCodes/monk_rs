# syntax

**variables**

```rust
let x: int = 10
```

**functions**

```rust
let add(a, b) = a + b
let print_a_then_b(a, b) = {
    print(a)
    print(b)
} 

print_a_then_b("a", "b") // because of this call paramaters a and b are infered to be of type string.
```

**while loops**

```rust
let x = 0 // Here the type of x is infered to be int
while(x < 10) {
    x = x + 1
}
```

**for loops**

```rust
for (x <= 0 ..= 10) {
    print(x)
}
```
