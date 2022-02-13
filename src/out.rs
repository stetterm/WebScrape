///
/// Module for a thread that loads
/// the data into the output file
/// every 10 blocks that it receives
/// from the mpsc channel.
/// 
pub mod out {

    use parking_lot::RwLock;

    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::sync::Arc;
    use std::sync::mpsc;
    use std::thread;

    const BUF_SIZE: usize = 16;

    ///
    /// Struct to represent the
    /// thread that loads the scraped
    /// data into the file.
    /// 
    /// pipe: receiving end of the mpcs channel.
    /// file_name: name of the output file.
    /// buf: the buffer that holds a number of strings before loading
    ///         to a file.
    /// 
    pub struct OutThread {
        file_name: &'static str,
        buf: Arc<RwLock<Vec<String>>>,
    }

    ///
    /// Implementation block for
    /// the output thread struct.
    /// 
    impl OutThread {
        pub fn new(pipe: mpsc::Receiver<String>, file_name: &'static str) -> OutThread {
            let mut buf = Arc::new(RwLock::new(Vec::with_capacity(BUF_SIZE)));
            let mut count = 0;

            let thread_buf = Arc::clone(&buf);
            thread::spawn(move || loop {
                let data = match pipe.recv() {
                    Ok(s) => s,
                    Err(e) => break,
                };
                if count == BUF_SIZE {
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(file_name) {
                            Ok(f) => f,
                            Err(e) => {
                                panic!("Could not open file: {}", file_name);
                            }
                    };
                    // let thread_buf = thread_buf.read();
                    // for i in 0..count {
                    //     if let Err(e) = file.write_all(thread_buf[i].as_bytes()) {
                    //         panic!("Could not write to file: {}", file_name);
                    //     }
                    // }
                }
                // let buf = buf.lock();

                // count += 1;
            });

            OutThread {
                file_name: file_name,
                buf: Arc::clone(&buf),
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn vec_test() {
        let mut m = Vec::with_capacity(10);
        m.push(6);
        dbg!(m);
    }
}