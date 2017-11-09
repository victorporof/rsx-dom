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

#![feature(never_type)]
#![feature(try_from)]
#![feature(box_syntax)]

#[macro_use]
extern crate rsx_dom;
extern crate rsx_shared;

use std::convert::TryInto;

use rsx_dom::types::*;
use rsx_shared::traits::*;
use rsx_shared::types::{
    DummyComputedBorderStyle,
    DummyComputedBoxShadow,
    DummyComputedColor,
    DummyComputedCursor,
    DummyComputedFontCaps,
    DummyComputedFontName,
    DummyComputedFontSize,
    DummyComputedFontStretch,
    DummyComputedFontStyle,
    DummyComputedFontWeight,
    DummyComputedTextShadow,
    DummyComputedVisibility
};

type DOMNode = rsx_dom::types::DOMNode<(), (), MockComputedStyles, MockLayoutNode>;

#[derive(Debug, PartialEq, Clone, Default)]
struct MockComputedStyles(Vec<String>);

impl TComputedStyles for MockComputedStyles {
    type BackgroundColor = DummyComputedColor;
    type Opacity = u32;
    type BorderSize = u32;
    type BorderColor = DummyComputedColor;
    type BorderStyle = DummyComputedBorderStyle;
    type BoxShadow = DummyComputedBoxShadow;

    fn make_initial_computed_styles<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        MockComputedStyles(Vec::new())
    }

    fn reset_custom_styles<T>(&mut self, _: T)
    where
        T: TryInto<KnownElementName>
    {
        unimplemented!()
    }

    fn apply_rules<'a, I>(&mut self, _: I)
    where
        I: Iterator<Item = &'a Self::Styles>
    {
        self.0.push("apply_rules()".to_string());
    }

    fn apply_styles(&mut self, _: &Self::Styles) {
        self.0.push("apply_styles()".to_string());
    }

    fn background_color(&self) -> Self::BackgroundColor {
        unimplemented!()
    }

    fn opacity(&self) -> Self::Opacity {
        unimplemented!()
    }

    fn border_bottom_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_bottom_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_bottom_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_left_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_left_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_left_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_right_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_right_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_right_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_top_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_top_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_top_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn box_shadows_copy(&self) -> Vec<Self::BoxShadow> {
        unimplemented!()
    }
}

impl TInheritedStyles for MockComputedStyles {
    type Styles = ();
    type Cursor = DummyComputedCursor;
    type Color = DummyComputedColor;
    type TextShadow = DummyComputedTextShadow;
    type FontName = DummyComputedFontName;
    type FontStyle = DummyComputedFontStyle;
    type FontCaps = DummyComputedFontCaps;
    type FontWeight = DummyComputedFontWeight;
    type FontSize = DummyComputedFontSize;
    type FontStretch = DummyComputedFontStretch;
    type Visibility = DummyComputedVisibility;

    fn inherit_styles(&mut self, _: &Self) {
        self.0.push("inherit_styles()".to_string());
    }

    fn cursor(&self) -> Self::Cursor {
        unimplemented!()
    }

    fn color(&self) -> Self::Color {
        unimplemented!()
    }

    fn text_shadows_copy(&self) -> Vec<Self::TextShadow> {
        unimplemented!()
    }

    fn font_names_copy(&self) -> Vec<Self::FontName> {
        unimplemented!()
    }

    fn font_style(&self) -> Self::FontStyle {
        unimplemented!()
    }

    fn font_caps(&self) -> Self::FontCaps {
        unimplemented!()
    }

    fn font_weight(&self) -> Self::FontWeight {
        unimplemented!()
    }

    fn font_size(&self) -> Self::FontSize {
        unimplemented!()
    }

    fn font_stretch(&self) -> Self::FontStretch {
        unimplemented!()
    }

    fn visibility(&self) -> Self::Visibility {
        unimplemented!()
    }

    fn find_font<F, O>(&self, _: F) -> Option<O>
    where
        F: FnMut(&Self::FontName) -> Option<O>
    {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default)]
struct MockLayoutNode(Vec<String>);

impl TLayoutNode for MockLayoutNode {
    type Styles = ();
    type Resources = ();
    type TextMeasureMetadata = MockComputedStyles;
    type ImageMeasureMetadata = ();
    type NormalMeasureMetadata = !;
    type ReflowDirection = ();
    type ClientPosition = ();
    type BoundingClientRect = ();
    type MeasuredImage = ();
    type ShapedText = ();

    fn make_initial_layout_node<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        Default::default()
    }

    fn reset_custom_styles<T>(&mut self, _: T)
    where
        T: TryInto<KnownElementName>
    {
        unimplemented!()
    }

    fn is_tainted(&self) -> bool {
        false
    }

    fn insert_child(&mut self, _: &mut Self, _: usize) {
        unimplemented!()
    }

    fn append_child(&mut self, _: &mut Self) {
        self.0.push("append_child()".to_string());
    }

    fn remove_child(&mut self, _: &mut Self) {
        unimplemented!()
    }

    fn apply_rules<'a, I>(&mut self, _: I)
    where
        I: Iterator<Item = &'a Self::Styles>
    {
        self.0.push("apply_rules()".to_string());
    }

    fn apply_styles(&mut self, _: &Self::Styles) {
        self.0.push("apply_styles()".to_string());
    }

    fn mark_dirty(&mut self) {
        unimplemented!()
    }

    fn measure_self_as_text<T>(&mut self, _: &Self::Resources, t: &T, _: &Self::TextMeasureMetadata)
    where
        T: TDOMText
    {
        self.0.push(format!("measure_self_as_text({:?})", t));
    }

    fn measure_self_as_image<T>(&mut self, _: &Self::Resources, t: &T, _: &Self::ImageMeasureMetadata)
    where
        T: TDOMText
    {
        self.0.push(format!("measure_self_as_image({:?})", t));
    }

    fn measure_self_as_normal(&mut self, _: &Self::Resources, _: &Self::NormalMeasureMetadata) {
        unreachable!()
    }

    fn reflow_subtree(&mut self, w: u32, h: u32, d: Self::ReflowDirection) {
        let string = format!("reflow_subtree({:?}, {:?}, {:?})", w, h, d);
        self.0.push(string);
    }

    fn set_computed_client_position(&mut self, _: Self::ClientPosition) {
        unimplemented!()
    }

    fn get_local_bounding_client_rect(&self) -> Self::BoundingClientRect {
        unimplemented!()
    }

    fn get_global_bounding_client_rect(&self) -> Self::BoundingClientRect {
        unimplemented!()
    }

    fn get_measured_image(&self) -> &Self::MeasuredImage {
        unimplemented!()
    }

    fn get_shaped_text(&self) -> &Self::ShapedText {
        unimplemented!()
    }
}

#[test]
fn test_simple_traverse() {
    let tree = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Div),
            vec![
                DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
            ],
            vec![
                DOMNode::from((
                    DOMTagName::from("bar"),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from("baz"),
                            DOMAttributeValue::from(false)
                        )),
                    ],
                    vec![DOMNode::from("Hello")]
                )),
                DOMNode::from({ "world" }),
                DOMNode::from({ "!" }),
            ]
        ))
    };

    let node = tree.document();

    assert_eq!(
        node.descendants_iter().collect::<Vec<_>>(),
        vec![
            &DOMNode::from((
                DOMTagName::from(KnownElementName::Div),
                vec![
                    DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
                ]
            )),
            &DOMNode::from((
                DOMTagName::from("bar"),
                vec![
                    DOMAttribute::from((
                        DOMAttributeName::from("baz"),
                        DOMAttributeValue::from(false)
                    )),
                ]
            )),
            &DOMNode::from("Hello"),
            &DOMNode::from("world"),
            &DOMNode::from("!"),
        ]
    );
}

#[test]
#[cfg(feature = "hashmap-arena")]
fn test_nested_traverse() {
    let build_subtree = || {
        fragment! {
            DOMNode::from((
                DOMTagName::from(KnownElementName::View),
                vec![
                    DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
                ],
                vec![
                    DOMNode::from((
                        DOMTagName::from("bar"),
                        vec![
                            DOMAttribute::from((
                                DOMAttributeName::from("baz"),
                                DOMAttributeValue::from(false)
                            )),
                        ],
                        vec![DOMNode::from("Hello")]
                    )),
                    DOMNode::from({ "world" }),
                    DOMNode::from({ "!" })
                ]
            ))
        }
    };

    let build_tree = || {
        fragment! {
            DOMNode::from((
                DOMTagName::from(KnownElementName::Div),
                vec![
                    DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
                ],
                vec![
                    DOMNode::from({
                        build_subtree()
                    })
                ]
            ))
        }
    };

    let tree = build_tree();
    let node = tree.document();

    assert_eq!(
        node.descendants_iter().collect::<Vec<_>>(),
        vec![
            &DOMNode::from((
                DOMTagName::from(KnownElementName::Div),
                vec![
                    DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
                ]
            )),
            &DOMNode::from((
                DOMTagName::from(KnownElementName::View),
                vec![
                    DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
                ]
            )),
            &DOMNode::from((
                DOMTagName::from("bar"),
                vec![
                    DOMAttribute::from((
                        DOMAttributeName::from("baz"),
                        DOMAttributeValue::from(false)
                    )),
                ]
            )),
            &DOMNode::from("Hello"),
            &DOMNode::from("world"),
            &DOMNode::from("!"),
        ]
    );
}

#[test]
fn test_simple_computed_styles() {
    let mut tree = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Div),
            vec![
                DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
            ],
            vec![
                DOMNode::from((
                    DOMTagName::from(KnownElementName::Image),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from(KnownAttributeName::Src),
                            DOMAttributeValue::from("url")
                        )),
                    ]
                )),
                DOMNode::from((
                    DOMTagName::from("bar"),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from("baz"),
                            DOMAttributeValue::from(false)
                        )),
                    ],
                    vec![DOMNode::from("Hello")]
                )),
                DOMNode::from({ "world" }),
                DOMNode::from({ "!" }),
            ]
        ))
    };

    assert_eq!(
        tree.document()
            .descendants_iter()
            .map(|v| v.computed_styles())
            .collect::<Vec<_>>(),
        vec![
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
        ],
    );

    tree.generate_layout_tree(&());

    assert_eq!(
        tree.document()
            .descendants_iter()
            .map(|v| v.computed_styles())
            .collect::<Vec<_>>(),
        vec![
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "inherit_styles()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "inherit_styles()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "inherit_styles()".to_string(),
            ]),
        ],
    );
}

#[test]
fn test_simple_layout() {
    let mut tree = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Div),
            vec![
                DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
            ],
            vec![
                DOMNode::from((
                    DOMTagName::from(KnownElementName::Image),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from(KnownAttributeName::Src),
                            DOMAttributeValue::from("url")
                        )),
                    ]
                )),
                DOMNode::from((
                    DOMTagName::from("bar"),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from("baz"),
                            DOMAttributeValue::from(false)
                        )),
                    ],
                    vec![DOMNode::from("Hello")]
                )),
                DOMNode::from({ "world" }),
                DOMNode::from({ "!" }),
            ]
        ))
    };

    assert_eq!(
        tree.document()
            .descendants_iter()
            .map(|v| v.computed_styles())
            .collect::<Vec<_>>(),
        vec![
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
            &MockComputedStyles(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
            ]),
        ],
    );

    tree.generate_layout_tree(&());

    assert_eq!(
        tree.document()
            .descendants_iter()
            .map(|v| v.layout_node())
            .collect::<Vec<_>>(),
        vec![
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "append_child()".to_string(),
                "append_child()".to_string(),
                "append_child()".to_string(),
                "append_child()".to_string(),
            ]),
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "measure_self_as_image(Static(\"url\"))".to_string(),
            ]),
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "append_child()".to_string(),
            ]),
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "measure_self_as_text(Static(\"Hello\"))".to_string(),
            ]),
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "measure_self_as_text(Static(\"world\"))".to_string(),
            ]),
            &MockLayoutNode(vec![
                "apply_styles()".to_string(),
                "apply_rules()".to_string(),
                "measure_self_as_text(Static(\"!\"))".to_string(),
            ]),
        ],
    );
}
