///
///


mod runner {
    fn _line(){
        println!("{:?}", "---------------------------------------");
    }

    fn study_derive_case_sample() {
        // 学习简单的 derive 过程宏



//        #[proc_macro_derive(HelloMacro)]
//        pub trait HelloMacro {
//            fn hello_macro() {
//                println!("Hello, Macro!");
//            }
//        }
//
//
//        #[derive(HelloMacro)]
//        struct Pancakes;
//        Pancakes::hello_macro();


//        #[proc_macro_derive(AnswerFn)]
        fn derive_answer_fn() {

        }

//        #[derive(AnswerFn)]
        struct Struct;


        _line();

    }

    pub fn run() {
        study_derive_case_sample();
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
