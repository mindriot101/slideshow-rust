use std::sync::mpsc::{Sender, Receiver, channel};
use std::path::PathBuf;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher, op, RecommendedWatcher};

pub struct Hotloader<'a> {
    pub messages: Receiver<RawEvent>,
    pub path: &'a str,
    watcher: RecommendedWatcher,
}

impl<'a> Hotloader<'a> {
    pub fn watch(path: &'a str) -> Result<Hotloader, super::notify::Error> {
        let (inputs, messages) = channel();
        let mut watcher = raw_watcher(inputs)?;
        watcher.watch(path, RecursiveMode::Recursive)?;

        Ok(Hotloader {
            messages: messages,
            path: path,
            watcher: watcher,
        })
    }

    pub fn has_event(&self) -> Option<PathBuf> {
        let result = match self.messages.try_recv() {
            Ok(RawEvent { path: Some(path), op: Ok(op), .. }) => {
                if op.contains(op::WRITE) {
                    Some(path)
                } else {
                    None
                }
            },
            _ => None,
        };

        result
    }
}
