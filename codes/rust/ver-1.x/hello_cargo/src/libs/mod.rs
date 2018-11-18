pub mod commuicator;




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod netwrok {
    pub mod client {
        pub fn connect() {
            println!("{:?}", "client is here!");

            // 第1种模型引用方式
            use libs::commuicator::show as commuicator_show;
            commuicator_show();

            // 第2种模型引用方式
            use super::super::commuicator;
            commuicator::show();

            // 第3种处理
            use super::super::commuicator as M1;
            M1::show();

            // 第4种方式
            use ::libs::commuicator as M2;
            M2::show();

            // 第5种付出
            {
            	use super::super::{commuicator};
            	commuicator::show();
            }

        }
    }
}