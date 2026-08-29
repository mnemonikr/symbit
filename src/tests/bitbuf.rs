use crate::{SymbolicBit, SymbolicBitBuf};

const ZERO: SymbolicBit = const { SymbolicBit::zero() };
const ONE: SymbolicBit = const { SymbolicBit::one() };

#[test]
fn ops_not() {
    let x: SymbolicBitBuf<1> = [ZERO].into();
    assert_eq!(u8::try_from(!x.clone()).unwrap(), 0x1);
    assert_eq!(u8::try_from(!!x).unwrap(), 0x0);
}

#[test]
fn ops_bitor() {
    let x: SymbolicBitBuf<4> = [ZERO, ZERO, ONE, ONE].into();
    let y: SymbolicBitBuf<4> = [ZERO, ONE, ZERO, ONE].into();
    assert_eq!(u8::try_from(x | y).unwrap(), 0x0E);
}

#[test]
fn ops_bitand() {
    let x: SymbolicBitBuf<4> = [ZERO, ZERO, ONE, ONE].into();
    let y: SymbolicBitBuf<4> = [ZERO, ONE, ZERO, ONE].into();
    assert_eq!(u8::try_from(x & y).unwrap(), 0x08);
}

#[test]
fn ops_shl_usize() {
    let x: SymbolicBitBuf<4> = [ONE, ZERO, ZERO, ZERO].into();
    let result = u8::try_from(x << 1).unwrap();
    assert_eq!(result, 0x2);
}

#[test]
fn ops_shl_sym() {
    let x: SymbolicBitBuf<4> = [ONE, ZERO, ZERO, ZERO].into();
    let result = u8::try_from(x.clone() << x).unwrap();
    assert_eq!(result, 0x2);
}

#[test]
fn ops_shr_usize() {
    let x: SymbolicBitBuf<4> = [ZERO, ONE, ZERO, ZERO].into();
    let result = u8::try_from(x >> 1).unwrap();
    assert_eq!(result, 0x1);
}

#[test]
fn ops_shr_sym() {
    let x: SymbolicBitBuf<4> = [ZERO, ONE, ZERO, ZERO].into();
    let y: SymbolicBitBuf<4> = [ONE, ZERO, ZERO, ZERO].into();
    let result = u8::try_from(x >> y).unwrap();
    assert_eq!(result, 0x1);
}
