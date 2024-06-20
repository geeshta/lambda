use crate::ast::term::Term;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Sub};

#[derive(Eq, PartialEq, Clone)]
pub struct VarSet {
    inner: HashSet<Term>,
}

impl Hash for VarSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for expr in &self.inner {
            expr.hash(state);
        }
    }
}

impl From<Term> for VarSet {
    fn from(term: Term) -> Self {
        let inner = HashSet::from([term]);
        VarSet { inner }
    }
}

impl FromIterator<Term> for VarSet {
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        let inner = HashSet::from_iter(iter);
        VarSet { inner }
    }
}

impl VarSet {
    pub fn new() -> VarSet {
        let inner = HashSet::new();
        VarSet { inner }
    }
    pub fn contains(&self, term: &Term) -> bool {
        self.inner.contains(&term)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn union(self, other: VarSet) -> VarSet {
        let inner = other
            .inner
            .into_iter()
            .chain(self.inner.into_iter())
            .collect();
        VarSet { inner }
    }

    pub fn intersection(self, other: VarSet) -> VarSet {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| other.inner.contains(e))
            .collect();
        VarSet { inner }
    }

    pub fn difference(self, other: VarSet) -> VarSet {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| !other.inner.contains(e))
            .collect();
        VarSet { inner }
    }

    pub fn symmetric_difference(self, other: VarSet) -> VarSet {
        let self_inner = self.inner.clone();
        let other_inner = other.inner.clone();

        let inner = self
            .inner
            .into_iter()
            .filter(|e| !other_inner.contains(e))
            .chain(other.inner.into_iter().filter(|e| !self_inner.contains(e)))
            .collect();
        VarSet { inner }
    }

    pub fn with(self, term: Term) -> VarSet {
        self.union(VarSet::from(term))
    }

    pub fn without(self, term: Term) -> VarSet {
        self.difference(VarSet::from(term))
    }
}

impl BitOr for VarSet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.union(other)
    }
}

impl BitAnd for VarSet {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        self.intersection(other)
    }
}

impl Div for VarSet {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.difference(other)
    }
}

impl BitXor for VarSet {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl Add<Term> for VarSet {
    type Output = Self;

    fn add(self, term: Term) -> Self::Output {
        self.with(term)
    }
}

impl Sub<Term> for VarSet {
    type Output = Self;

    fn sub(self, term: Term) -> Self::Output {
        self.without(term)
    }
}

impl fmt::Debug for VarSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
