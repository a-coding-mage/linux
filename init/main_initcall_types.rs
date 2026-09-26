// SPDX-License-Identifier: GPL-2.0-only
//! Callable initcalls, distinct from nullable linker-entry storage.

/// A live initcall that the boot/module initializer will unconditionally call.
///
/// The bare C function pointer preserves the original nested KCFI type.
/// Bindgen's nullable Option<fn> remains appropriate for stored linker entries,
/// but has a different nominal CFI encoding when used as a callback argument.
pub(super) type InitcallFn = unsafe extern "C" fn() -> kernel::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
