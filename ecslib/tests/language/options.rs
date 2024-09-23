#[test]
fn options() {
    dbg!(hello(Some("fdxxw")));
    dbg!(hello(None));
    dbg!(hello2(Some("fdxxw")));
}

fn hello(name: Option<&str>) -> String {
    // if let Some(name) = name {
    //   format!("hello {}", name)
    // } else {
    //   "hello stranger".to_string()
    // }
    match name {
        Some(name) => format!("hello {}", name),
        None => "hello stranger".to_string(),
    }
}

fn hello2(name: Option<&str>) -> Option<String> {
    name.map(|name| format!("hello {}", name))
}

fn hello3(name: Option<&str>) -> String {
    // let unwrapped_Name = name.unwrap_or_default();
    // let unwrapped_name = name.unwrap_or("stranger");
    let unwrapped_name = name.unwrap_or_else(|| "stranger");
    format!("hello {}", unwrapped_name)
}
fn hello4(name: Option<MyName>) -> String {
    let unwrapped_name = name.unwrap_or_else(|| MyName {
        name: "stranger".to_string(),
    });
    format!("hello {}", unwrapped_name.name)
}

struct MyName {
    name: String,
}
