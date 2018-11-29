/// Test File
///
///
/// # Examples
///
///


use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt::Display;


pub fn show(id: i32) {
    println!("Ways {0}. I'm a commuicator's log ...", id);

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("\t====>{:?} --- {:?}", scores, scores.get("Blue"));

    scores.insert("Blue", 44);
    println!("\t====>{:?} --- {:?}", scores, scores.get("Blue"));

}

///
/// Cacher
///
#[derive(Debug)]
struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    // add code here
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                println!("[0022] will call the calculation ...");
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


///
/// 动态调用生成输出结果
pub fn generate_workout(intensity: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num + 2999
    });

    println!("Do ---> {:?} ", expensive_result.value(intensity));

    let mut c = Cacher::new(|num| num);
    let v1 = c.value(1);
    let v2 = c.value(2);


    /// This will be not right at here
    println!("v1 = {0}, v2 = {1}", v1, v2);
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));

}
