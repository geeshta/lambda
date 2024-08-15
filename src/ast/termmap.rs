use crate::ast::AST;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::BitOr;

/// Type represents the mapping of all previsously evaluated terms
#[derive(Clone)]
pub struct TermMap {
    inner: BTreeMap<AST, AST>,
}

impl From<(AST, AST)> for TermMap {
    fn from(entry: (AST, AST)) -> Self {
        let inner = match entry {
            (key, value) => BTreeMap::from([(key, value)]),
        };
        TermMap { inner }
    }
}

impl FromIterator<(AST, AST)> for TermMap {
    fn from_iter<T: IntoIterator<Item = (AST, AST)>>(iter: T) -> Self {
        let inner = iter.into_iter().collect();
        TermMap { inner }
    }
}

impl IntoIterator for TermMap {
    type Item = (AST, AST);
    type IntoIter = std::collections::btree_map::IntoIter<AST, AST>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a TermMap {
    type Item = (&'a AST, &'a AST);
    type IntoIter = std::collections::btree_map::Iter<'a, AST, AST>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl TermMap {
    pub fn new() -> Self {
        TermMap {
            inner: BTreeMap::new(),
        }
    }

    pub fn contains(&self, ast: &AST) -> bool {
        self.inner.contains_key(&ast)
    }

    /// Insert a pair of terms and return a new copy
    pub fn with(self, key: AST, value: AST) -> Self {
        let mut inner = self.inner;
        inner.insert(key, value);
        TermMap { inner }
    }

    /// Extend with other memo and return a new copy
    pub fn extended_with(self, other: TermMap) -> Self {
        let mut inner = self.inner;
        inner.extend(other);
        TermMap { inner }
    }

    /// If a term was previously evaluated, return it. Otherwise return back the parameter
    pub fn get(&self, key: AST) -> AST {
        match self.inner.get(&key).cloned() {
            Some(ast) => {
                // println!("Found memoized value for {:?}", key);
                ast
            }
            None => key,
        }
    }
}

impl BitOr for TermMap {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.extended_with(other)
    }
}

impl fmt::Debug for TermMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner.keys())
    }
}
