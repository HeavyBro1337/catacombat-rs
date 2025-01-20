pub trait Item: Send + Sync {
    fn use_item(&mut self) -> bool;
}
