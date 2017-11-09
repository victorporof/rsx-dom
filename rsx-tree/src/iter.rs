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

use types::Ref;

#[derive(Debug)]
pub struct Children<'a, T: 'a> {
    front: Option<Ref<'a, T>>,
    back: Option<Ref<'a, T>>
}

impl<'a, 'b, T: 'a> From<&'b Ref<'a, T>> for Children<'a, T> {
    fn from(root: &Ref<'a, T>) -> Self {
        Children {
            front: root.first_child(),
            back: root.last_child()
        }
    }
}

impl<'a, T: 'a> Eq for Children<'a, T> {}

impl<'a, T: 'a> PartialEq for Children<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.front == other.front && self.back == other.back
    }
}

impl<'a, T: 'a> Copy for Children<'a, T> {}

impl<'a, T: 'a> Clone for Children<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: 'a> Iterator for Children<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.and_then(|v| v.next_sibling());
            node
        }
    }
}

#[derive(Debug)]
pub enum Edge<'a, T: 'a> {
    Open(Ref<'a, T>),
    Close(Ref<'a, T>)
}

impl<'a, T: 'a> Eq for Edge<'a, T> {}

impl<'a, T: 'a> PartialEq for Edge<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Edge::Open(a), &Edge::Open(b)) | (&Edge::Close(a), &Edge::Close(b)) => a == b,
            _ => false
        }
    }
}

impl<'a, T: 'a> Copy for Edge<'a, T> {}

impl<'a, T: 'a> Clone for Edge<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: 'a> Edge<'a, T> {
    pub fn node(self) -> Ref<'a, T> {
        match self {
            Edge::Open(v) | Edge::Close(v) => v
        }
    }
}

#[derive(Debug)]
pub struct Traverse<'a, T: 'a> {
    root: Ref<'a, T>,
    edge: Option<Edge<'a, T>>
}

impl<'a, 'b, T: 'a> From<&'b Ref<'a, T>> for Traverse<'a, T> {
    fn from(root: &Ref<'a, T>) -> Self {
        let root = *root;
        let edge = None;
        Traverse { root, edge }
    }
}

impl<'a, T: 'a> Eq for Traverse<'a, T> {}

impl<'a, T: 'a> PartialEq for Traverse<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.edge == other.edge
    }
}

impl<'a, T: 'a> Copy for Traverse<'a, T> {}

impl<'a, T: 'a> Clone for Traverse<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: 'a> Iterator for Traverse<'a, T> {
    type Item = Edge<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            None => {
                self.edge = Some(Edge::Open(self.root));
            }
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node));
                }
            }
            Some(Edge::Close(node)) => {
                if node == self.root {
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
        }
        self.edge
    }
}

#[derive(Debug)]
pub struct Descendants<'a, T: 'a> {
    iter: Traverse<'a, T>
}

impl<'a, 'b, T: 'a> From<&'b Ref<'a, T>> for Descendants<'a, T> {
    fn from(root: &Ref<'a, T>) -> Self {
        Descendants {
            iter: Traverse::from(root)
        }
    }
}

impl<'a, T: 'a> Eq for Descendants<'a, T> {}

impl<'a, T: 'a> PartialEq for Descendants<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter
    }
}

impl<'a, T: 'a> Copy for Descendants<'a, T> {}

impl<'a, T: 'a> Clone for Descendants<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: 'a> Iterator for Descendants<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Edge::Open(node)) => return Some(node),
                Some(Edge::Close(_)) => {}
                None => return None
            }
        }
    }
}
