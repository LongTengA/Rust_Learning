use std::collections::HashMap;
pub struct HttpParser {
    pub k_v: HashMap<String, String>,
    pub body: String,
}
enum ReqParts {
    StartLine,
    Headers,
    Body,
}
#[derive(Debug)]
pub enum ParserErrorKind {
    BadHttpFormat,
}
impl HttpParser {
    pub fn new(request: &str) -> Result<HttpParser, ParserErrorKind> {
        let mut parser: HttpParser = HttpParser {
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
                        return Err(ParserErrorKind::BadHttpFormat)
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
