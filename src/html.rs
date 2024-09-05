/*
    A Simple HTML Dialect

    This parser can handle simple pages like this:
        <html>
            <body>
                <h1>Title</h1>
                <div id="main" class="test">
                    <p>Hello <em>world</em>!</p>
                </div>
            </body>
        </html>

    The following syntax is allowed:
        - Balanced tags: <p>...</p>
        - Attributes with quoted values: id="main"
        - Text nodes: <em>world</em>

    Everything else is unsupported, including:
        - Comments
        - Doctype declarations
        - Escaped characters (like &amp;) and CDATA sections
        - Self-closing tags: <br/> or <br> with no closing tag
        - Error handling (e.g. unbalanced or improperly nested tags)
        - Namespaces and other XHTML syntax: <html:body>
        - Character encoding detection
 */

/*
    Let's walk through this HTML parser, keeping in mind that this is just one way to do it (and
    probably not the best way). Its structure is based loosely on the [tokenizer](https://github.com/servo/rust-cssparser/blob/032e7aed7acc31350fadbbc3eb5a9bbf6f4edb2e/src/tokenizer.rs)
    module from Servo's [cssparser](https://github.com/servo/rust-cssparser) libaray.
    It has no real error handling; in most cases, it just abouts when faced with unexpected syntax.
 */

/*
    The parser stores its input string and a current position within the string.
    The position is the index of the next character we haven't processed yet.
 */
use std::io::Chain;
use std::thread::sleep;
use crate::dom;
use crate::dom::{element, Node};

struct Parser {
    pos: usize, // "usize" is an unsigned integer, similar to "size_t" in C
    input: String,
}


/*
    We can use this to implement some simple methods for peeking at the next characters in the input.
 */

impl Parser {
    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Does the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// If the exact string `s` is found at the current position, consume it. Otherwise, panic.
    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos);
        }
    }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }


    /*
        Rust strings are stored as [UTF-8](https://en.wikipedia.org/wiki/UTF-8) byte arrays.
        To go to the next character, we can't just advance by one byte.
     */

    /// Return the current character, and advance `self.pos` to the next character.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }


    /*
        Often we will want to consume a string of consecutive character. The `consume_while` method
        consumes characters that meet a given condition, and returns them as a string.
        This method's argument is a function that takes a char and returns a bool.
     */

    /// Consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }


    /*
        We can use this to ignore a sequence of space characters,
        or to consume a string of alphanumeric characters.
     */

    /// Consume and discard zero or more whitespace character.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parse a tag or attribute name.
    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }


    /*
        Now we're ready to start parsing HTML. To parse a single node,
        we look at its first character to see if it is an element or a text node.
     */

    /// Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            // parse element
            dom::Node {
                children: Vec::new(),
                node_type: dom::NodeType::Text(String::new()),
            }
        } else {
            self.parse_text()
        }
    }


    /*
        In our simplified version of HTML, a text node can contain any character expect "<".
     */

    /// Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }
}
