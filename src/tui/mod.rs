mod widgets;

use crate::crawler::{Crawler, Stats};
use crate::model::Model;
use crate::Options;

use ratatui::{prelude::*, widgets::*};
use crossterm::{terminal, event::{self, *}, ExecutableCommand};

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io;


pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    shared_stats: Arc<Mutex<Stats>>,
    model: Arc<Mutex<Model>>,
    stats: Stats,
    runtime: Instant,
    options: Options,
    should_close: bool,
}

impl Tui {
    pub fn new(options: Options) -> Result<Tui, Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        io::stdout().execute(terminal::EnterAlternateScreen)?;

        Ok(Tui {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
            shared_stats: Arc::new(Mutex::new(Stats::default())),
            model: Arc::new(Mutex::new(Model::new())),
            stats: Stats::default(),
            runtime: Instant::now(),
            options,
            should_close: false,
        })
    }

    fn handle_keypress(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.should_close = true;
            },
            _ => {},
        }
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        self.handle_keypress(key.code);
                    }
                },
                _ => {},
            }
        }

        Ok(())
    }

    fn syncronize_stats(&mut self) {
        if let Ok(lock) = self.shared_stats.lock() {
            self.stats = lock.clone();
        }
    }

    fn start_crawler(&mut self) {
        let shared_stats = self.shared_stats.clone();
        let options = self.options.clone();
        let model = self.model.clone();

        thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut crawler = Crawler::new(shared_stats, model, options.clone())?;

            crawler.crawl()?;

            Ok(())
        });
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.start_crawler();

        while !self.should_close {
            self.syncronize_stats();
            self.handle_events()?;

            self.terminal.draw(|frame| {
                ui(frame, self.stats, self.runtime);
            })?;
        }

        if let Ok(mut model) = self.model.lock() {
            model.write(&self.options.output)?;
        }

        terminal::disable_raw_mode()?;
        io::stdout().execute(terminal::LeaveAlternateScreen)?;

        Ok(())
    }
}

fn ui(frame: &mut Frame, stats: Stats, runtime: Instant) {
    widgets::control_panel(frame, stats, runtime);
}


