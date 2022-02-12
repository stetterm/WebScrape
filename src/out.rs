///
/// Module for a thread that loads
/// the data into the output file
/// every 10 blocks that it receives
/// from the mpsc channel.
/// 
pub mod out {

    use std::sync::mpsc;

    ///
    /// Struct to represent the
    /// thread that loads the scraped
    /// data into the file.
    /// 
    /// pipe: receiving end of the mpcs channel
    /// file_name: name of the output file
    /// 
    pub struct OutThread {
        pipe: mpsc::Receiver<String>,
        file_name: &'static str,
    }

    
}