use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Safe {
    safe: HashMap<std::any::TypeId, Box<dyn Any + Send + Sync>>
}

impl Safe {
    pub fn new() -> Self {
        Safe {
            safe: HashMap::new()
        }
    }

    pub fn store<T>(&mut self, t: T) where T: Any + Send + Sync + 'static {
        self.safe.insert(t.type_id(), Box::new(t));
    }

    pub fn get<T>(&self) -> Option<&Box<T>> where T: Any + Send + Sync + 'static {
        let val = self.safe.get(&TypeId::of::<T>());
        match val {
            None => None,
            Some(o) => if o.is::<T>() {
                unsafe {
                    Some(std::mem::transmute::<&Box<dyn Any + Send + Sync>, &Box<T>>(o))
                }
            } else {
                panic!("Error while retrieving storage. Invalid type")
            }
        }
    }
    #[allow(dead_code)]
    pub fn get_mut<T>(&mut self) -> Option<&mut Box<T>> where T: Any + Send + Sync + 'static {
        let val = self.safe.get_mut(&TypeId::of::<T>());
        match val {
            None => None,
            Some(o) => if o.is::<T>() {
                unsafe {
                    Some(std::mem::transmute::<&mut Box<dyn Any + Send + Sync>, &mut Box<T>>(o))
                }
            } else {
                panic!("Error while retrieving storage. Invalid type")
            }
        }
    }

    #[allow(dead_code)]
    pub fn exists<T>(&self) -> bool where T: Any + Send + Sync + 'static {
        self.safe.contains_key(&TypeId::of::<T>())
    }
}