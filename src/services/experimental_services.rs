use curl::easy::Easy;
use curl::easy::Transfer;

pub struct ExperimentalServices{}

impl ExperimentalServices
{
    fn check_service(service: &str) -> String {
        let mut easy = Easy::new();
        easy.url(service).unwrap();
    
        let mut answer:String = String::from("");
        let mut transfer: Transfer = easy.transfer();
        
        transfer.write_function(|data| {
            answer = String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap();
        
        transfer.perform().unwrap();
        drop(transfer);
    
        answer
    }
}