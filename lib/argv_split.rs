// SPDX-License-Identifier: GPL-2.0
//! Split an owned snapshot at kernel whitespace while retaining its hidden owner.

use kernel::bindings::{self, gfp_t};
use kernel::ctype::isspace;
use kernel::ffi::{c_char, c_int};

// The kernel compiles the original signed counter with wrapping arithmetic.
unsafe fn count_argc(mut string: *const c_char) -> c_int {
    let mut count: c_int = 0;
    let mut was_space = true;
    // SAFETY: the caller supplies the uniquely owned, terminated snapshot.
    unsafe {
        while string.read() != 0 {
            if isspace(string.read() as c_int) {
                was_space = true;
            } else if was_space {
                was_space = false;
                count = count.wrapping_add(1);
            }
            string = string.wrapping_add(1);
        }
    }
    count
}

/// Free the hidden string owner and then its original pointer allocation.
///
/// # Safety
///
/// `argv` must be an outstanding return value from `argv_split`, including
/// empty vectors. Its hidden slot at `argv[-1]` must remain intact. Public
/// argument pointers may have changed; they do not own individual allocations.
#[no_mangle]
pub unsafe extern "C" fn argv_free(argv: *mut *mut c_char) {
    let owner = argv.wrapping_sub(1);
    // SAFETY: the hidden slot and both allocations are owned by this vector.
    unsafe {
        bindings::kfree(owner.read().cast());
        bindings::kfree(owner.cast());
    }
}

/// Copy the source once and split that copy on the kernel's whitespace bytes.
///
/// Quotes and backslashes have no special meaning. The returned vector is
/// NULL-terminated and must be released with `argv_free`. Allocation failure
/// leaves `argcp` unchanged, and a null source also returns null.
///
/// # Safety
///
/// A nonnull source must satisfy `kstrndup`'s readable-string contract through
/// its terminator or `KMALLOC_MAX_SIZE - 1` bytes. Source changes permitted by
/// that C API never affect the later count or split of the private snapshot.
/// A nonnull `argcp` must be writable and exclusively accessible. The GFP mask
/// must be valid for the caller's allocation context.
#[no_mangle]
pub unsafe extern "C" fn argv_split(
    gfp: gfp_t,
    string: *const c_char,
    argcp: *mut c_int,
) -> *mut *mut c_char {
    // SAFETY: the source and GFP mask satisfy the original C allocation API.
    let snapshot = unsafe {
        bindings::kstrndup(string, bindings::KMALLOC_MAX_SIZE as usize - 1, gfp)
    };
    if snapshot.is_null() {
        return core::ptr::null_mut();
    }

    // SAFETY: kstrndup returned a private, terminated, writable allocation.
    let argc = unsafe { count_argc(snapshot) };
    let slots = argc.wrapping_add(2) as usize;
    let bytes = slots.checked_mul(core::mem::size_of::<*mut c_char>());
    let owner: *mut *mut c_char = match bytes {
        Some(bytes) => {
            // The existing Rust slab helper is the same entry point used by
            // Kmalloc. A null old allocation requests contiguous fresh memory;
            // keep all caller GFP bits and the original array overflow check.
            // No function symbol is invented for the C kmalloc_array macro.
            // Its successful NULL-old path also traces kfree(NULL); this has
            // no ownership effect, but allocation tracing is not identical.
            // SAFETY: null is accepted, and pointer alignment is a power of two.
            unsafe {
                bindings::krealloc_node_align(
                    core::ptr::null(),
                    bytes,
                    core::mem::align_of::<*mut c_char>(),
                    gfp,
                    bindings::NUMA_NO_NODE,
                ).cast()
            }
        }
        None => core::ptr::null_mut(),
    };
    if owner.is_null() {
        // SAFETY: the snapshot is still wholly owned and has not escaped.
        unsafe { bindings::kfree(snapshot.cast()) };
        return core::ptr::null_mut();
    }

    // SAFETY: the checked allocation has argc+2 pointer slots: one hidden
    // string owner, argc arguments and the public NULL terminator. Counting
    // and splitting use the same exclusively owned snapshot and ctype rules.
    unsafe {
        owner.write(snapshot);
        let result = owner.wrapping_add(1);
        let mut output = result;
        let mut cursor = snapshot;
        let mut was_space = true;
        while cursor.read() != 0 {
            if isspace(cursor.read() as c_int) {
                was_space = true;
                cursor.write(0);
            } else if was_space {
                was_space = false;
                output.write(cursor);
                output = output.wrapping_add(1);
            }
            cursor = cursor.wrapping_add(1);
        }
        output.write(core::ptr::null_mut());
        if !argcp.is_null() {
            argcp.write(argc);
        }
        result
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
