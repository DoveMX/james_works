///
///

mod runner {
    fn _line(){
        println!("{:?}", "---------------------------------------");
    }

//    #[cfg(feature = "server")]
    fn study_WebServer_case_sample() {
        // 学习简单的单线程、网络服务器

        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
        }


        _line();

    }

    pub fn run() {
        study_WebServer_case_sample();
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
