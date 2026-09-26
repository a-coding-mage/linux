// SPDX-License-Identifier: GPL-2.0-only
//! Native printk boundary and index records for the staged init/main owner.

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
#[repr(transparent)]
pub(crate) struct PrintkIndexEntry(pub(crate) super::bindings::pi_entry);

// SAFETY: every field is null or points to permanent immutable string storage.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
unsafe impl Sync for PrintkIndexEntry {}

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
#[repr(transparent)]
pub(crate) struct PrintkIndexPointer(pub(crate) *const super::bindings::pi_entry);

// SAFETY: the pointer names a permanent immutable index record.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
unsafe impl Sync for PrintkIndexPointer {}

/// Emit a constant level-prefixed C format with its original C function name.
///
/// The caller supplies an unsafe context, live C pointers and the exact promoted
/// C variadic argument types. With PRINTK disabled, arguments are still evaluated
/// once, as for the original inline `_printk` stub, and the result is zero.
macro_rules! main_printk {
    ($function:literal, $format:expr $(, $argument:expr)* $(,)?) => {{
        $crate::main_printk::main_printk!(@index true, $function, $format $(, $argument)*)
    }};
    (@index $indexed:expr, $function:literal, $format:expr $(, $argument:expr)* $(,)?) => {{
        #[cfg(CONFIG_PRINTK)]
        {
            #[cfg(CONFIG_PRINTK_INDEX)]
            {
                // C removes function-local records in constant-dead branches.
                // Rust's #[used] survives such branches; a zero-length array
                // preserves the same absence without inventing config flags.
                const INDEXED: bool = $indexed;
                static ENTRY: $crate::main_printk::PrintkIndexEntry =
                    $crate::main_printk::PrintkIndexEntry($crate::bindings::pi_entry {
                        fmt: $format.as_ptr().cast(),
                        func: concat!($function, "\0").as_ptr().cast(),
                        file: concat!(file!(), "\0").as_ptr().cast(),
                        line: line!(),
                        level: core::ptr::null(),
                        subsys_fmt_prefix: core::ptr::null(),
                    });
                #[used]
                #[link_section = ".printk_index"]
                static POINTER: [$crate::main_printk::PrintkIndexPointer; INDEXED as usize] = {
                    let mut pointers = [$crate::main_printk::PrintkIndexPointer(core::ptr::null()); INDEXED as usize];
                    let mut index = 0;
                    while index < pointers.len() {
                        pointers[index] = $crate::main_printk::PrintkIndexPointer(&ENTRY.0);
                        index += 1;
                    }
                    pointers
                };
            }
            $crate::bindings::_printk($format.as_ptr().cast(), $($argument),*)
        }
        #[cfg(not(CONFIG_PRINTK))]
        {
            let _ = ($format, $($argument),*);
            0 as kernel::ffi::c_int
        }
    }};
}

pub(crate) use main_printk;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
