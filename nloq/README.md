# Natural Object Query Language

The goal of the Natural Object Query Language (or NOQL) is to have an plain old
English representation of an object query

```rust
type QueryList<'a> = Vec<Query<'a>>;

enum Query<'a> {
  Index {
    index: usize,
    from_end: bool
  },
  Key(&'a str)
}
```

where an object can take a list of query segments and either index a list or get a key from a map

## Index

```
the ((first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|#st|#nd|#rd|#th) [to last]|last) item
```

where `#` is a positive integer

## Key

```
the [key|`key`|`multi word key`]
```

handlers of the key can choose to "dehumanize" the key however they want

## Query List

```
(index|key) [of (index|key) [of (index|key) ...]]
```

query segments are chained with `of`
