#[derive(Debug)]
pub(crate) struct MalwareInfo {
    pub(crate) desc: String,
}

#[derive(Debug)]
pub(crate) enum FileInfo {
    Unknown,
    Clean(u32),
    Malware(MalwareInfo),
}
