//! microcmlparser.rs
//! 2020-01-12  Luciano Bestia

//MicroXML is a subset of XML. It is dramatically simpler.
//https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html
//MicroXML is actualy well-formed Xml. But Xml is not always well-formed MicroXML.
//This parser cannot parse the full XML.
//Limitations: only utf-8 (rust Strings are checked for correct utf-8),
//normalization: CRLF is converted to LF, comments are removed
//special difference: LF inside a Text remains (in xml is replaced with a space)
//only 3 characters are white space: #x9 | #xA | #x20

//this parser is used for small html fragments.
//They must be well-formed MicroXml.
//This fragments are meant for a html templating for dodrio.
//Because of the small size of fragments, I can put all the text in memory in a string.

use crate::log1;
use unwrap::unwrap;

pub enum TagState {
    InsideOfTag,
    OutsideOfTag,
    Attributes,
}

#[derive(Clone, Debug)]
pub enum Event {
    /// Start tag (with attributes) `<tag attr="value">`.
    StartElement(String),
    /// End tag `</tag>`.
    EndElement(String),
    /// Attribute
    Attribute(String, String),
    /// Character data between `Start` and `End` element.
    Text(String),
    /// End of XML document.
    Eof,
}

pub struct Parser {
    input: String,
    //for parsing I need to know the position
    pos: usize,
    //and I need to know the TagState
    tagstate: TagState,
}

pub fn test_my_parser() {
    let input = r#"
    <div id="my_project">
        firstdiv
        <h1 id="hello"> hahahah5555 </h1>
        <!-- comment -->
        <h2 id="lala"> lalala
        uhuhuh 
        </h2>
        testfromdiv
    </div>
         "#;
    let mut pp = Parser::new(input);
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                log1(&format!("StartElement(name) {}", &name));
            }
            Event::Attribute(name, value) => {
                log1(&format!("Attribute(name, value) {} {}", &name, &value));
            }
            Event::Text(txt) => {
                log1(&format!("Text(txt) {}", &txt));
            }
            Event::EndElement(name) => {
                log1(&format!("EndElement(name) {}", &name));
            }
            Event::Eof => {
                log1("eof");
                break;
            }
        }
    }
}

impl Parser {
    ///constructor. String is borrowed here.
    pub fn new(input: &str) -> Parser {
        let mut parser = Parser {
            pos: 0,
            input: String::new(),
            tagstate: TagState::OutsideOfTag,
        };
        //normalization makes a new string and put it in input field
        parser.normalization(input);
        //return
        parser
    }

    ///MicroXml normalization
    ///trim whitespace before and after
    ///change crlf and cr to lf
    ///remove comments
    #[allow(clippy::too_many_lines)]
    pub fn normalization(&mut self, input: &str) {
        pub enum CommentState {
            InsideOfComment,
            OutsideOfComment,
            StartDelim1,
            StartDelim2,
            StartDelim3,
            EndDelim1,
            EndDelim2,
        }
        let input = input.trim().replace("\r\n", "\n").replace("\r", "\n");
        let mut new_input = String::with_capacity(input.len());
        // xml comments looks like <!--  xxx -->
        let mut comment_state = CommentState::OutsideOfComment;
        for this_char in input.chars() {
            match comment_state {
                CommentState::OutsideOfComment => {
                    if this_char == '<' {
                        comment_state = CommentState::StartDelim1;
                    } else {
                        new_input.push(this_char);
                    }
                }
                CommentState::StartDelim1 => {
                    if this_char == '!' {
                        comment_state = CommentState::StartDelim2;
                    } else {
                        comment_state = CommentState::OutsideOfComment;
                        new_input.push('<');
                        new_input.push(this_char);
                    }
                }
                CommentState::StartDelim2 => {
                    if this_char == '-' {
                        comment_state = CommentState::StartDelim3;
                    } else {
                        comment_state = CommentState::OutsideOfComment;
                        new_input.push('<');
                        new_input.push('!');
                        new_input.push(this_char);
                    }
                }
                CommentState::StartDelim3 => {
                    if this_char == '-' {
                        comment_state = CommentState::InsideOfComment;
                    } else {
                        comment_state = CommentState::OutsideOfComment;
                        new_input.push('<');
                        new_input.push('!');
                        new_input.push('-');
                        new_input.push(this_char);
                    }
                }
                CommentState::InsideOfComment => {
                    if this_char == '-' {
                        comment_state = CommentState::EndDelim1;
                    }
                }
                CommentState::EndDelim1 => {
                    if this_char == '-' {
                        comment_state = CommentState::EndDelim2;
                    } else {
                        comment_state = CommentState::InsideOfComment;
                        new_input.push('-');
                        new_input.push(this_char);
                    }
                }
                CommentState::EndDelim2 => {
                    if this_char == '>' {
                        comment_state = CommentState::OutsideOfComment;
                    } else {
                        comment_state = CommentState::InsideOfComment;
                        new_input.push('-');
                        new_input.push('-');
                        new_input.push(this_char);
                    }
                }
            }
        }
        log1(&new_input);
        self.input = new_input;
    }

    ///read next event
    #[allow(clippy::integer_arithmetic, clippy::nonminimal_bool)]
    fn read_event(&mut self) -> Event {
        if self.pos >= self.input.len() {
            Event::Eof
        } else {
            match &self.tagstate {
                TagState::OutsideOfTag => {
                    let mut this_char = self.char_at_pos_inc_pos_non_whitespace();
                    if this_char.eq(&'<') {
                        self.tagstate = TagState::InsideOfTag;
                        this_char = self.char_at_pos_inc_pos_non_whitespace();
                        if this_char == '/' {
                            this_char = self.char_at_pos_inc_pos_non_whitespace();
                            let mut elem_name = "".to_owned();
                            while this_char != '>' {
                                elem_name.push(this_char);
                                this_char = self.char_at_pos_inc_pos_non_whitespace();
                            }
                            self.tagstate = TagState::OutsideOfTag;
                            //return
                            Event::EndElement(elem_name)
                        } else {
                            //read until space, / or >
                            let mut elem_name = "".to_owned();
                            while !(this_char.is_whitespace()
                                || this_char == '/'
                                || this_char == '>')
                            {
                                elem_name.push(this_char);
                                this_char = self.char_at_pos_inc_pos();
                            }
                            if this_char.is_whitespace() {
                                this_char = self.char_at_pos_inc_pos_non_whitespace();
                            }
                            if this_char == '/' {
                                this_char = self.char_at_pos_inc_pos();
                            }
                            if this_char == '>' {
                                self.tagstate = TagState::OutsideOfTag;
                            } else {
                                #[allow(clippy::integer_arithmetic)]
                                {
                                    self.pos -= 1;
                                }
                                self.tagstate = TagState::Attributes;
                            }
                            //return
                            Event::StartElement(elem_name)
                        }
                    } else {
                        // text element
                        let mut text = "".to_owned();
                        while !(this_char == '<') {
                            text.push(this_char);
                            this_char = self.char_at_pos_inc_pos();
                        }
                        #[allow(clippy::integer_arithmetic)]
                        {
                            self.pos -= 1;
                        }
                        self.tagstate = TagState::OutsideOfTag;
                        //newline or whitespace in the middle is ok, but at the end is not ok.
                        text = text.trim().to_owned();
                        //return
                        Event::Text(text)
                    }
                }
                TagState::Attributes => {
                    let mut attr_name = "".to_owned();
                    let mut this_char = self.char_at_pos_inc_pos();
                    #[allow(clippy::nonminimal_bool)]
                    while !(this_char.is_whitespace() || this_char == '=') {
                        attr_name.push(this_char);
                        this_char = self.char_at_pos_inc_pos();
                    }
                    while this_char.is_whitespace() || this_char == '=' || this_char == '"' {
                        this_char = self.char_at_pos_inc_pos();
                    }
                    let mut attr_value = "".to_owned();
                    while !(this_char == '"') {
                        attr_value.push(this_char);
                        this_char = self.char_at_pos_inc_pos();
                    }
                    this_char = self.char_at_pos_inc_pos();
                    if this_char.is_whitespace() {
                        this_char = self.char_at_pos_inc_pos_non_whitespace();
                    }
                    if this_char == '/' {
                        this_char = self.char_at_pos_inc_pos();
                    }
                    if this_char == '>' {
                        self.tagstate = TagState::OutsideOfTag;
                    } else {
                        self.pos -= 1;
                        self.tagstate = TagState::Attributes;
                    }
                    //return
                    Event::Attribute(attr_name, attr_value)
                }
                TagState::InsideOfTag => {
                    //cannot be inside if it was not started as outside
                    Event::Eof
                }
            }
        }
    }
    fn char_at_pos_inc_pos_non_whitespace(&mut self) -> char {
        let mut this_char = self.char_at_pos_inc_pos();
        while this_char.is_whitespace() {
            this_char = self.char_at_pos_inc_pos();
        }
        this_char
    }

    #[allow(clippy::integer_arithmetic)]
    fn char_at_pos_inc_pos(&mut self) -> char {
        let this_char = unwrap!(self.input.chars().nth(self.pos));
        self.pos += 1;
        //return
        this_char
    }
}
