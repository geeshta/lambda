use crate::ast::Term;
use crate::ast::AST;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Sub};

/// Type represents a set of variables
/// Used to keep track of free and bound variables
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
    /// Create a new set form a variable
    fn from(term: Term) -> Self {
        let inner = HashSet::from([term]);
        VarSet { inner }
    }
}

impl FromIterator<Term> for VarSet {
    /// Create a new set from an iterable of variables
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        let inner = HashSet::from_iter(iter);
        VarSet { inner }
    }
}

impl IntoIterator for VarSet {
    type Item = Term;
    type IntoIter = std::collections::hash_set::IntoIter<Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a VarSet {
    type Item = &'a Term;
    type IntoIter = std::collections::hash_set::Iter<'a, Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl VarSet {
    /// Create an empty set
    pub fn new() -> VarSet {
        let inner = HashSet::new();
        VarSet { inner }
    }
    /// Check whether a variable is in the set
    pub fn contains(&self, term: &Term) -> bool {
        self.inner.contains(&term)
    }

    /// Check whether it is an empty set
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn zip<'a>(&'a self, other: &'a VarSet) -> impl Iterator<Item = (&'a Term, &'a Term)> {
        self.inner.iter().zip(other.inner.iter())
    }

    /// Return the union of two sets
    pub fn union(self, other: VarSet) -> VarSet {
        let inner = other
            .inner
            .into_iter()
            .chain(self.inner.into_iter())
            .collect();
        VarSet { inner }
    }

    /// Return the intersection of two sets
    pub fn intersection(self, other: VarSet) -> VarSet {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| other.inner.contains(e))
            .collect();
        VarSet { inner }
    }

    /// Return the difference of two sets
    pub fn difference(self, other: VarSet) -> VarSet {
        let inner = self
            .inner
            .into_iter()
            .filter(|e| !other.inner.contains(e))
            .collect();
        VarSet { inner }
    }

    /// Return the symmetric difference of two sets
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

    /// Return a new set extended with the given variable
    pub fn with(self, term: Term) -> VarSet {
        self.union(VarSet::from(term))
    }

    /// Return a new set with the variable removed
    pub fn without(self, term: Term) -> VarSet {
        self.difference(VarSet::from(term))
    }

    /// Generate a set of the same length with all varaibles unique
    pub fn fresh_set(&self) -> VarSet {
        let mut unique_set = VarSet::new();
        for _ in self.into_iter() {
            let fresh_term = AST::fresh(self.clone() | unique_set.clone()).term;
            unique_set = unique_set.with(fresh_term.clone());
        }
        unique_set
    }
}

impl BitOr for VarSet {
    type Output = Self;

    /// Map the union operation to |
    fn bitor(self, other: Self) -> Self::Output {
        self.union(other)
    }
}

impl BitAnd for VarSet {
    type Output = Self;

    /// Map the intersection operation to &
    fn bitand(self, other: Self) -> Self::Output {
        self.intersection(other)
    }
}

impl Div for VarSet {
    type Output = Self;

    /// Map the difference operion to /
    fn div(self, other: Self) -> Self::Output {
        self.difference(other)
    }
}

impl BitXor for VarSet {
    type Output = Self;

    /// Map the symmetric difference operion to ^
    fn bitxor(self, other: Self) -> Self::Output {
        self.symmetric_difference(other)
    }
}

impl Add<Term> for VarSet {
    type Output = Self;

    /// Map the "with" operation to +
    fn add(self, term: Term) -> Self::Output {
        self.with(term)
    }
}

impl Sub<Term> for VarSet {
    type Output = Self;

    /// Map the "without" operation to -
    fn sub(self, term: Term) -> Self::Output {
        self.without(term)
    }
}

impl fmt::Debug for VarSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
