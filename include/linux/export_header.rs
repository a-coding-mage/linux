/* SPDX-License-Identifier: GPL-2.0-only */

// Translation of linux/export.h.  The C include dependencies are supplied by
// the surrounding kernel translation unit.

/*
 * This comment block is used by fixdep. Please do not remove.
 *
 * When CONFIG_MODVERSIONS is changed from n to y, all source files having
 * EXPORT_SYMBOL variants must be re-compiled because genksyms is run as a
 * side effect of the *.o build rule.
 */

// LLVM integrated assembler can merge adjacent string literals passed to
// `.ascii`, but not to `.asciz`; the original macros consequently emit these
// records as separate assembler directives.

#[cfg(not(__DISABLE_EXPORTS))]
#[macro_export]
macro_rules! ___EXPORT_SYMBOL {
    ($sym:ident, $license:expr, $($ns:expr)*) => {
        // C expansion:
        // .section ".export_symbol","a"
        // __export_symbol_<sym>:
        // .asciz <license>
        // .ascii <namespace> "\0"
        // .balign 8/.balign 4; .quad/.long <sym>; .previous
        // The section record is emitted by the target assembler integration.
        const _: () = ();
    };
}

#[cfg(__DISABLE_EXPORTS)]
#[macro_export]
macro_rules! ___EXPORT_SYMBOL {
    ($sym:ident, $license:expr, $($ns:expr)*) => {};
}

// CONFIG_GENDWARFKSYMS emits a discarded pointer for each exported symbol.
#[cfg(CONFIG_GENDWARFKSYMS)]
#[macro_export]
macro_rules! __GENDWARFKSYMS_EXPORT {
    ($sym:ident) => {
        // static typeof($sym) *__gendwarfksyms_ptr_<sym> __used
        //     __section(".discard.gendwarfksyms") = &$sym;
        const _: () = ();
    };
}

#[cfg(not(CONFIG_GENDWARFKSYMS))]
#[macro_export]
macro_rules! __GENDWARFKSYMS_EXPORT {
    ($sym:ident) => {};
}

#[cfg(__DISABLE_EXPORTS)]
#[macro_export]
macro_rules! __EXPORT_SYMBOL {
    ($sym:ident, $license:expr, $ns:expr) => {};
}

#[cfg(all(not(__DISABLE_EXPORTS), __GENKSYMS__))]
#[macro_export]
macro_rules! __EXPORT_SYMBOL {
    ($sym:ident, $license:expr, $ns:expr) => {
        // __GENKSYMS_EXPORT_SYMBOL($sym)
        const _: () = ();
    };
}

#[cfg(all(not(__DISABLE_EXPORTS), not(__GENKSYMS__)))]
#[macro_export]
macro_rules! __EXPORT_SYMBOL {
    ($sym:ident, $license:expr, $ns:expr) => {
        // C also declares `extern typeof($sym) $sym`, marks it addressable,
        // emits __GENDWARFKSYMS_EXPORT, and attaches ___EXPORT_SYMBOL as asm.
        $crate::__GENDWARFKSYMS_EXPORT!($sym);
        $crate::___EXPORT_SYMBOL!($sym, $license, $ns);
    };
}

#[cfg(DEFAULT_SYMBOL_NAMESPACE)]
#[macro_export]
macro_rules! _EXPORT_SYMBOL {
    ($sym:ident, $license:expr) => {
        $crate::__EXPORT_SYMBOL!($sym, $license, DEFAULT_SYMBOL_NAMESPACE);
    };
}

#[cfg(not(DEFAULT_SYMBOL_NAMESPACE))]
#[macro_export]
macro_rules! _EXPORT_SYMBOL {
    ($sym:ident, $license:expr) => {
        $crate::__EXPORT_SYMBOL!($sym, $license, "");
    };
}

#[macro_export]
macro_rules! EXPORT_SYMBOL { ($sym:ident) => { $crate::_EXPORT_SYMBOL!($sym, ""); }; }
#[macro_export]
macro_rules! EXPORT_SYMBOL_GPL { ($sym:ident) => { $crate::_EXPORT_SYMBOL!($sym, "GPL"); }; }
#[macro_export]
macro_rules! EXPORT_SYMBOL_NS { ($sym:ident, $ns:expr) => { $crate::__EXPORT_SYMBOL!($sym, "", $ns); }; }
#[macro_export]
macro_rules! EXPORT_SYMBOL_NS_GPL { ($sym:ident, $ns:expr) => { $crate::__EXPORT_SYMBOL!($sym, "GPL", $ns); }; }
#[macro_export]
macro_rules! EXPORT_SYMBOL_FOR_MODULES {
    ($sym:ident, $mods:expr) => { $crate::__EXPORT_SYMBOL!($sym, "GPL", concat!("module:", $mods)); };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
