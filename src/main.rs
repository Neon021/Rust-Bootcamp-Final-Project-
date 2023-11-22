fn concatenate_strings(str1: &str, str2: &str) -> String{
    let mut result: String = String::new();
    result.push_str(str1);
    result.push_str(str2);

    return result;
}

fn main(){
    let str1: &str = "Furkan";
    let str2: &str = "Bilal";

    let result: String = concatenate_strings(str1, str2);
    println!("{}", result);
}