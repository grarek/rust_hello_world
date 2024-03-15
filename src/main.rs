fn main() {
    let mut parsed = json::parse(
        r#"
    {
        "code": "200"
    }
    "#,
    )
    .unwrap();
    parsed["test"] = "42".into();

    println!("{} is a json string", parsed);
    print!("Hello, world!\n");
}
