///
/// Module for performing the web scrape
/// operations. Accessed by the threads
/// spawned in lib.
/// 
pub mod scrape {

    ///
    /// Struct used to hold the url to
    /// target and the regex to use.
    /// 
    pub struct ScrapeUtil<'a> {
        url: &'a str,
        regex: &'static str,
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
        pub fn new(url: &'a str, regex: &'static str) -> ScrapeUtil<'a> {
            ScrapeUtil {
                url,
                regex,
            }
        }

        ///
        /// Function called by the threads to
        /// scrape the specified url using the
        /// specified regex.
        /// 
        pub fn scrape(&self) {}
    }
}