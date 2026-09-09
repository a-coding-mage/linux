// SPDX-License-Identifier: GPL-2.0
// The declarations used below are supplied by the surrounding kernel port.

#[cfg(feature = "have_efficient_unaligned_access")]
macro_rules! is_unaligned { ($src:expr, $dst:expr) => { 0 }; }
#[cfg(not(feature = "have_efficient_unaligned_access"))]
macro_rules! is_unaligned {
    ($src:expr, $dst:expr) => {
        (((($dst as isize) | ($src as isize)) & (core::mem::size_of::<c_long>() - 1)) != 0)
    };
}

extern "C" {
    static TASK_SIZE_MAX: usize;
    fn might_fault();
    fn should_fail_usercopy() -> bool;
    fn kasan_check_write(dst: *mut c_char, count: c_long);
    fn check_object_size(dst: *mut c_char, count: c_long, is_source: bool);
    fn can_do_masked_user_access() -> bool;
    fn masked_user_read_access_begin(src: *const c_char) -> *const c_char;
    fn user_read_access_end();
    fn untagged_addr(src: *const c_char) -> *const c_char;
    fn user_read_access_begin(src: *const c_char, max: c_ulong) -> bool;
    fn has_zero(c: c_ulong, data: *mut c_ulong, constants: *const word_at_a_time) -> bool;
    fn prep_zero_mask(c: c_ulong, data: c_ulong, constants: *const word_at_a_time) -> c_ulong;
    fn create_zero_mask(data: c_ulong) -> c_ulong;
    fn zero_bytemask(data: c_ulong) -> c_ulong;
    fn find_zero(data: c_ulong) -> c_ulong;
}

#[repr(C)]
pub struct word_at_a_time {
    _private: [u8; 0],
}

type c_char = i8;
type c_long = isize;
type c_ulong = usize;

const EFAULT: c_long = 14;

// The kernel's unsafe_get_user operation branches to its supplied label on fault.
macro_rules! goto_byte_at_a_time { ($label:lifetime) => { break 'word_loop }; }
macro_rules! unsafe_get_user_failed { () => { false }; }

#[inline(always)]
unsafe fn do_strncpy_from_user(
    dst: *mut c_char,
    src: *const c_char,
    count: c_ulong,
    mut max: c_ulong,
) -> c_long {
    // WORD_AT_A_TIME_CONSTANTS is supplied by asm/word-at-a-time.h.
    let constants: word_at_a_time = unsafe { core::mem::zeroed() };
    let mut res: c_ulong = 0;

    if is_unaligned!(src, dst) {
        goto_byte_at_a_time!(byte_at_a_time);
    }

    'word_loop: while max >= core::mem::size_of::<c_ulong>() {
        let c = (src.add(res) as *const c_ulong).read();
        let mut data: c_ulong = 0;

        /* Fall back to byte-at-a-time if we get a page fault. */
        if unsafe_get_user_failed!() {
            goto_byte_at_a_time!(byte_at_a_time);
        }

        if has_zero(c, &mut data, &constants) {
            data = prep_zero_mask(c, data, &constants);
            data = create_zero_mask(data);
            let mask = zero_bytemask(data);
            (dst.add(res) as *mut c_ulong).write(c & mask);
            return (res + find_zero(data)) as c_long;
        }

        (dst.add(res) as *mut c_ulong).write(c);
        res += core::mem::size_of::<c_ulong>();
        max -= core::mem::size_of::<c_ulong>();
    }

byte_at_a_time:
    while max != 0 {
        let c = (src.add(res) as *const c_char).read();
        (dst.add(res)).write(c);
        if c == 0 {
            return res as c_long;
        }
        res += 1;
        max -= 1;
    }

    if res >= count {
        return res as c_long;
    }

efault:
    -EFAULT
}

#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user(
    dst: *mut c_char,
    mut src: *const c_char,
    count: c_long,
) -> c_long {
    might_fault();
    if should_fail_usercopy() {
        return -EFAULT;
    }
    if count <= 0 {
        return 0;
    }

    kasan_check_write(dst, count);
    check_object_size(dst, count, false);

    if can_do_masked_user_access() {
        src = masked_user_read_access_begin(src);
        let retval = do_strncpy_from_user(dst, src, count as c_ulong, count as c_ulong);
        user_read_access_end();
        return retval;
    }

    let max_addr = TASK_SIZE_MAX;
    let src_addr = untagged_addr(src) as usize;
    if src_addr < max_addr {
        let mut max = max_addr - src_addr;
        if max > count as c_ulong {
            max = count as c_ulong;
        }
        if user_read_access_begin(src, max) {
            let retval = do_strncpy_from_user(dst, src, count as c_ulong, max);
            user_read_access_end();
            return retval;
        }
    }
    -EFAULT
}

// EXPORT_SYMBOL(strncpy_from_user);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
