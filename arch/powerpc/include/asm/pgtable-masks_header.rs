/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: _ASM_POWERPC_PGTABLE_MASKS_H

// These definitions are conditional in the C header: they apply only when
// the corresponding symbols have not already been defined by dependencies.
// Rust macro definitions cannot test for prior macro definitions directly.
macro_rules! _PAGE_NA {
    () => { 0 };
}
macro_rules! _PAGE_NAX {
    () => { _PAGE_EXEC };
}
macro_rules! _PAGE_RO {
    () => { _PAGE_READ };
}
macro_rules! _PAGE_ROX {
    () => { _PAGE_READ | _PAGE_EXEC };
}
macro_rules! _PAGE_RW {
    () => { _PAGE_READ | _PAGE_WRITE };
}
macro_rules! _PAGE_RWX {
    () => { _PAGE_READ | _PAGE_WRITE | _PAGE_EXEC };
}

/* Permission flags for kernel mappings */
// These definitions are conditional in the C header; see the note above.
macro_rules! _PAGE_KERNEL_RO {
    () => { _PAGE_RO };
}
macro_rules! _PAGE_KERNEL_ROX {
    () => { _PAGE_ROX };
}
macro_rules! _PAGE_KERNEL_RW {
    () => { _PAGE_RW | _PAGE_DIRTY };
}
macro_rules! _PAGE_KERNEL_RWX {
    () => { _PAGE_RWX | _PAGE_DIRTY };
}

/* Permission masks used to generate the __P and __S table */
macro_rules! PAGE_NONE {
    () => { __pgprot(_PAGE_BASE | _PAGE_NA) };
}
macro_rules! PAGE_EXECONLY_X {
    () => { __pgprot(_PAGE_BASE | _PAGE_NAX) };
}
macro_rules! PAGE_SHARED {
    () => { __pgprot(_PAGE_BASE | _PAGE_RW) };
}
macro_rules! PAGE_SHARED_X {
    () => { __pgprot(_PAGE_BASE | _PAGE_RWX) };
}
macro_rules! PAGE_COPY {
    () => { __pgprot(_PAGE_BASE | _PAGE_RO) };
}
macro_rules! PAGE_COPY_X {
    () => { __pgprot(_PAGE_BASE | _PAGE_ROX) };
}
macro_rules! PAGE_READONLY {
    () => { __pgprot(_PAGE_BASE | _PAGE_RO) };
}
macro_rules! PAGE_READONLY_X {
    () => { __pgprot(_PAGE_BASE | _PAGE_ROX) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
