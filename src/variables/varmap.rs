use crate::ast::Term;
use crate::variables::VarSet;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

/// Type represents a bidirectional mapping of variable names
/// Used for renaming in order to create an alpha variant of an expression
#[derive(Eq, PartialEq, Clone)]
pub struct VarMap {
    inner: HashMap<Term, Term>,
}

impl Hash for VarMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for expr in &self.inner {
            expr.hash(state);
        }
    }
}

impl From<(Term, Term)> for VarMap {
    /// Create a new map from a single variable pair
    /// Insert both commutative pairs
    fn from(entry: (Term, Term)) -> Self {
        let inner = match entry {
            (term1, term2) => HashMap::from([(term1.clone(), term2.clone()), (term2, term1)]),
        };
        VarMap { inner }
    }
}

impl FromIterator<(Term, Term)> for VarMap {
    /// Create a new map form an iterable of variable pairs
    /// Insert both commutative pairs for each one
    fn from_iter<T: IntoIterator<Item = (Term, Term)>>(iter: T) -> Self {
        let inner = iter
            .into_iter()
            .flat_map(|(term1, term2)| vec![(term1.clone(), term2.clone()), (term2, term1)])
            .collect();
        VarMap { inner }
    }
}

impl VarMap {
    /// Create a new map from two sets, zipping them together and adding both pairs
    pub fn from_sets(first: VarSet, second: VarSet) -> VarMap {
        let iter = first.zip(&second).map(|(t1, t2)| (t1.clone(), t2.clone()));
        let result = VarMap::from_iter(iter);
        result
    }
    /// If a variable is mapped, return its mapping. Otherwise, return it back
    pub fn get(&self, term: Term) -> Term {
        self.inner.get(&term).cloned().unwrap_or(term)
    }
}

impl fmt::Debug for VarMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
