fn borrow<'a>(i: &'a i32, _j: &'a i32) -> &'a i32 {
    i
}

fn main() {
    borrow(&1, &2);
}
