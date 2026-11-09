use std::sync::{Arc, atomic::AtomicBool};

pub struct Entry {
    pub name: String,

    pub is_file: bool,
    pub is_dir: bool,

    pub children: Vec<Self>,

    pub expanded: Arc<AtomicBool>,
}

pub struct FileSystem {
    pub entries: Vec<Entry>,
}

impl FileSystem {
    pub fn new() -> Self {
        let default_files = ["main.rune", "test.rune"]
            .iter()
            .map(|x| Entry {
                children: vec![],
                is_dir: false,
                is_file: true,
                name: x.to_string(),
                expanded: Arc::new(AtomicBool::new(false)),
            })
            .chain([Entry {
                children: vec![
                    Entry {
                        children: vec![],
                        is_dir: false,
                        is_file: true,
                        name: "jajaja".to_string(),
                        expanded: Arc::new(AtomicBool::new(false)),
                    },
                    Entry {
                        children: vec![],
                        is_dir: false,
                        is_file: true,
                        name: "OKOKOK".to_string(),
                        expanded: Arc::new(AtomicBool::new(false)),
                    },
                    Entry {
                        children: vec![],
                        is_dir: false,
                        is_file: true,
                        name: "OKOKOK_2".to_string(),
                        expanded: Arc::new(AtomicBool::new(false)),
                    },
                ],
                is_dir: true,
                is_file: false,
                name: "test".to_string(),
                expanded: Arc::new(AtomicBool::new(true)),
            }])
            .collect();

        Self {
            entries: default_files,
        }
    }
}
