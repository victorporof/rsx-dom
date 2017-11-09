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

use types::Id;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub(crate) parent_id: Option<Id<T>>,
    pub(crate) prev_sibling_id: Option<Id<T>>,
    pub(crate) next_sibling_id: Option<Id<T>>,
    pub(crate) first_child_id: Option<Id<T>>,
    pub(crate) last_child_id: Option<Id<T>>,
    pub(crate) value: T
}

impl<T> Node<T> {
    pub(crate) fn new(value: T) -> Self {
        Node {
            parent_id: None,
            prev_sibling_id: None,
            next_sibling_id: None,
            first_child_id: None,
            last_child_id: None,
            value
        }
    }
}
