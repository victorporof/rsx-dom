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

use std::ops::{Deref, DerefMut};

use rsx_shared::traits::{TComputedStyles, TDOMNode, TGenericEvent, TLayoutNode, TStyleDeclarations};
use rsx_tree::types::{Ref, RefMut};

use types::{DOMArenaRef, DOMArenaRefMutPair, DOMNode, DOMNodeEdgeIds, DOMNodeId, DOMNodeIdPair, DOMNodeSiblingIds, DOMTree};

#[derive(Debug, PartialEq)]
pub struct DOMArenaRefMut<'a, E: 'a, S: 'a, C: 'a, L: 'a> {
    raw: RefMut<'a, DOMNode<E, S, C, L>>
}

impl<'a, E, S, C, L> From<RefMut<'a, DOMNode<E, S, C, L>>> for DOMArenaRefMut<'a, E, S, C, L> {
    fn from(raw: RefMut<'a, DOMNode<E, S, C, L>>) -> Self {
        DOMArenaRefMut { raw }
    }
}

impl<'a, E, S, C, L> Into<DOMArenaRef<'a, E, S, C, L>> for DOMArenaRefMut<'a, E, S, C, L> {
    fn into(self) -> DOMArenaRef<'a, E, S, C, L> {
        DOMArenaRef::from(Into::<Ref<DOMNode<E, S, C, L>>>::into(self.raw))
    }
}

impl<'a, E, S, C, L> Deref for DOMArenaRefMut<'a, E, S, C, L> {
    type Target = DOMNode<E, S, C, L>;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, E, S, C, L> DerefMut for DOMArenaRefMut<'a, E, S, C, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}

impl<'a, E, S, C, L> DOMArenaRefMut<'a, E, S, C, L> {
    pub(crate) fn value(&self) -> &DOMNode<E, S, C, L> {
        self.raw.try_value().expect("Node deallocated")
    }

    pub(crate) fn value_mut(&mut self) -> &mut DOMNode<E, S, C, L> {
        self.raw.try_value_mut().expect("Node deallocated")
    }

    pub(crate) fn into_value(self) -> &'a mut DOMNode<E, S, C, L> {
        self.raw.try_into_value().expect("Node deallocated")
    }

    pub fn get(&mut self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRef<E, S, C, L> {
        DOMArenaRef::from(self.raw.tree().get(id))
    }

    pub fn get_mut(&mut self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRefMut<E, S, C, L> {
        DOMArenaRefMut::from(self.raw.tree_mut().get_mut(id))
    }

    pub fn get_mut_pair(&mut self, ids: DOMNodeIdPair<E, S, C, L>) -> DOMArenaRefMutPair<E, S, C, L> {
        DOMArenaRefMutPair::from(self.raw.tree_mut().get_mut_pair(ids))
    }

    pub fn get_mut_self_and(&mut self, id: DOMNodeId<E, S, C, L>) -> DOMArenaRefMutPair<E, S, C, L> {
        let ids = (self.id(), id);
        DOMArenaRefMutPair::from(self.raw.tree_mut().get_mut_pair(ids))
    }

    pub fn id(&self) -> DOMNodeId<E, S, C, L> {
        self.raw.id()
    }

    pub fn parent_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.parent_id()
    }

    pub fn parent(&mut self) -> Option<DOMArenaRefMut<E, S, C, L>> {
        self.raw.parent().map(DOMArenaRefMut::from)
    }

    pub fn prev_sibling_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.prev_sibling_id()
    }

    pub fn prev_sibling(&mut self) -> Option<DOMArenaRefMut<E, S, C, L>> {
        self.raw.prev_sibling().map(DOMArenaRefMut::from)
    }

    pub fn next_sibling_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.next_sibling_id()
    }

    pub fn next_sibling(&mut self) -> Option<DOMArenaRefMut<E, S, C, L>> {
        self.raw.next_sibling().map(DOMArenaRefMut::from)
    }

    pub fn first_child_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.first_child_id()
    }

    pub fn first_child(&mut self) -> Option<DOMArenaRefMut<E, S, C, L>> {
        self.raw.first_child().map(DOMArenaRefMut::from)
    }

    pub fn last_child_id(&self) -> Option<DOMNodeId<E, S, C, L>> {
        self.raw.last_child_id()
    }

    pub fn last_child(&mut self) -> Option<DOMArenaRefMut<E, S, C, L>> {
        self.raw.last_child().map(DOMArenaRefMut::from)
    }

    pub fn sibling_ids(&self) -> DOMNodeSiblingIds<E, S, C, L> {
        self.raw.sibling_ids()
    }

    pub fn edge_ids(&self) -> DOMNodeEdgeIds<E, S, C, L> {
        self.raw.edge_ids()
    }

    pub fn append_tree(&mut self, other: DOMTree<E, S, C, L>) -> bool {
        self.raw.append_tree(other.into_inner())
    }

    pub fn prepend_tree(&mut self, other: DOMTree<E, S, C, L>) -> bool {
        self.raw.prepend_tree(other.into_inner())
    }

    pub fn append(&mut self, node: DOMNode<E, S, C, L>) -> DOMArenaRefMut<E, S, C, L> {
        DOMArenaRefMut::from(self.raw.append(node))
    }

    pub fn prepend(&mut self, node: DOMNode<E, S, C, L>) -> DOMArenaRefMut<E, S, C, L> {
        DOMArenaRefMut::from(self.raw.prepend(node))
    }

    pub fn detach(&mut self) {
        self.raw.detach();
    }
}

impl<'a, E, S, C, L> DOMArenaRefMut<'a, E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    pub fn append_with_layout(&mut self, child_id: DOMNodeId<E, S, C, L>, resources: &L::Resources) -> Result<(), ()>
    where
        L: TLayoutNode<TextMeasureMetadata = C, ImageMeasureMetadata = (), NormalMeasureMetadata = !>
    {
        debug_assert_eq!(self.get(child_id).parent_id(), None);

        self.raw.append_id(child_id);

        let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
        child_node.apply_measurement_metadata_to_layout(resources, &this_node.computed_styles);
        child_node.append_to_layout_node(this_node);

        Ok(())
    }

    pub fn remove_with_layout(&mut self, child_id: DOMNodeId<E, S, C, L>) -> Result<(), ()> {
        debug_assert_eq!(self.get(child_id).parent_id(), Some(self.id()));

        self.raw.tree_mut().get_mut(child_id).detach();

        let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
        child_node.remove_from_layout_node(this_node);

        Ok(())
    }

    pub fn build_layout(&mut self, resources: &L::Resources)
    where
        L: TLayoutNode<TextMeasureMetadata = C, ImageMeasureMetadata = (), NormalMeasureMetadata = !>
    {
        debug_assert_eq!(self.layout_node().is_tainted(), false);

        let mut next_child_id = self.first_child_id();
        while let Some(child_id) = next_child_id {
            {
                let mut child_ref = self.get_mut(child_id);
                child_ref.build_layout(resources);
            }
            {
                let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
                child_node.apply_measurement_metadata_to_layout(resources, &this_node.computed_styles);
                child_node.append_to_layout_node(this_node);
            }
            next_child_id = self.get(child_id).next_sibling_id();
        }
    }
}
