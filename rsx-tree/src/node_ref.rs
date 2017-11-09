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

use types::{Children, Descendants, Id, Node, Traverse, Tree};

#[derive(Debug)]
pub struct Ref<'a, T: 'a> {
    cached: Option<&'a Node<T>>,
    tree: &'a Tree<T>,
    id: Id<T>
}

impl<'a, T: 'a> Eq for Ref<'a, T> {}

impl<'a, T: 'a> PartialEq for Ref<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a, T: 'a> Copy for Ref<'a, T> {}

impl<'a, T: 'a> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: 'a> Ref<'a, T> {
    pub(crate) fn new(tree: &'a Tree<T>, id: Id<T>) -> Self {
        let cached = tree.arena.get(id);
        Ref { cached, tree, id }
    }

    pub fn tree(&self) -> &'a Tree<T> {
        self.tree
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn try_value(&self) -> Option<&'a T> {
        Some(&self.cached?.value)
    }

    pub fn try_into_value(self) -> Option<&'a T> {
        Some(&self.cached?.value)
    }

    pub fn parent_id(&self) -> Option<Id<T>> {
        self.cached?.parent_id
    }

    pub fn parent(&self) -> Option<Ref<'a, T>> {
        let id = self.parent_id()?;
        Some(self.tree.get(id))
    }

    pub fn prev_sibling_id(&self) -> Option<Id<T>> {
        self.cached?.prev_sibling_id
    }

    pub fn prev_sibling(&self) -> Option<Ref<'a, T>> {
        let id = self.prev_sibling_id()?;
        Some(self.tree.get(id))
    }

    pub fn next_sibling_id(&self) -> Option<Id<T>> {
        self.cached?.next_sibling_id
    }

    pub fn next_sibling(&self) -> Option<Ref<'a, T>> {
        let id = self.next_sibling_id()?;
        Some(self.tree.get(id))
    }

    pub fn first_child_id(&self) -> Option<Id<T>> {
        self.cached?.first_child_id
    }

    pub fn first_child(&self) -> Option<Ref<'a, T>> {
        let id = self.first_child_id()?;
        Some(self.tree.get(id))
    }

    pub fn last_child_id(&self) -> Option<Id<T>> {
        self.cached?.last_child_id
    }

    pub fn last_child(&self) -> Option<Ref<'a, T>> {
        let id = self.last_child_id()?;
        Some(self.tree.get(id))
    }

    pub fn sibling_ids(&self) -> (Option<Id<T>>, Option<Id<T>>) {
        if let Some(node) = self.tree.arena.get(self.id) {
            (node.prev_sibling_id, node.next_sibling_id)
        } else {
            (None, None)
        }
    }

    pub fn edge_ids(&self) -> (Option<Id<T>>, Option<Id<T>>) {
        if let Some(node) = self.tree.arena.get(self.id) {
            (node.first_child_id, node.last_child_id)
        } else {
            (None, None)
        }
    }

    pub fn children_refs_iter(&self) -> Children<'a, T> {
        Children::from(self)
    }

    pub fn children_ids_iter(&self) -> impl Iterator<Item = Id<T>> + 'a {
        self.children_refs_iter().map(|v| v.id())
    }

    pub fn children_values_iter(&self) -> impl Iterator<Item = &'a T> {
        self.children_refs_iter().filter_map(|v| v.try_value())
    }

    pub fn descendants_refs_iter(&self) -> Descendants<'a, T> {
        Descendants::from(self)
    }

    pub fn descendants_ids_iter(&self) -> impl Iterator<Item = Id<T>> + 'a {
        self.descendants_refs_iter().map(|v| v.id())
    }

    pub fn descendants_values_iter(&self) -> impl Iterator<Item = &'a T> {
        self.descendants_refs_iter().filter_map(|v| v.try_value())
    }

    pub fn traverse_edges_iter(&self) -> Traverse<'a, T> {
        Traverse::from(self)
    }

    pub fn traverse_refs_iter(&self) -> impl Iterator<Item = Ref<'a, T>> {
        self.traverse_edges_iter().map(|v| v.node())
    }

    pub fn traverse_ids_iter(&self) -> impl Iterator<Item = Id<T>> + 'a {
        self.traverse_refs_iter().map(|v| v.id())
    }

    pub fn traverse_values_iter(&self) -> impl Iterator<Item = &'a T> {
        self.traverse_refs_iter().filter_map(|v| v.try_value())
    }
}
