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

use types::{IdPair, Tree};

#[derive(Debug)]
pub struct RefMutPair<'a, T: 'a> {
    tree: &'a mut Tree<T>,
    ids: IdPair<T>
}

impl<'a, T: 'a> Eq for RefMutPair<'a, T> {}

impl<'a, T: 'a> PartialEq for RefMutPair<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids
    }
}

impl<'a, T: 'a> RefMutPair<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, ids: IdPair<T>) -> Self {
        RefMutPair { tree, ids }
    }

    pub fn try_values(&mut self) -> Option<(&mut T, &mut T)> {
        let (first, second) = self.tree.arena.get_mut_pair(self.ids.0, self.ids.1);
        Some((&mut first?.value, &mut second?.value))
    }

    pub fn try_into_values(self) -> Option<(&'a mut T, &'a mut T)> {
        let (first, second) = self.tree.arena.get_mut_pair(self.ids.0, self.ids.1);
        Some((&mut first?.value, &mut second?.value))
    }
}
