Natural Language Structured Documents or NLSDs for short are English representations of serializable/deserializable data. The purpose of such a document is to make equal the understanding of data and the data itself. This is important for example when users sign off on data (e.g. with a cryptographic signature) that the presentation of the data itself is what they are signing.

An NLSD attempts to represent the following data and datatypes

```
Null
Boolean
Number
String
List
Map
NamedList
NamedMap
Enum
```

## Null

```
empty | nothing
```

## Boolean

```
true | false | on | off | enabled | disabled
```

## Number

```
0.0
```

## String

```
`string`
```

Strings need to escape ``` with `\``

## List

```
the [empty] (list|`variant`) [henceforth `name`] [where an item is ___] [and another item [of `name`] is ___]
```

When deserializing variant if present will get converted into `UpperCamelCase`

## Map

```
the [empty] (object|`variant`) [henceforth `name`] [where [the] `key` is ___] [and [the] `key` [of `name`] is ___]
```

Note the ``key`` will replaced with `snake_case` when parsing for easier mapping. 

Notice the `the` is optional when building out key/value pairs. When serializing the `the` will always be placed *unless* the ``key`` begins with `is` denoting an adjective (e.g. "is_enabled")

## Enum (New Type Variant)

```
the `type` which is ___
```

# Names

Names are important when deserializing nested structures. The `'name'` must be in scope when deserializing in order to associate the object with its parent. When searching for a `'name'` the deserializer will walk up the tree to find the name. When the `'name'` is found the new scope is set to that level in the tree. If no `'name'` is provided the current scope 

When serializing an object to NLSD, `henceforth 'name'` will be added unless the object is the leaf of the tree, and `of 'name'` will be added to keys/items unless the scope referred to is a parent of the current scope

default name for or a list item

```
[parent name] item [index number]
```

default name for a struct or map item

```
[parent name] [key]
```

where the root name is 'the list' for a list, 'the object' for an object and the struct name for a struct

# Examples

## User

```rust
struct User {
  id: u32,
  public_key: String,
}

let user = User { id: 1, public_key: "A7sg..." };
```

```
the `user` where the `id` is 1 and the `public key` is `A7sg...`
```

## Transactions

```rust
struct Transaction {
  from: String,
  to: String,
  amount: f64
}

let transactions = vec![
  Transaction { from: "address1", to: "address2", amount: 10.2 },
  Transaction { from: "address2", to: "address3", amount: 5.8 },
];
```

```
the list henceforth `the list` where an item is the `transaction` where the `from` is `address1` and the `to` is `address2` and the `amount` is 10.2 and another item of `the list` is the `transaction` where the `from` is `address2` and the `to` is `address3` and the `amount` is 5.8
```
