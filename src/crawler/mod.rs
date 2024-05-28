mod download;
mod doc;

use crate::model::*;
use crate::Options;

use scraper::{Html, Selector};
use download::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone, Copy)]
pub enum Status {
    Downloading,
    Indexing,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Status::Downloading => f.write_str("downloading")?,
            Status::Indexing => f.write_str("indexing")?,
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub chunk_count: usize,
    pub chunk_index: usize,
    pub visited: usize,
    pub indexed: usize,
    pub depth: usize,
    pub status: Status,
}

impl Default for Stats {
    fn default() -> Stats {
        Stats {
            chunk_count: 0,
            chunk_index: 0,
            visited: 0,
            indexed: 0,
            depth: 0,
            status: Status::Downloading,
        }
    }
}

pub struct Crawler {
    pub visited: Arc<Mutex<HashMap<String, ()>>>,
    pub queue: Vec<String>,
    pub stats: Arc<Mutex<Stats>>,
    pub model: Arc<Mutex<Model>>,
    downloader: Download,
    options: Options,
}

impl Crawler {
    pub fn new(
        stats: Arc<Mutex<Stats>>,
        model: Arc<Mutex<Model>>,
        options: Options,
    ) -> Result<Crawler, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Crawler {
            visited: Arc::new(Mutex::new(HashMap::new())),
            queue: Vec::new(),
            stats,
            model,
            downloader: Download::new(),
            options,
        })
    }

    fn queue_pages_threaded(&mut self, pages: Vec<Page>, queue: Arc<Mutex<Vec<String>>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut threads = Vec::new();

        // TODO: ascending crawler

        for page in pages {
            if (self.options.limit && page.url.starts_with(&self.options.seed)) || !self.options.limit {
                let queue = queue.clone();
                let visited = self.visited.clone();
                let model = self.model.clone();

                let handle = thread::spawn(|| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                    doc::queue_page(
                        page,
                        queue,
                        visited,
                        model,
                    )?;

                    Ok(())
                });

                threads.push(handle);
            }
        }

        for handle in threads {
            if let Err(err) = handle.join() {
                println!("[+] failed to join thread: {:?}", err);
            }
        }

        Ok(())
    }

    fn map_stats<F>(&mut self, map: F) where F: Fn(&mut Stats) {
        if let Ok(mut stats) = self.stats.lock() {
            map(&mut stats);
        }
    }

    fn update_stats(&mut self) {
        let visited = self.visited.lock().map(|x| x.len()).unwrap_or_default();
        let indexed = self.model.lock().map(|model| model.docs.len()).unwrap_or_default();

        self.map_stats(|stats| {
            stats.visited = visited;
            stats.indexed = indexed;
        });
    }

    pub fn crawl(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.queue = vec![self.options.seed.clone()];

        while !self.queue.is_empty() {
            let next_queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

            let clone = self.queue.clone();
            let chunks = clone.chunks(self.options.threads);

            self.map_stats(|stats| { stats.chunk_count = chunks.len(); });

            for (index, chunk) in chunks.enumerate() {
                self.map_stats(|stats| {
                    stats.chunk_index = index + 1;
                    stats.status = Status::Downloading;
                });

                let pages = self.downloader.download(&chunk)?;

                self.map_stats(|stats| stats.status = Status::Indexing);

                self.queue_pages_threaded(pages, next_queue.clone())?;

                self.update_stats();
            }

            self.map_stats(|stats| stats.depth += 1);

            if let Ok(next_queue) = next_queue.lock() {
                self.queue = next_queue.clone();
            };
        }

        Ok(())
    }
}


