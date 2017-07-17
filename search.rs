use std::io;

fn main() {
    // this is a macro, not a function
    println!("Search numbers in an array");

    // rust have an inferential type
    // lineal seach
    // static array
    let number_list = [1,2,3,4,5,6,7,8,9,10];

    let mut number_to_find = String::new();

    // {:?} - allows to execute in debugger mode
    // {:#?} pretty print
    println!("Found Element {} {:?}", number_list[3], number_list);
    println!("{:#?}", number_list);
    println!("Found element {mydata:?} {}", number_list[3], mydata = number_list);

    io::stdin().read_line(&mut number_to_find)
    .expect("error to read the line");

    // let number_to_find = 3;
    let number_to_find: u32 = number_to_find.trim().parse()
    .expect("Please write a number");

    let is_found = linear_search(number_to_find, number_list);
    if is_found {
        println!("Found Element {}", number_to_find);
    } else {
        println!("Found Element: {} in an array: {:#?}", number_to_find, number_list);
    }
}

fn linear_search(key: u32, number_list: [u32; 10]) -> bool {
    for number in number_list.iter() {
        if key == *number {
            return true;
        }
    }
    return false;
}
