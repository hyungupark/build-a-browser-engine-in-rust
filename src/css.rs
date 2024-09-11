//! A simple parser for a tiny subset of CSS.
//!
//! Here's an example of CSS source code:
//!     h1, h2, h3 { margin: auto; color: #cc0000; }
//!     div.note { margin-bottom: 20px; padding: 10px; }
//!     #answer { display: none; }

// Data structures

/// Stylesheet structure
/*
    A CSS stylesheet is a series of rules. (In the example stylesheet above,
    each line contains one rule.)
 */
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}


/// Rule structure
/*
    A rule includes one or more selectors separated by commas, followed by a
    series of declarations enclosed in braces.

    Rule = Selector (External/Internal CSS) + Declaration (Inline CSS)
 */
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}


/// Selector enum (only support simple selectors)
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


/// SimpleSelector structure (select elements based on name, id, class)
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

/// Declaration structure
/*
    A declaration is just a name/value pair, separated by a colon and ending
    with a semicolon. For example, "margin: auto;" is a declaration.

    Declaration is Inline CSS.

    e.g.
        Declaration { name: "display", value: Value::Keyword("block") }
 */
pub struct Declaration {
    pub name: String,
    pub value: Value,
}


/// This engine supports only a handful of CSS's many value types.
/*
    e.g.
        Value::Keywords("block")
        Value::Length(30, Unit::Px)
        Value::ColorValue(Color { r: 0, g: 0, b: 0, a: 1 })
 */
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

/// Unit enum
/*
    Unit of length.

    e.g.
        Unit::Px, Unit::Em, Unit::Rem
 */
pub enum Unit {
    Px,
    // insert more units here
}


/// Color struct with rgba(red, green, red, alpha)
/*
    e.g.
        Color { r: 0, g: 0, b: 0, a: 1 }       => black
        Color { r: 255, g: 255, b: 255, a: 1 } => white

    Rust note: u8 is an 8-bit unsigned integer, and f32 is a 32-bit float
 */
pub struct Color {
    r: u8, // red
    g: u8, // green
    b: u8, // blue
    a: u8, // alpha
}


/// Specificity type
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


// impl

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
    CSS has a straightforward [grammar](https://www.w3.org/TR/CSS2/grammar.html),
    making it easier to parse correctly than its quirky cousin HTML.
    When a standards-compliant CSS parser encounters a [parse error](https://www.w3.org/TR/CSS2/syndata.html#parsing-errors),
    it discards the unrecognized part of the stylesheet but still process the remaining portions.
    This is useful because it allows stylesheets to include new syntax but still produce well-defined
    output in older browsers.
 */

/// CSS Parser structure
/*
    e.g.
        Parser {
            input: "str type css input",
            pos: 0, // current position of css input
        }
 */
struct Parser {
    input: String,
    position: usize,
}

// impl

impl Parser {
    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    /// Return the current character, and advance self.position to the next character.
    fn consume_char(&mut self) -> char {
        let c: char = self.next_char();
        self.position += c.len_utf8();
        c
    }

    /// If the exact string `s` is found at the current position, consume it.
    /// Otherwise, panic.
    fn expect_char(&mut self, c: char) {
        if self.consume_char() != c {
            panic!("Expected {:?} at byte {} but it was not found", c, self.position);
        }
    }

    /// Consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result: String = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parse a property name or keyword.
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    /// Parse two hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s: &str = &self.input[self.position..self.position + 2];
        self.position += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    /// Parse color.
    fn parse_color(&mut self) -> Value {
        self.expect_char('#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    /// Parse unit
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }

    /// Parse float
    fn parse_float(&mut self) -> f32 {
        self.consume_while(|c: char| matches!(c, '0'..='9' | '.')).parse().unwrap()
    }

    // Methods for parsing values

    /// Parse length.
    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    /// Parse value.
    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    /// Parse one `<property>: <value>;` declaration (Inline CSS).
    fn parse_declaration(&mut self) -> Declaration {
        let name: String = self.parse_identifier();
        self.consume_whitespace();
        self.expect_char(':');
        self.consume_whitespace();

        let value: Value = self.parse_value();
        self.consume_whitespace();
        self.expect_char(';');

        Declaration { name, value }
    }

    /// Parse a list of declarations (Inline CSSs) enclosed in `{ ... }`
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        self.expect_char('{');
        let mut declarations: Vec<Declaration> = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    /// Parse one simple selector, e.g: `type#id.class1.class2.class3`
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        selector
    }

    /// Parse a comma-separated list of selectors.
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        // Return selectors with highest specificity first, for use in matching.
        selectors.sort_by_key(|s: &Selector| s.specificity());
        selectors
    }

    /// Parse a rule set: `<selectors> { <declarations> }`.
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    /// Parse a list of rule sets, separated by optional whitespace.
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }
}


/// check validation of char input
/*
    char input must be a-z or A-Z or 0-9 or - or _
 */
fn valid_identifier_char(c: char) -> bool {
    // TODO: Include U+00A0 and higher.
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}


/// Parse a whole CSS stylesheet.
pub fn parse(source: String) -> Stylesheet {
    let mut parser: Parser = Parser { input: source, position: 0 };
    Stylesheet { rules: parser.parse_rules() }
}