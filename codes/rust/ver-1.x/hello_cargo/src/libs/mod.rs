
mod commuicator;

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

            commuicator::show();
        }
    }
}