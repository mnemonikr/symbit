use std::rc::Rc;

const ZERO: SymbolicBit = SymbolicBit(Constraint::Literal(false));
const ONE: SymbolicBit = SymbolicBit(Constraint::Literal(true));

/// A binary bit with constraints on its underlying value. The bit is considered to be concrete if
/// the underlying value is either 1 (true) or 0 (false). Otherwise the bit is considered symbolic.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SymbolicBit(Constraint);

/// A value that can be used to represent a variable bit, possibly with constraints on its value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// A literal `true` or `false` value.
    Literal(bool),

    /// A variable value. The parameter is the identifier for this variable. Two variables with the
    /// same identifier are equivalent.
    Variable(usize),

    /// The negation of a symbolic bit. The `!` operator should be preferred to this, as it has the
    /// opportunity to perform simplications where a direct construction does not.
    Not(Rc<Self>),

    /// The conjunction of two symbolic bits. The `&` operator should be preferred to this, as it
    /// has the opportunity to perform simpliciations where a direct construction does not.
    And(Rc<Self>, Rc<Self>),
}

impl SymbolicBit {
    /// Concrete `true` bit 1.
    pub const fn one() -> Self {
        ONE
    }

    /// Concrete `false` bit 0.
    pub const fn zero() -> Self {
        ZERO
    }

    /// Concrete bit that is the `literal` provided.
    pub const fn literal(literal: bool) -> Self {
        if literal { ONE } else { ZERO }
    }

    /// Symbolic bit associated with a variable assigned the opaque identifier. Variables with the
    /// same identifier are considered to be identical, which has consequences for logic operations.
    pub const fn variable(id: usize) -> Self {
        Self(Constraint::Variable(id))
    }

    /// Returns underlying concrete value if bit is concrete, `None` otherwise.
    pub const fn maybe_literal(&self) -> Option<bool> {
        match &self.0 {
            Constraint::Literal(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns underlying variable id if bit is a variable, `None` otherwise.
    pub const fn maybe_variable(&self) -> Option<usize> {
        match &self.0 {
            Constraint::Variable(id) => Some(*id),
            _ => None,
        }
    }

    pub fn constraint(&self) -> &Constraint {
        &self.0
    }
}

impl SymbolicBit {
    pub fn equals(self, rhs: Self) -> Self {
        (self.clone() & rhs.clone()) | (!self & !rhs)
    }

    pub fn select(self, lhs: Self, rhs: Self) -> Self {
        (self.clone() & lhs) | (!self & rhs)
    }
}

impl Default for SymbolicBit {
    fn default() -> Self {
        ZERO
    }
}

impl std::ops::Not for SymbolicBit {
    type Output = Self;

    fn not(self) -> Self::Output {
        let constraint = match self.0 {
            Constraint::Literal(b) => Constraint::Literal(!b),
            Constraint::Not(y) => Rc::unwrap_or_clone(y),
            _ => Constraint::Not(Rc::new(self.0)),
        };

        Self(constraint)
    }
}

impl std::ops::BitAnd for SymbolicBit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.0.is_identical(&rhs.0) {
            return self;
        }

        match (&self.0, &rhs.0) {
            (Constraint::Literal(false), _) | (_, Constraint::Literal(false)) => ZERO,
            (Constraint::Literal(true), _) => rhs,
            (_, Constraint::Literal(true)) => self,
            (Constraint::Not(x), y) if x.is_identical(y) => ZERO,
            (x, Constraint::Not(y)) if x.is_identical(y) => ZERO,
            _ => Self(Constraint::And(Rc::new(self.0), Rc::new(rhs.0))),
        }
    }
}

impl std::ops::BitOr for SymbolicBit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        !(!self & !rhs)
    }
}

impl std::ops::BitXor for SymbolicBit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.clone() & !rhs.clone()) | (!self & rhs)
    }
}

impl Constraint {
    pub fn is_identical(&self, rhs: &Self) -> bool {
        match self {
            Self::Literal(x) => {
                if let Self::Literal(y) = rhs {
                    return *x == *y;
                }
            }
            Self::Variable(x) => {
                if let Self::Variable(y) = rhs {
                    return *x == *y;
                }
            }
            Self::Not(x) => {
                if let Self::Not(y) = rhs {
                    if Rc::ptr_eq(x, y) {
                        return true;
                    } else if let Self::Variable(x) = **x
                        && let Self::Variable(y) = **y
                    {
                        // Check if same variable
                        return x == y;
                    }
                }
            }
            Self::And(x, y) => {
                if let Self::And(u, v) = rhs {
                    if Rc::ptr_eq(x, u) && Rc::ptr_eq(y, v) || Rc::ptr_eq(x, v) && Rc::ptr_eq(y, u)
                    {
                        return true;
                    } else if let Self::Variable(x) = **x
                        && let Self::Variable(y) = **y
                        && let Self::Variable(u) = **u
                        && let Self::Variable(v) = **v
                    {
                        // Check if same variables
                        return x == u && y == v || x == v && y == u;
                    }
                }
            }
        }

        false
    }
}
