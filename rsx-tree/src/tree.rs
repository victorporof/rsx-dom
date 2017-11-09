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

#[cfg(all(not(feature = "vec-arena"), not(feature = "hashmap-arena")))]
use rsx_arena::types::VecArena as Arena;

#[cfg(feature = "hashmap-arena")]
use rsx_arena::types::HashmapArena as Arena;
#[cfg(feature = "vec-arena")]
use rsx_arena::types::VecArena as Arena;

use types::{Id, IdPair, Node, Ref, RefMut, RefMutPair};

#[derive(Debug, PartialEq)]
pub struct Tree<T> {
    pub(crate) arena: Arena<Node<T>>,
    pub(crate) root: Id<T>
}

impl<T> Tree<T> {
    pub fn new<U>(root: U) -> Self
    where
        U: Into<T>
    {
        let mut arena = Arena::new();
        let root = arena.alloc(Node::new(U::into(root)));
        Tree { arena, root }
    }

    pub fn root(&self) -> Id<T> {
        self.root
    }

    pub fn alloc<U>(&mut self, value: U) -> Id<T>
    where
        U: Into<T>
    {
        self.arena.alloc(Node::new(U::into(value)))
    }

    pub fn get(&self, id: Id<T>) -> Ref<T> {
        Ref::new(self, id)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> RefMut<T> {
        RefMut::new(self, id)
    }

    pub fn get_mut_pair(&mut self, ids: IdPair<T>) -> RefMutPair<T> {
        RefMutPair::new(self, ids)
    }
}
