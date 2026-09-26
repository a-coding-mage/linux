// SPDX-License-Identifier: GPL-2.0-only
//! Original DEBUG-enabled init/main diagnostics and dynamic-debug records.

#[cfg(CONFIG_DYNAMIC_DEBUG)]
use super::bindings;
#[cfg(CONFIG_DYNAMIC_DEBUG)]
use kernel::ffi::c_char;

#[cfg(CONFIG_DYNAMIC_DEBUG)]
#[repr(transparent)]
pub(crate) struct DebugDescriptor(pub(crate) core::cell::UnsafeCell<bindings::_ddebug>);

// SAFETY: As for C's __dyndbg records, the dynamic-debug and static-key
// subsystems own mutation. Access uses raw pointers, never shared Rust field
// references, and the record has permanent static storage.
#[cfg(CONFIG_DYNAMIC_DEBUG)]
unsafe impl Sync for DebugDescriptor {}

#[cfg(CONFIG_DYNAMIC_DEBUG)]
impl DebugDescriptor {
    pub(crate) const fn new(
        function: *const c_char,
        format: *const c_char,
        file: *const c_char,
        line: u32,
    ) -> Self {
        // SAFETY: The canonical C descriptor contains only integer, raw-pointer
        // and static-key fields, all of which admit an all-zero representation.
        let mut value: bindings::_ddebug = unsafe { core::mem::zeroed() };
        value.modname = b"main\0".as_ptr().cast();
        value.function = function;
        value.filename = file;
        value.format = format;
        // Bindgen's generated bitfield setters are not const. Encode exactly
        // dynamic_debug.h's 18-bit line, CLS_BITS class and 8-bit flags in the
        // canonical generated storage; native C metadata tests verify this.
        let class_bits = bindings::RUST_INIT_MAIN_DPRINTK_CLASS_BITS;
        let line_bits = 32 - class_bits - 8;
        let line = line & ((1 << line_bits) - 1);
        let class = bindings::RUST_INIT_MAIN_DPRINTK_CLASS_DEFAULT;
        let flags = bindings::RUST_INIT_MAIN_DPRINTK_FLAGS_PRINT;
        #[cfg(target_endian = "little")]
        let bits = line | (class << line_bits) | (flags << 24);
        #[cfg(target_endian = "big")]
        let bits = (line << (class_bits + 8)) | (class << 8) | flags;
        value._bitfield_1 = bindings::__BindgenBitfieldUnit::new(bits.to_ne_bytes());
        #[cfg(CONFIG_JUMP_LABEL)]
        {
            // Initialize the canonical true-key member of the union, as
            // STATIC_KEY_TRUE_INIT does in C; no union member is read here.
            value.key.dd_key_true.key.enabled.counter = 1;
            value.key.dd_key_true.key.__bindgen_anon_1.type_ = bindings::JUMP_TYPE_TRUE as _;
        }
        Self(core::cell::UnsafeCell::new(value))
    }
}

/// Emit an original main.c pr_debug call with its original function name.
///
/// The caller supplies an unsafe context, live C pointers and exact promoted
/// C varargs. The literal format excludes the log level and terminating NUL.
/// Dynamic debug evaluates varargs only when enabled, exactly as the C macro.
macro_rules! main_debug {
    ($function:literal, $format:literal $(, $argument:expr)* $(,)?) => {{
        #[cfg(CONFIG_DYNAMIC_DEBUG)]
        {
            #[link_section = "__dyndbg"]
            static DESCRIPTOR: $crate::main_debug::DebugDescriptor =
                $crate::main_debug::DebugDescriptor::new(
                    concat!($function, "\0").as_ptr().cast(),
                    concat!($format, "\0").as_ptr().cast(),
                    concat!(file!(), "\0").as_ptr().cast(),
                    line!(),
                );
            let descriptor = DESCRIPTOR.0.get();
            #[cfg(CONFIG_JUMP_LABEL)]
            let enabled = !kernel::jump_label::arch_static_branch!(
                @offset DESCRIPTOR, core::mem::offset_of!($crate::bindings::_ddebug, key), true
            );
            #[cfg(not(CONFIG_JUMP_LABEL))]
            let enabled = $crate::bindings::_ddebug::flags_raw(descriptor)
                & $crate::bindings::RUST_INIT_MAIN_DPRINTK_FLAGS_PRINT != 0;
            if enabled {
                $crate::bindings::__dynamic_pr_debug(
                    descriptor, concat!($format, "\0").as_ptr().cast() $(, $argument)*);
                if $crate::bindings::_ddebug::flags_raw(descriptor)
                    & $crate::bindings::RUST_INIT_MAIN_DPRINTK_FLAGS_STACK != 0
                {
                    $crate::bindings::dump_stack();
                }
            }
        }
        // main.c defines DEBUG unconditionally, independently of Rust's
        // debug_assertions setting and CONFIG_DYNAMIC_DEBUG_CORE alone.
        #[cfg(not(CONFIG_DYNAMIC_DEBUG))]
        { $crate::main_printk::main_printk!($function, concat!("\x017", $format, "\0").as_bytes()
                                           $(, $argument)*); }
    }};
}

pub(crate) use main_debug;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
