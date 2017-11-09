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

use smallvec::SmallVec;

use hashmap::bucket::HashmapBucket;
use types::ArenaItemId;
use util::as_mut;

#[derive(Debug, PartialEq)]
pub struct HashmapArena<T> {
    buckets: SmallVec<[HashmapBucket<T>; 1]>
}

impl<T> Default for HashmapArena<T> {
    fn default() -> Self {
        HashmapArena {
            buckets: SmallVec::from_buf([HashmapBucket::new()])
        }
    }
}

impl<T> HashmapArena<T> {
    pub fn new() -> Self {
        HashmapArena::default()
    }

    pub fn alloc(&mut self, value: T) -> ArenaItemId<T> {
        self.buckets[0].alloc(value)
    }

    pub fn dealloc(&mut self, id: ArenaItemId<T>) -> Option<T> {
        self.buckets.iter_mut().find(|v| v.owns(id))?.dealloc(id)
    }

    #[inline]
    pub fn get(&self, id: ArenaItemId<T>) -> Option<&T> {
        self.buckets.iter().find(|v| v.owns(id))?.get(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: ArenaItemId<T>) -> Option<&mut T> {
        self.buckets.iter_mut().find(|v| v.owns(id))?.get_mut(id)
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

impl<T> AddAssign<Self> for HashmapArena<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.buckets.extend(rhs.buckets)
    }
}
