// SPDX-License-Identifier: GPL-2.0-only
//! Formatted warnings used by the staged x86-64 and ARM64 init/main owner.
//!
//! x86-64 keeps the format and arguments in its native WARN static-call path;
//! ARM64 uses the original printk-then-trap path. This is not a generic substitute
//! for architecture warning implementations which have not been reconstructed.

#[cfg(all(CONFIG_BUG, CONFIG_X86_64, CONFIG_HAVE_STATIC_CALL_INLINE, not(MODULE)))]
#[repr(transparent)]
pub(crate) struct WarnStaticCallKey(pub(crate) *const super::bindings::static_call_key);

// SAFETY: the address names the permanent native WARN static-call key.
#[cfg(all(CONFIG_BUG, CONFIG_X86_64, CONFIG_HAVE_STATIC_CALL_INLINE, not(MODULE)))]
unsafe impl Sync for WarnStaticCallKey {}

/// The x86 entry points at the next instruction and contains relative native
/// format/file pointers. Keep the key visible so objtool can build call sites.
#[cfg(all(CONFIG_BUG, CONFIG_X86_64))]
macro_rules! x86_warning {
    ($format:expr $(, $argument:expr)* $(,)?) => {{
        const FLAGS: u32 = $crate::bindings::BUGFLAG_WARNING
            | $crate::bindings::BUGFLAG_ARGS | ($crate::bindings::TAINT_WARN << 8);
        #[cfg(all(CONFIG_HAVE_STATIC_CALL_INLINE, not(MODULE)))]
        {
            #[used]
            #[link_section = ".discard.addressable"]
            static KEY: $crate::main_warn::WarnStaticCallKey =
                $crate::main_warn::WarnStaticCallKey(core::ptr::addr_of!($crate::bindings::__SCK__WARN_trap));
        }
        let entry: *mut $crate::bindings::bug_entry;
        // SAFETY: this emits native bug metadata and computes its address. It
        // does not issue a trap; the real static-call target consumes the entry.
        // Relative fields are filled by the linker via these array symbols.
        const FORMAT_BYTES: &[u8] = $format;
        static FORMAT_DATA: [u8; FORMAT_BYTES.len()] = {
            let mut bytes = [0; FORMAT_BYTES.len()];
            let mut index = 0;
            while index < bytes.len() { bytes[index] = FORMAT_BYTES[index]; index += 1; }
            bytes
        };
        #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
        static FILE_DATA: [u8; concat!(file!(), "\0").len()] = {
            let source = concat!(file!(), "\0").as_bytes();
            let mut bytes = [0; concat!(file!(), "\0").len()];
            let mut index = 0;
            while index < bytes.len() { bytes[index] = source[index]; index += 1; }
            bytes
        };
        #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
        core::arch::asm!(
            "lea {entry}, [rip + 2f]", "3:",
            ".pushsection __bug_table,\"aw\"",
            ".if {objtool}", "912:",
            ".pushsection .discard.annotate_data,\"M\",@progbits,8",
            ".long 912b - ., {special}", ".popsection", ".endif",
            "2:", ".long 3b - .", ".long {format} - .",
            ".long {file} - .", ".short {line}", ".short {flags}",
            ".org 2b + {size}", ".popsection",
            entry = out(reg) entry, format = sym FORMAT_DATA, file = sym FILE_DATA,
            line = const line!(), flags = const FLAGS,
            objtool = const cfg!(CONFIG_OBJTOOL) as u8,
            special = const $crate::bindings::ANNOTYPE_DATA_SPECIAL,
            size = const core::mem::size_of::<$crate::bindings::bug_entry>(),
            options(nostack, preserves_flags),
        );
        #[cfg(not(CONFIG_DEBUG_BUGVERBOSE))]
        core::arch::asm!(
            "lea {entry}, [rip + 2f]", "3:",
            ".pushsection __bug_table,\"aw\"",
            ".if {objtool}", "912:",
            ".pushsection .discard.annotate_data,\"M\",@progbits,8",
            ".long 912b - ., {special}", ".popsection", ".endif",
            "2:", ".long 3b - .", ".long {format} - .", ".short {flags}",
            ".org 2b + {size}", ".popsection",
            entry = out(reg) entry, format = sym FORMAT_DATA, flags = const FLAGS,
            objtool = const cfg!(CONFIG_OBJTOOL) as u8,
            special = const $crate::bindings::ANNOTYPE_DATA_SPECIAL,
            size = const core::mem::size_of::<$crate::bindings::bug_entry>(),
            options(nostack, preserves_flags),
        );
        $crate::bindings::__SCT__WARN_trap(entry, $($argument),*);
        // The original empty asm prevents a tail call into the warning trap.
        core::arch::asm!("", options(nostack, preserves_flags));
    }};
}

#[cfg(all(CONFIG_BUG, CONFIG_X86_64))]
pub(crate) use x86_warning;

/// Evaluate a boolean condition once; evaluate format arguments only on the
/// enabled warning path, preserving WARN's CONFIG_BUG-disabled no_printk rule.
///
/// The caller supplies an unsafe context and exact C variadic promotions.
macro_rules! main_warn {
    ($condition:expr, $format:expr $(, $argument:expr)* $(,)?) => {{
        let condition: bool = $condition;
        #[cfg(CONFIG_BUG)]
        if condition {
            #[cfg(CONFIG_X86_64)]
            $crate::main_warn::x86_warning!($format $(, $argument)*);
            #[cfg(CONFIG_ARM64)]
            {
                $crate::bindings::__warn_printk($format.as_ptr().cast(), $($argument),*);
                kernel::warn_flags!(file!(), $crate::bindings::BUGFLAG_NO_CUT_HERE
                    | ($crate::bindings::TAINT_WARN << 8));
            }
            #[cfg(not(any(CONFIG_X86_64, CONFIG_ARM64)))]
            compile_error!("staged formatted WARN requires the original architecture implementation");
        }
        #[cfg(not(CONFIG_BUG))]
        if false { let _ = ($format, $($argument),*); }
        condition
    }};
}

pub(crate) use main_warn;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
