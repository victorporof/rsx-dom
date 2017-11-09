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

use std::borrow::Cow;
use std::rc::Rc;

use rsx_shared::traits::{TComputedStyles, TGenericEvent, TLayoutNode, TStyleDeclarations};

use types::{
    Closure,
    DOMAttribute,
    DOMAttributeName,
    DOMAttributeValue,
    DOMAttributes,
    DOMData,
    DOMNode,
    DOMNormalNode,
    DOMTagName,
    DOMText,
    DOMTextNode,
    DOMTree,
    EventType,
    KnownAttributeName,
    KnownElementName,
    Prop
};

impl<E, S, C, L> From<()> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from(_: ()) -> Self {
        DOMNode::new(DOMData::Void)
    }
}

impl<E, S, C, L> From<&'static str> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from(text: &'static str) -> Self {
        DOMNode::new(DOMData::Text(DOMTextNode {
            content: DOMText::from(text)
        }))
    }
}

impl<E, S, C, L> From<String> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from(text: String) -> Self {
        DOMNode::new(DOMData::Text(DOMTextNode {
            content: DOMText::from(text)
        }))
    }
}

impl_text_node_from_stringifiable!(bool);
impl_text_node_from_stringifiable!(i8);
impl_text_node_from_stringifiable!(u8);
impl_text_node_from_stringifiable!(i16);
impl_text_node_from_stringifiable!(u16);
impl_text_node_from_stringifiable!(i32);
impl_text_node_from_stringifiable!(u32);
impl_text_node_from_stringifiable!(i64);
impl_text_node_from_stringifiable!(u64);
impl_text_node_from_stringifiable!(f32);
impl_text_node_from_stringifiable!(f64);
impl_text_node_from_stringifiable!(isize);
impl_text_node_from_stringifiable!(usize);
impl_text_node_from_stringifiable!(char);

impl<E, S, C, L> From<DOMTagName> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from(tag: DOMTagName) -> Self {
        let attributes = vec![];
        DOMNode::new(DOMData::Normal(DOMNormalNode { tag, attributes }))
    }
}

impl<E, S, C, L> From<(DOMTagName, DOMAttributes<E, S, C, L>)> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from((tag, attributes): (DOMTagName, DOMAttributes<E, S, C, L>)) -> Self {
        DOMNode::new(DOMData::Normal(DOMNormalNode { tag, attributes }))
    }
}

impl<E, S, C, L> From<DOMTree<E, S, C, L>> for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn from(tree: DOMTree<E, S, C, L>) -> Self {
        DOMNode::new(DOMData::ShadowHost(tree))
    }
}

impl From<&'static str> for DOMText {
    fn from(value: &'static str) -> Self {
        DOMText::Static(Cow::from(value))
    }
}

impl From<String> for DOMText {
    fn from(value: String) -> Self {
        DOMText::Owned(Rc::new(value))
    }
}

impl From<KnownElementName> for DOMTagName {
    fn from(name: KnownElementName) -> Self {
        DOMTagName::KnownName(name)
    }
}

impl From<&'static str> for DOMTagName {
    fn from(name: &'static str) -> Self {
        DOMTagName::Simple(name)
    }
}

impl From<(&'static str, &'static str)> for DOMTagName {
    fn from((namespace, name): (&'static str, &'static str)) -> Self {
        DOMTagName::NamedspacedName(namespace, name)
    }
}

impl<E, S, C, L> From<(DOMAttributeName, DOMAttributeValue<E, S, C, L>)> for DOMAttribute<E, S, C, L> {
    fn from((name, value): (DOMAttributeName, DOMAttributeValue<E, S, C, L>)) -> Self {
        DOMAttribute(name, value)
    }
}

impl From<KnownAttributeName> for DOMAttributeName {
    fn from(name: KnownAttributeName) -> Self {
        DOMAttributeName::KnownName(name)
    }
}

impl From<EventType> for DOMAttributeName {
    fn from(name: EventType) -> Self {
        DOMAttributeName::EventType(name)
    }
}

impl From<&'static str> for DOMAttributeName {
    fn from(name: &'static str) -> Self {
        DOMAttributeName::Simple(name)
    }
}

impl From<(&'static str, &'static str)> for DOMAttributeName {
    fn from((namespace, name): (&'static str, &'static str)) -> Self {
        DOMAttributeName::NamedspacedName(namespace, name)
    }
}

impl<E, S, C, L> From<bool> for DOMAttributeValue<E, S, C, L> {
    fn from(value: bool) -> Self {
        DOMAttributeValue::Boolean(value)
    }
}

impl<E, S, C, L> From<f64> for DOMAttributeValue<E, S, C, L> {
    fn from(value: f64) -> Self {
        DOMAttributeValue::Number(value)
    }
}

impl_number_attribute_from_countable!(i8);
impl_number_attribute_from_countable!(u8);
impl_number_attribute_from_countable!(i16);
impl_number_attribute_from_countable!(u16);
impl_number_attribute_from_countable!(i32);
impl_number_attribute_from_countable!(u32);
impl_number_attribute_from_countable!(i64);
impl_number_attribute_from_countable!(u64);
impl_number_attribute_from_countable!(f32);
impl_number_attribute_from_countable!(isize);
impl_number_attribute_from_countable!(usize);

impl<E, S, C, L> From<char> for DOMAttributeValue<E, S, C, L> {
    fn from(value: char) -> Self {
        DOMAttributeValue::Char(value)
    }
}

impl<E, S, C, L> From<&'static str> for DOMAttributeValue<E, S, C, L> {
    fn from(value: &'static str) -> Self {
        DOMAttributeValue::Str(DOMText::from(value))
    }
}

impl<E, S, C, L> From<String> for DOMAttributeValue<E, S, C, L> {
    fn from(value: String) -> Self {
        DOMAttributeValue::Str(DOMText::from(value))
    }
}

impl<E, S, C, L> From<DOMText> for DOMAttributeValue<E, S, C, L> {
    fn from(value: DOMText) -> Self {
        DOMAttributeValue::Str(value)
    }
}

impl<E, S, C, L> From<S> for DOMAttributeValue<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    fn from(value: S) -> Self {
        DOMAttributeValue::Styles(value)
    }
}

impl<E, S, C, L> From<Prop> for DOMAttributeValue<E, S, C, L> {
    fn from(value: Prop) -> Self {
        DOMAttributeValue::Prop(value)
    }
}

impl<E, S, C, L> From<Closure<E>> for DOMAttributeValue<E, S, C, L> {
    fn from(value: Closure<E>) -> Self {
        DOMAttributeValue::EventListener(value)
    }
}

impl<E, S, C, L> From<DOMNode<E, S, C, L>> for DOMAttributeValue<E, S, C, L> {
    fn from(value: DOMNode<E, S, C, L>) -> Self {
        DOMAttributeValue::Node(value)
    }
}
