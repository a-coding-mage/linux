// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// #include <stdbool.h>
// #include <stdlib.h>
// #include <string.h>
// #include "tests.h"
// #include "dso.h"
// #include "debug.h"
// #include "event.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct test_suite {
	_private: [u8; 0],
}

#[repr(C)]
pub struct kmod_path {
	pub name: *mut c_char,
	pub kmod: bool,
	pub comp: c_int,
}

unsafe extern "C" {
	fn __kmod_path__parse(m: *mut kmod_path, path: *const c_char, alloc_name: bool) -> c_int;
	fn is_kernel_module(path: *const c_char, cpumode: c_int) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn free(ptr: *mut c_void);
}

unsafe fn test(
	path: *const c_char,
	alloc_name: bool,
	kmod: bool,
	comp: c_int,
	name: *const c_char,
) -> c_int {
	let mut m: kmod_path = mem::zeroed();

	TEST_ASSERT_VAL!(
		b"kmod_path__parse\0".as_ptr() as *const c_char,
		__kmod_path__parse(&mut m, path, alloc_name) == 0
	);

	pr_debug!(
		b"%s - alloc name %d, kmod %d, comp %d, name '%s'\n\0".as_ptr() as *const c_char,
		path,
		alloc_name as c_int,
		m.kmod as c_int,
		m.comp,
		m.name
	);

	TEST_ASSERT_VAL!(b"wrong kmod\0".as_ptr() as *const c_char, m.kmod == kmod);
	TEST_ASSERT_VAL!(b"wrong comp\0".as_ptr() as *const c_char, m.comp == comp);

	if !name.is_null() {
		TEST_ASSERT_VAL!(
			b"wrong name\0".as_ptr() as *const c_char,
			!m.name.is_null() && strcmp(name, m.name) == 0
		);
	} else {
		TEST_ASSERT_VAL!(b"wrong name\0".as_ptr() as *const c_char, m.name.is_null());
	}

	free(m.name as *mut c_void);
	return 0;
}

unsafe fn test_is_kernel_module(path: *const c_char, cpumode: c_int, expect: bool) -> c_int {
	TEST_ASSERT_VAL!(
		b"is_kernel_module\0".as_ptr() as *const c_char,
		((is_kernel_module(path, cpumode) != 0) as c_int) == ((expect != false) as c_int)
	);
	pr_debug!(
		b"%s (cpumode: %d) - is_kernel_module: %s\n\0".as_ptr() as *const c_char,
		path,
		cpumode,
		if expect {
			b"true\0".as_ptr() as *const c_char
		} else {
			b"false\0".as_ptr() as *const c_char
		}
	);
	return 0;
}

macro_rules! T {
	($path:expr, $an:expr, $k:expr, $c:expr, $n:expr) => {
		TEST_ASSERT_VAL!(
			b"failed\0".as_ptr() as *const c_char,
			test($path.as_ptr() as *const c_char, $an, $k, $c, $n) == 0
		)
	};
}

macro_rules! M {
	($path:expr, $c:expr, $e:expr) => {
		TEST_ASSERT_VAL!(
			b"failed\0".as_ptr() as *const c_char,
			test_is_kernel_module($path.as_ptr() as *const c_char, $c, $e) == 0
		)
	};
}

unsafe fn test__kmod_path__parse(_t: *mut test_suite, _subtest: c_int) -> c_int {
	/* path                alloc_name  kmod  comp   name   */
	T!(b"/xxxx/xxxx/x-x.ko\0", true, true, 0, b"[x_x]\0".as_ptr() as *const c_char);
	T!(b"/xxxx/xxxx/x-x.ko\0", false, true, 0, ptr::null());
	T!(b"/xxxx/xxxx/x-x.ko\0", true, true, 0, b"[x_x]\0".as_ptr() as *const c_char);
	T!(b"/xxxx/xxxx/x-x.ko\0", false, true, 0, ptr::null());
	M!(b"/xxxx/xxxx/x-x.ko\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
	M!(b"/xxxx/xxxx/x-x.ko\0", PERF_RECORD_MISC_KERNEL, true);
	M!(b"/xxxx/xxxx/x-x.ko\0", PERF_RECORD_MISC_USER, false);

	// Original C condition: #ifdef HAVE_ZLIB_SUPPORT
	#[cfg(HAVE_ZLIB_SUPPORT)]
	{
		/* path                alloc_name   kmod  comp  name  */
		T!(b"/xxxx/xxxx/x.ko.gz\0", true, true, 1, b"[x]\0".as_ptr() as *const c_char);
		T!(b"/xxxx/xxxx/x.ko.gz\0", false, true, 1, ptr::null());
		T!(b"/xxxx/xxxx/x.ko.gz\0", true, true, 1, b"[x]\0".as_ptr() as *const c_char);
		T!(b"/xxxx/xxxx/x.ko.gz\0", false, true, 1, ptr::null());
		M!(b"/xxxx/xxxx/x.ko.gz\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
		M!(b"/xxxx/xxxx/x.ko.gz\0", PERF_RECORD_MISC_KERNEL, true);
		M!(b"/xxxx/xxxx/x.ko.gz\0", PERF_RECORD_MISC_USER, false);

		/* path              alloc_name  kmod   comp  name  */
		T!(b"/xxxx/xxxx/x.gz\0", true, false, 1, b"x.gz\0".as_ptr() as *const c_char);
		T!(b"/xxxx/xxxx/x.gz\0", false, false, 1, ptr::null());
		T!(b"/xxxx/xxxx/x.gz\0", true, false, 1, b"x.gz\0".as_ptr() as *const c_char);
		T!(b"/xxxx/xxxx/x.gz\0", false, false, 1, ptr::null());
		M!(b"/xxxx/xxxx/x.gz\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
		M!(b"/xxxx/xxxx/x.gz\0", PERF_RECORD_MISC_KERNEL, false);
		M!(b"/xxxx/xxxx/x.gz\0", PERF_RECORD_MISC_USER, false);

		/* path   alloc_name  kmod   comp  name   */
		T!(b"x.gz\0", true, false, 1, b"x.gz\0".as_ptr() as *const c_char);
		T!(b"x.gz\0", false, false, 1, ptr::null());
		T!(b"x.gz\0", true, false, 1, b"x.gz\0".as_ptr() as *const c_char);
		T!(b"x.gz\0", false, false, 1, ptr::null());
		M!(b"x.gz\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
		M!(b"x.gz\0", PERF_RECORD_MISC_KERNEL, false);
		M!(b"x.gz\0", PERF_RECORD_MISC_USER, false);

		/* path      alloc_name  kmod  comp  name  */
		T!(b"x.ko.gz\0", true, true, 1, b"[x]\0".as_ptr() as *const c_char);
		T!(b"x.ko.gz\0", false, true, 1, ptr::null());
		T!(b"x.ko.gz\0", true, true, 1, b"[x]\0".as_ptr() as *const c_char);
		T!(b"x.ko.gz\0", false, true, 1, ptr::null());
		M!(b"x.ko.gz\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
		M!(b"x.ko.gz\0", PERF_RECORD_MISC_KERNEL, true);
		M!(b"x.ko.gz\0", PERF_RECORD_MISC_USER, false);
	}

	/* path            alloc_name  kmod  comp   name           */
	T!(b"[test_module]\0", true, true, false as c_int, b"[test_module]\0".as_ptr() as *const c_char);
	T!(b"[test_module]\0", false, true, false as c_int, ptr::null());
	T!(b"[test_module]\0", true, true, false as c_int, b"[test_module]\0".as_ptr() as *const c_char);
	T!(b"[test_module]\0", false, true, false as c_int, ptr::null());
	M!(b"[test_module]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
	M!(b"[test_module]\0", PERF_RECORD_MISC_KERNEL, true);
	M!(b"[test_module]\0", PERF_RECORD_MISC_USER, false);

	/* path            alloc_name  kmod  comp   name           */
	T!(b"[test.module]\0", true, true, false as c_int, b"[test.module]\0".as_ptr() as *const c_char);
	T!(b"[test.module]\0", false, true, false as c_int, ptr::null());
	T!(b"[test.module]\0", true, true, false as c_int, b"[test.module]\0".as_ptr() as *const c_char);
	T!(b"[test.module]\0", false, true, false as c_int, ptr::null());
	M!(b"[test.module]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
	M!(b"[test.module]\0", PERF_RECORD_MISC_KERNEL, true);
	M!(b"[test.module]\0", PERF_RECORD_MISC_USER, false);

	/* path     alloc_name  kmod   comp   name    */
	T!(b"[vdso]\0", true, false, false as c_int, b"[vdso]\0".as_ptr() as *const c_char);
	T!(b"[vdso]\0", false, false, false as c_int, ptr::null());
	T!(b"[vdso]\0", true, false, false as c_int, b"[vdso]\0".as_ptr() as *const c_char);
	T!(b"[vdso]\0", false, false, false as c_int, ptr::null());
	M!(b"[vdso]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
	M!(b"[vdso]\0", PERF_RECORD_MISC_KERNEL, false);
	M!(b"[vdso]\0", PERF_RECORD_MISC_USER, false);

	T!(b"[vdso32]\0", true, false, false as c_int, b"[vdso32]\0".as_ptr() as *const c_char);
	T!(b"[vdso32]\0", false, false, false as c_int, ptr::null());
	T!(b"[vdso32]\0", true, false, false as c_int, b"[vdso32]\0".as_ptr() as *const c_char);
	T!(b"[vdso32]\0", false, false, false as c_int, ptr::null());
	M!(b"[vdso32]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
	M!(b"[vdso32]\0", PERF_RECORD_MISC_KERNEL, false);
	M!(b"[vdso32]\0", PERF_RECORD_MISC_USER, false);

	T!(b"[vdsox32]\0", true, false, false as c_int, b"[vdsox32]\0".as_ptr() as *const c_char);
	T!(b"[vdsox32]\0", false, false, false as c_int, ptr::null());
	T!(b"[vdsox32]\0", true, false, false as c_int, b"[vdsox32]\0".as_ptr() as *const c_char);
	T!(b"[vdsox32]\0", false, false, false as c_int, ptr::null());
	M!(b"[vdsox32]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
	M!(b"[vdsox32]\0", PERF_RECORD_MISC_KERNEL, false);
	M!(b"[vdsox32]\0", PERF_RECORD_MISC_USER, false);

	/* path         alloc_name  kmod   comp   name        */
	T!(b"[vsyscall]\0", true, false, false as c_int, b"[vsyscall]\0".as_ptr() as *const c_char);
	T!(b"[vsyscall]\0", false, false, false as c_int, ptr::null());
	T!(b"[vsyscall]\0", true, false, false as c_int, b"[vsyscall]\0".as_ptr() as *const c_char);
	T!(b"[vsyscall]\0", false, false, false as c_int, ptr::null());
	M!(b"[vsyscall]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
	M!(b"[vsyscall]\0", PERF_RECORD_MISC_KERNEL, false);
	M!(b"[vsyscall]\0", PERF_RECORD_MISC_USER, false);

	/* path                alloc_name  kmod   comp   name      */
	T!(b"[kernel.kallsyms]\0", true, false, false as c_int, b"[kernel.kallsyms]\0".as_ptr() as *const c_char);
	T!(b"[kernel.kallsyms]\0", false, false, false as c_int, ptr::null());
	T!(b"[kernel.kallsyms]\0", true, false, false as c_int, b"[kernel.kallsyms]\0".as_ptr() as *const c_char);
	T!(b"[kernel.kallsyms]\0", false, false, false as c_int, ptr::null());
	M!(b"[kernel.kallsyms]\0", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
	M!(b"[kernel.kallsyms]\0", PERF_RECORD_MISC_KERNEL, false);
	M!(b"[kernel.kallsyms]\0", PERF_RECORD_MISC_USER, false);

	return 0;
}

DEFINE_SUITE!(b"kmod_path__parse\0".as_ptr() as *const c_char, kmod_path__parse);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
