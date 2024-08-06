use std::collections::HashMap;

pub trait StructFields<K,V> {
    fn fields(&self) -> HashMap<K,V>;
}
