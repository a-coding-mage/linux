// Translation of x86/include/asm/unwind_hints.h.
//
// The original assembler branch defines assembler macros.  The symbols
// UNWIND_HINT, VALIDATE_UNRET_BEGIN, and the ORC/UNWIND_HINT_TYPE constants
// are supplied by the surrounding low-level environment.

#[macro_export]
macro_rules! UNWIND_HINT_END_OF_STACK {
    () => {
        UNWIND_HINT!(type = UNWIND_HINT_TYPE_END_OF_STACK);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_UNDEFINED {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_UNDEFINED, 0, 0, 0);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_ENTRY {
    () => {
        VALIDATE_UNRET_BEGIN!();
        UNWIND_HINT_END_OF_STACK!();
    };
}

// The assembler macro selects sp_reg from base, then selects the hint type
// and adjusts sp_offset according to partial and extra.
#[macro_export]
macro_rules! UNWIND_HINT_REGS {
    (base = $base:expr, offset = $offset:expr, indirect = $indirect:expr,
     extra = $extra:expr, partial = $partial:expr, signal = $signal:expr) => {{
        // X86_RSP/X86_RBP/X86_RDI/X86_RDX/X86_R10 represent the assembler
        // register tokens %rsp/%rbp/%rdi/%rdx/%r10 in Rust callers.
        let sp_reg = if $base == X86_RSP {
            if $indirect { ORC_REG_SP_INDIRECT } else { ORC_REG_SP }
        } else if $base == X86_RBP {
            ORC_REG_BP
        } else if $base == X86_RDI {
            ORC_REG_DI
        } else if $base == X86_RDX {
            ORC_REG_DX
        } else if $base == X86_R10 {
            ORC_REG_R10
        } else {
            panic!("UNWIND_HINT_REGS: bad base register")
        };
        let mut sp_offset = $offset;
        let hint_type = if $partial || $extra == 0 {
            if $extra == 0 && !$partial {
                sp_offset = $offset + (16 * 8);
            }
            UNWIND_HINT_TYPE_REGS_PARTIAL
        } else {
            UNWIND_HINT_TYPE_REGS
        };
        UNWIND_HINT!(sp_reg = sp_reg, sp_offset = sp_offset,
                     type = hint_type, signal = $signal);
    }};
}

#[macro_export]
macro_rules! UNWIND_HINT_IRET_REGS {
    (base = $base:expr, offset = $offset:expr, signal = $signal:expr) => {
        UNWIND_HINT_REGS!(base = $base, offset = $offset, partial = 1,
                          signal = $signal, indirect = 0, extra = 1);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_IRET_ENTRY {
    (base = $base:expr, offset = $offset:expr, signal = $signal:expr) => {
        VALIDATE_UNRET_BEGIN!();
        UNWIND_HINT_IRET_REGS!(base = $base, offset = $offset, signal = $signal);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_FUNC {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_FUNC, ORC_REG_SP, 8, 0);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_SAVE {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_SAVE, 0, 0, 0);
    };
}

#[macro_export]
macro_rules! UNWIND_HINT_RESTORE {
    () => {
        UNWIND_HINT!(UNWIND_HINT_TYPE_RESTORE, 0, 0, 0);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
