fn main() {
    let v = [1,2,3,4,5];

    unsafe {
        let value = v.get_unchecked(2);
        println!("{value}");
    }
}
