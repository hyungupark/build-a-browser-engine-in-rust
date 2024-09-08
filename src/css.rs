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