// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the Linux kernel headers are supplied externally.

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
unsafe fn do_strnlen_user(src: *const core::ffi::c_char, count: usize, mut max: usize) -> isize {
    let constants: word_at_a_time = WORD_AT_A_TIME_CONSTANTS;
    let mut align: usize;
    let mut res: usize = 0;
    let mut c: usize;

    /*
     * Do everything aligned. But that means that we
     * need to also expand the maximum..
     */
    align = (core::mem::size_of::<usize>() - 1) & (src as usize);
    let mut src = src.sub(align);
    max += align;

    unsafe_get_user!(c, src as *const usize, efault);
    c |= aligned_byte_mask(align);

    loop {
        let mut data: usize;
        if has_zero(c, &mut data, &constants) {
            data = prep_zero_mask(c, data, &constants);
            data = create_zero_mask(data);
            return (res + find_zero(data) + 1 - align) as isize;
        }
        res += core::mem::size_of::<usize>();
        /* We already handled 'unsigned long' bytes. Did we do it all ? */
        if unlikely(max <= core::mem::size_of::<usize>()) {
            break;
        }
        max -= core::mem::size_of::<usize>();
        unsafe_get_user!(c, src.add(res) as *const usize, efault);
    }
    res -= align;

    /*
     * Uhhuh. We hit 'max'. But was that the user-specified maximum
     * too? If so, return the marker for "too long".
     */
    if res >= count {
        return (count + 1) as isize;
    }

    /*
     * Nope: we hit the address space limit, and we still had more
     * characters the caller would have wanted. That's 0.
     */
efault:
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
pub unsafe fn strnlen_user(str_: *const core::ffi::c_char, count: isize) -> isize {
    let max_addr: usize;
    let src_addr: usize;

    if unlikely(count <= 0) {
        return 0;
    }

    if can_do_masked_user_access() {
        let mut retval: isize;

        let str_ = masked_user_read_access_begin(str_);
        retval = do_strnlen_user(str_, count as usize, count as usize);
        user_read_access_end();
        return retval;
    }

    max_addr = TASK_SIZE_MAX;
    src_addr = untagged_addr(str_) as usize;
    if likely(src_addr < max_addr) {
        let mut max = max_addr - src_addr;
        let mut retval: isize;

        /*
         * Truncate 'max' to the user-specified limit, so that we
         * only have one limit we need to check in the loop
         */
        if max > count as usize {
            max = count as usize;
        }

        if user_read_access_begin(str_, max) {
            retval = do_strnlen_user(str_, count as usize, max);
            user_read_access_end();
            return retval;
        }
    }
    0
}

// EXPORT_SYMBOL(strnlen_user);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
