/// 学习Box
/// 1. 学习智能指针的一种Box。封装任何数据，存储在堆上
/// 2. 学习std::rc::Rc 引用计数， 实现共享只读数据
/// 3. 学习自定义智能指针，实现Drop trait中的drop方式
/// 4. 学习std::rc::Weak; 弱引用，避免引用循环。内存泄露


use std::rc::Rc;

///
/// BoxList 计算 "递归类型" 数据 - 解决Rust无法编译不知道大小的类型
/// Box<T> 可以处理在堆上存储的数据，而且可以确定大小。
/// Box<T> 是智能指针的一种，除了能够让数据存储到堆上，而不是栈上之外，Box 没有任何性能损失
/// Cons list (Cons 列表)，就是典型的函数式编程常见的类型。第一项存储一个值，第二项存储的是另外一个Cons list。
/// 这样，就很难知道这种类型的数据到底需要多大空间来存储。
/// Box<T> 只允许有一个所有者
enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

///
/// RecList 计算 "递归类型 + 共享只读数据"
/// Rec<T> 引用计数，是另外的智能指针的手段。
/// Rec<T> 允许相同数据的多个所有者。
/// 注意：Rec<T> 只能应用于单线程中，
/// 仅仅允许在编译时执行不可变的借用检查
enum RecList {
    Cons(i32, Rc<RecList>),
    Nil,
}


///
/// Mock 对象
///
trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {

    // 创建
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if (percentage_of_max >= 0.75 && percentage_of_max < 0.9) {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if (percentage_of_max >= 0.9 && percentage_of_max < 1.0)  {
            self.messenger.send("Urgent Warning: You've used up over 90% of your quota!");
        } else if (percentage_of_max >= 1.0) {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}


mod runner {
    use super::*;

    fn study_box() {
        // Box<T> 将一个已知的大小并指向分配在堆上的数据。
        //
        let list_rc = BoxList::Cons(1,
                                    Box::new(BoxList::Cons(2,
                                                           Box::new(BoxList::Cons(3,
                                                                                  Box::new(BoxList::Nil))))));
    }

    fn study_rc() {
        // Rc<T> 记录了堆上数据的引用数量以便可以拥有多个所有者。
        // 学习共享引用计数: Rc 引用计数的智能指针
        // 共享方式，Rc<T> 只允许读取不可变，只读的共享数据
        //
        let list_rc = Rc::new(RecList::Cons(1,
                                            Rc::new(RecList::Cons(2,
                                                                  Rc::new(RecList::Cons(3,
                                                                                        Rc::new(RecList::Nil)))))));
        println!("Count after list_ c = {}", Rc::strong_count(&list_rc)); // list_c = 1
        {
            let list_2 = RecList::Cons(10, Rc::clone(&list_rc));
            println!("Count after list_2 c = {}", Rc::strong_count(&list_rc)); // list_c = 2
            let list_3 = RecList::Cons(12, Rc::clone(&list_rc));
            println!("Count after list_3 c = {}", Rc::strong_count(&list_rc)); // list_c = 3
        }
        println!("Count after list_ c = {}", Rc::strong_count(&list_rc)); // list_c = 1


    }

    fn study_custom_smart_point() {
        // 学习自己封装智能指针。重点在Drop trait
        // 自定义部分，必须实现Drop trait

        struct CustomSmartPointer {
            data: String,
        }

        impl CustomSmartPointer {
            fn new(data: &str) -> CustomSmartPointer {
                CustomSmartPointer { data: String::from(data)}
            }
        }


        // 实现Drop trait
        use std::ops::Drop;
        impl Drop for CustomSmartPointer {
            fn drop (&mut self) {
                println!("Dropping CustomSmartPointer with data `{}` !", self.data);
            }
        }


        /**
        // 实现Deref trait
        use std::ops::Deref;
        impl Deref for CustomSmartPointer {
            type Target = CustomSmartPointer;
            fn deref(&self) -> &CustomSmartPointer {

            }
        }
        **/


        let c = CustomSmartPointer::new("Hi !!!!");


        // 自己封装的指针，实现了Drop trait，那么生命周期结束后，自动调用drop 方法。
        // 手动调用drop方法会出现错误
        // 输出结果： Dropping CustomSmartPointer with data `Hi` !
    }


    fn study_ref_cell() {
        // RefCell<T> 和其内部可变性提供了一个可以用于当需要不可变类型，但是需要改变其内部值能力的类型
        // 并在运行时而不是编译时检查借用规则。

        // 学习ref_cell 构建的mock对待
        use std::cell::RefCell;

        struct MockMessenger {
            send_message: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger { send_message: RefCell::new(vec![]) }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                // self 是不可变得。但是self中的send_message成员可以通过 borrow_mut() 函数变成可变的
                // 这种方式就是借用了 RefCell的机制
                self.send_message.borrow_mut().push(String::from(message));
                println!("{:?}", message);
            }
        }

        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

        }


        it_sends_an_over_75_percent_warning_message();
    }

    fn study_weak_ref() {
        use std::cell::RefCell;
        use std::rc::Weak;


        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 20,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        });

        let branch = Rc::new(Node {
            value: 35,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        println!("leaf strong = {:?}, weak = {:?} ", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // leaf strong = 2, weak = 0
        println!("branch strong = {:?}, weak = {:?}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // branch strong = 1, weak = 0

        // 使用Rc::downgrade() 方法，得到 &branch 的weak引用
        // 这样，branch 的弱引用就 +1
        let wk_branch = Rc::downgrade(&branch);
        *(leaf.parent.borrow_mut()) = wk_branch;

        println!("leaf strong = {:?}, weak = {:?} ", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // leaf strong = 2, weak = 0
        println!("branch strong = {:?}, weak = {:?}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // branch strong = 1, weak = 1

        // 使用 upgrade() 方法，获得 强引用
        let it_parent = leaf.parent.borrow().upgrade();
        println!("leaf parent = {:?}", it_parent);
        println!("leaf strong = {:?}, weak = {:?} ", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // leaf strong = 2, weak = 0
        println!("branch strong = {:?}, weak = {:?}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // branch strong = 1, weak = 1


    }


    pub fn run() {
        study_box();
        study_rc();
        study_custom_smart_point();
        study_ref_cell();
        study_weak_ref();
    }
}


pub fn run() {
    runner::run();
}







#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

