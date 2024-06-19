use crate::ast::ast::Term;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Sub};

#[derive(Eq, PartialEq, Clone)]
pub struct FreeVars {
    inner: HashSet<Term>,
}

impl Hash for FreeVars {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for expr in &self.inner {
            expr.hash(state);
        }
    }
}

impl From<Term> for FreeVars {
    fn from(term: Term) -> Self {
        let inner = HashSet::from([term]);
        FreeVars { inner }
    }
}

impl FromIterator<Term> for FreeVars {
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        let inner = HashSet::from_iter(iter);
        FreeVars { inner }
    }
}

impl FreeVars {
    pub fn contains(&self, term: &Term) -> bool {
        self.inner.contains(&term)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn union(self, other: FreeVars) -> FreeVars {
        let inner = other
            .inner
            .into_iter()
            .chain(self.inner.into_iter())
            .collect();
        FreeVars { inner }
    }

    pub fn intersection(self, other: FreeVars) -> FreeVars {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| other.inner.contains(e))
            .collect();
        FreeVars { inner }
    }

    pub fn difference(self, other: FreeVars) -> FreeVars {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| !other.inner.contains(e))
            .collect();
        FreeVars { inner }
    }

    pub fn symmetric_difference(self, other: FreeVars) -> FreeVars {
        let self_inner = self.inner.clone();
        let other_inner = other.inner.clone();

        let inner = self
            .inner
            .into_iter()
            .filter(|e| !other_inner.contains(e))
            .chain(other.inner.into_iter().filter(|e| !self_inner.contains(e)))
            .collect();
        FreeVars { inner }
    }

    pub fn with(self, term: Term) -> FreeVars {
        self.union(FreeVars::from(term))
    }

    pub fn without(self, term: Term) -> FreeVars {
        self.difference(FreeVars::from(term))
    }
}

impl BitOr for FreeVars {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.union(other)
    }
}

impl BitAnd for FreeVars {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        self.intersection(other)
    }
}

impl Div for FreeVars {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.difference(other)
    }
}

impl BitXor for FreeVars {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl Add<Term> for FreeVars {
    type Output = Self;

    fn add(self, term: Term) -> Self::Output {
        self.with(term)
    }
}

impl Sub<Term> for FreeVars {
    type Output = Self;

    fn sub(self, term: Term) -> Self::Output {
        self.without(term)
    }
}

impl fmt::Debug for FreeVars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
