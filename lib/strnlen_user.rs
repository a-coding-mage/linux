// SPDX-License-Identifier: GPL-2.0
// Dependencies: linux/kernel.h, linux/export.h, linux/uaccess.h, linux/mm.h,
// linux/bitops.h, asm/word-at-a-time.h.

use core::ffi::{c_char, c_long, c_ulong};
use core::mem::size_of;

/*
 * Do a strnlen, return length of string *with* final '\0'.
 * 'count' is the user-supplied count, while 'max' is the
 * address space maximum.
 *
 * Return 0 for exceptions (which includes hitting the address
 * space maximum), or 'count+1' if hitting the user-supplied
 * maximum count.
 *
 * NOTE! We can sometimes overshoot the user-supplied maximum
 * if it fits in a aligned 'long'. The caller needs to check
 * the return value against "> max".
 */
#[inline(always)]
unsafe fn do_strnlen_user(mut src: *const c_char, count: c_ulong, mut max: c_ulong) -> c_long {
    let constants: word_at_a_time = WORD_AT_A_TIME_CONSTANTS;
    let mut res: c_ulong = 0;
    let mut c: c_ulong;

    /*
     * Do everything aligned. But that means that we
     * need to also expand the maximum..
     */
    let align: c_ulong = (size_of::<c_ulong>() as c_ulong - 1) & (src as c_ulong);
    src = src.wrapping_sub(align as usize);
    max += align;

    'efault: {
        unsafe_get_user!(c, src as *const c_ulong, 'efault);
        c |= aligned_byte_mask(align);

        loop {
            let mut data: c_ulong = 0;
            if has_zero(c, &mut data, &constants) != 0 {
                data = prep_zero_mask(c, data, &constants);
                data = create_zero_mask(data);
                return (res + find_zero(data) as c_ulong + 1 - align) as c_long;
            }
            res += size_of::<c_ulong>() as c_ulong;
            /* We already handled 'unsigned long' bytes. Did we do it all ? */
            if unlikely(max <= size_of::<c_ulong>() as c_ulong) {
                break;
            }
            max -= size_of::<c_ulong>() as c_ulong;
            unsafe_get_user!(c, src.wrapping_add(res as usize) as *const c_ulong, 'efault);
        }
        res -= align;

        /*
         * Uhhuh. We hit 'max'. But was that the user-specified maximum
         * too? If so, return the marker for "too long".
         */
        if res >= count {
            return (count + 1) as c_long;
        }
    }

    /*
     * Nope: we hit the address space limit, and we still had more
     * characters the caller would have wanted. That's 0.
     */
    // efault:
    0
}

/**
 * strnlen_user: - Get the size of a user string INCLUDING final NUL.
 * @str: The string to measure.
 * @count: Maximum count (including NUL character)
 *
 * Context: User context only. This function may sleep if pagefaults are
 *          enabled.
 *
 * Get the size of a NUL-terminated string in user space.
 *
 * Returns the size of the string INCLUDING the terminating NUL.
 * If the string is too long, returns a number larger than @count. User
 * has to check the return value against "> count".
 * On exception (or invalid count), returns 0.
 *
 * NOTE! You should basically never use this function. There is
 * almost never any valid case for using the length of a user space
 * string, since the string can be changed at any time by other
 * threads. Use "strncpy_from_user()" instead to get a stable copy
 * of the string.
 */
#[no_mangle]
pub unsafe extern "C" fn strnlen_user(mut str: *const c_char, count: c_long) -> c_long {
    if unlikely(count <= 0) {
        return 0;
    }

    if can_do_masked_user_access() {
        str = masked_user_read_access_begin(str);
        let retval = do_strnlen_user(str, count as c_ulong, count as c_ulong);
        user_read_access_end();
        return retval;
    }

    let max_addr: c_ulong = TASK_SIZE_MAX;
    let src_addr: c_ulong = untagged_addr(str) as c_ulong;
    if likely(src_addr < max_addr) {
        let mut max: c_ulong = max_addr - src_addr;

        /*
         * Truncate 'max' to the user-specified limit, so that
         * we only have one limit we need to check in the loop
         */
        if max > count as c_ulong {
            max = count as c_ulong;
        }

        if user_read_access_begin(str, max) {
            let retval = do_strnlen_user(str, count as c_ulong, max);
            user_read_access_end();
            return retval;
        }
    }
    0
}
// EXPORT_SYMBOL(strnlen_user);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
