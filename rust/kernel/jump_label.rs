// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2024 Google LLC.

//! Logic for static keys.
//!
//! C header: [`include/linux/jump_label.h`](srctree/include/linux/jump_label.h).

use crate::{bindings, types::Opaque};

/// Rust-owned storage for an initially enabled static key.
///
/// Declare this as a `static` and inspect it with [`static_branch_likely!`]. Its
/// transparent representation also permits C code to use the same storage as a
/// `struct static_key_true`. Mutation must go through the kernel's static-key
/// APIs, with their normal initialization and locking requirements.
#[repr(transparent)]
pub struct StaticKeyTrue {
    key: Opaque<bindings::static_key_true>,
}

// SAFETY: The opaque storage admits C-side mutation. The static-key subsystem
// synchronizes both its atomic count and updates to its jump-table entries;
// Rust never reads those fields through shared references.
unsafe impl Sync for StaticKeyTrue {}

impl StaticKeyTrue {
    /// Creates a key with the exact `STATIC_KEY_TRUE_INIT` initial state.
    pub const fn new() -> Self {
        // SAFETY: This generated C type contains only integers and, with jump
        // labels enabled, a union of integers and raw pointers. Zero is a valid
        // initial representation; the two initialized fields below match C.
        let mut key: bindings::static_key_true = unsafe { core::mem::zeroed() };
        key.key.enabled.counter = 1;
        #[cfg(CONFIG_JUMP_LABEL)]
        {
            key.key.__bindgen_anon_1.type_ = bindings::JUMP_TYPE_TRUE as _;
        }
        Self {
            key: Opaque::new(key),
        }
    }

    /// Returns the underlying C key without creating references to its fields.
    ///
    /// Before registering this pointer with C, the key must have a permanent
    /// address, normally supplied by a `static`. Dereferencing or mutating the
    /// pointer remains unsafe and must follow the C static-key API's contract.
    #[inline]
    pub const fn as_ptr(&self) -> *mut bindings::static_key {
        self.key.get().cast()
    }
}

impl Default for StaticKeyTrue {
    fn default() -> Self {
        Self::new()
    }
}

/// Branch based on an initially enabled, Rust-owned static key.
///
/// The argument must name a `static` [`StaticKeyTrue`]. With jump labels this
/// emits the architecture's patchable, initially non-branching instruction;
/// otherwise it reads the key through the existing C atomic-count helper.
#[macro_export]
macro_rules! static_branch_likely {
    ($key:path) => {{
        // Reject references or Deref wrappers: asm's `sym` must name the key's
        // storage itself, not a pointer slot that merely coerces to its type.
        let _: *const $crate::jump_label::StaticKeyTrue = ::core::ptr::addr_of!($key);
        let _key: &'static $crate::jump_label::StaticKeyTrue = &$key;

        #[cfg(not(CONFIG_JUMP_LABEL))]
        {
            // SAFETY: The typed static owns a permanently live, initialized
            // key. The helper reads its count with the C atomic API.
            unsafe { $crate::bindings::static_key_count(_key.as_ptr()) > 0 }
        }

        #[cfg(CONFIG_JUMP_LABEL)]
        {
            // SAFETY: StaticKeyTrue is transparent over the real C key at
            // offset zero, with initial type=true. Inverting the architecture
            // branch matches C's static_branch_likely on a static_key_true.
            unsafe { !$crate::jump_label::arch_static_branch!(@offset $key, 0, true) }
        }
    }};
}
pub use static_branch_likely;

/// Branch based on a static key.
///
/// Takes three arguments:
///
/// * `key` - the path to the static variable containing the `static_key`.
/// * `keytyp` - the type of `key`.
/// * `field` - the name of the field of `key` that contains the `static_key`.
///
/// # Safety
///
/// The macro must be used with a real static key defined by C.
#[macro_export]
macro_rules! static_branch_unlikely {
    ($key:path, $keytyp:ty, $field:ident) => {{
        let _key: *const $keytyp = ::core::ptr::addr_of!($key);
        let _key: *const $crate::bindings::static_key_false = ::core::ptr::addr_of!((*_key).$field);
        let _key: *const $crate::bindings::static_key = _key.cast();

        #[cfg(not(CONFIG_JUMP_LABEL))]
        {
            $crate::bindings::static_key_count(_key.cast_mut()) > 0
        }

        #[cfg(CONFIG_JUMP_LABEL)]
        $crate::jump_label::arch_static_branch! { $key, $keytyp, $field, false }
    }};
}
pub use static_branch_unlikely;

/// Assert that the assembly block evaluates to a string literal.
#[cfg(CONFIG_JUMP_LABEL)]
const _: &str = include!(concat!(
    env!("OBJTREE"),
    "/rust/kernel/generated_arch_static_branch_asm.rs"
));

#[macro_export]
#[doc(hidden)]
#[cfg(CONFIG_JUMP_LABEL)]
macro_rules! arch_static_branch {
    ($key:path, $keytyp:ty, $field:ident, $branch:expr) => {
        $crate::jump_label::arch_static_branch!(
            @offset $key, ::core::mem::offset_of!($keytyp, $field), $branch
        )
    };
    (@offset $key:path, $offset:expr, $branch:expr) => {'my_label: {
        $crate::asm!(
            include!(concat!(env!("OBJTREE"), "/rust/kernel/generated_arch_static_branch_asm.rs"));
            l_yes = label {
                break 'my_label true;
            },
            symb = sym $key,
            off = const $offset,
            branch = const $crate::jump_label::bool_to_int($branch),
        );

        break 'my_label false;
    }};
}

#[cfg(CONFIG_JUMP_LABEL)]
pub use arch_static_branch;

/// A helper used by inline assembly to pass a boolean to as a `const` parameter.
///
/// Using this function instead of a cast lets you assert that the input is a boolean, and not some
/// other type that can also be cast to an integer.
#[doc(hidden)]
pub const fn bool_to_int(b: bool) -> i32 {
    b as i32
}
