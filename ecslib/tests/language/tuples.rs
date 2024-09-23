#[test]
fn tuples() {
    let dimensions = (100.0, 100.0);
    print_dimensioins(dimensions);
    print_dimensioins((200.0, 200.0));
}

fn print_dimensioins(dimensions: (f32, f32)) {
    dbg!(dimensions.0);
}
fn print_dimensioins2((x, _y): (f32, f32)) {
    dbg!(x);
}
