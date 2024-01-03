pub mod redr {
    use std::rc::Rc;

    // pub trait BinaryReader {
    //     fn get_io_read(&self) -> Rc<dyn std::io::Read>;
    // }
    //
    // struct IoReader {
    //
    // }

    pub struct FileReader {
        reader: Rc<std::fs::File>,
    }

    impl FileReader {
        pub fn new(rc_file: std::fs::File) -> Self {
            Self { reader: Rc::new(rc_file) }
        }

        pub fn get_io_read(&self) -> Rc<std::fs::File> {
            self.reader.clone()
        }
    }
}
