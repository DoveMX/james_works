pub mod commuicator;
pub mod error;


pub mod netwrok {
    pub mod client {
        pub fn connect() {
            println!("{:?}", "client is here!");

            let way_id: i32 = 0;


            // 第1种模型引用方式
            {
            	use libs::commuicator::show as commuicator_show;
            	commuicator_show(way_id+1);
            }


			{
	            // 第2种模型引用方式
            	use super::super::commuicator;
           	 	commuicator::show(way_id + 1);
			}


			{
	            // 第3种处理
            	use super::super::commuicator as M1;
            	M1::show(way_id + 1);
			}


            {
            	// 第4种方式
            	use ::libs::commuicator as M2;
            	M2::show(way_id + 1);
            }


            
            {
            	// 第5种方式
            	use super::super::{commuicator};
            	commuicator::show(way_id + 1);
            }

        }
    }
}




#[cfg(test)]
mod tests {

	#[derive(Debug)]
	struct Guess {
		value: u32
	}

	impl Guess {
		// add code here
		pub fn new(value: u32) -> Guess {
			if value < 1 || value > 100 {
				panic!("Guess value must be between 1 and 100, got {}.", value);
			}

			Guess {
				value
			}
		}
	}


    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
    	Guess::new(200);
    }

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
	pub fn add(a: i32, b: i32) -> i32 {
	    a + b
	}	
}