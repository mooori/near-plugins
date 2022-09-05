pub trait AccessControllable {
    fn storage_prefix(&self) -> &[u8];
}
