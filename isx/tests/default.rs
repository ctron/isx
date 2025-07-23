use isx::IsDefault;

#[derive(IsDefault, Default, Debug, PartialEq)]
struct MyStruct {
    foo: &'static str,
    bar: bool,
}

#[derive(IsDefault, Default, Debug, PartialEq)]
enum MyEnum {
    #[default]
    Unit,
    Tuple(u8, &'static str),
    Struct {
        x: bool,
    },
}

#[test]
fn struct_default() {
    assert!(MyStruct::default().is_default());
    assert!(
        !MyStruct {
            foo: "hi",
            bar: false
        }
        .is_default()
    );
}

#[test]
fn enum_default() {
    assert!(MyEnum::default().is_default());
    assert!(!MyEnum::Tuple(0, "").is_default());
    assert!(!MyEnum::Struct { x: false }.is_default());
}

#[test]
fn unit_default() {
    assert!(().is_default());
}

#[test]
fn option_default() {
    assert!(Option::<u8>::None.is_default());
    assert!(!Some(1u8).is_default());
}

#[cfg(feature = "std")]
#[test]
fn vec_default() {
    assert!(Vec::<u8>::new().is_default());
    assert!(!vec![1u8].is_default());
}

#[cfg(feature = "std")]
#[test]
fn string_default() {
    assert!(String::new().is_default());
    assert!(!"hi".to_string().is_default());
}

#[cfg(feature = "std")]
#[test]
fn hashmap_default() {
    use std::collections::HashMap;
    assert!(HashMap::<u8, u8>::new().is_default());
    assert!(!HashMap::from([(1u8, 2u8)]).is_default());
}

#[cfg(feature = "std")]
#[test]
fn hashset_default() {
    use std::collections::HashSet;
    assert!(HashSet::<u8>::new().is_default());
    assert!(!HashSet::from([1u8]).is_default());
}

#[cfg(feature = "std")]
#[test]
fn btreemap_default() {
    use std::collections::BTreeMap;
    assert!(BTreeMap::<u8, u8>::new().is_default());
    assert!(!BTreeMap::from([(1u8, 2u8)]).is_default());
}

#[cfg(feature = "std")]
#[test]
fn btreeset_default() {
    use std::collections::BTreeSet;
    assert!(BTreeSet::<u8>::new().is_default());
    assert!(!BTreeSet::from([1u8]).is_default());
}

#[cfg(feature = "std")]
#[test]
fn binaryheap_default() {
    use std::collections::BinaryHeap;
    assert!(BinaryHeap::<u8>::new().is_default());
    let mut h = BinaryHeap::new();
    h.push(1u8);
    assert!(!h.is_default());
}
