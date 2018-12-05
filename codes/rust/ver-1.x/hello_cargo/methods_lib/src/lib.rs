///
///


mod runner {
    fn _line(){
        println!("{:?}", "---------------------------------------");
    }

    fn study_method_case_sample() {
        // 学习简单的 derive 过程宏

        use std;

        trait HasArea {
            fn area(&self) -> f64;
        }

        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        struct Square {
            x: f64,
            y: f64,
            side: f64,
        }

        impl Circle {
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x,
                    y,
                    radius,
                }
            }

            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        impl HasArea for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        impl HasArea for Square {
            fn area(&self) -> f64 {
                self.side * self.side
            }
        }


        fn print_area<T: HasArea>(shape: T) {
            println!("This shape has an area of {}", shape.area());
        }


        /// 演示代码
        let c = Circle{ x: 0.0, y: 0.0, radius: 2.0};
        print_area(c);

        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String)
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        println!("{:?}", home);

        macro_rules! vector {
            ($($x:expr), *) => {
                {
                    let mut temp_vec = Vec::new();
                    $(temp_vec.push($x);)*
                    temp_vec
                }
            };
        }

        let a = vector![1,2,4,5,6];
        println!("{:?}", a);


        _line();

    }

    fn study_macro_case_sample() {
        // 学习宏的使用

        macro_rules! m_root_1 {
            () => {};
        }

        // 宏 m_root_1 这里是可用的
        m_root_1!();

        mod foo {
            // 宏 m_root_1 这里是可用的
            m_root_1!();

            #[macro_export]
            macro_rules! m_foo_1 {
                () => {};
            }

            m_root_1!();
            m_foo_1!();

        }

        macro_rules! m_root_3 {
            () => {};
        }

        mod bar {

            m_root_1!();
            m_root_3!();

            macro_rules! m_bar_1 {
                () => {};
            }

            m_root_1!();

            //#[macro_use]
            //m_foo_1!();

            m_root_3!();
            m_bar_1!();

            mod sub {
                m_root_1!();
                m_root_3!();

                #[macro_export]
                macro_rules! m_sub_1 {
                    () => {};
                }

            }

            m_root_1!();
            m_root_3!();
            m_bar_1!();

            #[macro_use]
            m_sub_1!();


        }

        m_root_1!();
        m_root_3!();


    }

    pub fn run() {
        study_method_case_sample();
        study_macro_case_sample();
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
