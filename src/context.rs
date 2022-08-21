use crate::publishers::PublisherRegistry;

pub struct Context {
    pub publishers: PublisherRegistry
}

impl Context {
    pub fn new() -> Self {
        Context {
            publishers: PublisherRegistry::new()
        }
    }
}