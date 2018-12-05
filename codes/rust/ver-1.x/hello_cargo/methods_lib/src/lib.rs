///
///
use std::fmt::Debug;

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
        }

        m_root_1!();
        m_root_3!();


    }

    fn study_format_case_code() {
        let s = format!("{1} 是个有着{0:>0width$}KG重，{height:?}cm高的大胖子! [{0}, {1}, {2}, {3}]",
                    81, "wayslog", width=4, height=178);
        //# 参数位置说明          
        //{0} 表示索引为0的参数，例如：这里是 81           
        //{1} 表示索引为1的参数，例如：这里是 "wayslog"
        //{2} 表示索引为2的参数，例如：这里是 4
        //{3} 表示索引为2的参数，例如：这里是 178
        
        //# key-value参数表达方式
        // width=4
        // height=178
        
        println!("{}", s);
        
        //# output
        // wayslog 是个有着0081KG重，178cm高的大胖子! [81, wayslog, 4, 178]
        
        
        _line();
    }
    
    fn study_trait_case_code() {
        println!("--> 测试 Trait 的使用 ...");
        
        trait IControler {
            fn build(&self) -> bool;
            fn scale(&self, zoom: f32);
        }
        
        trait IDisplay {
            fn draw(&self);
        }
        
        test_signle_inheritance();
        test_multiple_inheritance();
        test_multiple_inheritance_conflict();
        test_generic_traits();


        /// 测试 trait 的单一继承
        fn test_signle_inheritance() {
            println!("fn test_signle_inheritance()");
            
            trait IToolBar : IControler {
                fn switch(&self);
            }
            
            struct WinToolBar;
       
            impl IControler for WinToolBar {
                fn build(&self) -> bool {
                    println!("call IControler build ... ");
                    false
                }
                
                fn scale(&self, zoom: f32) {
                    println!("call IControler scale ... ");
                }
            }
            
            impl IToolBar for WinToolBar {
                fn switch(&self) {
                    println!("call IToolBar switch ... ");
                }
            }
            

            let bar = WinToolBar{};
            bar.build();
            bar.scale(0.5f32);
            bar.switch();
        }
        
        /// 测试 trait 的多继承
        fn test_multiple_inheritance() {
            println!("fn test_multiple_inheritance()");
        
            trait IToolBar : IControler +  IDisplay{
                fn switch(&self);
            }
            
            struct WinToolBar;
       
            impl IControler for WinToolBar {
                fn build(&self) -> bool {
                    println!("call IControler build ... ");
                    false
                }

                fn scale(&self, zoom: f32) {
                    println!("call IControler scale ... ");
                }
            }
            
            impl IToolBar for WinToolBar {
                fn switch(&self) {
                    println!("call IToolBar switch ... ");
                }
            }
            
            impl IDisplay for WinToolBar {
                fn draw(&self) {
                    println!("call IDisplay draw ... ");
                }
            }
            
            let bar = WinToolBar{};
            bar.build();
            bar.scale(0.5f32);
            bar.switch();
            bar.draw();
        }
    
        /// 测试 trait 继承中的冲突问题
        fn test_multiple_inheritance_conflict() {
            println!("fn test_multiple_inheritance_conflict()");
        
            trait IToolBar : IControler +  IDisplay{
                fn build(&self) -> bool {
                    println!("call IToolBar build ... ");
                    false
                }
                fn switch(&self);
            }
            
            struct WinToolBar;
       
            impl IControler for WinToolBar {
                fn build(&self) -> bool {
                    println!("call IControler build ... ");
                    false
                }

                fn scale(&self, zoom: f32) {
                    println!("call IControler scale ... ");
                }
            }
            
            impl IToolBar for WinToolBar {
                fn switch(&self) {
                    println!("call IToolBar switch ... ");
                }
            }
            
            impl IDisplay for WinToolBar {
                fn draw(&self) {
                    println!("call IDisplay draw ... ");
                }
            }
            
            let bar = WinToolBar{};
            /// 由于IControler中定义了build函数，IToolBar 也定义了build函数，
            /// 那么执行 bar.build() 就会得到以下错误
            //// 309 |             bar.build();
            //       |                 ^^^^^ multiple `build` found
            /// 因此，需要完全限定语法，消除歧义
            IToolBar::build(&bar);
            IControler::build(&bar);
            
            bar.scale(0.5f32);
            bar.switch();
            bar.draw();
        }
    
        fn test_generic_traits() {
            println!("fn test_generic_traits()");
            
            use std::fmt::Debug;
            
            trait Seq<T> {
                fn dummy(&self, _: T) where T: Debug;
            }

            impl<T> Seq<T> for Vec<T> {
                /* */
                fn dummy(&self, v: T) where T: Debug{
                    println!("call dummy ... {:?}", v);
                }
            }
            
            let m = Vec::<bool>::new() ;
            println!("{:?}", m);
            m.dummy(true);
        }
        _line();
    }
    
    pub fn run() {
        study_method_case_sample();
        study_macro_case_sample();
        study_format_case_code();
        study_trait_case_code();
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
