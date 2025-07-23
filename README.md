# is-x

[![crates.io](https://img.shields.io/crates/v/isx.svg)](https://crates.io/crates/isx)
[![docs.rs](https://docs.rs/isx/badge.svg)](https://docs.rs/isx)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/tag/ctron/isx?sort=semver)](https://github.com/ctron/isx/releases)
[![CI](https://github.com/ctron/isx/actions/workflows/ci.yaml/badge.svg)](https://github.com/ctron/isx/actions/workflows/ci.yaml)

Traits for checking certain conditions of values: is empty? is default?

Also see: <https://internals.rust-lang.org/t/traits-for-is-empty-and-or-is-default/21114>

## Examples

For the `IsDefault` trait:

```rust
use isx::IsDefault;

fn test() {
    assert!(false.is_default());
    assert!(true.is_not_default());
}
```

For the `IsEmpty` trait:

```rust
use isx::IsEmpty;

fn test() {
    assert!(vec![].is_empty());
    assert!(None::<()>.is_empty());
}
```

Using a `derive` this can be implemented easily for custom types:

```rust
use isx::{IsDefault, IsEmpty};

#[derive(Default, IsDefault, IsEmpty)]
struct MyStruct {
    s: String,
}
```

## Why?

Because in same cases, it would be great to have a common pattern:

```rust

#[derive(Default, IsDefault, IsEmpty, serde::Serialize, serde::Deserialize)]
struct MySubData {
    // […]
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct MyData {
    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    list: Vec<String>,
    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    map: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    optional: Option<String>,

    #[serde(default, skip_serializing_if = "IsDefault::is_default")]
    flag: bool,

    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    sub_data: MySubData,
}
```

Having that in `std` or `core`, might actually convince people to go for this:

```rust
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct MyData {
    #[serde(default, skip_serializing_empty)]
    list: Vec<String>,
    #[serde(default, skip_serializing_empty)]
    map: HashMap<String, String>,

    #[serde(default, skip_serializing_default)]
    flag: bool,
}
```
