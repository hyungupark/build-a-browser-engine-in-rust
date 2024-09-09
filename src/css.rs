//! A simple parser for a tiny subset of CSS.
//!
//! Here's an example of CSS source code:
//!     h1, h2, h3 { margin: auto; color: #cc0000; }
//!     div.note { margin-bottom: 20px; padding: 10px; }
//!     #answer { display: none; }

// Data structures

/*
    A CSS stylesheet is a series of rules. (In the example stylesheet above,
    each line contains one rule.)
 */
use std::num::ParseIntError;
use std::process::id;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

/*
    A rule includes one or more selectors separated by commas, followed by a
    series of declarations enclosed in braces.
 */
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/*
    A selector can be a simple selector, or it can be a chain of selectors
    joined by combinators.

    In here, a simple selector can include a tag name, an ID prefixed by '#',
    any number of class names prefixed by '.', or some combination of the above.
    If the tag name is empty or '*' then it is a “universal selector” that can
    match any tag.
 */
pub enum Selector {
    Simple(SimpleSelector),
}

/*
    e.g.
        SimpleSelector {
            tag_name: "div",
            id: "div-id",
            class: "div-class",
        }
 */
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

/*
    A declaration is just a name/value pair, separated by a colon and ending
    with a semicolon. For example, "margin: auto;" is a declaration.
 */
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

/*
    e.g.
        Value {
            Keyword("margin"),
            Length(f32, Unit),
            ColorValue(Color),
        }

        Value {
            Keyword("background-color"),
            Length(f32, Unit),
            ColorValue(Color),
        }
 */
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

/*
    Unit of length.

    e.g.
        px, em, rem
 */
pub enum Unit {
    Px,
    // insert more units here
}

/*
    Rust note: u8 is an 8-bit unsigned integer, and f32 is a 32-bit float

    e.g.
        Color { r: 0, g: 0, b: 0, a: 1 }       => black
        Color { r: 255, g: 255, b: 255, a: 1 } => white
 */
pub struct Color {
    r: u8, // red
    g: u8, // green
    b: u8, // blue
    a: u8, // alpha
}

/*
    Specificity is one of the ways a rendering engine decides which style overrides
    the other in a conflict. If a stylesheet contains two rules that match an element,
    the rule with the matching selector of higher specificity can override values from
    the one with lower specificity.

    The specificity of a selector is based on its components. An ID selector is more
    specific than a class selector, which is more specific than a tag selector.
    Within each of these “levels,” more selectors beats fewer.

    Count of (id, class, tag)
 */
pub type Specificity = (usize, usize, usize);


// Implementations
impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let id_count: usize = simple.id.iter().count();
        let class_count: usize = simple.class.len();
        let tag_count: usize = simple.tag_name.iter().count();
        (id_count, class_count, tag_count)
    }
}

impl Value {
    /// Return the size of a length in px, or zero for non-lengths.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

// Parser

/*
    CSS Parser struct

    e.g.
        Parser {
            input: "string type css input",
            pos: position of input string,
        }
 */
struct Parser {
    input: String,
    position: usize,
}

// Implementation of Parser
impl Parser {
    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}