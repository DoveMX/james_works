mod runner {
    fn _line(){
        println!("{:?}", "---------------------------------------");
    }

    fn study_pattern_case_match() {
//        match 分支
//
//        如第六章所讨论的，一个模式常用的位置是 match 表达式的分支。在形式上 match 表达式由 match 关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式：
//
//        match VALUE {
//            PATTERN => EXPRESSION,
//            PATTERN => EXPRESSION,
//            PATTERN => EXPRESSION,
//        }
//
//        match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到。一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式 —— 比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。
//
//        有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。这在例如希望忽略任何未指定值的情况很有用。本章之后会详细讲解。


        _line();

    }


    fn study_pattern_case_if_let() {
//        第六章讨论过了 if let 表达式，以及它是如何主要用于编写等同于只关心一个情况的 match 语句简写的。if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行。
//
//        示例 18-1 展示了也可以组合并匹配 if let、else if 和 else if let 表达式。这相比 match 表达式一次只能将一个值与模式比较提供了更多灵活性；一系列 if let/else if/else if let 分支并不要求其条件相互关联。
//
//        示例 18-1 中的代码展示了一系列针对不同条件的检查来决定背景颜色应该是什么。为了达到这个例子的目的，我们创建了硬编码值的变量，在真实程序中则可能由询问用户获得。
//
//        如果用户指定了中意的颜色，将使用其作为背景颜色。如果今天是星期二，背景颜色将是绿色。如果用户指定了他们的年龄字符串并能够成功将其解析为数字的话，我们将根据这个数字使用紫色或者橙色。最后，如果没有一个条件符合，背景颜色将是蓝色：
//

        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color { // None, 不运行
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday { // false
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {  // 枚举类型：age = 34
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }

        _line();
    }

    fn study_pattern_case_while_let(){
//   while let 条件循环
//
//   一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。
//   示例 18-2 展示了一个使用 while let 的例子，它使用 vector 作为栈并以先进后出的方式打印出 vector 中的值：
//

        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        _line();
    }

    fn study_pattern_case_for(){
//   如同第三章所讲的，for 循环是 Rust 中最常见的循环结构，不过还没有讲到的是 for 可以获取一个模式。
//   在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。
//
//   示例 18-3 中展示了如何使用 for 循环来解构，或拆开一个元组作为 for 循环的一部分：



        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }

        _line();
    }

    fn study_pattern_case_let() {
//let 语句
//
//在本章之前，我们只明确的讨论过通过 match 和 if let 使用模式，不过事实上也在别地地方使用过模式，包括 let 语句。例如，考虑一下这个直白的 let 变量赋值：
//
//
//let x = 5;
//
//本书进行了不下百次这样的操作。你可能没有发觉，不过你这正是在使用模式！let 语句更为正式的样子如下：
//
//let PATTERN = EXPRESSION;
//
//像 let x = 5; 这样的语句中变量名位于 PATTERN 位置，变量名不过是形式特别朴素的模式。我们将表达式与模式比较，并为任何找到的名称赋值。所以例如 let x = 5; 的情况，x 是一个模式代表 “将匹配到的值绑定到变量 x”。同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
//
//为了更清楚的理解 let 的模式匹配方面的内容，考虑示例 18-4 中使用 let 和模式解构一个元组：
//
//
//let (x, y, z) = (1, 2, 3);
//
//示例 18-4: 使用模式解构元组并一次创建三个变量
//
//这里将一个元组与模式匹配。Rust 会比较值 (1, 2, 3) 与模式 (x, y, z) 并发现此值匹配这个模式。在这个例子中，将会把 1 绑定到 x，2 绑定到 y 并将 3 绑定到 z。你可以将这个元组模式看作是将三个独立的变量模式结合在一起。
//
//如果模式中元素的数量不匹配元组中元素的数量，则整个类型不匹配，并会得到一个编译时错误。例如，示例 18-5 展示了尝试用两个变量解构三个元素的元组，这是不行的：
//
//let (x, y) = (1, 2, 3);
//
//示例 18-5: 一个错误的模式结构，其中变量的数量不符合元组中元素的数量
//
//尝试编译这段代码会给出如下类型错误：
//
//error[E0308]: mismatched types
//    --> src/main.rs:2:9
//    |
//    2 |     let (x, y) = (1, 2, 3);
//|         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
//    |
//    = note: expected type `({integer}, {integer}, {integer})`
//found type `(_, _)`
//
//如果希望忽略元组中一个或多个值，也可以使用 _ 或 ..，如 “忽略模式中的值” 部分所示。如果问题是模式中有太多的变量，则解决方法是通过去掉变量使得变量数与元组中元素数相等。
//
        let (x, y, z) = (1, 2, 3);
        println!("x = {0}, y = {1}, z = {2}", x, y, z);

        _line();
    }


    fn study_pattern_case_function_args(){
//函数参数
//
//函数参数也可以是模式。列表 18-6 中的代码声明了一个叫做 foo 的函数，它获取一个 i32 类型的参数 x，现在这看起来应该很熟悉：
//
//
//fn foo(x: i32) {
//    // code goes here
//}
//
//列表 18-6: 在参数中使用模式的函数签名
//
//x 部分就是一个模式！类似于之前对 let 所做的，可以在函数参数中匹配元组。列表 18-7 将传递给函数的元组拆分为值：
//
//文件名: src/main.rs
//
//fn print_coordinates(&(x, y): &(i32, i32)) {
//    println!("Current location: ({}, {})", x, y);
//}
//
//fn main() {
//    let point = (3, 5);
//    print_coordinates(&point);
//}
//
//列表 18-7: 一个在参数中解构元组的函数
//
//这会打印出 Current location: (3, 5)。值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5。

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);

        _line();
    }


    fn study_pattern_case_all_ways() {

        fn pattern_for_num() {
//        匹配字面值
//
//        如第六章所示，可以直接匹配字面值模式。如下代码给出了一些例子：

            #![allow(unused_variables)]
            fn main() {
                let x = 1;

                match x {
                    1 => println!("one"),
                    2 => println!("two"),
                    3 => println!("three"),
                    _ => println!("anything"),
                }
            }

            main();
            _line();
        }

        fn pattern_for_variable() {
//        匹配命名变量
//
//        命名变量是匹配任何值的不可反驳模式，这在之前已经使用过数次。然而当其用于 match 表达式时情况会有些复杂。
//        因为 match 会开始一个新作用域，match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量
//        ———— 与所有变量一样。在示例 18-11 中，声明了一个值为 Some(5) 的变量 x 和一个值为 10 的变量 y。
//        接着在值 x 上创建了一个 match 表达式。观察匹配分支中的模式和结尾的 println!，
//        并尝试在运行代码之前计算出会打印什么，或者继续阅读：

//        示例 18-11: 一个 match 语句其中一个分支引入了覆盖变量 y
//
//        让我们看看当 match 语句运行的时候发生了什么。第一个匹配分支的模式并不匹配 x 中定义的值，所以继续。
//
//        第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为我们在 match 表达式的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 y。这个新的 y 绑定会匹配任何 Some 中的值，在这里是 x 中的值。因此这个 y 绑定了 x 中 Some 内部的值。这个值是 5，所以这个分支的表达式将会执行并打印出 Matched, y = 5。
//
//        如果 x 的值是 None 而不是 Some(5)，头两个分支的模式不会匹配，所以会匹配下划线。这个分支的模式中没有引入变量 x，所以此时表达式中的 x 会是外部没有被覆盖的 x。在这个假想的例子中，match 将会打印 Default case, x = None。
//
//        一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。最后的 println! 会打印 at the end: x = Some(5), y = 10。
//
//        为了创建能够比较外部 x 和 y 的值，而不引入覆盖变量的 match 表达式，我们需要相应的使用带有条件的匹配守卫（match guard）。本部分的后面会讨论匹配守卫。

            fn main() {
                let x = Some(5);
                let y = 10;

                match x {
                    Some(50) => println!("Got 50"),

                    //注意：match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量
                    //那么，Some(y)中的y 与 之前的y 不是同一个
                    Some(y) => println!("Matched, y = {:?}", y),
                    _ => println!("Default case, x = {:?}", x),
                }

                println!("at the end: x = {:?}, y = {:?}", x, y);
            }

            main();
            _line();
        }

        fn pattern_for_more() {
//    多个模式
//
//    在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或（or）的意思。例如，如下代码将 x 的值与匹配分支向比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任一个值，它就会运行：
//
//
//    let x = 1;
//
//    match x {
//        1 | 2 => println!("one or two"),
//        3 => println!("three"),
//        _ => println!("anything"),
//    }
//
//    上面的代码会打印 one or two。

            #![allow(unused_variables)]
            fn main1() {
                let x = 1;

                match x {
                    1 | 2 => println!("one or two"),
                    3 => println!("three"),
                    _ => println!("anything"),
                }

            }


//通过 ... 匹配值的范围
//    ... 语法允许你匹配一个闭区间范围内的值。在如下代码中，当模式匹配任何在此范围内的值时，该分支会执行：
//如果 x 是 1、2、3、4 或 5，第一个分支就会匹配。这相比使用 | 运算符表达相同的意思更为方便；相比 1 ... 5，使用 | 则不得不指定 1 | 2 | 3 | 4 | 5。相反指定范围就简短的多，特别是在希望匹配比如从 1 到 1000 的数字的时候！
//
//范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 唯一知道范围是否为空的类型。
//
//如下是一个使用 char 类型值范围的例子：


            fn main2() {
                let x = 5;

                match x {
                    1 ... 5 => println!("one through five"),
                    _ => println!("something else"),
                }

                let y = 'c';

                match y {
                    'a' ... 'j' => println!("early ASCII letter"),
                    'k' ... 'z' => println!("late ASCII letter"),
                    _ => println!("something else"),
                }
            }


            main1();
            main2();
            _line();
        }

        fn pattern_for_unwrap() {
//    解构并分解值
//
//    也可以使用模式来解构结构体、枚举、元组和引用，以便使用这些值的不同部分。让我们来分别看一看。
//    解构结构体
//
//    示例 18-12 展示带有两个字段 x 和 y 的结构体 Point，可以通过带有模式的 let 语句将其分解：

            fn main_1() {
                struct Point {
                    x: i32,
                    y: i32,
                }

                let p = Point { x: 0, y: 7 };

                let Point { x: a, y: b } = p;
                assert_eq!(0, a);
                assert_eq!(7, b);
            }


//    示例 18-12: 解构一个结构体的字段为单独的变量
//
//    这段代码创建了变量 a 和 b 来匹配变量 p 中的 x 和 y 字段。
//
//    这个例子展示了模式中的变量名不必与结构体中的字段名一致，不过通常希望变量名与字段名一致以便于理解变量来自于哪些字段。
//    因为变量名匹配字段名是常见的，同时因为 let Point { x: x, y: y } = p; 包含了很多重复，
//    所以对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，
//    则模式创建的变量会有相同的名称。示例 18-13 展示了与示例 18-12 有着相同行为的代码，不过 let 模式创建的变量为 x 和 y 而不是 a 和 b：



            fn main_2() {
                struct Point {
                    x: i32,
                    y: i32,
                }

                let p = Point { x: 0, y: 7 };

                let Point { x, y } = p;
                assert_eq!(0, x);
                assert_eq!(7, y);
            }

//这段代码创建了变量 x 和 y，与变量 p 中的 x 和 y 相匹配。其结果是变量 x 和 y 包含结构体 p 中的值。
//
//也可以在部分结构体模式中使用字面值进行结构，而不是为所有的字段创建变量。这允许我们测试一些字段为特定值的同时创建其他字段的变量。
//
//示例 18-14 展示了一个 match 语句将 Point 值分成了三种情况：直接位于 x 轴上（此时 y = 0 为真）、位于 y 轴上（x = 0）或其他的点：

            fn main_3() {
                struct Point {
                    x: i32,
                    y: i32,
                }

                let p = Point { x: 0, y: 7 };

                match p {
                    Point { x, y: 0 } => println!("On the x axis at {}", x),
                    Point { x: 0, y } => println!("On the y axis at {}", y),
                    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
                }
            }

//            示例 18-14: 解构和匹配模式中的字面值
//
//            第一个分支通过指定字段 y 匹配字面值 0 来匹配任何位于 x 轴上的点。此模式仍然创建了变量 x 以便在分支的代码中使用。类似的，第二个分支通过指定字段 x 匹配字面值 0 来匹配任何位于 y 轴上的点，并为字段 y 创建了变量 y。第三个分支没有指定任何字面值，所以其会匹配任何其他的 Point 并为 x 和 y 两个字段创建变量。
//
//            在这个例子中，值 p 因为其 x 包含 0 而匹配第二个分支，因此会打印出 On the y axis at 7。




            fn unwrap_enum() {

//                解构枚举
//
//                本书之前的部分曾经解构过枚举，
//                比如第六章中示例 6-5 中解构了一个 Option<i32>。
//                一个当时没有明确提到的细节是解构枚举的模式需要对应枚举所定义的储存数据的方式。
//                让我们以示例 6-2 中的 Message 枚举为例，编写一个 match 使用模式解构每一个内部值，如示例 18-15 所示

                enum Message {
                    Quit,
                    Move { x: i32, y: i32 },
                    Write(String),
                    ChangeColor(i32, i32, i32),
                }

                fn main() {
                    let msg = Message::ChangeColor(0, 160, 255);

                    match msg {
                        Message::Quit => {
                            println!("The Quit variant has no data to destructure.")
                        },
                        Message::Move { x, y } => {
                            println!(
                                "Move in the x direction {} and in the y direction {}",
                                x,
                                y
                            );
                        }
                        Message::Write(text) => println!("Text message: {}", text),
                        Message::ChangeColor(r, g, b) => {
                            println!(
                                "Change the color to red {}, green {}, and blue {}",
                                r,
                                g,
                                b
                            )
                        }
                    }
                }

                main();

            }

            main_1();
            main_2();
            main_3();
            unwrap_enum();
        }


        // 调用子过程
        _line();
        pattern_for_num();
        pattern_for_variable();
        pattern_for_more();
        pattern_for_unwrap();
    }

    pub fn run() {
        study_pattern_case_match();
        study_pattern_case_if_let();
        study_pattern_case_while_let();
        study_pattern_case_let();
        study_pattern_case_function_args();
        study_pattern_case_all_ways();

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
