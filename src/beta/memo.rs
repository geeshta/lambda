use crate::ast::ast::AST;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::BitOr;
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

impl BitOr for Memo {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.extended_with(other)
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

    pub fn init(self, ast: AST) -> Self {
        self.with(ast.clone(), ast)
    }

    pub fn with(self, key: AST, value: AST) -> Self {
        let mut inner = self.inner;
        inner.insert(key, value);
        Memo { inner }
    }

    pub fn extended_with(self, other: Memo) -> Self {
        let mut inner = self.inner;
        inner.extend(other);
        Memo { inner }
    }

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

impl fmt::Debug for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner.keys())
    }
}
