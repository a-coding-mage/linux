// SPDX-License-Identifier: GPL-2.0
// Dependencies: linux/compiler.h, linux/export.h, linux/fault-inject-usercopy.h,
// linux/kasan-checks.h, linux/thread_info.h, linux/uaccess.h, linux/kernel.h,
// linux/errno.h, linux/mm.h, asm/byteorder.h, asm/word-at-a-time.h.

use core::ffi::{c_char, c_long, c_ulong};
use core::mem::size_of;

#[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
#[inline(always)]
fn IS_UNALIGNED(_src: *const c_char, _dst: *mut c_char) -> bool {
    false
}

#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
#[inline(always)]
fn IS_UNALIGNED(src: *const c_char, dst: *mut c_char) -> bool {
    ((dst as c_long | src as c_long) & (size_of::<c_long>() as c_long - 1)) != 0
}

/*
 * Do a strncpy, return length of string without final '\0'.
 * 'count' is the user-supplied count (return 'count' if we
 * hit it), 'max' is the address space maximum (and we return
 * -EFAULT if we hit it).
 */
#[inline(always)]
unsafe fn do_strncpy_from_user(
    dst: *mut c_char,
    src: *const c_char,
    count: c_ulong,
    mut max: c_ulong,
) -> c_long {
    let constants: word_at_a_time = WORD_AT_A_TIME_CONSTANTS;
    let mut res: c_ulong = 0;
    const WORD: c_ulong = size_of::<c_ulong>() as c_ulong;

    'efault: {
        'byte_at_a_time: {
            if IS_UNALIGNED(src, dst) {
                break 'byte_at_a_time;
            }

            while max >= WORD {
                let c: c_ulong;
                let mut data: c_ulong = 0;

                /* Fall back to byte-at-a-time if we get a page fault */
                unsafe_get_user!(c, src.wrapping_add(res as usize) as *const c_ulong, 'byte_at_a_time);

                /*
                 * Note that we mask out the bytes following the NUL. This is
                 * important to do because string oblivious code may read past
                 * the NUL. For those routines, we don't want to give them
                 * potentially random bytes after the NUL in `src`.
                 *
                 * One example of such code is BPF map keys. BPF treats map keys
                 * as an opaque set of bytes. Without the post-NUL mask, any BPF
                 * maps keyed by strings returned from strncpy_from_user() may
                 * have multiple entries for semantically identical strings.
                 */
                if has_zero(c, &mut data, &constants) != 0 {
                    data = prep_zero_mask(c, data, &constants);
                    data = create_zero_mask(data);
                    let mask: c_ulong = zero_bytemask(data);
                    dst.add(res as usize).cast::<c_ulong>().write_unaligned(c & mask);
                    return (res + find_zero(data) as c_ulong) as c_long;
                }

                dst.add(res as usize).cast::<c_ulong>().write_unaligned(c);

                res += WORD;
                max -= WORD;
            }
        }
        // byte_at_a_time:
        while max != 0 {
            let c: c_char;

            unsafe_get_user!(c, src.wrapping_add(res as usize), 'efault);
            *dst.add(res as usize) = c;
            if c == 0 {
                return res as c_long;
            }
            res += 1;
            max -= 1;
        }

        /*
         * Uhhuh. We hit 'max'. But was that the user-specified maximum
         * too? If so, that's ok - we got as much as the user asked for.
         */
        if res >= count {
            return res as c_long;
        }
    }

    /*
     * Nope: we hit the address space limit, and we still had more
     * characters the caller would have wanted. That's an EFAULT.
     */
    // efault:
    -(EFAULT as c_long)
}

/**
 * strncpy_from_user: - Copy a NUL terminated string from userspace.
 * @dst:   Destination address, in kernel space.  This buffer must be at
 *         least @count bytes long.
 * @src:   Source address, in user space.
 * @count: Maximum number of bytes to copy, including the trailing NUL.
 *
 * Copies a NUL-terminated string from userspace to kernel space.
 *
 * On success, returns the length of the string (not including the trailing
 * NUL).
 *
 * If access to userspace fails, returns -EFAULT (some data may have been
 * copied).
 *
 * If @count is smaller than the length of the string, copies @count bytes
 * and returns @count.
 */
#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user(dst: *mut c_char, mut src: *const c_char, count: c_long) -> c_long {
    might_fault();
    if should_fail_usercopy() {
        return -(EFAULT as c_long);
    }
    if unlikely(count <= 0) {
        return 0;
    }

    kasan_check_write(dst.cast(), count as _);
    check_object_size(dst.cast(), count as c_ulong, false);

    if can_do_masked_user_access() {
        src = masked_user_read_access_begin(src);
        let retval = do_strncpy_from_user(dst, src, count as c_ulong, count as c_ulong);
        user_read_access_end();
        return retval;
    }

    let max_addr: c_ulong = TASK_SIZE_MAX;
    let src_addr: c_ulong = untagged_addr(src) as c_ulong;
    if likely(src_addr < max_addr) {
        let mut max: c_ulong = max_addr - src_addr;

        /*
         * Truncate 'max' to the user-specified limit, so that
         * we only have one limit we need to check in the loop
         */
        if max > count as c_ulong {
            max = count as c_ulong;
        }

        if user_read_access_begin(src, max) {
            let retval = do_strncpy_from_user(dst, src, count as c_ulong, max);
            user_read_access_end();
            return retval;
        }
    }
    -(EFAULT as c_long)
}
// EXPORT_SYMBOL(strncpy_from_user);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
