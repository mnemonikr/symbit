use std::rc::Rc;

use aiger_circuit::circuit::{AigerCircuit, AndOperand, AsAigerCircuit};

use crate::bit::Constraint;

impl<'a> AsAigerCircuit<'a> for Constraint {
    type Inner = Self;

    fn as_aiger_circuit(&'a self) -> AigerCircuit<'a, Self::Inner> {
        match self {
            Constraint::Literal(value) => AigerCircuit::Literal(*value),
            Constraint::Variable(id) => AigerCircuit::Variable(*id),
            Constraint::Not(x) => AigerCircuit::Not(x.as_ref()),
            Constraint::And(x, y) => AigerCircuit::And(
                AndOperand {
                    id: Rc::as_ptr(x) as usize,
                    value: x.as_ref(),
                },
                AndOperand {
                    id: Rc::as_ptr(y) as usize,
                    value: y.as_ref(),
                },
            ),
        }
    }
}
