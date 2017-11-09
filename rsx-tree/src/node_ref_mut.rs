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

use types::{Id, Ref, Tree};

#[derive(Debug)]
pub struct RefMut<'a, T: 'a> {
    tree: &'a mut Tree<T>,
    id: Id<T>
}

impl<'a, T: 'a> Eq for RefMut<'a, T> {}

impl<'a, T: 'a> PartialEq for RefMut<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a, T> Into<Ref<'a, T>> for RefMut<'a, T> {
    fn into(self) -> Ref<'a, T> {
        Ref::new(self.tree, self.id)
    }
}

impl<'a, T: 'a> RefMut<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, id: Id<T>) -> Self {
        RefMut { tree, id }
    }

    pub fn tree(&self) -> &Tree<T> {
        self.tree
    }

    pub fn tree_mut(&mut self) -> &mut Tree<T> {
        &mut self.tree
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn try_value(&self) -> Option<&T> {
        Some(&self.tree.arena.get(self.id)?.value)
    }

    pub fn try_value_mut(&mut self) -> Option<&mut T> {
        Some(&mut self.tree.arena.get_mut(self.id)?.value)
    }

    pub fn try_into_value(self) -> Option<&'a mut T> {
        Some(&mut self.tree.arena.get_mut(self.id)?.value)
    }

    pub fn parent_id(&self) -> Option<Id<T>> {
        self.tree.arena.get(self.id)?.parent_id
    }

    pub fn parent(&mut self) -> Option<RefMut<T>> {
        let id = self.parent_id()?;
        Some(self.tree.get_mut(id))
    }

    pub fn prev_sibling_id(&self) -> Option<Id<T>> {
        self.tree.arena.get(self.id)?.prev_sibling_id
    }

    pub fn prev_sibling(&mut self) -> Option<RefMut<T>> {
        let id = self.prev_sibling_id()?;
        Some(self.tree.get_mut(id))
    }

    pub fn next_sibling_id(&self) -> Option<Id<T>> {
        self.tree.arena.get(self.id)?.next_sibling_id
    }

    pub fn next_sibling(&mut self) -> Option<RefMut<T>> {
        let id = self.next_sibling_id()?;
        Some(self.tree.get_mut(id))
    }

    pub fn first_child_id(&self) -> Option<Id<T>> {
        self.tree.arena.get(self.id)?.first_child_id
    }

    pub fn first_child(&mut self) -> Option<RefMut<T>> {
        let id = self.first_child_id()?;
        Some(self.tree.get_mut(id))
    }

    pub fn last_child_id(&self) -> Option<Id<T>> {
        self.tree.arena.get(self.id)?.last_child_id
    }

    pub fn last_child(&mut self) -> Option<RefMut<T>> {
        let id = self.last_child_id()?;
        Some(self.tree.get_mut(id))
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

    pub fn append_tree(&mut self, other: Tree<T>) -> bool {
        let children: Vec<_> = other.get(other.root()).children_ids_iter().collect();
        self.tree.arena += other.arena;
        children.into_iter().all(|id| self.append_id(id).is_some())
    }

    pub fn prepend_tree(&mut self, other: Tree<T>) -> bool {
        let children: Vec<_> = other.get(other.root()).children_ids_iter().collect();
        self.tree.arena += other.arena;
        children.into_iter().all(|id| self.prepend_id(id).is_some())
    }

    pub fn append(&mut self, value: T) -> RefMut<T> {
        let id = self.tree.alloc(value);
        self.append_id(id);
        self.tree.get_mut(id)
    }

    pub fn prepend(&mut self, value: T) -> RefMut<T> {
        let id = self.tree.alloc(value);
        self.prepend_id(id);
        self.tree.get_mut(id)
    }

    pub fn detach(&mut self) {
        self.detach_own_id();
    }

    pub fn append_id(&mut self, child_id: Id<T>) -> Option<()> {
        let old_last_child_id = self.tree.arena.get(self.id)?.last_child_id;

        // Update the new node's parent and previous sibling.
        {
            let new_child_node = self.tree.arena.get_mut(child_id)?;
            new_child_node.parent_id = Some(self.id);
            new_child_node.prev_sibling_id = old_last_child_id;
        }

        // Update the old last child's next sibling.
        if let Some(old_last_child_id) = old_last_child_id {
            let last_child_node = self.tree.arena.get_mut(old_last_child_id)?;
            last_child_node.next_sibling_id = Some(child_id);
        }

        // Update this node's first and last child.
        {
            let this_node = self.tree.arena.get_mut(self.id)?;
            this_node.last_child_id = Some(child_id);
            if this_node.first_child_id == None {
                this_node.first_child_id = this_node.last_child_id;
            }
        }

        Some(())
    }

    pub fn prepend_id(&mut self, child_id: Id<T>) -> Option<()> {
        let old_first_child_id = self.tree.arena.get(self.id)?.first_child_id;

        // Update the new node's parent and next sibling.
        {
            let new_child_node = self.tree.arena.get_mut(child_id)?;
            new_child_node.parent_id = Some(self.id);
            new_child_node.next_sibling_id = old_first_child_id;
        }

        // Update the old first child's prev sibling.
        if let Some(old_first_child_id) = old_first_child_id {
            let old_first_child_node = self.tree.arena.get_mut(old_first_child_id)?;
            old_first_child_node.prev_sibling_id = Some(child_id);
        }

        // Update this node's first and last child.
        {
            let this_node = self.tree.arena.get_mut(self.id)?;
            this_node.first_child_id = Some(child_id);
            if this_node.last_child_id == None {
                this_node.last_child_id = this_node.first_child_id;
            }
        }

        Some(())
    }

    pub fn detach_own_id(&mut self) -> Option<()> {
        let old_parent_id = self.parent_id()?;
        let (old_prev_sibling_id, old_next_sibling_id) = self.sibling_ids();

        // Update this node's parent, prev and next siblings.
        {
            let this_node = self.tree.arena.get_mut(self.id)?;
            this_node.parent_id = None;
            this_node.prev_sibling_id = None;
            this_node.next_sibling_id = None;
        }

        // Link the old prev and next siblings together.
        if let Some(old_prev_sibling_id) = old_prev_sibling_id {
            let old_prev_sibling_node = self.tree.arena.get_mut(old_prev_sibling_id)?;
            old_prev_sibling_node.next_sibling_id = old_next_sibling_id;
        }

        if let Some(old_next_sibling_id) = old_next_sibling_id {
            let old_next_sibling_node = self.tree.arena.get_mut(old_next_sibling_id)?;
            old_next_sibling_node.prev_sibling_id = old_prev_sibling_id;
        }

        // Update the old parent node's first and last children.
        let old_parent_node = self.tree.arena.get_mut(old_parent_id)?;
        let old_parents_first_child_id = old_parent_node.first_child_id;
        let old_parents_last_child_id = old_parent_node.last_child_id;

        if old_parents_first_child_id == old_parents_last_child_id {
            old_parent_node.first_child_id = None;
            old_parent_node.last_child_id = None;
        } else if old_parents_first_child_id == Some(self.id) {
            old_parent_node.first_child_id = old_next_sibling_id;
            old_parent_node.last_child_id = old_parents_last_child_id;
        } else if old_parents_last_child_id == Some(self.id) {
            old_parent_node.first_child_id = old_parents_first_child_id;
            old_parent_node.last_child_id = old_prev_sibling_id;
        }

        Some(())
    }
}
