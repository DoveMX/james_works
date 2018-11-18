use std::collections::HashMap;


static mut SHOW_COUNT: i32 = 0;

pub fn show() {

    unsafe {
        SHOW_COUNT = SHOW_COUNT + 1;
        println!("{0}. I'm a commuicator's log ...", SHOW_COUNT);
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("====>{:?} --- {:?}", scores, scores.get("Blue"));

    scores.entry("Blue").or_insert(44);
    println!("====>{:?} --- {:?}", scores, scores.get("Blue"));
}

