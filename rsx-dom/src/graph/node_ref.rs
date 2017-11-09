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

use std::ops::Deref;

use rsx_tree::types::Ref;

use types::{DOMNode, DOMNodeEdgeIds, DOMNodeId, DOMNodeSiblingIds};

#[derive(Debug, PartialEq)]
pub struct DOMArenaRef<'a, E: 'a, S: 'a, C: 'a, L: 'a> {
    raw: Ref<'a, DOMNode<E, S, C, L>>
}

impl<'a, E, S, C, L> From<Ref<'a, DOMNode<E, S, C, L>>> for DOMArenaRef<'a, E, S, C, L> {
    fn from(raw: Ref<'a, DOMNode<E, S, C, L>>) -> Self {
        DOMArenaRef { raw }
    }
}

impl<'a, E, S, C, L> Deref for DOMArenaRef<'a, E, S, C, L> {
    type Target = DOMNode<E, S, C, L>;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, E, S, C, L> DOMArenaRef<'a, E, S, C, L> {
    pub(crate) fn value(&self) -> &'a DOMNode<E, S, C, L> {
        self.raw.try_value().expect("Node deallocated")
    }

    pub(crate) fn into_value(self) -> &'a DOMNode<E, S, C, L> {
        self.raw.try_into_value().expect("Node deallocated")
    }

    pub fn get(&self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRef<'a, E, S, C, L> {
        DOMArenaRef::from(self.raw.tree().get(id))
    }

    pub fn id(&self) -> DOMNodeId<E, S, C, L> {
        self.raw.id()
    }

    pub fn parent_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.parent_id()
    }

    pub fn parent(&self) -> Option<DOMArenaRef<'a, E, S, C, L>> {
        self.raw.parent().map(DOMArenaRef::from)
    }

    pub fn prev_sibling_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.prev_sibling_id()
    }

    pub fn prev_sibling(&self) -> Option<DOMArenaRef<'a, E, S, C, L>> {
        self.raw.prev_sibling().map(DOMArenaRef::from)
    }

    pub fn next_sibling_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.next_sibling_id()
    }

    pub fn next_sibling(&self) -> Option<DOMArenaRef<'a, E, S, C, L>> {
        self.raw.next_sibling().map(DOMArenaRef::from)
    }

    pub fn first_child_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.first_child_id()
    }

    pub fn first_child(&self) -> Option<DOMArenaRef<'a, E, S, C, L>> {
        self.raw.first_child().map(DOMArenaRef::from)
    }

    pub fn last_child_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.last_child_id()
    }

    pub fn last_child(&self) -> Option<DOMArenaRef<'a, E, S, C, L>> {
        self.raw.last_child().map(DOMArenaRef::from)
    }

    pub fn sibling_ids(&self) -> DOMNodeSiblingIds<E, S, C, L> {
        self.raw.sibling_ids()
    }

    pub fn edge_ids(&self) -> DOMNodeEdgeIds<E, S, C, L> {
        self.raw.edge_ids()
    }

    pub fn children_iter(&self) -> impl Iterator<Item = &'a DOMNode<E, S, C, L>> {
        self.raw.children_values_iter()
    }

    pub fn descendants_iter(&self) -> impl Iterator<Item = &'a DOMNode<E, S, C, L>> {
        self.raw.descendants_values_iter()
    }

    pub fn traverse_iter(&self) -> impl Iterator<Item = &'a DOMNode<E, S, C, L>> {
        self.raw.traverse_values_iter()
    }
}
