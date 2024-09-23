#[test]
fn filter_map() {
    let mut numbers = vec![];
    for i in 0..100 {
        numbers.push(i);
    }

    let wrapped_numbers = numbers
        .iter()
        // .map(|number| WrappedNumber {
        //     value: *number,
        //     is_even: number % 2 == 0,
        // })
        // .filter(|wrapper_number| wrapper_number.is_even)
        .filter_map(|number| {
            let is_even = number % 2 == 0;
            if is_even {
                Some(WrappedNumber {
                    value: *number,
                    is_even,
                })
            } else {
                None
            }
        })
        .collect::<Vec<WrappedNumber>>();
    dbg!(wrapped_numbers);
}

#[derive(Debug)]
struct WrappedNumber {
    value: i32,
    is_even: bool,
}
