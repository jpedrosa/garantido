// aliasopts.rs
// Copyright (c) 2019 Joao Pedrosa

use std::collections::HashMap;
use std::env;


#[derive(Debug, PartialEq)]
pub enum CT {
    CTString,
    Flag,
    Alias
}

#[derive(Debug)]
pub struct AliasOpts {
    patterns: HashMap<String, (CT, String)>,
    values: HashMap<String, String>,
    loose: Vec<String>,
}

impl AliasOpts {

    pub fn new() -> AliasOpts {
        AliasOpts {
            patterns: HashMap::new(),
            values: HashMap::new(),
            loose: Vec::new()
        } 
    }

    pub fn add(& mut self, s: &str, ct: CT) -> & mut AliasOpts {
        self.patterns.insert(s.to_string(), (ct, String::new()));
        self
    }

    pub fn add_alias(&mut self, alias: &str, alias_for: &str) -> 
        &mut AliasOpts {
        if let Some(v) = self.patterns.get(alias_for) {
            if v.0 != CT::Alias {
                self.patterns.insert(alias.to_string(),
                    (CT::Alias, alias_for.to_string()));
            }
        }
        self
    }

    pub fn parse(&mut self, args: Vec<String>) -> &mut AliasOpts {
        let mut i = 0;
        let mut pattern_keys: Vec<String> = Vec::new();
        let mut initialized_keys = false;
        while i < args.len() {
            if let Some(mut v) = self.patterns.get(&args[i]) {
                let mut k = &args[i];
                if v.0 == CT::Alias {
                    k = &v.1;
                    v = self.patterns.get(k).unwrap();
                }
                if v.0 == CT::Flag {
                    self.values.insert(k.to_string(), String::new());
                } else if i + 1 < args.len() {
                    // Check if it's moving from one --command to --another.
                    if !self.patterns.contains_key(&args[i + 1]) {
                        self.values.insert(k.to_string(), 
                            args[i + 1].to_string());
                        i += 1;
                    }
                }
            } else {
                if !initialized_keys {
                    for k in self.patterns.keys() {
                        pattern_keys.push([k, "="].concat().to_string());
                    }
                    initialized_keys = true;
                }
                let s = args[i].to_string();
                if let Some(k) = pattern_keys.iter().find(|&k| 
                    s.starts_with(k)) {
                    let z = &s[k.len()..];
                    if z.len() > 0 {
                        let mut ck = &k[..k.len() - 1];
                        let mut v = self.patterns.get(ck).unwrap();
                        if v.0 == CT::Alias {
                            ck = &v.1;
                            v = self.patterns.get(&v.1).unwrap();
                        }
                        // Taking a --flag would be wrong in the context of
                        // --flag=something.
                        if v.0 != CT::Flag {
                            self.values.insert(ck.to_string(), z.to_string());
                        }
                    }
                } else {
                    self.loose.push(args[i].to_string());
                }
            }
            i += 1;
        }
        self
    }

    pub fn parse_args(&mut self) -> &mut AliasOpts {
        let a: Vec<String> = env::args().collect();
        self.parse(a[1..].to_vec())
    }

    pub fn got(&self, k: &str) -> bool {
        self.values.get(k) != None
    }

    pub fn get(&self, k: &str) -> Option<String> {
        if let Some(v) = self.values.get(k) {
            Some(v.to_string())
        } else {
            None
        }
    }

    pub fn get_loose(&self, i: usize) -> Option<String> {
        if let Some(v) = self.loose.get(i) {
            Some(v.to_string())
        } else {
            None
        }
    }

    pub fn values(&self) -> HashMap<String, String> {
        self.values.clone()
    }

    pub fn loose(&self) -> Vec<String> {
        self.loose.clone()
    }

    pub fn loose_len(&self) -> usize {
        self.loose.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0 && self.loose.len() == 0
    }

}
