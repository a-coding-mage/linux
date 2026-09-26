// SPDX-License-Identifier: GPL-2.0-only
//! Invoke the original linker constructor range when kernel startup owns it.

/// Run constructors in linker order, except on UML where ELF startup ran them.
///
/// # Safety
///
/// The caller owns the original constructor phase. The linker range contains
/// aligned, initialized, nonnull C-ABI callbacks, and the range and targets
/// remain live for the entire iteration. Each callback's prerequisites hold.
#[link_section = ".init.text"]
pub(super) unsafe fn do_ctors() {
    #[cfg(all(CONFIG_CONSTRUCTORS, not(CONFIG_UML)))]
    {
        use super::bindings;
        let mut entry = core::ptr::addr_of!(bindings::__ctors_start).cast::<bindings::ctor_fn_t>();
        let end = core::ptr::addr_of!(bindings::__ctors_end).cast::<bindings::ctor_fn_t>();
        // SAFETY: the original linker/caller contract supplies live callbacks.
        // Read one entry at a time, so callback side effects retain C ordering.
        unsafe {
            while entry < end {
                entry.read().unwrap_unchecked()();
                entry = entry.wrapping_add(1);
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
