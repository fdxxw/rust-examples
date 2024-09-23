use std::any::{Any, TypeId};

#[test]
fn generics() {
    let doubled = double_it(15.0_f32);
    dbg!(doubled);
    let doubled = double_it(15_i32);
    dbg!(doubled);

    let number1 = NumberWrapper { data: 1_u32 };
    let number2 = NumberWrapper { data: 2.0_f32 };
    let numbers = Numbers {
        data: vec![10_u32],
        other_data: vec![15.0_f32],
    };

    let type_id = get_type_id(15_u32);
    dbg!(type_id);
    let type_id = get_type_id2::<u32>();
    dbg!(type_id);
}

fn double_it<T>(number: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    number + number
}

struct NumberWrapper<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    data: T,
}

struct Numbers<T, S>
where
    T: std::ops::Add<Output = T> + Copy,
    S: std::ops::Add<Output = S> + Copy,
{
    data: Vec<T>,
    other_data: Vec<S>,
}


fn get_type_id<T>(somgthing: T) -> TypeId where T: Any {
  somgthing.type_id()
}

fn get_type_id2<T>() -> TypeId where T: Any {
  TypeId::of::<T>()
}
