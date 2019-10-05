use std::any::Any;
use std::borrow::ToOwned;
use std::collections::HashMap;

pub struct Safe {
    safe: HashMap<String, Box<dyn Any + Send + Sync>>
}

impl Safe {
    pub fn new() -> Self {
        Safe {
            safe: HashMap::new()
        }
    }

    pub fn store<T>(&mut self, s: &str, t: T) where T: Any + Send + Sync {
        self.safe.insert(s.to_owned(), Box::new(t));
    }

    pub fn get<T>(&self, s: &str) -> Option<&Box<T>> where T: Any + Send + Sync {
        let val = self.safe.get(s);
        match val {
            None => None,
            Some(o) => if o.is::<T>() {
                unsafe {
                    Some(std::mem::transmute::<&Box<dyn Any + Send + Sync>, &Box<T>>(o))
                }
            } else {
                panic!("Error while retrieving storage of {}. Invalid type", s)
            }
        }
    }

    pub fn get_mut<T>(&mut self, s: &str) -> Option<&mut Box<T>> where T: Any + Send + Sync {
        let val = self.safe.get_mut(s);
        match val {
            None => None,
            Some(o) => if o.is::<T>() {
                unsafe {
                    Some(std::mem::transmute::<&mut Box<dyn Any + Send + Sync>, &mut Box<T>>(o))
                }
            } else {
                panic!("Error while retrieving storage of {}. Invalid type", s)
            }
        }
    }

    pub fn exists(&self, s: &str) -> bool {
        self.safe.contains_key(s)
    }
}