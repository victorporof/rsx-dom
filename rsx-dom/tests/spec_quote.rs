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

// #[macro_use]
// extern crate quote;
// extern crate rsx_dom;
// extern crate syn;

// use rsx_dom::rsx_parser::parse;

// #[test]
// fn test_from_rsx_1() {
//     let (ast, _) = parse("<div/>").unwrap();

//     let tokens = quote! { DOMNode::from(DOMTagName::from(KnownElementName::Div)) };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_2() {
//     let (ast, _) = parse("<foo-bar/>").unwrap();

//     let tokens = quote! { DOMNode::from(DOMTagName::from("foo-bar")) };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_3() {
//     let (ast, _) = parse("<foo:bar/>").unwrap();

//     let tokens = quote! { DOMNode::from(DOMTagName::from(("foo", "bar"))) };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_4() {
//     let (ast, _) = parse("<foo.bar/>").unwrap();

//     let tokens = quote! { DOMNode::from(DOMTagName::from(box ["foo", "bar"])) };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_5() {
//     let (ast, _) = parse("<div foo></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_6() {
//     let (ast, _) = parse("<div foo={true}></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_7() {
//     let (ast, _) = parse("<div foo={42}></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from(42f64)
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_8() {
//     let (ast, _) = parse("<div foo='bar'></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from("bar")
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_9() {
//     let (ast, _) = parse("<div foo=\"bar\"></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from("bar")
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_10() {
//     let (ast, _) = parse("<div foo={'bar'}></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from("bar")
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_11() {
//     let (ast, _) = parse("<div foo={\"bar\"}></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from("bar")
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_12() {
//     let (ast, _) = parse("<div foo=<bar/>></div>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from(KnownElementName::Div),
//             box [
//                 DOMAttribute::from((
//                     DOMAttributeName::from("foo"),
//                     DOMAttributeValue::from(DOMNode::from(DOMTagName::from("bar")))
//                 )),
//             ]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_13() {
//     let (ast, _) = parse("<foo>bar</foo>").unwrap();

//     let tokens = quote! { DOMNode::from((DOMTagName::from("foo"), box [DOMNode::from("bar")])) };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_14() {
//     let (ast, _) = parse("<foo><bar/></foo>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from("foo"),
//             box [DOMNode::from(DOMTagName::from("bar"))]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }

// #[test]
// fn test_from_rsx_15() {
//     let (ast, _) = parse("<foo><bar/>baz</foo>").unwrap();

//     let tokens = quote! {
//         DOMNode::from((
//             DOMTagName::from("foo"),
//             box [DOMNode::from(DOMTagName::from("bar")), DOMNode::from("baz")]
//         ))
//     };

//     assert_eq!(
//         syn::parse_expr(quote! { #ast }.as_str()),
//         syn::parse_expr(tokens.as_str())
//     );
// }
