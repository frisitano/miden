use super::{BaseElement, StackTrace, STACK_TOP_SIZE};
use vm_core::FieldElement;
use winterfell::Trace;

// VM EXECUTION TRACE
// ================================================================================================

/// TODO: for now this consists only of stack trace, but will need to include decoder trace,
/// auxiliary table traces etc.
pub struct ExecutionTrace {
    meta: Vec<u8>,
    stack: StackTrace,
}

impl ExecutionTrace {
    pub fn new(stack_trace: StackTrace) -> Self {
        Self {
            meta: Vec::new(),
            stack: stack_trace,
        }
    }

    /// TODO: add docs
    pub fn init_stack_state(&self) -> [BaseElement; STACK_TOP_SIZE] {
        let mut result = [BaseElement::ZERO; STACK_TOP_SIZE];
        self.read_row_into(0, &mut result);
        result
    }

    /// TODO: add docs
    pub fn last_stack_state(&self) -> [BaseElement; STACK_TOP_SIZE] {
        let mut result = [BaseElement::ZERO; STACK_TOP_SIZE];
        self.read_row_into(self.length() - 1, &mut result);
        result
    }
}

// TRACE TRAIT IMPLEMENTATION
// ================================================================================================

impl Trace for ExecutionTrace {
    type BaseField = BaseElement;

    fn width(&self) -> usize {
        self.stack.len()
    }

    fn length(&self) -> usize {
        self.stack[0].len()
    }

    fn get(&self, col_idx: usize, row_idx: usize) -> Self::BaseField {
        self.stack[col_idx][row_idx]
    }

    fn meta(&self) -> &[u8] {
        &self.meta
    }

    fn read_row_into(&self, step: usize, target: &mut [Self::BaseField]) {
        for (i, register) in self.stack.iter().enumerate() {
            target[i] = register[step];
        }
    }

    fn into_columns(self) -> Vec<Vec<Self::BaseField>> {
        self.stack.into()
    }
}