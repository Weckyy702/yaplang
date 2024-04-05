use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, Mutex, MutexGuard},
};

use lazy_static::lazy_static;

type StringStorage = HashMap<String, Arc<str>>;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InternedString(Arc<str>);

impl InternedString {
    fn from_str(s: &str) -> Self {
        let mut strings = Self::get_strings();

        if let Some(s) = strings.get(s) {
            return Self(s.clone());
        }

        let k = s.into();
        let v: Arc<str> = s.into();

        strings.insert(k, v.clone());

        Self(v)
    }

    fn from_string(s: String) -> Self {
        let mut strings = Self::get_strings();

        if let Some(s) = strings.get(&s) {
            return Self(s.clone());
        }

        let k = s.clone();
        let v: Arc<str> = s.into();

        strings.insert(k, v.clone());

        Self(v)
    }

    fn get_strings() -> MutexGuard<'static, StringStorage> {
        lazy_static! {
            static ref STRINGS: Mutex<StringStorage> = Mutex::new(HashMap::new());
        };

        STRINGS.try_lock().expect("Can acquire lock")
    }
}

impl From<&str> for InternedString {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for InternedString {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl Debug for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
