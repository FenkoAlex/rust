use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {third}");
    println!("vec: {:?}", v);

    // let mut someOfVec = &v[1];
    let someOfVec = v.get(1);
    {
        let mut first = v.get(0);
        // first = Some(&22);
        // v.push(6);

        match first {
            Some(value) => println!("{}", value),
            // Some(value) => someOfVec = value,
            _ => {}
        }
    }
    v.push(6);
    // println!("someOfVec: {:?}", someOfVec);
    println!("vec: {:?}", v);

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{}", &s3[..1]);
    println!("{}", s);

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let team_name = "Blue";
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", scores);
    println!("{}", score);

    scores.entry("Red").or_insert(50);
    scores.entry("Blue").or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
