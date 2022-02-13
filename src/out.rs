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
    use std::io::Error;
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
            let buf: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::with_capacity(BUF_SIZE)));
            let mut count = 0;

            let thread_buf = Arc::clone(&buf);
            let mut ret_out_thread = OutThread {
                file_name: file_name,
                buf: Arc::clone(&buf),
            };
            thread::spawn(move || loop {
                let data = match pipe.recv() {
                    Ok(s) => s,
                    Err(_) => { 
                        if let Err(_) = ret_out_thread.flush() {
                            panic!("Could not flush the buffer.");
                        }
                        break;
                    },
                };
                if count == BUF_SIZE {
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(file_name) {
                            Ok(f) => f,
                            Err(_) => {
                                panic!("Could not open file: {}", file_name);
                            }
                    };
                    {
                    let thread_buf = thread_buf.read();
                    for i in 0..count {
                        if let Err(_) = file.write_all(&[thread_buf[i].as_bytes(), "\n".as_bytes()].concat()) {
                            panic!("Could not write to file: {}", file_name);
                        }
                    }
                    }
                    {
                    let mut thread_buf = thread_buf.write();
                    thread_buf.clear();
                    }
                    count = 0;
                }
                {
                let mut thread_buf = thread_buf.write();
                thread_buf.push(data);
                }
                count += 1;
            });

            OutThread {
                file_name: file_name,
                buf: Arc::clone(&buf),
            }
        }

        fn flush(&mut self) -> Result<(), Error> {
            let buffer = self.buf.write();
            let mut file = match OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.file_name) {
                    Ok(f) => f,
                    Err(_) => panic!("Could not open file"),
            };
            for i in 0..buffer.len() {
                file.write(&[buffer[i].as_bytes(), "\n".as_bytes()].concat())?;
            }

            Ok(())
        }
    }
}

mod tests {
    use super::*;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn vec_test() {
        let mut m = Vec::with_capacity(10);
        m.push(6);
        dbg!(m);
    }

    #[test]
    fn test_data_output() {
        let (sender, receiver) = mpsc::channel();
        let _test = out::OutThread::new(receiver, "out.txt");
        for _ in 0..33 {
            sender.send("hello".to_string()).unwrap();
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}