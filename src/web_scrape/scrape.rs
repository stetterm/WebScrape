pub use scrape::ScrapeJob;

///
/// Module for performing the web scrape
/// operations. Accessed by the threads
/// spawned in lib.
/// 
pub mod scrape {

    use std::sync::{Arc, Mutex, mpsc};
    use regex::Regex;

    ///
    /// Struct used to hold the url to
    /// target and the regex to use.
    /// 
    pub struct ScrapeJob {
        out: mpsc::Sender<String>,
        regex: String,
        url: String,
    }

    ///
    /// Implementation block for the webscraping
    /// struct.
    /// 
    impl ScrapeJob {

        ///
        /// Returns a new instance of the ScrapeUtil
        /// with the specified url and regex.
        /// 
        pub fn new(out: mpsc::Sender<String>, regex: String, url: String) -> ScrapeJob {
            ScrapeJob {
                out,
                regex,
                url,
            }
        }

        ///
        /// Function called by the threads to
        /// scrape the specified url using the
        /// specified regex.
        /// 
        pub fn scrape(&self) -> Result<(), Box<dyn std::error::Error>> {
            let regex = Regex::new(&[r"", &self.regex].concat())?;
            let body = reqwest::blocking::get(&self.url)?
                .text()?;
            for line in body.split(".") {
                if regex.is_match(&line) {
                    while let Err(_) = self.out.send([line.to_string(), String::from([": ", &self.url, "\n"].concat())].concat()) {}
                }
            }

            Ok(())
        }
    }

    pub type JobPipe<'a> = Arc<Mutex<mpsc::Receiver<ScrapeJob>>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;
    
    use std::sync::mpsc;

    #[test]
    fn regex_test() {
        let test = Regex::new(
            r"(\d{3})-(\d{3})-(\d{4})"
        ).unwrap();
        let test_data = "508-298-5308 hello there\n";
        for line in test_data.split("\n") {
            if test.is_match(line) {
                println!("{}", &line);
            }
        }
    }

    #[test]
    fn simple_regex() {
        let test = Regex::new(
            r"hello"
        ).unwrap();
        let text = "hello there sir\ni am here";
        for line in text.split('\n') {
            if test.is_match(&line) {
                println!("{}", &line);
            }
        }
    }

    #[test]
    fn download_google_homepage() {
        let body = reqwest::blocking::get("https://google.com").unwrap()
            .text().unwrap();
        println!("{}", body);
    }

    #[test]
    fn test_scrape() {
        let (test_send, test_recv) = mpsc::channel();
        let test_job = ScrapeJob::new(
        test_send,
            String::from("google"),
            String::from("https://www.google.com"),
        );
        test_job.scrape().unwrap();
        let mut next_scrape = test_recv.recv();
        while let Ok(ref s) = next_scrape {
            println!("{}", s);
            next_scrape = test_recv.recv();
        }
    }
}