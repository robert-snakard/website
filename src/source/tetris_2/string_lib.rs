extern {
    fn alert();
}

#[no_mangle]
pub fn run() {
    alert();
}
