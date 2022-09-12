// 这里有一个编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，
// 并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，
// 则整个字符串就是一个单词，所以应该返回整个字符串。
fn main() {
    let string = String::from("Fuck好");
    if first_word(&string) == string.len() {
        println!("{}", string);
    } else {
        println!("{}", first_word(&string));
    }

    println!("{} {}", slice_word(&string), slice_word(&string).len());

    array_slice();
}

fn first_word(str: &String) -> usize {
    let byte = str.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return str.len();
}
//“字符串 slice” 的类型声明写作 &str：
fn slice_word(str: &String) -> &str {
    let byte = str.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    return &str[..];
}

fn array_slice(){
    let arr = [1,2,3,4,5,6];
    let slice = &arr[..3];
    assert_eq!(&[1,2,3],slice);
}