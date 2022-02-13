///
/// Module for a thread that loads
/// the data into the output file
/// every 10 blocks that it receives
/// from the mpsc channel.
/// 
pub mod out {

    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Error;
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
    }

    ///
    /// Implementation block for
    /// the output thread struct.
    /// 
    impl OutThread {
        pub fn new(pipe: mpsc::Receiver<String>, file_name: &'static str, exit_sig: mpsc::Sender<Result<(), Error>>) -> OutThread {
            let mut buf: Vec<String> = Vec::with_capacity(BUF_SIZE);
            let mut count = 0;

            let mut ret_out_thread = OutThread {
                file_name: file_name,
            };
            thread::spawn(move || loop {
                let data = match pipe.recv() {
                    Ok(s) => s,
                    Err(_) => { 
                        if let Err(_) = ret_out_thread.flush(buf) {
                            panic!("Could not flush the buffer.");
                        }
                        while let Err(_) = exit_sig.send(Ok(())) {}
                        break;
                    },
                };
                if count == BUF_SIZE {
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(file_name) {
                            Ok(f) => f,
                            Err(_) => {
                                panic!("Could not open file: {}", file_name);
                            }
                    };
                    for i in 0..count {
                        if let Err(_) = file.write_all(&[buf[i].as_bytes(), "\n".as_bytes()].concat()) {
                            panic!("Could not write to file: {}", file_name);
                        }
                    }
                    buf.clear();
                    count = 0;
                }
                buf.push(data);
                count += 1;
            });

            OutThread {
                file_name: file_name,
            }
        }

        fn flush(&mut self, buf: Vec<String>) -> Result<(), Error> {
            let mut file = match OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(self.file_name) {
                    Ok(f) => f,
                    Err(_) => panic!("Could not open file"),
            };
            for i in 0..buf.len() {
                file.write(&[buf[i].as_bytes(), "\n".as_bytes()].concat())?;
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
        let (sig, join) = mpsc::channel();
        {
        let (sender, receiver) = mpsc::channel();
        let _ = out::OutThread::new(receiver, "out.txt", sig);
        for _ in 0..33 {
            sender.send("hello".to_string()).unwrap();
        }
        }
        if let Err(_) = join.recv() {
            panic!("Could not get exit signal");
        }
    }
}