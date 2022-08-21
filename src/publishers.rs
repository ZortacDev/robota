use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::{Result};

pub struct PublisherRegistry {
    publishers: HashMap<String, Mutex<Option<Arc<dyn Any + Send + Sync + 'static>>>>
}

impl PublisherRegistry {
    pub fn new() -> Self {
        PublisherRegistry {
            publishers: HashMap::new()
        }
    }

    pub fn register(&mut self, name: &str) -> Result<()> {
        if !self.publishers.contains_key(name) {
            self.publishers.insert(name.to_string(), Mutex::new(None));
            Ok(())
        } else {
            Err(()) // FIXME: Make this a proper error
        }
    }

    pub fn publish<T: Any + Send + Sync + 'static>(&self, name: &str, data: Arc<T>) -> Result<()> {
        let mut topic = self.publishers.get(name)
            .ok_or(())? // FIXME: Make this a proper error
            .lock().unwrap();

        *topic = Some(data);

        Ok(())
    }

    pub fn get<T: Any + Send + Sync + 'static>(&self, name: &str) -> Result<Arc<T>> {
        let guard = self.publishers.get(name)
            .ok_or(())? // FIXME: Make this a proper error (invalid name)
            .lock().map_err(|_| ())?;

        let val = (*guard)
            .as_ref()
            .ok_or(())?
            .clone()
            .downcast::<T>().map_err(|_| ())?;

        Ok(val)
    }
}