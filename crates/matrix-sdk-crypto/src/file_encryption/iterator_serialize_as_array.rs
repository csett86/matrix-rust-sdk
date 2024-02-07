// Copyright 2024 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::Cell;

use serde::{ser::SerializeSeq, Serialize};

/// A wrapper around an Iterator that can be serialized into an array (sequence) using serde.
///
/// Warning: this may only be serialized once! If you attempt to serialize again (when the iterator
/// has been used up) we will panic.
///
/// # Example
///
/// ```requires_private_module
/// // Note that we never create a Vec of numbers.
/// let numbers = 1..=5;
/// let wrapper = IteratorSerializeAsArray::new(numbers);
/// let s = serde_json::to_string(&wrapper).unwrap();
/// assert_eq!(s, "[1,2,3,4,5]");
/// ```
pub struct IteratorSerializeAsArray<T, I>(Cell<Option<T>>)
where
    T: Iterator<Item = I>;

impl<T, I> IteratorSerializeAsArray<T, I>
where
    T: Iterator<Item = I>,
{
    pub fn new(iterator: T) -> Self {
        Self(Cell::new(Some(iterator)))
    }
}

impl<T, I> Serialize for IteratorSerializeAsArray<T, I>
where
    T: Iterator<Item = I>,
    I: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let iterator =
            self.0.take().expect("This Iterator has already been serialized. Can't do it again!");

        let mut seq = serializer.serialize_seq(None)?;
        for e in iterator {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializing_an_empty_iterator_produces_an_empty_array() {
        assert_eq!(
            serde_json::to_string(&IteratorSerializeAsArray::new(std::iter::empty::<i32>()))
                .unwrap(),
            "[]"
        );
    }

    #[test]
    fn serializing_an_iterator_creates_an_array() {
        let numbers = 1..=5;
        let wrapper = IteratorSerializeAsArray::new(numbers);
        let s = serde_json::to_string(&wrapper).unwrap();
        assert_eq!(s, "[1,2,3,4,5]");
    }

    #[test]
    #[should_panic = "already been serialized"]
    fn serializing_twice_panics() {
        let numbers = 1..=5;
        let wrapper = IteratorSerializeAsArray::new(numbers);
        serde_json::to_string(&wrapper).unwrap();
        serde_json::to_string(&wrapper).unwrap();
    }
}
