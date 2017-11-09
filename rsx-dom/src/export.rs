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

use std::convert::TryInto;

use rsx_shared::traits::{TComputedStyles, TGenericEvent, TLayoutNode, TStyleDeclarations};

use types::{DOMData, DOMTagName, KnownElementName};

impl<'a, E, S, C, L> TryInto<KnownElementName> for &'a DOMData<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    type Error = ();

    fn try_into(self) -> Result<KnownElementName, Self::Error> {
        if let Some(&DOMTagName::KnownName(name)) = self.tag() {
            Ok(name)
        } else {
            Err(())
        }
    }
}
