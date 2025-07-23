use super::{default_impl, empty_impl};
use std::collections::{HashMap, HashSet};

default_impl!(std::path::PathBuf);

empty_impl!(HashMap<K,V>);
empty_impl!(HashSet<T>);
