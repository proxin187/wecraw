use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone)]
pub struct Page {
    pub url: String,
    pub content: String,
}

pub struct Download {
    pages: Arc<Mutex<Vec<Page>>>,
}

impl Download {
    pub fn new() -> Download {
        Download {
            pages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[inline]
    fn clear(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.pages.lock().map_err(|err| err.to_string())?.drain(..);

        Ok(())
    }

    pub fn download(&mut self, queue: &[String]) -> Result<Vec<Page>, Box<dyn std::error::Error + Send + Sync>> {
        self.clear()?;

        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

        for url in queue {
            let pages = self.pages.clone();
            let url = url.clone();

            let handle = thread::spawn(move || {
                if let Ok(response) = reqwest::blocking::get(url.clone()) {
                    if let Ok(mut lock) = pages.lock() {
                        lock.push(Page {
                            url,
                            content: response.text().unwrap_or_default(),
                        });
                    }
                }
            });

            threads.push(handle);
        }

        for handle in threads {
            if let Err(err) = handle.join() {
                println!("[+] failed to join thread: {:?}", err);
            }
        }

        Ok(self.pages.lock().map_err(|err| err.to_string())?.to_vec())
    }
}


