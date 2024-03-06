// #[derive(Debug)]
// pub enum Action {
//     Delete,
//     //todo more actions
// }
//
// #[derive(Debug)]
// pub struct MalwareInfo {
//     pub(crate) desc: String,
//     pub(crate) action: Action,
// }
//
// #[derive(Debug)]
// pub enum FileInfo {
//     Unknown,
//     Clean(u32),
//     Malware(MalwareInfo),
// }

#[derive(Debug)]
pub enum FileInfo {
    Clean,
    Malicious(String),
}
