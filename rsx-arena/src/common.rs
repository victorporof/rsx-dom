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

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(PartialOrd, Ord)]
pub(crate) struct BucketId<T>(u32, PhantomData<T>);

impl<T> fmt::Debug for BucketId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "BucketId({})", self.0)
    }
}

impl<T> Eq for BucketId<T> {}

impl<T> PartialEq for BucketId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Copy for BucketId<T> {}

impl<T> Clone for BucketId<T> {
    fn clone(&self) -> BucketId<T> {
        *self
    }
}

impl<T> Hash for BucketId<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
    }
}

impl<T> BucketId<T> {
    pub(crate) fn new(value: u32) -> Self {
        BucketId(value, PhantomData)
    }

    pub(crate) fn generate() -> Self {
        static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
        BucketId::new(NEXT_ID.fetch_add(1, Ordering::Relaxed) as u32)
    }
}

#[derive(PartialOrd, Ord)]
pub(crate) struct EntryId<T>(u32, PhantomData<T>);

impl<T> fmt::Debug for EntryId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "EntryId({})", self.0)
    }
}

impl<T> Eq for EntryId<T> {}

impl<T> PartialEq for EntryId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Copy for EntryId<T> {}

impl<T> Clone for EntryId<T> {
    fn clone(&self) -> EntryId<T> {
        *self
    }
}

impl<T> Hash for EntryId<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
    }
}

impl<T> EntryId<T> {
    pub(crate) fn new(value: u32) -> Self {
        EntryId(value, PhantomData)
    }

    pub(crate) fn generate(&mut self) -> Self {
        let index = self.0;
        self.0 += 1;
        EntryId::new(index)
    }

    pub(crate) fn to_index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(PartialOrd, Ord)]
pub struct ArenaItemId<T> {
    pub(crate) bucket_id: BucketId<T>,
    pub(crate) entry_id: EntryId<T>
}

impl<T> fmt::Debug for ArenaItemId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "ArenaItemId({:?}, {:?})", self.bucket_id, self.entry_id)
    }
}

impl<T> Eq for ArenaItemId<T> {}

impl<T> PartialEq for ArenaItemId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.bucket_id == other.bucket_id && self.entry_id == other.entry_id
    }
}

impl<T> Copy for ArenaItemId<T> {}

impl<T> Clone for ArenaItemId<T> {
    fn clone(&self) -> ArenaItemId<T> {
        *self
    }
}

impl<T> Hash for ArenaItemId<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.bucket_id.hash(hasher);
        self.entry_id.hash(hasher);
    }
}

impl<T> ToPrimitive for ArenaItemId<T> {
    fn to_i64(&self) -> Option<i64> {
        None
    }

    fn to_u64(&self) -> Option<u64> {
        let bucket_id = u64::from(self.bucket_id.0);
        let entry_id = u64::from(self.entry_id.0);
        Some(bucket_id << 32 | entry_id)
    }
}

impl<T> FromPrimitive for ArenaItemId<T> {
    fn from_i64(_: i64) -> Option<Self> {
        None
    }

    fn from_u64(value: u64) -> Option<Self> {
        let bucket_id = BucketId::new(((value >> 32) & 0xffff_ffff) as u32);
        let entry_id = EntryId::new((value & 0xffff_ffff) as u32);
        Some(ArenaItemId {
            bucket_id,
            entry_id
        })
    }
}
