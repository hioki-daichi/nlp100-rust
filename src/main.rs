fn main() {
    t00();
}

// 00. 文字列の逆順
fn t00() {
    assert_eq!("stressed".chars().rev().collect::<String>(), "desserts");
}
