use isx::IsEmpty;

#[derive(IsEmpty, Default, Debug, PartialEq)]
struct MyStruct {
    foo: &'static str,
    bar: &'static [u8],
}

#[derive(IsEmpty, Default, Debug, PartialEq)]
enum MyEnum {
    #[default]
    Unit,
    Tuple(&'static [u8], &'static str),
    Struct {
        x: &'static [u8],
    },
}

#[test]
fn struct_empty() {
    assert!(MyStruct::default().is_empty());
    assert!(
        !MyStruct {
            foo: "hi",
            bar: &[]
        }
        .is_empty()
    );
}

#[test]
fn enum_empty() {
    assert!(MyEnum::default().is_empty());
    assert!(MyEnum::Tuple(&[], "").is_empty());
    assert!(MyEnum::Struct { x: &[] }.is_empty());
}

#[test]
fn slice_empty() {
    assert!((&[] as &[u8]).is_empty());
    assert!(!(&[1u8, 2u8] as &[u8]).is_empty());
}

#[allow(clippy::const_is_empty)]
#[test]
fn array_empty() {
    assert!([0u8; 0].is_empty());
    assert!(![1u8, 2u8].is_empty());
}

#[test]
fn unit_empty() {
    assert!(().is_empty());
}

#[test]
fn option_empty() {
    assert!(Option::<u8>::None.is_empty());
    assert!(!Some(1u8).is_empty());
}

#[cfg(feature = "std")]
#[test]
fn vec_empty() {
    assert!(Vec::<u8>::new().is_empty());
    assert!(!vec![1u8].is_empty());
}

#[cfg(feature = "std")]
#[test]
fn string_empty() {
    assert!(String::new().is_empty());
    assert!(!"hi".to_string().is_empty());
}

#[cfg(feature = "std")]
#[test]
fn hashmap_empty() {
    use std::collections::HashMap;
    assert!(HashMap::<u8, u8>::new().is_empty());
    assert!(!HashMap::from([(1u8, 2u8)]).is_empty());
}

#[cfg(feature = "std")]
#[test]
fn hashset_empty() {
    use std::collections::HashSet;
    assert!(HashSet::<u8>::new().is_empty());
    assert!(!HashSet::from([1u8]).is_empty());
}

#[cfg(feature = "std")]
#[test]
fn btreemap_empty() {
    use std::collections::BTreeMap;
    assert!(BTreeMap::<u8, u8>::new().is_empty());
    assert!(!BTreeMap::from([(1u8, 2u8)]).is_empty());
}

#[cfg(feature = "std")]
#[test]
fn btreeset_empty() {
    use std::collections::BTreeSet;
    assert!(BTreeSet::<u8>::new().is_empty());
    assert!(!BTreeSet::from([1u8]).is_empty());
}

#[cfg(feature = "std")]
#[test]
fn binaryheap_empty() {
    use std::collections::BinaryHeap;
    assert!(BinaryHeap::<u8>::new().is_empty());
    let mut h = BinaryHeap::new();
    h.push(1u8);
    assert!(!h.is_empty());
}
