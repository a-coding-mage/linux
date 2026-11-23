// SPDX-License-Identifier: GPL-2.0-only

// Dependencies: linux/bitmap.h, linux/ctype.h, linux/errno.h, linux/err.h,
// linux/hex.h, linux/mm.h, linux/string.h, lib/kstrtox.h.

use core::ffi::CStr;
use core::ptr::{addr_of, null, write_bytes};
use kernel::bindings::{
    __bitmap_clear, __bitmap_set, _ctype, _find_next_bit, kasprintf, kfree, loff_t,
    memdup_user_nul, memory_read_from_buffer, strlen, strncasecmp, strnchrnul, EINVAL, ENOMEM, EOVERFLOW,
    ERANGE, GFP_KERNEL,
};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};

extern "C" {
    fn hex_to_bin(ch: u8) -> c_int;
    /// From lib/kstrtox.h, which the generated bindings do not include.
    fn _parse_integer_limit(
        s: *const c_char,
        base: c_uint,
        res: *mut c_ulonglong,
        max_chars: usize,
        init: c_ulonglong,
    ) -> c_uint;
}

/// KSTRTOX_OVERFLOW from lib/kstrtox.h; lib/kstrtox_helpers.c asserts it.
const KSTRTOX_OVERFLOW: c_uint = 1 << 31;
/// `_S` from linux/ctype.h; lib/string_helpers_helpers.c asserts it.
const CTYPE_S: u8 = 0x20;
/// MAX_ERRNO from linux/err.h, for IS_ERR().
const MAX_ERRNO: c_ulong = 4095;
const BITS_PER_LONG: c_uint = c_ulong::BITS;

fn is_err<T>(ptr: *const T) -> bool {
    ptr as c_ulong >= MAX_ERRNO.wrapping_neg()
}

fn err_ptr(error: u32) -> *const c_char {
    (-(error as c_long)) as *const c_char
}

/// PTR_ERR() narrowed to the int returns of this unit.
fn ptr_err<T>(ptr: *const T) -> c_int {
    ptr as c_long as c_int
}

fn isspace(c: u8) -> bool {
    // SAFETY: the C declaration is an incomplete array; the definition has
    // 256 entries indexed by the unsigned char value.
    unsafe { *addr_of!(_ctype).cast::<u8>().add(usize::from(c)) & CTYPE_S != 0 }
}

/// bitmap_zero() for a non-constant size: memset() of bitmap_size(nbits).
///
/// # Safety
/// `dst` covers `nbits` bits rounded up to whole words.
unsafe fn bitmap_zero(dst: *mut c_ulong, nbits: c_uint) {
    let len = (nbits.wrapping_add(BITS_PER_LONG - 1) & !(BITS_PER_LONG - 1)) / 8;

    // SAFETY: forwarded caller contract.
    unsafe { write_bytes(dst.cast::<u8>(), 0, len as usize) };
}

/**
 * bitmap_parse_user - convert an ASCII hex string in a user buffer into a bitmap
 *
 * @ubuf: pointer to user buffer containing string.
 * @ulen: buffer size in bytes.  If string is smaller than this
 *    then it must be terminated with a \0.
 * @maskp: pointer to bitmap array that will contain result.
 * @nmaskbits: size of bitmap, in bits.
 */
/// # Safety
/// `ubuf` is a user pointer checked by memdup_user_nul(); `maskp` covers
/// `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_parse_user(ubuf: *const c_char, ulen: c_uint, maskp: *mut c_ulong, nmaskbits: c_int) -> c_int {
    // SAFETY: the copied string is NUL-terminated and freed here.
    unsafe {
        let buf = memdup_user_nul(ubuf.cast::<c_void>(), ulen as usize).cast::<c_char>();
        if is_err(buf) {
            return ptr_err(buf);
        }

        let ret = bitmap_parse(buf, c_uint::MAX, maskp, nmaskbits);

        kfree(buf.cast::<c_void>());
        ret
    }
}
// EXPORT_SYMBOL(bitmap_parse_user);

/**
 * bitmap_print_to_buf  - convert bitmap to list or hex format ASCII string
 * @list: indicates whether the bitmap must be list
 *      true:  print in decimal list format
 *      false: print in hexadecimal bitmask format
 * @buf: buffer into which string is placed
 * @maskp: pointer to bitmap to convert
 * @nmaskbits: size of bitmap, in bits
 * @off: in the string from which we are copying, We copy to @buf
 * @count: the maximum number of bytes to print
 */
/// # Safety
/// `buf` is writable for `count` bytes and `maskp` covers `nmaskbits` bits.
unsafe fn bitmap_print_to_buf(
    list: bool,
    buf: *mut c_char,
    maskp: *const c_ulong,
    nmaskbits: c_int,
    mut off: loff_t,
    count: usize,
) -> c_int {
    let fmt: &CStr = if list { c"%*pbl\n" } else { c"%*pb\n" };

    // SAFETY: the formatted string is copied out and freed here.
    unsafe {
        let data = kasprintf(GFP_KERNEL, fmt.as_ptr().cast(), nmaskbits, maskp);
        if data.is_null() {
            return -(ENOMEM as c_int);
        }

        let size = memory_read_from_buffer(
            buf.cast::<c_void>(),
            count,
            &mut off,
            data.cast::<c_void>(),
            strlen(data).wrapping_add(1),
        );
        kfree(data.cast::<c_void>());

        size as c_int
    }
}

/**
 * bitmap_print_bitmask_to_buf  - convert bitmap to hex bitmask format ASCII string
 * @buf: buffer into which string is placed
 * @maskp: pointer to bitmap to convert
 * @nmaskbits: size of bitmap, in bits
 * @off: in the string from which we are copying, We copy to @buf
 * @count: the maximum number of bytes to print
 *
 * The bitmap_print_to_pagebuf() is used indirectly via its cpumap wrapper
 * cpumap_print_to_pagebuf() or directly by drivers to export hexadecimal
 * bitmask and decimal list to userspace by sysfs ABI.
 * Drivers might be using a normal attribute for this kind of ABIs. A
 * normal attribute typically has show entry as below::
 *
 *   static ssize_t example_attribute_show(struct device *dev,
 *		struct device_attribute *attr, char *buf)
 *   {
 *	...
 *	return bitmap_print_to_pagebuf(true, buf, &mask, nr_trig_max);
 *   }
 *
 * show entry of attribute has no offset and count parameters and this
 * means the file is limited to one page only.
 * bitmap_print_to_pagebuf() API works terribly well for this kind of
 * normal attribute with buf parameter and without offset, count::
 *
 *   bitmap_print_to_pagebuf(bool list, char *buf, const unsigned long *maskp,
 *			   int nmaskbits)
 *   {
 *	return scnprintf(buf, PAGE_SIZE - offset_in_page(buf), nr_trig_max, &mask);
 *   }
 *
 * The problem is once we have a large bitmap, we have a chance to get a
 * bitmask or list more than one page. Especially for list, it could be
 * as complex as 0,3,5,7,9,... We have no simple way to know it exact size.
 * It turns out bin_attribute is a way to break this limit. bin_attribute
 * has show entry as below::
 *
 *   static ssize_t
 *   example_bin_attribute_show(struct file *filp, struct kobject *kobj,
 *		struct bin_attribute *attr, char *buf,
 *		loff_t offset, size_t count)
 *   {
 *	...
 *   }
 *
 * With the new offset and count parameters, this makes sysfs ABI be able
 * to support file size more than one page. For example, offset could be
 * >= 4096.
 * bitmap_print_bitmask_to_buf(), bitmap_print_list_to_buf() wit their
 * cpumap wrapper cpumap_print_bitmask_to_buf(), cpumap_print_list_to_buf()
 * make those drivers be able to support large bitmask and list after they
 * move to use bin_attribute. In result, we have to pass the corresponding
 * parameters such as off, count from bin_attribute show entry to this API.
 *
 * The role of cpumap_print_bitmask_to_buf() and cpumap_print_list_to_buf()
 * is similar with cpumap_print_to_pagebuf(),  the difference is that
 * bitmap_print_to_pagebuf() mainly serves sysfs attribute with the assumption
 * the destination buffer is exactly one page and won't be more than one page.
 * cpumap_print_bitmask_to_buf() and cpumap_print_list_to_buf(), on the other
 * hand, mainly serves bin_attribute which doesn't work with exact one page,
 * and it can break the size limit of converted decimal list and hexadecimal
 * bitmask.
 *
 * Returns: the number of characters actually printed to @buf
 */
/// # Safety
/// `buf` is writable for `count` bytes and `maskp` covers `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_print_bitmask_to_buf(
    buf: *mut c_char,
    maskp: *const c_ulong,
    nmaskbits: c_int,
    off: loff_t,
    count: usize,
) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe { bitmap_print_to_buf(false, buf, maskp, nmaskbits, off, count) }
}
// EXPORT_SYMBOL(bitmap_print_bitmask_to_buf);

/**
 * bitmap_print_list_to_buf  - convert bitmap to decimal list format ASCII string
 * @buf: buffer into which string is placed
 * @maskp: pointer to bitmap to convert
 * @nmaskbits: size of bitmap, in bits
 * @off: in the string from which we are copying, We copy to @buf
 * @count: the maximum number of bytes to print
 *
 * Everything is same with the above bitmap_print_bitmask_to_buf() except
 * the print format.
 *
 * Returns: the number of characters actually printed to @buf
 */
/// # Safety
/// `buf` is writable for `count` bytes and `maskp` covers `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_print_list_to_buf(
    buf: *mut c_char,
    maskp: *const c_ulong,
    nmaskbits: c_int,
    off: loff_t,
    count: usize,
) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe { bitmap_print_to_buf(true, buf, maskp, nmaskbits, off, count) }
}
// EXPORT_SYMBOL(bitmap_print_list_to_buf);

/*
 * Region 9-38:4/10 describes the following bitmap structure:
 * 0	   9  12    18			38	     N
 * .........****......****......****..................
 *	    ^  ^     ^			 ^	     ^
 *      start  off   group_len	       end	 nbits
 */
/// `struct region`, private to the original translation unit.
#[derive(Default)]
struct Region {
    start: c_uint,
    off: c_uint,
    group_len: c_uint,
    end: c_uint,
    nbits: c_uint,
}

/// # Safety
/// `bitmap` covers `r.nbits` bits.
unsafe fn bitmap_set_region(r: &Region, bitmap: *mut c_ulong) {
    let mut start = r.start;

    while start <= r.end {
        // bitmap_set() for a non-constant length: __bitmap_set(), whose
        // length is an int.
        let len = core::cmp::min(r.end.wrapping_sub(start).wrapping_add(1), r.off);
        // SAFETY: bitmap_check_region() bounded the region by nbits.
        unsafe { __bitmap_set(bitmap, start, len as c_int) };
        start = start.wrapping_add(r.group_len);
    }
}

fn bitmap_check_region(r: &Region) -> c_int {
    if r.start > r.end || r.group_len == 0 || r.off > r.group_len {
        return -(EINVAL as c_int);
    }

    if r.end >= r.nbits {
        return -(ERANGE as c_int);
    }

    0
}

/// # Safety
/// `str` is a NUL-terminated string.
unsafe fn bitmap_getnum(str: *const c_char, num: &mut c_uint, lastbit: c_uint) -> *const c_char {
    let mut n: c_ulonglong = 0;

    // SAFETY: the parse stops at the first non-digit.
    unsafe {
        if *str == b'N' {
            *num = lastbit;
            return str.add(1);
        }

        // _parse_integer(str, 10, &n)
        let len = _parse_integer_limit(str, 10, &mut n, c_int::MAX as usize, 0);
        if len == 0 {
            return err_ptr(EINVAL);
        }
        if len & KSTRTOX_OVERFLOW != 0 || n != c_ulonglong::from(n as c_uint) {
            return err_ptr(EOVERFLOW);
        }

        *num = n as c_uint;
        str.add(len as usize)
    }
}

fn end_of_str(c: c_char) -> bool {
    c == 0 || c == b'\n'
}

fn __end_of_region(c: c_char) -> bool {
    isspace(c) || c == b','
}

fn end_of_region(c: c_char) -> bool {
    __end_of_region(c) || end_of_str(c)
}

/*
 * The format allows commas and whitespaces at the beginning
 * of the region.
 */
/// # Safety
/// `str` is a NUL-terminated string.
unsafe fn bitmap_find_region(mut str: *const c_char) -> *const c_char {
    // SAFETY: the scan stops at the terminator, which is not a separator.
    unsafe {
        while __end_of_region(*str) {
            str = str.add(1);
        }

        if end_of_str(*str) {
            null()
        } else {
            str
        }
    }
}

/// # Safety
/// `start..=end` lies in one string.
unsafe fn bitmap_find_region_reverse(start: *const c_char, mut end: *const c_char) -> *const c_char {
    // SAFETY: *end is only read while it is not before start.
    unsafe {
        while start <= end && __end_of_region(*end) {
            end = end.wrapping_sub(1);
        }
    }

    end
}

/// # Safety
/// `str` is a NUL-terminated string.
unsafe fn bitmap_parse_region(mut str: *const c_char, r: &mut Region) -> *const c_char {
    let lastbit = r.nbits.wrapping_sub(1);

    // SAFETY: every read stays within the NUL-terminated region string.
    unsafe {
        let no_end = if strncasecmp(str, c"all".as_ptr().cast(), 3) == 0 {
            r.start = 0;
            r.end = lastbit;
            str = str.add(3);

            false // goto check_pattern
        } else {
            str = bitmap_getnum(str, &mut r.start, lastbit);
            if is_err(str) {
                return str;
            }

            if end_of_region(*str) {
                true // goto no_end
            } else {
                if *str != b'-' {
                    return err_ptr(EINVAL);
                }

                str = bitmap_getnum(str.add(1), &mut r.end, lastbit);
                if is_err(str) {
                    return str;
                }
                false
            }
        };

        if no_end {
            // no_end:
            r.end = r.start;
        } else {
            // check_pattern:
            if !end_of_region(*str) {
                if *str != b':' {
                    return err_ptr(EINVAL);
                }

                str = bitmap_getnum(str.add(1), &mut r.off, lastbit);
                if is_err(str) {
                    return str;
                }

                if *str != b'/' {
                    return err_ptr(EINVAL);
                }

                return bitmap_getnum(str.add(1), &mut r.group_len, lastbit);
            }
        }

        // no_pattern:
        r.off = r.end.wrapping_add(1);
        r.group_len = r.end.wrapping_add(1);

        if end_of_str(*str) {
            null()
        } else {
            str
        }
    }
}

/**
 * bitmap_parselist - convert list format ASCII string to bitmap
 * @buf: read user string from this buffer; must be terminated
 *    with a \0 or \n.
 * @maskp: write resulting mask here
 * @nmaskbits: number of bits in mask to be written
 *
 * Input format is a comma-separated list of decimal numbers and
 * ranges.  Consecutively set bits are shown as two hyphen-separated
 * decimal numbers, the smallest and largest bit numbers set in
 * the range.
 * Optionally each range can be postfixed to denote that only parts of it
 * should be set. The range will divided to groups of specific size.
 * From each group will be used only defined amount of bits.
 * Syntax: range:used_size/group_size
 * Example: 0-1023:2/256 ==> 0,1,256,257,512,513,768,769
 * The value 'N' can be used as a dynamically substituted token for the
 * maximum allowed value; i.e (nmaskbits - 1).  Keep in mind that it is
 * dynamic, so if system changes cause the bitmap width to change, such
 * as more cores in a CPU list, then any ranges using N will also change.
 *
 * Returns: 0 on success, -errno on invalid input strings. Error values:
 *
 *   - ``-EINVAL``: wrong region format
 *   - ``-EINVAL``: invalid character in string
 *   - ``-ERANGE``: bit number specified too large for mask
 *   - ``-EOVERFLOW``: integer overflow in the input parameters
 */
/// # Safety
/// `buf` is NUL- or newline-terminated and `maskp` covers `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_parselist(mut buf: *const c_char, maskp: *mut c_ulong, nmaskbits: c_int) -> c_int {
    let mut r = Region {
        nbits: nmaskbits as c_uint,
        ..Region::default()
    };

    // SAFETY: parsing stays within the terminated buffer; regions are
    // checked against nbits before bits are set.
    unsafe {
        bitmap_zero(maskp, r.nbits);

        while !buf.is_null() {
            buf = bitmap_find_region(buf);
            if buf.is_null() {
                return 0;
            }

            buf = bitmap_parse_region(buf, &mut r);
            if is_err(buf) {
                return ptr_err(buf);
            }

            let ret = bitmap_check_region(&r);
            if ret != 0 {
                return ret;
            }

            bitmap_set_region(&r, maskp);
        }
    }

    0
}
// EXPORT_SYMBOL(bitmap_parselist);

/**
 * bitmap_parselist_user() - convert user buffer's list format ASCII
 * string to bitmap
 *
 * @ubuf: pointer to user buffer containing string.
 * @ulen: buffer size in bytes.  If string is smaller than this
 *    then it must be terminated with a \0.
 * @maskp: pointer to bitmap array that will contain result.
 * @nmaskbits: size of bitmap, in bits.
 *
 * Wrapper for bitmap_parselist(), providing it with user buffer.
 */
/// # Safety
/// `ubuf` is a user pointer checked by memdup_user_nul(); `maskp` covers
/// `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_parselist_user(
    ubuf: *const c_char,
    ulen: c_uint,
    maskp: *mut c_ulong,
    nmaskbits: c_int,
) -> c_int {
    // SAFETY: the copied string is NUL-terminated and freed here.
    unsafe {
        let buf = memdup_user_nul(ubuf.cast::<c_void>(), ulen as usize).cast::<c_char>();
        if is_err(buf) {
            return ptr_err(buf);
        }

        let ret = bitmap_parselist(buf, maskp, nmaskbits);

        kfree(buf.cast::<c_void>());
        ret
    }
}
// EXPORT_SYMBOL(bitmap_parselist_user);

/// # Safety
/// `start..=end` lies in one string.
unsafe fn bitmap_get_x32_reverse(start: *const c_char, mut end: *const c_char, num: *mut u32) -> *const c_char {
    let mut ret: u32 = 0;

    // SAFETY: *end is only read while it is not before start.
    unsafe {
        'out: {
            let mut i: c_int = 0;
            while i < 32 {
                let c = hex_to_bin(*end);
                end = end.wrapping_sub(1);
                if c < 0 {
                    return err_ptr(EINVAL);
                }

                ret |= (c as u32) << i;

                if start > end || __end_of_region(*end) {
                    break 'out;
                }
                i += 4;
            }

            let c = hex_to_bin(*end);
            end = end.wrapping_sub(1);
            if c >= 0 {
                return err_ptr(EOVERFLOW);
            }
        }
        // out:
        *num = ret;
    }
    end
}

/**
 * bitmap_parse - convert an ASCII hex string into a bitmap.
 * @start: pointer to buffer containing string.
 * @buflen: buffer size in bytes.  If string is smaller than this
 *    then it must be terminated with a \0 or \n. In that case,
 *    UINT_MAX may be provided instead of string length.
 * @maskp: pointer to bitmap array that will contain result.
 * @nmaskbits: size of bitmap, in bits.
 *
 * Commas group hex digits into chunks.  Each chunk defines exactly 32
 * bits of the resultant bitmask.  No chunk may specify a value larger
 * than 32 bits (%-EOVERFLOW), and if a chunk specifies a smaller value
 * then leading 0-bits are prepended.  %-EINVAL is returned for illegal
 * characters. Grouping such as "1,,5", ",44", "," or "" is allowed.
 * Leading, embedded and trailing whitespace accepted.
 */
/// # Safety
/// `start` is terminated within `buflen` bytes and `maskp` covers
/// `nmaskbits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_parse(start: *const c_char, buflen: c_uint, maskp: *mut c_ulong, nmaskbits: c_int) -> c_int {
    // BITS_TO_U32(nmaskbits): size_t arithmetic on the sign-extended int.
    let bits_to_u32 = (nmaskbits as isize as usize).wrapping_add(31) / 32;
    let mut chunks = bits_to_u32 as c_int;
    let bitmap = maskp.cast::<u32>();

    // SAFETY: chunks are written within BITS_TO_U32(nmaskbits) u32 words of
    // maskp; parsing stays inside the terminated buffer.
    unsafe {
        let mut end: *const c_char = strnchrnul(start, buflen as usize, c_int::from(b'\n')).cast_const().wrapping_sub(1);

        let mut chunk: c_int = 0;
        loop {
            end = bitmap_find_region_reverse(start, end);
            if start > end {
                break;
            }

            let remaining = chunks;
            chunks = chunks.wrapping_sub(1);
            if remaining == 0 {
                return -(EOVERFLOW as c_int);
            }

            #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
            let slot = bitmap.offset((chunk ^ 1) as isize);
            #[cfg(not(all(target_pointer_width = "64", target_endian = "big")))]
            let slot = bitmap.offset(chunk as isize);
            end = bitmap_get_x32_reverse(start, end, slot);
            if is_err(end) {
                return ptr_err(end);
            }
            chunk = chunk.wrapping_add(1);
        }

        let unset_bit = (bits_to_u32.wrapping_sub(chunks as isize as usize).wrapping_mul(32)) as c_int;
        if unset_bit < nmaskbits {
            // bitmap_clear() for a non-constant length: __bitmap_clear().
            __bitmap_clear(maskp, unset_bit as c_uint, nmaskbits.wrapping_sub(unset_bit));
            return 0;
        }

        // find_next_bit() for a non-constant size.
        if _find_next_bit(maskp, unset_bit as isize as c_ulong, nmaskbits as isize as c_ulong)
            != unset_bit as isize as c_ulong
        {
            return -(EOVERFLOW as c_int);
        }
    }

    0
}
// EXPORT_SYMBOL(bitmap_parse);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
