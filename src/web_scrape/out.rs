pub use out::OutThread;

///
/// Module for a thread that loads
/// the data into the output file
/// every 64 blocks that it receives
/// from the mpsc channel.
/// 
pub mod out {

    use std::fs::OpenOptions;
    use std::io::Error;
    use std::io::prelude::*;
    use std::sync::mpsc;
    use std::thread;

    // Change value to adjust buffer size for IO
    const BUF_SIZE: usize = 64;

    ///
    /// Struct to represent the
    /// thread that loads the scraped
    /// data into the file.
    /// 
    pub struct OutThread;

    ///
    /// Implementation block for
    /// the output thread struct.
    /// 
    impl OutThread {

        ///
        /// Called to initialize the OutThread struct.
        /// pipe: Receiver of type string to get the strings
        ///         to write to the file.
        /// file_name: name of the file to use.
        /// exit_sig: Sender of Result type to send a signal
        ///         to the calling thread when execution has finished.
        /// 
        /// After new is called, the calling thread must call
        /// recv on the receiving end of exit_sig, otherwise the thread
        /// will likely not finished writing to the file.
        /// 
        pub fn init(pipe: mpsc::Receiver<String>, file_name: String, exit_sig: mpsc::Sender<Result<(), Error>>) {

            // buf is the buffer used for file IO
            let mut buf: Vec<String> = Vec::with_capacity(BUF_SIZE);

            // Creates the struct used by the thread
            // to flush the buffer.
            let out = OutThread;

            // Here is the main thread loop for
            // the output.
            thread::spawn(move || loop {

                // Receives new data from the pipe or
                // flushes the buffer and returns if
                // there are no more senders available.
                let data = match pipe.recv() {
                    Ok(s) => s,
                    Err(_) => { 
                        if let Err(_) = out.flush(buf, String::from(&file_name)) {
                            panic!("Cannot write to output file");
                        }
                        while let Err(_) = exit_sig.send(Ok(())) {}
                        break;
                    },
                };

                // If the buffer is full, then the thread writes
                // all the data in the buffer to the file.
                if buf.len() == BUF_SIZE {
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&file_name) {
                            Ok(f) => f,
                            Err(_) => {
                                panic!("Could not open file: {}", &file_name);
                            }
                    };
                    for i in 0..buf.len() {
                        if let Err(_) = file.write_all(&[buf[i].as_bytes(), "\n".as_bytes()].concat()) {
                            panic!("Could not write to file: {}", &file_name);
                        }
                    }

                    // Clears the buffer after writing.
                    buf.clear();
                }

                // Pushes the new value from the channel
                // into the buffer.
                buf.push(data);
            });
        }

        ///
        /// Flushes the remaining strings in the
        /// buffer and writes them to the file.
        /// If the file cannot be opened, it panics.
        /// If the file cannot be written to, it returns
        /// 
        fn flush(&self, buf: Vec<String>, file_name: String) -> Result<(), Error> {
            
            // Opens the file or returns Error.
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_name)?;

            // Writes the buffer contents to the
            // file or returns Error.
            for i in 0..buf.len() {
                file.write(&[buf[i].as_bytes(), "\n".as_bytes()].concat())?;
            }

            Ok(())
        }
    }
}

///
/// Tests module
/// 
mod tests {
    use super::*;

    use std::fs::{OpenOptions, self};
    use std::io::Read;
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
        out::OutThread::init(receiver, "out.txt".to_string(), sig);
        for _ in 0..33 {
            sender.send("hello".to_string()).unwrap();
        }
        }
        if let Err(_) = join.recv() {
            panic!("Could not get exit signal");
        }

        let mut file = match OpenOptions::new()
            .read(true)
            .open("out.txt") {
                Ok(f) => f,
                Err(_) => panic!("Could not open file"),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut expected = String::new();
        for _ in 0..33 {
            expected += "hello\n";
        }
        assert_eq!(expected, contents);

        fs::remove_file("out.txt").unwrap();
    }
}