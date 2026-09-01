// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/workloads/jitdump.c.
// Dependencies originally provided by:
//   #include "util/jitdump.h"
//   #include "../tests.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

const CHK_BYTE: u32 = 0x5a;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const JITDUMP_HEADER_FLAGS: u32 = JITDUMP_FLAGS_ARCH_TIMESTAMP;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
const JITDUMP_HEADER_FLAGS: u32 = 0;

#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type pid_t = c_int;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
struct timespec {
	tv_sec: i64,
	tv_nsec: i64,
}

// External declarations/constants from C headers and perf-local headers.
const CLOCK_MONOTONIC: c_int = 1;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const MAP_PRIVATE: c_int = 0x02;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_RDWR: c_int = 0o2;
const PROT_EXEC: c_int = 0x4;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const SYS_gettid: c_long = 186;

extern "C" {
	static mut errno: c_int;

	static JITHEADER_MAGIC: u32;
	static JITHEADER_VERSION: u32;
	static JITDUMP_FLAGS_ARCH_TIMESTAMP: u32;
	static JIT_CODE_LOAD: u32;

	fn __clear_cache(begin: *mut c_char, end: *mut c_char);
	fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn fclose(stream: *mut FILE) -> c_int;
	fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
	fn fileno(stream: *mut FILE) -> c_int;
	fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
	fn getpagesize() -> c_int;
	fn getpid() -> pid_t;
	fn mmap(
		addr: *mut c_void,
		length: size_t,
		prot: c_int,
		flags: c_int,
		fd: c_int,
		offset: isize,
	) -> *mut c_void;
	fn munmap(addr: *mut c_void, length: size_t) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
	fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
	fn strerror(errnum: c_int) -> *mut c_char;
	fn strlen(s: *const c_char) -> size_t;
	fn syscall(number: c_long, ...) -> c_long;
	fn unlink(pathname: *const c_char) -> c_int;

	fn pr_err(format: *const c_char, ...);
}

#[repr(C)]
struct jitheader {
	magic: u32,
	version: u32,
	total_size: u32,
	elf_mach: u32,
	pad1: u32,
	pid: u32,
	timestamp: u64,
	flags: u64,
}

#[repr(C)]
struct jr_prefix {
	id: u32,
	total_size: u32,
	timestamp: u64,
}

#[repr(C)]
struct jr_code_load {
	p: jr_prefix,
	pid: u32,
	tid: u32,
	vma: u64,
	code_addr: u64,
	code_size: u64,
	code_index: u64,
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
unsafe fn gettid() -> pid_t {
	syscall(SYS_gettid) as pid_t
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn gettid() -> pid_t {
	syscall(SYS_gettid) as pid_t
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_timestamp() -> u64 {
	let low: u32;
	let high: u32;

	core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));

	(low as u64) | ((high as u64) << 32)
}

#[cfg(target_arch = "x86")]
unsafe fn get_timestamp() -> u64 {
	let low: u32;
	let high: u32;

	core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));

	(low as u64) | ((high as u64) << 32)
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
unsafe fn get_timestamp() -> u64 {
	let mut ts = timespec {
		tv_sec: 0,
		tv_nsec: 0,
	};
	let ret: c_int;

	ret = clock_gettime(CLOCK_MONOTONIC, &mut ts);
	if ret != 0 {
		return 0;
	}

	((ts.tv_sec as u64) * 1000000000) + ts.tv_nsec as u64
}

unsafe fn open_jitdump() -> *mut FILE {
	let header = jitheader {
		magic: JITHEADER_MAGIC,
		version: JITHEADER_VERSION,
		total_size: mem::size_of::<jitheader>() as u32,
		elf_mach: 0,
		pad1: 0,
		pid: getpid() as u32,
		timestamp: get_timestamp(),
		flags: JITDUMP_HEADER_FLAGS as u64,
	};
	let mut filename = [0 as c_char; 256];
	let fd: c_int;
	let f: *mut FILE;
	let m: *mut c_void;

	snprintf(
		filename.as_mut_ptr(),
		filename.len(),
		b"jit-%d.dump\0".as_ptr() as *const c_char,
		getpid(),
	);
	/* Securely open using O_CREAT | O_EXCL to prevent symlink attacks. */
	fd = open(filename.as_ptr(), O_CREAT | O_EXCL | O_RDWR, 0o644);
	if fd < 0 {
		pr_err(
			b"Failed to open jitdump '%s': %s\n\0".as_ptr() as *const c_char,
			filename.as_ptr(),
			strerror(errno),
		);
		return ptr::null_mut();
	}
	f = fdopen(fd, b"w+\0".as_ptr() as *const c_char);
	if f.is_null() {
		pr_err(
			b"Failed to associate stream with fd for '%s'\n\0".as_ptr() as *const c_char,
			filename.as_ptr(),
		);
		close(fd);
		unlink(filename.as_ptr());
		return ptr::null_mut();
	}
	/* Create an MMAP event for the jitdump file. That is how perf tool finds it. */
	m = mmap(
		ptr::null_mut(),
		getpagesize() as size_t,
		PROT_READ | PROT_EXEC,
		MAP_PRIVATE,
		fileno(f),
		0,
	);
	if m == MAP_FAILED {
		pr_err(
			b"mmap failed: %s\n\0".as_ptr() as *const c_char,
			strerror(errno),
		);
		fclose(f);
		unlink(filename.as_ptr());
		return ptr::null_mut();
	}
	munmap(m, getpagesize() as size_t);

	if fwrite(
		&header as *const jitheader as *const c_void,
		mem::size_of::<jitheader>(),
		1,
		f,
	) != 1
	{
		pr_err(b"Error writing jitdump header\n\0".as_ptr() as *const c_char);
		fclose(f);
		unlink(filename.as_ptr());
		return ptr::null_mut();
	}
	f
}

unsafe fn write_jitdump(
	f: *mut FILE,
	addr: *mut c_void,
	dat: *const c_void,
	sz: size_t,
	idx: *mut u64,
) -> c_int {
	let sym = b"jit_workload\0";
	let sym_len = strlen(sym.as_ptr() as *const c_char) + 1;
	*idx = (*idx).wrapping_add(1);
	let rec = jr_code_load {
		p: jr_prefix {
			id: JIT_CODE_LOAD,
			total_size: (mem::size_of::<jr_code_load>() + sym_len + sz) as u32,
			timestamp: get_timestamp(),
		},
		pid: getpid() as u32,
		tid: gettid() as u32,
		vma: addr as c_ulong as u64,
		code_addr: addr as c_ulong as u64,
		code_size: sz as u64,
		code_index: *idx,
	};

	if fwrite(
		&rec as *const jr_code_load as *const c_void,
		mem::size_of::<jr_code_load>(),
		1,
		f,
	) != 1
		|| fwrite(sym.as_ptr() as *const c_void, sym_len, 1, f) != 1
		|| fwrite(dat, sz, 1, f) != 1
	{
		return -1;
	}
	0
}

unsafe fn close_jitdump(f: *mut FILE) {
	fclose(f);
}

#[no_mangle]
pub unsafe extern "C" fn jitdump(argc: c_int, argv: *const *const c_char) -> c_int {
	let _ = argc;
	let _ = argv;

	#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
	/* Code to execute: mov CHK_BYTE, %eax ; ret */
	let dat: [u8; 6] = [0xb8, CHK_BYTE as u8, 0x00, 0x00, 0x00, 0xc3];
	#[cfg(target_arch = "aarch64")]
	/* Code to execute: mov w0, #CHK_BYTE ; ret */
	let dat: [u8; 8] = [
		((CHK_BYTE << 5) & 0xff) as u8,
		((CHK_BYTE >> 3) & 0xff) as u8,
		0x80,
		0x52,
		0xc0,
		0x03,
		0x5f,
		0xd6,
	];
	#[cfg(target_arch = "riscv64")]
	/* Code to execute: li a0, CHK_BYTE ; ret */
	let dat: [u8; 8] = [
		0x13,
		0x05,
		((CHK_BYTE << 4) & 0xff) as u8,
		((CHK_BYTE >> 4) & 0xff) as u8,
		0x67,
		0x80,
		0x00,
		0x00,
	];
	#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
	/* Code to execute: li r3, CHK_BYTE ; blr */
	let dat: [u32; 2] = [0x38600000 | (CHK_BYTE & 0xffff), 0x4e800020];
	#[cfg(target_arch = "s390x")]
	/* Code to execute: lhi %r2, CHK_BYTE ; br %r14 */
	let dat: [u8; 6] = [
		0xa7,
		0x28,
		((CHK_BYTE >> 8) & 0xff) as u8,
		(CHK_BYTE & 0xff) as u8,
		0x07,
		0xfe,
	];
	#[cfg(target_arch = "arm")]
	/* Code to execute: mov r0, #CHK_BYTE ; bx lr */
	let dat: [u8; 8] = [
		(CHK_BYTE & 0xff) as u8,
		0x00,
		0xa0,
		0xe3,
		0x1e,
		0xff,
		0x2f,
		0xe1,
	];
	#[cfg(target_arch = "mips")]
	/* Code to execute: addiu $v0, $zero, CHK_BYTE ; jr $ra ; nop */
	let dat: [u32; 3] = [
		0x24020000 | (CHK_BYTE & 0xffff),
		0x03e00008,
		0x00000000,
	];
	#[cfg(target_arch = "loongarch64")]
	/* Code to execute: addi.w $a0, $zero, CHK_BYTE ; jirl $zero, $ra, 0 */
	let dat: [u32; 2] = [0x02800004 | ((CHK_BYTE & 0xfff) << 10), 0x4c000020];
	#[cfg(not(any(
		target_arch = "x86_64",
		target_arch = "x86",
		target_arch = "aarch64",
		target_arch = "riscv64",
		target_arch = "powerpc",
		target_arch = "powerpc64",
		target_arch = "s390x",
		target_arch = "arm",
		target_arch = "mips",
		target_arch = "loongarch64"
	)))]
	let dat: [u32; 0] = [];

	let addr: *mut c_void;
	let f: *mut FILE;
	let mut idx: u64 = 0;
	let mut ret: c_int = 1;

	/* Reachable fallback check for unsupported architectures right at start. */
	if mem::size_of_val(&dat) == 0 {
		pr_err(
			b"JITDUMP workload not supported on this architecture\n\0".as_ptr() as *const c_char,
		);
		return 1;
	}

	/* Get a memory page to store executable code. */
	addr = mmap(
		ptr::null_mut(),
		getpagesize() as size_t,
		PROT_READ | PROT_WRITE | PROT_EXEC,
		MAP_ANONYMOUS | MAP_PRIVATE,
		-1,
		0,
	);
	if addr == MAP_FAILED {
		pr_err(b"Failed to map 1 -rwx page\n\0".as_ptr() as *const c_char);
		return 1;
	}

	f = open_jitdump();
	if f.is_null() {
		pr_err(b"Failed to open JITDUMP\n\0".as_ptr() as *const c_char);
		munmap(addr, getpagesize() as size_t);
		return 1;
	}
	/* Copy executable code to executable memory page. */
	ptr::copy_nonoverlapping(
		dat.as_ptr() as *const c_void as *const u8,
		addr as *mut u8,
		mem::size_of_val(&dat),
	);
	/* Synchronize the Instruction and Data caches. */
	__clear_cache(
		addr as *mut c_char,
		(addr as *mut c_char).add(mem::size_of_val(&dat)),
	);

	/* Record it in the jitdump file */
	if write_jitdump(
		f,
		addr,
		dat.as_ptr() as *const c_void,
		mem::size_of_val(&dat),
		&mut idx,
	) == 0
	{
		let fn_: extern "C" fn() -> c_int = mem::transmute(addr);

		/* Call the function. */
		ret = fn_() - CHK_BYTE as c_int;
	}
	close_jitdump(f);
	munmap(addr, getpagesize() as size_t);
	ret
}

// DEFINE_WORKLOAD(jitdump);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
