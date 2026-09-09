/* SPDX-License-Identifier: GPL-2.0-only */

// These have to be macros rather than functions: csr_read()/csr_write()
// stringify their CSR argument into the inline asm template via __ASM_STR(),
// so the CSR number must be a literal token at preprocessing time. Passing it
// as a function parameter emits "csrr %0, iregcsr", which no assembler can
// resolve. RISC-V has no register-indirect form of csrr/csrw - the CSR is a
// 12-bit immediate - so the sireg CSR selecting the indirect window cannot
// itself be a variable.

#[macro_export]
macro_rules! csr_indirect_read {
    ($iregcsr:tt, $iselbase:expr, $iseloff:expr) => {{
        let mut __value: ::core::primitive::usize = 0;
        let mut __flags: ::core::primitive::usize;
        local_irq_save!(__flags);
        csr_write!(CSR_ISELECT, ($iselbase) + ($iseloff));
        __value = csr_read!($iregcsr);
        local_irq_restore!(__flags);
        __value
    }};
}

#[macro_export]
macro_rules! csr_indirect_write {
    ($iregcsr:tt, $iselbase:expr, $iseloff:expr, $value:expr) => {{
        let mut __flags: ::core::primitive::usize;
        local_irq_save!(__flags);
        csr_write!(CSR_ISELECT, ($iselbase) + ($iseloff));
        csr_write!($iregcsr, ($value));
        local_irq_restore!(__flags);
    }};
}

#[macro_export]
macro_rules! csr_indirect_warl {
    ($iregcsr:tt, $iselbase:expr, $iseloff:expr, $warl_val:expr) => {{
        let mut __old_val: ::core::primitive::usize = 0;
        let mut __value: ::core::primitive::usize = 0;
        let mut __flags: ::core::primitive::usize;
        local_irq_save!(__flags);
        csr_write!(CSR_ISELECT, ($iselbase) + ($iseloff));
        __old_val = csr_read!($iregcsr);
        csr_write!($iregcsr, ($warl_val));
        __value = csr_read!($iregcsr);
        csr_write!($iregcsr, __old_val);
        local_irq_restore!(__flags);
        __value
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
