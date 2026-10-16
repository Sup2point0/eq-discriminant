If you find this too long:

```rs
std::mem::discriminant(left) == std::mem::discriminant(right)
```

This crate provides:

```rs
eq_discriminant(left, right)
```

Or if you want even shorter:

```rs
eq_discr!(left, right)
```

And if you need to compare multiple:

```rs
eq_discr!(first, second, third)
```
