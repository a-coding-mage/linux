// SPDX-License-Identifier: GPL-2.0
/*
 * Payload for the binfmt_misc 'L' (loader substitution) selftest. It is
 * executed as the MAIN image - a fully native exec - with the registered
 * interpreter substituted for its PT_INTERP, and asserts the native
 * identity from the inside. Exits 0 when every surface checks out.
 *
 * Modes, selected by the orchestrator via the environment:
 *  - default:                full assertions, path-based ones included
 *  - BINFMT_TEST_MEMFD=1:    executed from an inaccessible memfd, skip
 *                            the path-based assertions
 *  - BINFMT_TEST_STATIC=1:   static build; the override was dropped, so
 *                            expect no interpreter at all
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type SizeT = usize;
type SSizeT = isize;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

/* Constants normally supplied by <sys/auxv.h>. */
const AT_BASE: c_ulong = 7;
const AT_FLAGS: c_ulong = 8;
const AT_ENTRY: c_ulong = 9;
const AT_PHDR: c_ulong = 3;
const AT_EXECFD: c_ulong = 2;
const AT_EXECFN: c_ulong = 31;

/* An image is never this large; used to bracket "within our image". */
const IMAGE_SPAN: c_ulong = 16usize.wrapping_shl(20) as c_ulong;

static mut failed: c_int = 0;

unsafe extern "C" {
	/* Start of our own mapped image, courtesy of the linker. */
	static __ehdr_start: [c_char; 0];

	static mut stderr: *mut FILE;

	static PAYLOAD_ARGV0: *const c_char;
	static PAYLOAD_ARG1: *const c_char;
	static PAYLOAD_ARG2: *const c_char;

	fn comm_is(comm: *const c_char) -> c_int;
	fn exe_is(exe: *const c_char) -> c_int;
	fn stat_codes(pid: c_int, start_code: *mut c_ulong, end_code: *mut c_ulong) -> c_int;
	fn write_denied(path: *const c_char) -> c_int;

	fn fclose(stream: *mut FILE) -> c_int;
	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn free(ptr: *mut c_void);
	fn getenv(name: *const c_char) -> *mut c_char;
	fn getline(lineptr: *mut *mut c_char, n: *mut SizeT, stream: *mut FILE) -> SSizeT;
	fn getauxval(type_: c_ulong) -> c_ulong;
	fn getpid() -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strlen(s: *const c_char) -> SizeT;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
	fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;

	fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
	*__errno_location()
}

unsafe fn check(cond: c_int, what: *const c_char) {
	if cond != 0 {
		return;
	}
	fprintf(
		stderr,
		b"[payload] FAILED: %s (errno %d)\n\0".as_ptr() as *const c_char,
		what,
		errno(),
	);
	failed = 1;
}

/* Return whether /proc/self/maps names a path starting with @prefix. */
unsafe fn maps_has_prefix(prefix: *const c_char) -> c_int {
	let mut line: *mut c_char = ptr::null_mut();
	let mut len: SizeT = 0;
	let mut found: c_int = 0;
	let f: *mut FILE;

	f = fopen(
		b"/proc/self/maps\0".as_ptr() as *const c_char,
		b"r\0".as_ptr() as *const c_char,
	);
	if f.is_null() {
		return -1;
	}
	while getline(&mut line, &mut len, f) > 0 {
		let path = strchr(line, b'/' as c_int);

		if !path.is_null() && strncmp(path, prefix, strlen(prefix)) == 0 {
			found = 1;
			break;
		}
	}
	free(line as *mut c_void);
	fclose(f);
	found
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let binary = getenv(b"BINFMT_TEST_BINARY\0".as_ptr() as *const c_char);
	let interp = getenv(b"BINFMT_TEST_INTERP\0".as_ptr() as *const c_char);
	let memfd_mode = (!getenv(b"BINFMT_TEST_MEMFD\0".as_ptr() as *const c_char).is_null()) as c_int;
	let static_mode = (!getenv(b"BINFMT_TEST_STATIC\0".as_ptr() as *const c_char).is_null()) as c_int;
	let self_ = __ehdr_start.as_ptr() as c_ulong;
	let base = getauxval(AT_BASE);
	let phdr = getauxval(AT_PHDR);
	let entry = getauxval(AT_ENTRY);
	let mut start_code: c_ulong = 0;
	let mut end_code: c_ulong = 0;

	/* The argument vector is exactly what the caller built. */
	check(
		(argc == 3
			&& strcmp(*argv.add(0), PAYLOAD_ARGV0) == 0
			&& strcmp(*argv.add(1), PAYLOAD_ARG1) == 0
			&& strcmp(*argv.add(2), PAYLOAD_ARG2) == 0) as c_int,
		b"argv was rewritten\0".as_ptr() as *const c_char,
	);

	/* Native from birth: no execfd, no dispatch marker. */
	check(
		(getauxval(AT_EXECFD) == 0) as c_int,
		b"AT_EXECFD present\0".as_ptr() as *const c_char,
	);
	check(
		(getauxval(AT_FLAGS) == 0) as c_int,
		b"AT_FLAGS not native\0".as_ptr() as *const c_char,
	);

	if static_mode != 0 {
		/* The override was dropped: no interpreter was loaded. */
		check(
			(base == 0) as c_int,
			b"AT_BASE set for a static payload\0".as_ptr() as *const c_char,
		);
	} else {
		/* A loader is mapped in the interpreter slot, not our image. */
		check(
			(base != 0) as c_int,
			b"AT_BASE missing\0".as_ptr() as *const c_char,
		);
		check(
			(base < self_ || base >= self_.wrapping_add(IMAGE_SPAN)) as c_int,
			b"AT_BASE inside our own image\0".as_ptr() as *const c_char,
		);
	}

	/* We occupy the main-image slot. */
	check(
		(phdr >= self_ && phdr < self_.wrapping_add(IMAGE_SPAN)) as c_int,
		b"AT_PHDR outside our image\0".as_ptr() as *const c_char,
	);
	check(
		(entry >= self_ && entry < self_.wrapping_add(IMAGE_SPAN)) as c_int,
		b"AT_ENTRY outside our image\0".as_ptr() as *const c_char,
	);

	/* The code statistics markers describe our image, natively placed. */
	if stat_codes(getpid(), &mut start_code, &mut end_code) == 0 {
		check(
			(start_code >= self_
				&& start_code < end_code
				&& end_code < self_.wrapping_add(IMAGE_SPAN)) as c_int,
			b"stat start_code/end_code not our image\0".as_ptr() as *const c_char,
		);
		check(
			(entry >= start_code && entry < end_code) as c_int,
			b"AT_ENTRY outside [start_code, end_code)\0".as_ptr() as *const c_char,
		);
	} else {
		check(0, b"cannot parse /proc/self/stat\0".as_ptr() as *const c_char);
	}

	if memfd_mode == 0 && !binary.is_null() {
		let execfn = getauxval(AT_EXECFN) as *const c_char;
		let mut base_name = strrchr(binary, b'/' as c_int);

		base_name = if !base_name.is_null() {
			base_name.add(1)
		} else {
			binary
		};

		/* exe link, AT_EXECFN and comm all follow the binary. */
		check(exe_is(binary), b"/proc/self/exe\0".as_ptr() as *const c_char);
		check(
			(!execfn.is_null() && strcmp(execfn, binary) == 0) as c_int,
			b"AT_EXECFN\0".as_ptr() as *const c_char,
		);
		check(comm_is(base_name), b"comm\0".as_ptr() as *const c_char);

		/* The running binary is write-denied, natively. */
		check(
			write_denied(binary),
			b"no ETXTBSY on the binary\0".as_ptr() as *const c_char,
		);
	}

	if !interp.is_null() {
		let found = maps_has_prefix(interp);

		if static_mode != 0 {
			/* Nothing was substituted, nothing may be mapped. */
			check(
				(found == 0) as c_int,
				b"loader mapped for a static payload\0".as_ptr() as *const c_char,
			);
		} else {
			/* The substituted loader shows under its real path. */
			check(
				(found == 1) as c_int,
				b"loader path not in /proc/self/maps\0".as_ptr() as *const c_char,
			);
		}
	}

	if failed != 0 {
		return 1;
	}
	printf(b"[payload] native identity checks out\n\0".as_ptr() as *const c_char);
	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
