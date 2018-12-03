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


        _line();

    }

    pub fn run() {
        study_method_case_sample();
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
