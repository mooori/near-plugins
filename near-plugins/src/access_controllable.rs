pub trait AccessControllable {
    fn acl_storage_prefix(&self) -> &[u8];
}
