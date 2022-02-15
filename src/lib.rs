///
/// Main module used for generating
/// the web scrape threads.
/// 
pub mod web_scrape {

    pub mod out;
    pub mod scrape;

    use crate::web_scrape::out::OutThread;
    use crate::web_scrape::scrape::ScrapeJob;

    use std::sync::{Arc, mpsc, Mutex};
    use std::thread::{self, JoinHandle};

    ///
    /// Main struct used for synchronizing
    /// the threads and controlling the
    /// flow of url and regex.
    /// file_name: name of output file.
    /// regex: regex to use for scraping.
    /// thread_count: number of threads to use.
    /// urls: list of urls to scrape.
    /// 
    pub struct WebScrape {
        file_name: String,
        regex: String,
        thread_count: usize,
        urls: Vec<String>,
    }

    ///
    /// Implementation block for the
    /// main WebScrape struct.
    /// 
    impl WebScrape {

        ///
        /// Returns a new instance of the WebScrape
        /// struct with the specified number of threads.
        /// thread_count: number of threads to use.
        /// 
        pub fn new(thread_count: usize) -> WebScrape {
            match thread_count {
                0 => panic!("Cannot use 0 threads"),
                _ => WebScrape {
                    file_name: String::from(""),
                    regex: String::from(""),
                    thread_count,
                    urls: vec![],
                }
            }
        }

        ///
        /// Adds a url to the list
        /// and returns a new Webscrape
        /// struct.
        /// url: new url to add.
        /// 
        pub fn add_url(&self, url: &str) -> WebScrape {
            WebScrape {
                file_name: String::from(&self.file_name),
                regex: String::from(&self.regex),
                thread_count: self.thread_count,
                urls: {
                    let mut new_url = vec![];
                    for old_url in self.urls.iter() {
                        new_url.push(String::from(old_url));
                    }
                    new_url.push(url.to_string());
                    new_url
                }
            }
        }

        ///
        /// Returns a new instance of the struct
        /// with the specified file name set.
        /// file_name: new file name.
        /// 
        pub fn set_file(&self, file_name: &str) -> WebScrape {
            WebScrape {
                file_name: String::from(file_name),
                regex: String::from(&self.regex),
                thread_count: self.thread_count,
                urls: self.urls.clone(),
            }
        }

        ///
        /// Returns a new instance of the struct
        /// with the specified regex string.
        /// regex: new regex.
        /// 
        pub fn set_regex(&self, regex: &str) -> WebScrape {
            WebScrape {
                file_name: String::from(&self.file_name),
                regex: String::from(regex),
                thread_count: self.thread_count,
                urls: self.urls.clone(),
            }
        }

        ///
        /// Executes the web scrape operation
        /// with the stored fields for file name,
        /// regex, urls, and thread count.
        /// 
        pub fn execute(&self) {
            let (exit_send, exit_recv) = mpsc::channel();
            let (job_send, job_recv) = mpsc::channel();
            let mut thread_handles = vec![];
            {
                let job_recv = Arc::new(Mutex::new(job_recv));
                let (io_send, io_recv) = mpsc::channel();
                OutThread::init(io_recv, String::from(&self.file_name), exit_send);
                for i in 0..self.thread_count {
                    let thread_receiver = Arc::clone(&job_recv);
                    thread_handles.push(thread::spawn(move || loop {
                        {
                            let thread_receiver = match thread_receiver.lock() {
                                Ok(t) => t,
                                Err(_) => continue,
                            };
                            let new_job: ScrapeJob = match thread_receiver.recv() {
                                Ok(j) => j,
                                Err(_) => break,
                            };
                            if let Err(_) = new_job.scrape() {
                                continue;
                            }
                        }
                    }));
                }
            }
        }
    }
}