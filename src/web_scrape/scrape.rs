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
    pub struct ScrapeJob<'a> {
        out: mpsc::Sender<String>,
        regex: &'a str,
        url: &'a str,
    }

    ///
    /// Implementation block for the webscraping
    /// struct.
    /// 
    impl<'a> ScrapeJob<'a> {

        ///
        /// Returns a new instance of the ScrapeUtil
        /// with the specified url and regex.
        /// 
        pub fn new(out: mpsc::Sender<String>, regex: &'a str, url: &'a str) -> ScrapeJob<'a> {
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
            let regex = Regex::new(self.regex)?;
            let body = reqwest::blocking::get(self.url)?
                .text()?;
            for line in body.split(".") {
                if regex.is_match(&line) {
                    while let Err(_) = self.out.send(line.to_string()) {}
                }
            }

            Ok(())
        }
    }

    pub type JobPipe<'a> = Arc<Mutex<mpsc::Receiver<ScrapeJob<'a>>>>;
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    
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
}