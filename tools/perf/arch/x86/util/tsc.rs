// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// <linux/types.h>, <math.h>, <string.h>, <stdlib.h>,
// "../../../util/debug.h", "../../../util/tsc.h", and "cpuid.h".

use core::arch::asm;
use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fclose(stream: *mut FILE) -> c_int;
	fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
	fn free(ptr: *mut c_void);
	fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

	fn pr_err(fmt: *const c_char, ...);
	fn get_cpuid_0(vendor: *mut c_char, lvl: *mut c_uint);
	fn cpuid(
		op: c_uint,
		count: c_uint,
		a: *mut c_uint,
		b: *mut c_uint,
		c: *mut c_uint,
		d: *mut c_uint,
	);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rdtsc() -> u64 {
	let low: c_uint;
	let high: c_uint;

	unsafe {
		asm!("rdtsc", out("eax") low, out("edx") high);
	}

	(low as u64) | ((high as u64) << 32)
}

/*
 * Derive the TSC frequency in Hz from the /proc/cpuinfo, for example:
 * ...
 * model name      : Intel(R) Xeon(R) Gold 6154 CPU @ 3.00GHz
 * ...
 * will return 3000000000.
 */
unsafe fn cpuinfo_tsc_freq() -> u64 {
	let mut result: u64 = 0;
	let cpuinfo: *mut FILE;
	let mut line: *mut c_char = core::ptr::null_mut();
	let mut len: size_t = 0;

	unsafe {
		cpuinfo = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
		if cpuinfo.is_null() {
			pr_err(c"Failed to read /proc/cpuinfo for TSC frequency\n".as_ptr());
			return 0;
		}
		while getline(&mut line, &mut len, cpuinfo) > 0 {
			if strncmp(line, c"model name".as_ptr(), 10) == 0 {
				let pos = strstr(line.add(11), c" @ ".as_ptr());
				let mut float_result: c_double = 0.0;

				if !pos.is_null() && sscanf(pos, c" @ %lfGHz".as_ptr(), &mut float_result) == 1 {
					float_result *= 1000000000.0;
					result = float_result as u64;
					break;
				}
			}
		}
		if result == 0 {
			pr_err(c"Failed to find TSC frequency in /proc/cpuinfo\n".as_ptr());
		}

		free(line as *mut c_void);
		fclose(cpuinfo);
	}
	result
}

static mut CACHED: bool = false;
static mut TSC: c_double = 0.0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_get_tsc_freq() -> u64 {
	let mut a: c_uint = 0;
	let mut b: c_uint = 0;
	let mut c: c_uint = 0;
	let mut d: c_uint = 0;
	let mut lvl: c_uint = 0;
	let mut vendor: [c_char; 16] = [0; 16];

	unsafe {
		if CACHED {
			return TSC as u64;
		}

		CACHED = true;
		get_cpuid_0(vendor.as_mut_ptr(), &mut lvl);
		if strstr(vendor.as_ptr(), c"Intel".as_ptr()).is_null() {
			return 0;
		}

		/*
		 * Don't support Time Stamp Counter and
		 * Nominal Core Crystal Clock Information Leaf.
		 */
		if lvl < 0x15 {
			TSC = cpuinfo_tsc_freq() as c_double;
			return TSC as u64;
		}

		cpuid(0x15, 0, &mut a, &mut b, &mut c, &mut d);
		/* TSC frequency is not enumerated */
		if a == 0 || b == 0 || c == 0 {
			TSC = cpuinfo_tsc_freq() as c_double;
			return TSC as u64;
		}

		TSC = ((c as u64).wrapping_mul(b as u64).wrapping_div(a as u64)) as c_double;
		TSC as u64
	}
}
