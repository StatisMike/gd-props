/// Type that can be used by structs deriving `RonSaver` and `RonLoader`
/// to store the Uid of resources. Mainly created by macro `#[godot_io_uid_map]`
pub type UidMap = std::sync::Mutex<std::collections::HashMap<String, i64>>;
