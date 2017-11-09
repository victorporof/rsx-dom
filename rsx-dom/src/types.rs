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

use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::rc::Rc;

use rsx_shared::traits::{TComputedStyles, TDOMNode, TDOMText, TGenericEvent, TLayoutNode, TStyleDeclarations};

#[cfg(feature = "rsx-parse")]
pub use rsx_parser::types::*;
pub use rsx_shared::types::{Closure, EventType, KnownAttributeName, KnownElementName, Prop};

use util::{find_src, is_event_listener, is_style};

pub use graph::*;

pub type DOMAttributes<E, S, C, L> = Vec<DOMAttribute<E, S, C, L>>;
pub type DOMChildren<E, S, C, L> = Vec<DOMNodeId<E, S, C, L>>;

#[derive(Debug, PartialEq)]
pub struct DOMNode<E, S, C, L> {
    pub(crate) data: DOMData<E, S, C, L>,
    pub(crate) computed_styles: C,
    pub(crate) layout_node: L
}

impl<E, S, C, L> Default for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    fn default() -> Self {
        DOMNode::from(DOMTagName::from(KnownElementName::Fragment))
    }
}

impl<E, S, C, L> DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles<Styles = S>,
    L: TLayoutNode<Styles = S>
{
    pub fn new(data: DOMData<E, S, C, L>) -> Self {
        let user_agent_styles = S::make_user_agent_styles(&data);

        let mut computed_styles = C::make_initial_computed_styles(&data);
        computed_styles.apply_styles(&user_agent_styles);
        computed_styles.apply_rules(data.get_styles());

        let mut layout_node = L::make_initial_layout_node(&data);
        layout_node.apply_styles(&user_agent_styles);
        layout_node.apply_rules(data.get_styles());

        DOMNode {
            computed_styles,
            layout_node,
            data
        }
    }

    pub fn shadow_dom(self) -> DOMTree<E, S, C, L> {
        match self.data {
            DOMData::ShadowHost(tree) => tree,
            DOMData::Void | DOMData::Text(_) | DOMData::Normal(_) => DOMTree::default()
        }
    }
}

impl<E, S, C, L> TDOMNode for DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    type Id = DOMNodeId<E, S, C, L>;
    type Data = DOMData<E, S, C, L>;
    type Event = E;
    type Styles = S;
    type ComputedStyles = C;
    type LayoutNode = L;

    fn data(&self) -> &DOMData<E, S, C, L> {
        &self.data
    }

    fn is_void(&self) -> bool {
        self.data.is_void()
    }

    fn is_shadow_host(&self) -> bool {
        self.data.is_shadow_host().is_some()
    }

    fn is_text(&self) -> bool {
        self.data.is_text().is_some()
    }

    fn is_normal(&self) -> bool {
        self.data.is_normal().is_some()
    }

    fn is_known(&self, name: KnownElementName) -> bool {
        self.data.is_known(name).is_some()
    }

    fn computed_styles(&self) -> &C {
        &self.computed_styles
    }

    fn layout_node(&self) -> &L {
        &self.layout_node
    }

    fn reflow_subtree(&mut self, width: u32, height: u32, direction: L::ReflowDirection) {
        self.layout_node.reflow_subtree(width, height, direction);
    }

    fn set_computed_client_position(&mut self, computed: L::ClientPosition) {
        self.layout_node.set_computed_client_position(computed)
    }

    fn get_local_bounding_client_rect(&self) -> L::BoundingClientRect {
        self.layout_node.get_local_bounding_client_rect()
    }

    fn get_global_bounding_client_rect(&self) -> L::BoundingClientRect {
        self.layout_node.get_global_bounding_client_rect()
    }

    fn get_measured_image(&self) -> &L::MeasuredImage {
        self.layout_node.get_measured_image()
    }

    fn get_shaped_text(&self) -> &L::ShapedText {
        self.layout_node.get_shaped_text()
    }
}

impl<E, S, C, L> DOMNode<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    pub fn set_text_content(&mut self, text: String, resources: &L::Resources)
    where
        L: TLayoutNode<TextMeasureMetadata = C>
    {
        let data = &mut self.data;
        let computed_styles = &self.computed_styles;
        let layout_node = &mut self.layout_node;

        let content = DOMText::from(text);
        layout_node.measure_self_as_text(resources, &content, computed_styles);
        *data = DOMData::Text(DOMTextNode { content });
    }

    pub fn reset_styles(&mut self) -> Option<()>
    where
        C: TComputedStyles<Styles = S>,
        L: TLayoutNode<Styles = S>
    {
        self.data.drop_styles()?;

        let user_agent_styles = S::make_user_agent_styles(&self.data);

        self.computed_styles.reset_custom_styles(&self.data);
        self.computed_styles.apply_styles(&user_agent_styles);

        self.layout_node.reset_custom_styles(&self.data);
        self.layout_node.apply_styles(&user_agent_styles);

        Some(())
    }

    pub fn apply_styles(&mut self, styles: S) -> Option<()>
    where
        C: TComputedStyles<Styles = S>,
        L: TLayoutNode<Styles = S>
    {
        self.data.attributes_mut()?.push(DOMAttribute::from((
            DOMAttributeName::from(KnownAttributeName::Style),
            DOMAttributeValue::from(styles)
        )));

        self.computed_styles.apply_rules(self.data.get_styles());
        self.layout_node.apply_rules(self.data.get_styles());

        // TODO: Update measurement metadata for the layout node if necessary.
        // E.g. if the font size changed the text measurement changes.

        Some(())
    }

    pub(crate) fn apply_measurement_metadata_to_layout(&mut self, resources: &L::Resources, inherited_styles: &C)
    where
        L: TLayoutNode<TextMeasureMetadata = C, ImageMeasureMetadata = (), NormalMeasureMetadata = !>
    {
        use self::KnownElementName::*;

        let data = &self.data;
        let computed_styles = &mut self.computed_styles;
        let layout_node = &mut self.layout_node;

        if let Some(text) = data.text() {
            computed_styles.inherit_styles(inherited_styles);
            layout_node.measure_self_as_text(resources, text, computed_styles);
        } else if let Some(src) = data.is_known(Image).and(find_src(data.get_attributes())) {
            layout_node.measure_self_as_image(resources, src, &());
        }
    }

    pub(crate) fn append_to_layout_node(&mut self, parent: &mut DOMNode<E, S, C, L>) {
        let parent = &mut parent.layout_node;
        let child = &mut self.layout_node;
        parent.append_child(child);
    }

    pub(crate) fn remove_from_layout_node(&mut self, parent: &mut DOMNode<E, S, C, L>) {
        let parent = &mut parent.layout_node;
        let child = &mut self.layout_node;
        parent.remove_child(child);
    }
}

#[derive(Debug, Eq, Ord, Clone, Serialize, Deserialize)]
pub enum DOMText {
    Static(Cow<'static, str>),
    Owned(Rc<String>)
}

impl TDOMText for DOMText {}

impl PartialEq for DOMText {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialOrd for DOMText {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Borrow<str> for DOMText {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for DOMText {
    fn as_ref(&self) -> &str {
        match self {
            &DOMText::Static(ref v) => v.as_ref(),
            &DOMText::Owned(ref v) => v.as_ref()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DOMData<E, S, C, L> {
    Void,
    ShadowHost(DOMTree<E, S, C, L>),
    Text(DOMTextNode),
    Normal(DOMNormalNode<E, S, C, L>)
}

#[allow(dead_code)]
impl<E, S, C, L> DOMData<E, S, C, L> {
    pub fn text(&self) -> Option<&DOMText> {
        match self {
            &DOMData::Text(DOMTextNode { ref content }) => Some(content),
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Normal(_) => None
        }
    }

    pub fn tag(&self) -> Option<&DOMTagName> {
        match self {
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Text(_) => None,
            &DOMData::Normal(DOMNormalNode { ref tag, .. }) => Some(tag)
        }
    }

    pub(crate) fn is_void(&self) -> bool {
        match self {
            &DOMData::Void => true,
            _ => false
        }
    }

    pub(crate) fn is_shadow_host(&self) -> Option<&DOMTree<E, S, C, L>> {
        match self {
            &DOMData::ShadowHost(ref value) => Some(value),
            _ => None
        }
    }

    pub(crate) fn is_text(&self) -> Option<&DOMTextNode> {
        match self {
            &DOMData::Text(ref value) => Some(value),
            _ => None
        }
    }

    pub(crate) fn is_normal(&self) -> Option<&DOMNormalNode<E, S, C, L>> {
        match self {
            &DOMData::Normal(ref value) => Some(value),
            _ => None
        }
    }

    pub(crate) fn is_known(&self, name: KnownElementName) -> Option<&DOMNormalNode<E, S, C, L>> {
        match self {
            &DOMData::Normal(ref value) if value.tag == DOMTagName::KnownName(name) => Some(value),
            _ => None
        }
    }

    pub(crate) fn drop_event_listeners(&mut self) -> Option<&mut Vec<DOMAttribute<E, S, C, L>>> {
        let attributes = self.attributes_mut()?;
        attributes.retain(|i| is_event_listener(i).is_none());
        Some(attributes)
    }

    pub(crate) fn drop_styles(&mut self) -> Option<&mut Vec<DOMAttribute<E, S, C, L>>> {
        let attributes = self.attributes_mut()?;
        attributes.retain(|i| is_style(i).is_none());
        Some(attributes)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub(crate) fn attributes_ref(&self) -> Option<&Vec<DOMAttribute<E, S, C, L>>> {
        match self {
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Text(_) => None,
            &DOMData::Normal(DOMNormalNode { ref attributes, .. }) => Some(attributes)
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub(crate) fn attributes_mut(&mut self) -> Option<&mut Vec<DOMAttribute<E, S, C, L>>> {
        match self {
            &mut DOMData::Void | &mut DOMData::ShadowHost(_) | &mut DOMData::Text(_) => None,
            &mut DOMData::Normal(DOMNormalNode { ref mut attributes, .. }) => Some(attributes)
        }
    }

    pub(crate) fn attributes_slice(&self) -> &[DOMAttribute<E, S, C, L>] {
        self.attributes_ref().map(|v| &v[..]).unwrap_or(&[])
    }

    pub(crate) fn attributes_slice_mut(&mut self) -> &mut [DOMAttribute<E, S, C, L>] {
        self.attributes_mut().map(|v| &mut v[..]).unwrap_or(&mut [])
    }

    #[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
    pub fn get_attributes<'a>(&'a self) -> impl Iterator<Item = &'a DOMAttribute<E, S, C, L>> {
        self.attributes_slice().iter()
    }

    #[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
    pub fn get_event_listeners<'a>(&'a self) -> impl Iterator<Item = &'a Closure<E>> {
        self.attributes_slice().iter().filter_map(is_event_listener)
    }

    #[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
    pub fn get_styles<'a>(&'a self) -> impl Iterator<Item = &'a S> {
        self.attributes_slice().iter().filter_map(is_style)
    }
}

#[derive(Debug, PartialEq)]
pub struct DOMTextNode {
    pub(crate) content: DOMText
}

#[derive(Debug, PartialEq)]
pub struct DOMNormalNode<E, S, C, L> {
    pub(crate) tag: DOMTagName,
    pub(crate) attributes: DOMAttributes<E, S, C, L>
}

#[derive(Debug, PartialEq)]
pub enum DOMTagName {
    KnownName(KnownElementName),
    Simple(&'static str),
    NamedspacedName(&'static str, &'static str)
}

#[derive(Debug, PartialEq)]
pub struct DOMAttribute<E, S, C, L>(pub DOMAttributeName, pub DOMAttributeValue<E, S, C, L>);

#[derive(Debug, PartialEq)]
pub enum DOMAttributeName {
    KnownName(KnownAttributeName),
    EventType(EventType),
    Simple(&'static str),
    NamedspacedName(&'static str, &'static str)
}

#[derive(Debug, PartialEq)]
pub enum DOMAttributeValue<E, S, C, L> {
    Boolean(bool),
    Number(f64),
    Char(char),
    Str(DOMText),
    Styles(S),
    Prop(Prop),
    EventListener(Closure<E>),
    Node(DOMNode<E, S, C, L>)
}
