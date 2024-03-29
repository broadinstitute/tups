use std::collections::BTreeMap;
use crate::error::Error;

pub(crate) struct Env {
    pub(crate) args: BTreeMap<String, Vec<String>>,
}

fn error_missing_arg(key: &str) -> Error {
    Error::from(format!("Missing argument {}.", key))
}

impl Env {
    fn ensure_args_key(args: &mut BTreeMap<String, Vec<String>>, key: &str) {
        if !args.contains_key(key) {
            args.insert(String::from(key), Vec::new());
        }
    }
    pub(crate) fn new() -> Env {
        let mut args: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut key = String::new();
        for arg in std::env::args() {
            if let Some(key_new) = arg.strip_prefix("--") {
                key = String::from(key_new);
                Env::ensure_args_key(&mut args, &key);
            } else if let Some(key_new) = arg.strip_prefix('-') {
                key = String::from(key_new);
                Env::ensure_args_key(&mut args, &key);
            } else {
                Env::ensure_args_key(&mut args, &key);
                if let Some(key_args) = args.get_mut(&key) {
                    key_args.push(arg)
                }
            }
        }
        Env { args }
    }
    pub(crate) fn get_arg(&self, key: &str) -> Result<&String, Error> {
        match self.args.get(key) {
            None => { Err(error_missing_arg(key)) }
            Some(values) => {
                if values.len() > 1 {
                    Err(Error::from(format!(
                        "Argument {} should have exactly one value, but has {}.", key,
                        values.len())
                    ))
                } else {
                    values.first().ok_or_else(|| {
                        error_missing_arg(key)
                    })
                }
            }
        }
    }
    pub(crate) fn get_opt_arg(&self, key: &str) -> Result<Option<&String>, Error> {
        match self.args.get(key) {
            None => { Ok(None) }
            Some(values) => {
                if values.len() > 1 {
                    Err(Error::from(format!(
                        "Argument {} should have exactly one value, but has {}.", key,
                        values.len())
                    ))
                } else {
                    values.first().ok_or_else(|| {
                        error_missing_arg(key)
                    }).map(Some)
                }
            }
        }
    }
    pub(crate) fn get_args(&self, key: &str) -> Result<&Vec<String>, Error> {
        match self.args.get(key) {
            None => { Err(error_missing_arg(key)) }
            Some(values) => { Ok(values) }
        }
    }
}