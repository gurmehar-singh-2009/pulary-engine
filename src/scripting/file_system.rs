pub struct Entry<'a> {
    name: &'a str,

    is_file: bool,
    is_dir: bool,

    children: Vec<Self>,
    parent: Box<Option<Self>>,
}

pub struct FileSystem<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> Self {
        let default_files = ["main.rune"]
            .iter()
            .map(|x| Entry {
                children: vec![],
                parent: Box::new(None),
                is_dir: false,
                is_file: true,
                name: x,
            })
            .collect();

        Self {
            entries: default_files,
        }
    }
}
