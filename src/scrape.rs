///
/// Module for performing the web scrape
/// operations. Accessed by the threads
/// spawned in lib.
/// 
pub mod scrape {

    use std::sync::mpsc;
    use regex::Regex;

    ///
    /// Struct used to hold the url to
    /// target and the regex to use.
    /// 
    pub struct ScrapeUtil<'a> {
        out: mpsc::Sender<String>,
        regex: Regex,
        url: &'a str,
    }

    ///
    /// Implementation block for the webscraping
    /// struct.
    /// 
    impl<'a> ScrapeUtil<'a> {

        ///
        /// Returns a new instance of the ScrapeUtil
        /// with the specified url and regex.
        /// 
        pub fn new(out: mpsc::Sender<String>, regex: Regex, url: &'a str) -> ScrapeUtil<'a> {
            ScrapeUtil {
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
            let body = reqwest::blocking::get(self.url)?
                .text()?;
            for line in body.split(".") {
                if self.regex.is_match(&line) {
                    while let Err(_) = self.out.send(line.to_string()) {}
                }
            }

            Ok(())
        }
    }
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