use std::{fs::File, io};

use parser::httpparser::*;
fn main() {
    {
        let raw_text = r##"POST https://www.bing.com/BrowserExtension/Rewards/GetNotification?evt=newtab&pc=U523 HTTP/1.1
Host: www.bing.com
Connection: keep-alive
Content-Length: 353
sec-ch-ua: "Microsoft Edge";v="105", " Not;A Brand";v="99", "Chromium";v="105"
X-BBE-CSRF: BrowserExtensionsNotifications
Content-Type: application/json;charset=utf-8
sec-ch-ua-mobile: ?0
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)
sec-ch-ua-platform: "Windows"
Accept: */*
Origin: chrome-extension://bnplfnhcidhhdapmblniehfaaompjlck
X-Edge-Shopping-Flag: 1
Sec-Fetch-Site: none
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
Accept-Encoding: gzip, deflate, br
Accept-Language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6
Cookie: _EDGE_V=1;
"##;

        // let mut text: String = String::new();
        // match io::stdin().read_line(&mut text) {
        //     Ok(_) => {}
        //     Err(_) => {}
        // };
        // //let test2 = "asdsa      asd asdawd \r\n    \r adasdwa asd a f";
        // let http = match _HttpParser::new(&raw_text) {
        //     Ok(res) => res,
        //     Err(err) => {
        //         err.get()
        //     }
        // };
        // http.show_header();
        // http.show_body();

        //assert_eq!(Some(0), text.find("foo"));
        //println!("{:?}{:?}", lines, parser);
    }
    let text = "Hello World!".to_string();
    for i in text.as_bytes() {
        print!("{}   ", i);
    }
    let text2:&str = &text[..5];
    let text3 = &text[5..];
    let len = text2.len();
    let len: usize;
    let str = unsafe {
        let ptr = text2.as_ptr();
        len = text2.len() + text3.len();
        (ptr, len)
    };

    
}
