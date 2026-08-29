use crate::*;

#[test]
fn bit_equality() {
    let x = SymbolicBit::literal(true);
    assert_eq!(x.clone().equals(x), SymbolicBit::literal(true));

    let x = SymbolicBit::literal(false);
    assert_eq!(x.clone().equals(x), SymbolicBit::literal(true));
}

#[test]
fn double_negation() {
    let x = SymbolicBit::variable(0);
    assert_eq!(!!x.clone(), x);
}

#[test]
fn conjunction_with_false() {
    let x = SymbolicBit::variable(0);
    assert_eq!(
        x.clone() & SymbolicBit::literal(false),
        SymbolicBit::literal(false),
    );
    assert_eq!(
        SymbolicBit::literal(false) & x.clone(),
        SymbolicBit::literal(false),
    );
}

#[test]
fn conjunction_with_true() {
    let x = SymbolicBit::variable(0);
    assert_eq!(x.clone() & SymbolicBit::literal(true), x);
    assert_eq!(SymbolicBit::literal(true) & x.clone(), x);
}

#[test]
fn conjunction_with_negated_self() {
    let x = SymbolicBit::variable(0);
    assert_eq!(x.clone() & !x.clone(), SymbolicBit::literal(false));
    assert_eq!(!x.clone() & x.clone(), SymbolicBit::literal(false));
}

#[test]
fn disjunction_with_false() {
    let x = SymbolicBit::variable(0);
    assert_eq!(x.clone() | SymbolicBit::literal(false), x);
    assert_eq!(SymbolicBit::literal(false) | x.clone(), x);
}

#[test]
fn disjunction_with_true() {
    let x = SymbolicBit::variable(0);
    assert_eq!(
        x.clone() | SymbolicBit::literal(true),
        SymbolicBit::literal(true)
    );
    assert_eq!(
        SymbolicBit::literal(true) | x.clone(),
        SymbolicBit::literal(true),
    );
}

#[test]
fn exclusive_or_with_same_variable() {
    let x = SymbolicBit::variable(0);
    assert_eq!(x.clone() ^ x.clone(), SymbolicBit::literal(false));
}

#[test]
fn exclusive_or_with_complex_self() {
    let x = complex_bit();
    assert_eq!(x.clone() ^ x.clone(), SymbolicBit::literal(false));
}

#[test]
fn exclusive_or_with_zero() {
    let x = SymbolicBit::variable(0);
    assert_eq!(x.clone() ^ SymbolicBit::literal(false), x);
    assert_eq!(SymbolicBit::literal(false) ^ x.clone(), x);
}

fn complex_bit() -> SymbolicBit {
    let mut bit = SymbolicBit::literal(true);
    for i in 0..5 {
        bit = SymbolicBit::variable(i) & bit;
    }
    bit
}
