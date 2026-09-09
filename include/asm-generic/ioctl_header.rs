/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <uapi/asm-generic/ioctl.h>

#[cfg(not(checker))]
extern "C" {
    pub static mut __invalid_size_argument_for_IOC: ::core::ffi::c_uint;
}

// Under the C static analyzer, _IOC_TYPECHECK(t) is sizeof(t).
#[cfg(checker)]
#[macro_export]
macro_rules! _IOC_TYPECHECK {
    ($t:ty) => {
        ::core::mem::size_of::<$t>()
    };
}

// For non-checker builds, preserve the C size validation and its fallback to
// the externally defined invalid-size symbol. The C expression sizeof(t[1])
// validates array element sizing; Rust callers should provide the equivalent
// type-level size expression where needed.
#[cfg(not(checker))]
#[macro_export]
macro_rules! _IOC_TYPECHECK {
    ($t:ty) => {{
        let __ioc_size = ::core::mem::size_of::<$t>();
        if __ioc_size < (1usize << _IOC_SIZEBITS) {
            __ioc_size
        } else {
            unsafe { $crate::__invalid_size_argument_for_IOC as usize }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
