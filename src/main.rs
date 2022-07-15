use dict_rs::dict::*;

fn main() {
    let mut dict: Dict<i32> = Dict::new(1000);
    dict.set("rust", 15);
    dict.set(1234, 32);
    dict.set("hi", 1);
    dict.set([1, 2, 3], 78);
    dict.print(&"hi");
    dict.print(&1234); 
    dict.print(&"bad key");
    dict.print(&[1, 2, 3])
}