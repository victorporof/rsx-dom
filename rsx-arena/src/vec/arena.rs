/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use std::ops::AddAssign;

use types::{ArenaItemId, BucketId, EntryId};
use util::as_mut;

#[derive(Debug)]
pub struct VecArena<T> {
    bucket_id: BucketId<T>,
    vec: Vec<T>
}

impl<T> PartialEq for VecArena<T> {
    fn eq(&self, other: &Self) -> bool {
        self.bucket_id == other.bucket_id
    }
}

impl<T> Default for VecArena<T> {
    fn default() -> Self {
        VecArena {
            bucket_id: BucketId::generate(),
            vec: Vec::with_capacity(1024)
        }
    }
}

impl<T> VecArena<T> {
    pub fn new() -> Self {
        VecArena::default()
    }

    pub fn owns(&self, id: ArenaItemId<T>) -> bool {
        self.bucket_id == id.bucket_id
    }

    pub fn alloc(&mut self, value: T) -> ArenaItemId<T> {
        let item_id = ArenaItemId {
            bucket_id: self.bucket_id,
            entry_id: EntryId::new(self.vec.len() as u32)
        };
        self.vec.push(value);
        item_id
    }

    pub fn dealloc(&mut self, _: ArenaItemId<T>) -> Option<T> {
        unimplemented!()
    }

    #[inline]
    pub fn get(&self, id: ArenaItemId<T>) -> Option<&T> {
        debug_assert_eq!(self.bucket_id, id.bucket_id);
        Some(unsafe { self.vec.get_unchecked(id.entry_id.to_index()) })
    }

    #[inline]
    pub fn get_mut(&mut self, id: ArenaItemId<T>) -> Option<&mut T> {
        debug_assert_eq!(self.bucket_id, id.bucket_id);
        Some(unsafe { self.vec.get_unchecked_mut(id.entry_id.to_index()) })
    }

    #[inline]
    pub unsafe fn get_as_mut<'a>(&mut self, id: ArenaItemId<T>) -> Option<&'a mut T> {
        as_mut(self.get_mut(id))
    }

    #[inline]
    pub fn get_mut_pair(&mut self, first_id: ArenaItemId<T>, second_id: ArenaItemId<T>) -> (Option<&mut T>, Option<&mut T>) {
        assert_ne!(first_id, second_id);
        let first = unsafe { self.get_as_mut(first_id) };
        let second = unsafe { self.get_as_mut(second_id) };
        (first, second)
    }
}

impl<T> AddAssign<Self> for VecArena<T> {
    fn add_assign(&mut self, _: Self) {
        panic!("Extending vec-based arenas is not supported.");
    }
}
