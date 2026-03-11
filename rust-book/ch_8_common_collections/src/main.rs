fn main() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    // let does_not_exist = &v[100];
    // println!("does_not_exist={:?}", does_not_exist);
    // panics
    let does_not_exist = v.get(100);
    println!("does_not_exist={:?}", does_not_exist);

    for i in &mut v {
        *i += 50;
    }
    println!("v={:?}", v);

    #[derive(Debug)]
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
    println!("row={:?}", row);
}

fn strings() {
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("s={s}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let ttt = format!("{tic}-{tac}-{toe}");
    println!("ttt={ttt}");

    let hello = "Здравствуйте";
    for (i, c) in hello.chars().enumerate() {
        println!("{hello}[{i}] = {c}");
    }
}

fn hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores={:?}", scores);

    let text = "your feelings are your feelings";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("\"{text}\"={:?}", map);
}
