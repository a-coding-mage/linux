/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * stdlib function definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* C dependencies removed: nolibc.h, std.h, arch.h, types.h, sys.h,
 * string.h, and linux/auxvec.h supply the external symbols referenced here.
 */

use core::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct nolibc_heap {
	pub len: size_t,
	/* flexible array member: char user_p[] __attribute__((__aligned__)) */
}

/* Buffer used to store int-to-ASCII conversions. Will only be implemented if
 * any of the related functions is implemented. The area is large enough to
 * store "18446744073709551615" or "-9223372036854775808" and the final zero.
 */
pub static mut itoa_buffer: [c_char; 21] = [0; 21];

/*
 * As much as possible, please keep functions alphabetically sorted.
 */

#[inline]
pub unsafe fn abs(j: c_int) -> c_int {
	if j >= 0 { j } else { j.wrapping_neg() }
}

#[inline]
pub unsafe fn labs(j: c_long) -> c_long {
	if j >= 0 { j } else { j.wrapping_neg() }
}

#[inline]
pub unsafe fn llabs(j: c_longlong) -> c_longlong {
	if j >= 0 { j } else { j.wrapping_neg() }
}

/* must be exported, as it's used by libgcc for various divide functions */
#[no_mangle]
#[link_section = ".text.nolibc_abort"]
pub unsafe extern "C" fn abort() -> ! {
	_sys_kill(_sys_getpid(), SIGABRT);
	loop {}
}

pub unsafe fn atol(mut s: *const c_char) -> c_long {
	let mut ret: c_ulong = 0;
	let mut d: c_ulong;
	let mut neg: c_int = 0;

	if *s == b'-' as c_char {
		neg = 1;
		s = s.add(1);
	}

	loop {
		d = ((*s) as c_ulong).wrapping_sub(b'0' as c_ulong);
		s = s.add(1);
		if d > 9 {
			break;
		}
		ret = ret.wrapping_mul(10);
		ret = ret.wrapping_add(d);
	}

	if neg != 0 { ret.wrapping_neg() as c_long } else { ret as c_long }
}

pub unsafe fn atoi(s: *const c_char) -> c_int {
	atol(s) as c_int
}

pub unsafe fn free(ptr_: *mut c_void) {
	let heap: *mut nolibc_heap;

	if ptr_.is_null() {
		return;
	}

	heap = (ptr_ as *mut u8).sub(size_of::<nolibc_heap>()) as *mut nolibc_heap;
	munmap(heap as *mut c_void, (*heap).len);
}

/* Original C condition: #ifndef NOLIBC_NO_RUNTIME.
 * getenv() tries to find the environment variable named <name> in the
 * environment array pointed to by global variable "environ" which must be
 * declared as a char **, and must be terminated by a NULL (it is recommended
 * to set this variable to the "envp" argument of main()). If the requested
 * environment variable exists its value is returned otherwise NULL is
 * returned.
 */
#[cfg(not(NOLIBC_NO_RUNTIME))]
pub unsafe fn getenv(name: *const c_char) -> *mut c_char {
	let mut idx: c_int;
	let mut i: c_int;

	if !environ.is_null() {
		idx = 0;
		while !(*environ.add(idx as usize)).is_null() {
			i = 0;
			while *name.add(i as usize) != 0 &&
			      *name.add(i as usize) == *(*environ.add(idx as usize)).add(i as usize) {
				i += 1;
			}
			if *name.add(i as usize) == 0 &&
			   *(*environ.add(idx as usize)).add(i as usize) == b'=' as c_char {
				return (*environ.add(idx as usize)).add((i + 1) as usize);
			}
			idx += 1;
		}
	}
	ptr::null_mut()
}

pub unsafe fn malloc(mut len: size_t) -> *mut c_void {
	let heap: *mut nolibc_heap;

	/* Always allocate memory with size multiple of 4096. */
	len = size_of::<nolibc_heap>().wrapping_add(len);
	len = (len.wrapping_add(4095usize)) & (0usize.wrapping_sub(4096usize));
	heap = mmap(ptr::null_mut(), len, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0)
		as *mut nolibc_heap;
	if heap == MAP_FAILED as *mut nolibc_heap {
		return ptr::null_mut();
	}

	(*heap).len = len;
	(heap as *mut u8).add(size_of::<nolibc_heap>()) as *mut c_void
}

pub unsafe fn calloc(size: size_t, nmemb: size_t) -> *mut c_void {
	let Some(x) = size.checked_mul(nmemb) else {
		SET_ERRNO(ENOMEM);
		return ptr::null_mut();
	};

	/*
	 * No need to zero the heap, the MAP_ANONYMOUS in malloc()
	 * already does it.
	 */
	malloc(x)
}

pub unsafe fn realloc(old_ptr: *mut c_void, new_size: size_t) -> *mut c_void {
	let heap: *mut nolibc_heap;
	let user_p_len: size_t;
	let ret: *mut c_void;

	if old_ptr.is_null() {
		return malloc(new_size);
	}

	heap = (old_ptr as *mut u8).sub(size_of::<nolibc_heap>()) as *mut nolibc_heap;
	user_p_len = (*heap).len.wrapping_sub(size_of::<nolibc_heap>());
	/*
	 * Don't realloc() if @user_p_len >= @new_size, this block of
	 * memory is still enough to handle the @new_size. Just return
	 * the same pointer.
	 */
	if user_p_len >= new_size {
		return old_ptr;
	}

	ret = malloc(new_size);
	if ret.is_null() {
		return ptr::null_mut();
	}

	memcpy(ret, (heap as *mut u8).add(size_of::<nolibc_heap>()) as *const c_void, user_p_len);
	munmap(heap as *mut c_void, (*heap).len);
	ret
}

/* Converts the unsigned 64bit integer <in> to base <base> ascii into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero. The buffer is filled from the first byte, and the number
 * of characters emitted (not counting the trailing zero) is returned.
 * The function uses 'multiply by reciprocal' for the divisions and
 * requires the caller pass the correct reciprocal.
 *
 * Note that unlike __div64_const32() in asm-generic/div64.h there isn't
 * an extra shift done (by ___p), the reciprocal has to be lower resulting
 * in a slightly low quotient.
 * Keep things simple by correcting for the error.
 * This also saves calculating the 'low * low' product (e2 below) which is
 * very unlikely to be significant.
 *
 * Some maths:
 *	recip = p2 / base - e1;		// With e1 < base.
 *	q = (recip * in - e2) / p2;	// With e2 < p2.
 *        = base / in - (e1 * in + e2) / p2;
 *        > base / in - (e1 * p2 + p2) / p2;
 *        = base / in - ((e1 + 1) * p2) / p2;
 *        > base / in - base;
 * So the maximum error is less than 'base'.
 * Hence the largest possible digit is '2 * base - 1'.
 * For base 10 e1 is 6 and you can get digits of 15 (eg from 2**64-1).
 * Error e1 is largest for a base that is a factor of 2**64+1, the smallest is 274177
 * and converting 2**42-1 in base 274177 does generate a digit of 274177+274175.
 * This all means only a single correction is needed rather than a loop.
 *
 * __int128 isn't used for mips because gcc prior to 10.0 will call
 * __multi3 for MIPS64r6. The same also happens for SPARC and clang.
 */
pub const fn _NOLIBC_U64TOA_RECIP(base: u64) -> u64 {
	if (base & 1) != 0 { !0u64 / base } else { (1u64 << 63) / (base / 2) }
}

#[inline(never)]
pub unsafe fn _nolibc_u64toa_base(mut in_: uint64_t, mut buffer: *mut c_char, base: c_uint, recip: uint64_t) -> c_int {
	let mut digits: c_uint = 0;
	let mut dig: c_uint;
	let mut q: uint64_t;
	let mut p: *mut c_char;

	/* Generate least significant digit first.
	 * Original C has a non-__int128 fallback for mips/sparc; Rust uses u128
	 * here as the direct equivalent of the primary C branch.
	 */
	loop {
		q = (((in_ as u128).wrapping_mul(recip as u128)) >> 64) as uint64_t;

		dig = in_.wrapping_sub(q.wrapping_mul(base as uint64_t)) as c_uint;
		/* Correct for any rounding errors */
		if dig >= base {
			dig = dig.wrapping_sub(base);
			q = q.wrapping_add(1);
		}
		if dig > 9 {
			dig = dig.wrapping_add((b'a' - b'0' - 10) as c_uint);
		}
		*buffer.add(digits as usize) = (b'0' as c_uint).wrapping_add(dig) as c_char;
		digits = digits.wrapping_add(1);

		in_ = q;
		if in_ == 0 {
			break;
		}
	}

	*buffer.add(digits as usize) = 0;

	/* Order reverse to result */
	p = buffer.add(digits as usize).sub(1);
	while p > buffer {
		dig = *buffer as c_uint;
		*buffer = *p;
		*p = dig as c_char;
		buffer = buffer.add(1);
		p = p.sub(1);
	}

	digits as c_int
}

/* Converts the unsigned long integer <in> to its hex representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (17 bytes for "ffffffffffffffff" or 9 for "ffffffff"). The
 * buffer is filled from the first byte, and the number of characters emitted
 * (not counting the trailing zero) is returned.
 */
#[inline]
pub unsafe fn utoh_r(in_: c_ulong, buffer: *mut c_char) -> c_int {
	_nolibc_u64toa_base(in_ as uint64_t, buffer, 16, _NOLIBC_U64TOA_RECIP(16))
}

/* converts unsigned long <in> to an hex string using the static itoa_buffer
 * and returns the pointer to that string.
 */
#[inline]
pub unsafe fn utoh(in_: c_ulong) -> *mut c_char {
	utoh_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* Converts the unsigned long integer <in> to its string representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (21 bytes for 18446744073709551615 in 64-bit, 11 for
 * 4294967295 in 32-bit). The buffer is filled from the first byte, and the
 * number of characters emitted (not counting the trailing zero) is returned.
 */
#[inline]
pub unsafe fn utoa_r(in_: c_ulong, buffer: *mut c_char) -> c_int {
	_nolibc_u64toa_base(in_ as uint64_t, buffer, 10, _NOLIBC_U64TOA_RECIP(10))
}

/* Converts the signed long integer <in> to its string representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (21 bytes for -9223372036854775808 in 64-bit, 12 for
 * -2147483648 in 32-bit). The buffer is filled from the first byte, and the
 * number of characters emitted (not counting the trailing zero) is returned.
 */
pub unsafe fn itoa_r(mut in_: c_long, buffer: *mut c_char) -> c_int {
	let mut ptr_ = buffer;
	let mut len: c_int = 0;

	if in_ < 0 {
		in_ = (in_ as c_ulong).wrapping_neg() as c_long;
		*ptr_ = b'-' as c_char;
		ptr_ = ptr_.add(1);
		len += 1;
	}
	len += utoa_r(in_ as c_ulong, ptr_);
	len
}

/* for historical compatibility, same as above but returns the pointer to the
 * buffer.
 */
#[inline]
pub unsafe fn ltoa_r(in_: c_long, buffer: *mut c_char) -> *mut c_char {
	itoa_r(in_, buffer);
	buffer
}

/* converts long integer <in> to a string using the static itoa_buffer and
 * returns the pointer to that string.
 */
#[inline]
pub unsafe fn itoa(in_: c_long) -> *mut c_char {
	itoa_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* converts long integer <in> to a string using the static itoa_buffer and
 * returns the pointer to that string. Same as above, for compatibility.
 */
#[inline]
pub unsafe fn ltoa(in_: c_long) -> *mut c_char {
	itoa_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* converts unsigned long integer <in> to a string using the static itoa_buffer
 * and returns the pointer to that string.
 */
#[inline]
pub unsafe fn utoa(in_: c_ulong) -> *mut c_char {
	utoa_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* Converts the unsigned 64-bit integer <in> to its hex representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (17 bytes for "ffffffffffffffff"). The buffer is filled from
 * the first byte, and the number of characters emitted (not counting the
 * trailing zero) is returned.
 */
#[inline]
pub unsafe fn u64toh_r(in_: uint64_t, buffer: *mut c_char) -> c_int {
	_nolibc_u64toa_base(in_, buffer, 16, _NOLIBC_U64TOA_RECIP(16))
}

/* converts uint64_t <in> to an hex string using the static itoa_buffer and
 * returns the pointer to that string.
 */
#[inline]
pub unsafe fn u64toh(in_: uint64_t) -> *mut c_char {
	u64toh_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* Converts the unsigned 64-bit integer <in> to its string representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (21 bytes for 18446744073709551615). The buffer is filled from
 * the first byte, and the number of characters emitted (not counting the
 * trailing zero) is returned.
 */
#[inline]
pub unsafe fn u64toa_r(in_: uint64_t, buffer: *mut c_char) -> c_int {
	_nolibc_u64toa_base(in_, buffer, 10, _NOLIBC_U64TOA_RECIP(10))
}

/* Converts the signed 64-bit integer <in> to its string representation into
 * buffer <buffer>, which must be long enough to store the number and the
 * trailing zero (21 bytes for -9223372036854775808). The buffer is filled from
 * the first byte, and the number of characters emitted (not counting the
 * trailing zero) is returned.
 */
pub unsafe fn i64toa_r(mut in_: int64_t, buffer: *mut c_char) -> c_int {
	let mut ptr_ = buffer;
	let mut len: c_int = 0;

	if in_ < 0 {
		in_ = (in_ as uint64_t).wrapping_neg() as int64_t;
		*ptr_ = b'-' as c_char;
		ptr_ = ptr_.add(1);
		len += 1;
	}
	len += u64toa_r(in_ as uint64_t, ptr_);
	len
}

/* converts int64_t <in> to a string using the static itoa_buffer and returns
 * the pointer to that string.
 */
#[inline]
pub unsafe fn i64toa(in_: int64_t) -> *mut c_char {
	i64toa_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

/* converts uint64_t <in> to a string using the static itoa_buffer and returns
 * the pointer to that string.
 */
#[inline]
pub unsafe fn u64toa(in_: uint64_t) -> *mut c_char {
	u64toa_r(in_, itoa_buffer.as_mut_ptr());
	itoa_buffer.as_mut_ptr()
}

pub unsafe fn __strtox(
	mut nptr: *const c_char,
	endptr: *mut *mut c_char,
	mut base: c_int,
	lower_limit: intmax_t,
	upper_limit: uintmax_t,
) -> uintmax_t {
	let signed_: c_char = (lower_limit != 0) as c_char;
	let mut neg: u8 = 0;
	let mut overflow: u8 = 0;
	let mut val: uintmax_t = 0;
	let mut limit: uintmax_t;
	let mut old_val: uintmax_t;
	let mut c: c_char;

	if base < 0 || base > 36 {
		SET_ERRNO(EINVAL);
		goto_out_set_endptr(nptr, endptr, neg, val)
	} else {
		while isspace(*nptr as c_int) != 0 {
			nptr = nptr.add(1);
		}

		if *nptr == b'+' as c_char {
			nptr = nptr.add(1);
		} else if *nptr == b'-' as c_char {
			neg = 1;
			nptr = nptr.add(1);
		}

		if signed_ != 0 && neg != 0 {
			limit = (lower_limit as uintmax_t).wrapping_neg();
		} else {
			limit = upper_limit;
		}

		if (base == 0 || base == 16) &&
		   (strncmp(nptr, c"0x".as_ptr(), 2) == 0 || strncmp(nptr, c"0X".as_ptr(), 2) == 0) {
			base = 16;
			nptr = nptr.add(2);
		} else if base == 0 && strncmp(nptr, c"0".as_ptr(), 1) == 0 {
			base = 8;
			nptr = nptr.add(1);
		} else if base == 0 {
			base = 10;
		}

		while *nptr != 0 {
			c = *nptr;

			if c >= b'0' as c_char && c <= b'9' as c_char {
				c = c.wrapping_sub(b'0' as c_char);
			} else if c >= b'a' as c_char && c <= b'z' as c_char {
				c = c.wrapping_sub(b'a' as c_char).wrapping_add(10);
			} else if c >= b'A' as c_char && c <= b'Z' as c_char {
				c = c.wrapping_sub(b'A' as c_char).wrapping_add(10);
			} else {
				break;
			}

			if c as c_int >= base {
				break;
			}

			nptr = nptr.add(1);
			old_val = val;
			val = val.wrapping_mul(base as uintmax_t);
			val = val.wrapping_add(c as uintmax_t);

			if val > limit || val < old_val {
				overflow = 1;
			}
		}

		if overflow != 0 {
			SET_ERRNO(ERANGE);
			val = limit;
		}
		if !endptr.is_null() {
			*endptr = nptr as *mut c_char;
		}
		if neg != 0 { val.wrapping_neg() } else { val }
	}
}

unsafe fn goto_out_set_endptr(
	nptr: *const c_char,
	endptr: *mut *mut c_char,
	neg: u8,
	val: uintmax_t,
) -> uintmax_t {
	if !endptr.is_null() {
		*endptr = nptr as *mut c_char;
	}
	if neg != 0 { val.wrapping_neg() } else { val }
}

pub unsafe fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
	__strtox(nptr, endptr, base, LONG_MIN, LONG_MAX as uintmax_t) as c_long
}

pub unsafe fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
	__strtox(nptr, endptr, base, 0, ULONG_MAX as uintmax_t) as c_ulong
}

pub unsafe fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong {
	__strtox(nptr, endptr, base, LLONG_MIN, LLONG_MAX as uintmax_t) as c_longlong
}

pub unsafe fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong {
	__strtox(nptr, endptr, base, 0, ULLONG_MAX as uintmax_t) as c_ulonglong
}

pub unsafe fn strtoimax(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t {
	__strtox(nptr, endptr, base, INTMAX_MIN, INTMAX_MAX as uintmax_t) as intmax_t
}

pub unsafe fn strtoumax(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> uintmax_t {
	__strtox(nptr, endptr, base, 0, UINTMAX_MAX)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
