use dict_rs::dict::*;

fn main() {
    let mut dict: Dict<&str, i32> = Dict::new();
    dict.set("rust", 15);
    dict.set("ferris", 32);
    dict.set("hi", 1);
    dict.set("rust is fun", 78);
    dict.print(&"hi");
    dict.print(&"ferris"); 
    dict.print(&"bad key");
    dict.print(&"rust is fun")
}