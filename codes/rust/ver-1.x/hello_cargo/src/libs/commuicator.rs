use std::collections::HashMap;


pub fn show(id: i32) {
    println!("Ways {0}. I'm a commuicator's log ...", id);

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("\t====>{:?} --- {:?}", scores, scores.get("Blue"));

    scores.insert("Blue", 44);
    println!("\t====>{:?} --- {:?}", scores, scores.get("Blue"));

}

