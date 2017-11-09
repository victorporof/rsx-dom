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

use rsx_tree::types::RefMutPair;

use types::DOMNode;

#[derive(Debug, PartialEq)]
pub struct DOMArenaRefMutPair<'a, E: 'a, S: 'a, C: 'a, L: 'a> {
    raw: RefMutPair<'a, DOMNode<E, S, C, L>>
}

impl<'a, E, S, C, L> From<RefMutPair<'a, DOMNode<E, S, C, L>>> for DOMArenaRefMutPair<'a, E, S, C, L> {
    fn from(raw: RefMutPair<'a, DOMNode<E, S, C, L>>) -> Self {
        DOMArenaRefMutPair { raw }
    }
}

impl<'a, E, S, C, L> DOMArenaRefMutPair<'a, E, S, C, L> {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    pub fn values(&mut self) -> (&mut DOMNode<E, S, C, L>, &mut DOMNode<E, S, C, L>) {
        self.raw.try_values().expect("Nodes deallocated")
    }

    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    pub fn into_values(self) -> (&'a mut DOMNode<E, S, C, L>, &'a mut DOMNode<E, S, C, L>) {
        self.raw.try_into_values().expect("Nodes deallocated")
    }
}
