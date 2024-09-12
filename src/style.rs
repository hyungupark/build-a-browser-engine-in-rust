//! Code for applying CSS styles to the DOM.
//!
//! I will call it "CSS Renderer"

use crate::css;
use crate::dom;
use std::collections::HashMap;

/*
    The output of this engine's style module is something I call the "style tree".
    Each node in this tree includes a pointer to a DOM node, plus its CSS property values.
 */

/// Map from CSS property names to value.
/*
    e.g.
        PropertyMap {
            "background-color": Value::ColorValue(Color { r: 0, g: 0, b: 0, a: 1 })
        }
 }
 */
pub type PropertyMap = HashMap<String, css::Value>;


/// A node with associated style data.
/*
    What’s with all the 'a stuff? Those are lifetimes, part of how Rust guarantees
    that pointers are memory-safe without requiring garbage collection. If you’re not
    working in Rust you can ignore them; they aren’t critical to the code’s meaning.

    e.g.
        StyledNode<'a> {
            node: &'a Node,
            specified_values: PropertyMap,
            children: Vec<StyledNode<'a>>,
        }
 */
#[derive(Clone)]
pub struct StyledNode<'a> {
    pub node: &'a dom::Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}


/// CSS's `display` enum
/*
    e.g.
        Display::Inline, Display::Block, Display::None
 */
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Return the specified value of a property if it exists, otherwise `None`.
    pub fn value(&self, name: &str) -> Option<css::Value> {
        self.specified_values.get(name).cloned()
    }

    /// Return the specified value of property `name`, or property `fallback_name`
    /// if that doesn't exist, or value `default` if neither does.
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &css::Value) -> css::Value {
        self.value(name).unwrap_or_else(
            || self.value(fallback_name).unwrap_or_else(|| default.clone())
        )
    }

    /// The value of the `display` property (defaults to inline).
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(css::Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}
