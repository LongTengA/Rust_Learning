use std::collections::HashMap;
enum ReqParts {
    StartLine,
    Headers,
    Body,
}
pub enum ParserErrorKind {
    BadHttpFormat(_HttpParser),
}

impl ParserErrorKind {
    pub fn get(self) -> _HttpParser {
        match self {
            ParserErrorKind::BadHttpFormat(e) => e,
        }
    }
}

trait Parser {
    fn null_parser() -> _HttpParser;
}
pub struct _HttpParser {
    pub k_v: HashMap<String, String>,
    pub body: String,
}

impl Parser for _HttpParser {
    fn null_parser() -> _HttpParser {
        _HttpParser {
            k_v: HashMap::new(),
            body: String::new(),
        }
    }
}

impl _HttpParser {
    pub fn new(request: &str) -> Result<_HttpParser, ParserErrorKind> {
        let mut parser: _HttpParser = _HttpParser {
            k_v: HashMap::new(),
            body: String::new(),
        };

        let mut part = ReqParts::StartLine;
        for i in request.lines() {
            match part {
                ReqParts::StartLine => {
                    part = ReqParts::Headers;
                    if let Some(pos) = i.find("HTTP") {
                        if pos == 0 {
                            let key_words = ["Version", "Status-Code", "Status"].iter();
                            for (k, v) in key_words.zip(i.split_ascii_whitespace()) {
                                parser.k_v.insert(k.to_string(), v.to_string());
                            }
                        } else {
                            let key_words = ["Method", "Path", "Version"].iter();
                            for (k, v) in key_words.zip(i.split_ascii_whitespace()) {
                                parser.k_v.insert(k.to_string(), v.to_string());
                            }
                        }
                    } else {
                        return Err(ParserErrorKind::BadHttpFormat(_HttpParser::null_parser()));
                    }
                }
                ReqParts::Headers => {
                    match i.find(":") {
                        Some(pos) => {
                            parser
                                .k_v
                                .insert(i[..pos].to_string(), i[pos + 2..].to_string());
                        }
                        None => part = ReqParts::Body,
                    };
                }
                ReqParts::Body => {
                    parser.body += i;
                    parser.body += "\n";
                }
            }
        }
        Ok(parser)
    }
    pub fn show_header(&self) {
        for (k, v) in &self.k_v {
            println!("{:<25} {:<20}", k, v);
        }
    }
    pub fn show_body(&self) {
        if self.body.is_empty() {
            println!("Empty body!");
        } else {
            println!("{}", self.body);
        }
    }

    pub fn find(&self, key: &str) -> Option<&String> {
        self.k_v.get(key)
    }
}

pub struct HttpParser<'a> {
    pub header: HashMap<&'a str, &'a str>,
    pub body: &'a str,
}

impl<'a> HttpParser<'a> {
    pub fn new(request: &'a str) -> Result<Self, ParserErrorKind> {
        let mut parser = Self::null_parser();
        let mut part = ReqParts::StartLine;
        for lines in request.lines() {
            match part {
                ReqParts::StartLine => {
                    if let Some(pos) = lines.find("HTTP") {
                        if pos == 0 {
                            let key_words = ["Version", "Status-Code", "Status"].iter();
                            for (k, v) in key_words.zip(lines.split_ascii_whitespace()) {
                                parser.header.insert(k, v);
                            }
                        } else {
                            let key_words = ["Method", "Path", "Version"].iter();
                            for (k, v) in key_words.zip(lines.split_ascii_whitespace()) {
                                parser.header.insert(k, v);
                            }
                        }
                    } else {
                        return Err(ParserErrorKind::BadHttpFormat(_HttpParser::null_parser()));
                    }
                    part = ReqParts::Headers;
                }
                ReqParts::Headers => {
                    match lines.find(":") {
                        Some(pos) => {
                            parser.header.insert(&lines[..pos], &lines[pos + 2..]);
                        }
                        None => part = ReqParts::Body,
                    };
                }
                ReqParts::Body => unsafe {
                    
                },
            }
        }
        Ok(parser)
    }

    pub fn null_parser() -> Self {
        HttpParser {
            header: HashMap::new(),
            body: "",
        }
    }
}
