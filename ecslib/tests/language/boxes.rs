use std::any::{Any, TypeId};

#[test]
fn boxes() {
    let type_id = get_type_id(Box::new(15_u32));
    dbg!(type_id);
}

fn get_type_id(boxed_thing: Box<dyn Any>) -> TypeId {
    boxed_thing.type_id()
}
