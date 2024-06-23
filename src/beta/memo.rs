use crate::ast::AST;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::BitOr;

/// Type represents the mapping of all previsously evaluated terms
#[derive(Clone)]
pub struct Memo {
    inner: BTreeMap<AST, AST>,
}

impl From<(AST, AST)> for Memo {
    fn from(entry: (AST, AST)) -> Self {
        let inner = match entry {
            (key, value) => BTreeMap::from([(key, value)]),
        };
        Memo { inner }
    }
}

impl FromIterator<(AST, AST)> for Memo {
    fn from_iter<T: IntoIterator<Item = (AST, AST)>>(iter: T) -> Self {
        let inner = iter.into_iter().collect();
        Memo { inner }
    }
}

impl IntoIterator for Memo {
    type Item = (AST, AST);
    type IntoIter = std::collections::btree_map::IntoIter<AST, AST>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Memo {
    type Item = (&'a AST, &'a AST);
    type IntoIter = std::collections::btree_map::Iter<'a, AST, AST>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Memo {
    pub fn new() -> Self {
        Memo {
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
        Memo { inner }
    }

    /// Extend with other memo and return a new copy
    pub fn extended_with(self, other: Memo) -> Self {
        let mut inner = self.inner;
        inner.extend(other);
        Memo { inner }
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

impl BitOr for Memo {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.extended_with(other)
    }
}

impl fmt::Debug for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner.keys())
    }
}
