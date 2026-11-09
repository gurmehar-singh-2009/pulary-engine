pub struct Entry<'a> {
    children: Box<Self>,

    is_dir: bool,
    is_file: bool,

    name: &'a str,
}
