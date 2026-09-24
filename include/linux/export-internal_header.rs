// SPDX-License-Identifier: GPL-2.0-only
//! Final exported-symbol tables emitted by modpost, not export-owner records.
//!
//! Translated from `linux/export-internal.h`. Only generated metadata imports
//! this header. Assembly references exact linker names without declaring fake
//! functions or objects; the real owner retains its ABI, DWARF and version.
//!
//! Linkage names use ASCII assembler identifiers (including `.` and `$`).
//! A namespace has the same meaning as the original C macro's string argument:
//! assembler string escapes are interpreted by the target assembler. UTF-8,
//! braces and control bytes are retained. Unescaped quotes, trailing escapes
//! and NUL are rejected so the argument cannot terminate the assembler string.
//! A generator must separately preserve the C source-literal decoding step;
//! passing raw modpost input here is not an arbitrary-byte escaping protocol.

/// Literal alignment directive for `concat!` assembly templates.
#[cfg(any(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, not(CONFIG_64BIT)))]
#[macro_export]
macro_rules! __KSYM_ALIGN {
    () => {
        ".balign 4"
    };
}
/// Literal alignment directive for `concat!` assembly templates.
#[cfg(all(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS), CONFIG_64BIT))]
#[macro_export]
macro_rules! __KSYM_ALIGN {
    () => {
        ".balign 8"
    };
}

/// Convert a C identifier or an explicitly quoted linkage name to text.
#[macro_export]
macro_rules! __KSYM_NAME {
    ($name:ident) => {
        stringify!($name)
    };
    ($name:literal) => {
        $name
    };
}

/// Validate text before interpolating it into assembly, without runtime code.
#[macro_export]
macro_rules! __KSYM_VALIDATE {
    ($name:tt, $ns:literal) => {
        const _: () = {
            let name = $crate::__KSYM_NAME!($name).as_bytes();
            assert!(!name.is_empty(), "empty exported linker name");
            assert!(!(name.len() == 1 && name[0] == b'.'), "current location is not a linker name");
            let mut i = 0;
            while i < name.len() {
                let b = name[i];
                assert!(b == b'_' || b == b'.' || b == b'$'
                    || b >= b'a' && b <= b'z' || b >= b'A' && b <= b'Z'
                    || i != 0 && b >= b'0' && b <= b'9',
                    "unsupported exported linker name");
                i += 1;
            }
            let namespace = $ns.as_bytes();
            i = 0;
            let mut escaped = false;
            while i < namespace.len() {
                let b = namespace[i];
                assert!(b != 0 && (b != b'"' || escaped),
                    "unsupported export namespace");
                escaped = b == b'\\' && !escaped;
                i += 1;
            }
            assert!(!escaped, "trailing namespace escape");
        };
    };
}

/// Form a native final-table reference, preserving PREL32 relocation semantics.
#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[macro_export]
macro_rules! __KSYM_REF {
    ($sym:ident) => {
        concat!(".long ", stringify!($sym), " - .")
    };
    ($sym:expr) => {
        concat!(".long ", $sym, " - .")
    };
}
/// Form a native final-table absolute reference.
#[cfg(all(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS), CONFIG_64BIT))]
#[macro_export]
macro_rules! __KSYM_REF {
    ($sym:ident) => {
        concat!(".quad ", stringify!($sym))
    };
    ($sym:expr) => {
        concat!(".quad ", $sym)
    };
}
/// Form a native final-table absolute reference.
#[cfg(all(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS), not(CONFIG_64BIT)))]
#[macro_export]
macro_rules! __KSYM_REF {
    ($sym:ident) => {
        concat!(".long ", stringify!($sym))
    };
    ($sym:expr) => {
        concat!(".long ", $sym)
    };
}

/// Architecture-specific function linkage expression, as in the C header.
#[cfg(all(CONFIG_PARISC, CONFIG_64BIT))]
#[macro_export]
macro_rules! KSYM_FUNC {
    ($name:tt) => {
        concat!("P%", $crate::__KSYM_NAME!($name))
    };
}
/// Architecture-specific function linkage expression, as in the C header.
#[cfg(not(all(CONFIG_PARISC, CONFIG_64BIT)))]
#[macro_export]
macro_rules! KSYM_FUNC {
    ($name:tt) => {
        $crate::__KSYM_NAME!($name)
    };
}

/// Emit one final kernel-symbol entry from already normalized C macro fields.
///
/// The generated-input frontend records the original final `__KSYMTAB` name,
/// expanded reference expression and decoded namespace argument separately.
/// These are trusted compiler-output strings, not raw modpost text. In
/// particular a name may be numeric after C macro expansion, and a reference
/// may be an assembler expression or the architecture-specific `P%` form.
/// Leave their assembler syntax and diagnostics to the target assembler,
/// exactly as the original C header does. Rust constructs the complete table
/// and relocation layout here; the frontend never supplies assembly records.
#[macro_export]
macro_rules! __KSYMTAB_NORMALIZED {
    ($name:tt, $sym:expr, $ns:literal) => {
        core::arch::global_asm!(
            concat!(
                ".section \"__ksymtab_strings\",\"aMS\",%progbits,1\n",
                "__kstrtab_",
                $crate::__KSYM_NAME!($name),
                ":\n",
                ".asciz \"",
                $crate::__KSYM_NAME!($name),
                "\"\n",
                "__kstrtabns_",
                $crate::__KSYM_NAME!($name),
                ":\n",
                ".asciz \"",
                $ns,
                "\"\n.previous\n",
                ".section \"___ksymtab+",
                $crate::__KSYM_NAME!($name),
                "\",\"a\"\n",
                $crate::__KSYM_ALIGN!(),
                "\n",
                "__ksymtab_",
                $crate::__KSYM_NAME!($name),
                ":\n",
                $crate::__KSYM_REF!($sym),
                "\n",
                $crate::__KSYM_REF!(concat!("__kstrtab_", $crate::__KSYM_NAME!($name))),
                "\n",
                $crate::__KSYM_REF!(concat!("__kstrtabns_", $crate::__KSYM_NAME!($name))),
                "\n.previous\n",
            ),
            options(raw)
        );
    };
}

/// Emit one final entry from an exact ordinary linker name and namespace.
#[macro_export]
macro_rules! __KSYMTAB {
    ($name:tt, $sym:expr, $ns:literal) => {
        $crate::__KSYM_VALIDATE!($name, $ns);
        $crate::__KSYMTAB_NORMALIZED!($name, $sym, $ns);
    };
}

/// Emit a function export using its exact architecture-specific linker name.
#[macro_export]
macro_rules! KSYMTAB_FUNC {
    ($name:tt, $ns:literal) => {
        $crate::__KSYMTAB!($name, $crate::KSYM_FUNC!($name), $ns);
    };
}
/// Emit a data export without inventing an external Rust object type.
#[macro_export]
macro_rules! KSYMTAB_DATA {
    ($name:tt, $ns:literal) => {
        $crate::__KSYMTAB!($name, $crate::__KSYM_NAME!($name), $ns);
    };
}

/// Emit the already computed CRC; never recompute a metadata object's version.
#[macro_export]
macro_rules! SYMBOL_CRC {
    ($name:tt, $crc:expr) => {
        $crate::__KSYM_VALIDATE!($name, "");
        const _: u32 = $crc;
        core::arch::global_asm!(
            concat!(
                ".section \"___kcrctab+",
                $crate::__KSYM_NAME!($name),
                "\",\"a\"\n",
                ".balign 4\n__crc_",
                $crate::__KSYM_NAME!($name),
                ":\n",
                ".long {crc}\n.previous\n"
            ),
            crc = const {
                let value: u32 = $crc;
                value
            }
        );
    };
}
/// Emit modpost's symbol flags, including the original GPL-only bit.
#[macro_export]
macro_rules! SYMBOL_FLAGS {
    ($name:tt, $flags:expr) => {
        $crate::__KSYM_VALIDATE!($name, "");
        const _: u8 = $flags;
        core::arch::global_asm!(
            concat!(
                ".section \"___kflagstab+",
                $crate::__KSYM_NAME!($name),
                "\",\"a\"\n",
                "__flags_",
                $crate::__KSYM_NAME!($name),
                ":\n",
                ".byte {flags}\n.previous\n"
            ),
            flags = const {
                let value: u8 = $flags;
                value
            }
        );
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
