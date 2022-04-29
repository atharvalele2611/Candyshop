use serde::{Deserialize, Serialize};

fn str_split_first(s: &str) -> Option<(char, &str)> {
    s.chars().next().map(|c| (c, &s[c.len_utf8()..]))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrieMap<V> {
    len: usize,
    val: Option<V>,
    children: Vec<(char, TrieMap<V>)>,
}

impl<V> TrieMap<V> {
    pub fn new() -> Self {
        TrieMap {
            len: 0,
            val: None,
            children: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn next(&self, c: char) -> Option<&Self> {
        match self.children.binary_search_by_key(&c, |&(c, _)| c) {
            Ok(i) => Some(&self.children[i].1),
            Err(_) => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        match str_split_first(key) {
            None => self.val.as_ref(),
            Some((c, rk)) => match self.children.binary_search_by_key(&c, |&(c, _)| c) {
                Ok(i) => self.children[i].1.get(rk),
                Err(_) => None,
            },
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn next_mut(&mut self, c: char) -> Option<&mut Self> {
        match self.children.binary_search_by_key(&c, |&(c, _)| c) {
            Ok(i) => Some(&mut self.children[i].1),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match str_split_first(key) {
            None => self.val.as_mut(),
            Some((c, rk)) => match self.children.binary_search_by_key(&c, |&(c, _)| c) {
                Ok(i) => self.children[i].1.get_mut(rk),
                Err(_) => None,
            },
        }
    }

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        let ov = match str_split_first(key) {
            None => std::mem::replace(&mut self.val, Some(value)),
            Some((c, rk)) => {
                let i = match self.children.binary_search_by_key(&c, |&(c, _)| c) {
                    Ok(i) => i,
                    Err(i) => {
                        self.children.insert(i, (c, TrieMap::new()));
                        i
                    }
                };
                self.children[i].1.insert(rk, value)
            }
        };
        if ov.is_none() {
            self.len += 1
        }
        ov
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        let ov = match str_split_first(key) {
            None => std::mem::replace(&mut self.val, None),
            Some((c, rk)) => match self.children.binary_search_by_key(&c, |&(c, _)| c) {
                Ok(i) => {
                    let tm = &mut self.children[i].1;
                    let ov = tm.remove(rk);
                    if tm.is_empty() {
                        self.children.remove(i);
                    }
                    ov
                }
                Err(_) => None,
            },
        };
        if ov.is_some() {
            self.len -= 1;
        };
        ov
    }
}

pub struct IntoIter<V> {
    len: usize,
    stk: Vec<(String, TrieMap<V>)>,
}

impl<V> IntoIterator for TrieMap<V> {
    type Item = (String, V);
    type IntoIter = IntoIter<V>;
    fn into_iter(self: TrieMap<V>) -> Self::IntoIter {
        let len = self.len;
        let stk = vec![(String::new(), self)];
        IntoIter { len, stk }
    }
}
impl<V> Iterator for IntoIter<V> {
    type Item = (String, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stk.pop() {
                None => return None,
                Some((s, tm)) => {
                    self.stk
                        .extend(tm.children.into_iter().rev().map(|(c, tm)| {
                            let mut s = s.clone();
                            s.push(c);
                            (s, tm)
                        }));
                    match tm.val {
                        None => (),
                        Some(v) => {
                            self.len -= 1;
                            return Some((s, v));
                        }
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IterMut<'a, V> {
    len: usize,
    stk: Vec<(String, &'a mut TrieMap<V>)>,
}

impl<V> TrieMap<V> {
    pub fn iter_mut(&mut self) -> IterMut<V> {
        let len = self.len;
        let stk = vec![(String::new(), self)];
        IterMut { len, stk }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (String, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stk.pop() {
                None => return None,
                Some((s, tm)) => {
                    self.stk.extend(tm.children.iter_mut().rev().map(|(c, tm)| {
                        let mut s = s.clone();
                        s.push(*c);
                        (s, tm)
                    }));
                    match tm.val.as_mut() {
                        None => (),
                        Some(v) => {
                            self.len -= 1;
                            return Some((s, v));
                        }
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct Iter<'a, V> {
    len: usize,
    stk: Vec<(String, &'a TrieMap<V>)>,
}
impl<V> TrieMap<V> {
    pub fn iter(&self) -> Iter<V> {
        let len = self.len;
        let stk = vec![(String::new(), self)];
        Iter { len, stk }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stk.pop() {
                None => return None,
                Some((s, tm)) => {
                    self.stk.extend(tm.children.iter().rev().map(|(c, tm)| {
                        let mut s = s.clone();
                        s.push(*c);
                        (s, tm)
                    }));
                    match tm.val.as_ref() {
                        None => (),
                        Some(v) => {
                            self.len -= 1;
                            return Some((s, v));
                        }
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
