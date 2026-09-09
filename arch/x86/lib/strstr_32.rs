// SPDX-License-Identifier: GPL-2.0

// Translated from the 32-bit x86 inline-assembly implementation.
// The original includes provide the declaration and export machinery.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strstr(
    cs: *const core::ffi::c_char,
    ct: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    // The assembly first computes the length of the search string, excluding
    // its terminating NUL, then compares that many bytes at each position.
    let mut length = 0usize;
    while *ct.add(length) != 0 {
        length = length.wrapping_add(1);
    }

    let mut current = cs;
    loop {
        let mut index = 0usize;
        while index < length && *current.add(index) == *ct.add(index) {
            index = index.wrapping_add(1);
        }

        if index == length {
            // This also returns cs for an empty search string.
            return current as *mut core::ffi::c_char;
        }

        if *current == 0 {
            return core::ptr::null_mut();
        }
        current = current.add(1);
    }
}

// EXPORT_SYMBOL(strstr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
