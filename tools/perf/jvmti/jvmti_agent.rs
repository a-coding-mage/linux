/*
 * jvmti_agent.c: JVMTI agent interface
 *
 * Adapted from the Oprofile code in opagent.c:
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Copyright 2007 OProfile authors
 * Jens Wilke
 * Daniel Hansel
 * Copyright IBM Corporation 2007
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uint};

/* Dependencies in the original C source:
 * #include "jvmti_agent.h"
 * #include "../util/jitdump.h"
 */

const PATH_MAX: usize = 4096;
const JIT_LANG: &[u8] = b"java\0";

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;

const PROT_READ: c_int = 0x1;
const PROT_EXEC: c_int = 0x4;
const MAP_PRIVATE: c_int = 0x02;
const CLOCK_MONOTONIC: c_int = 1;
const _SC_PAGESIZE: c_int = 30;
const EEXIST: c_int = 17;

#[cfg(target_arch = "x86_64")]
const __NR_GETTID: c_long = 186;
#[cfg(target_arch = "x86")]
const __NR_GETTID: c_long = 224;
#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
const __NR_GETTID: c_long = 186;

const JITHEADER_MAGIC: u32 = 0x4A695444;
const JITHEADER_VERSION: u32 = 1;
const JIT_CODE_LOAD: u32 = 0;
const JIT_CODE_MOVE: u32 = 1;
const JIT_CODE_DEBUG_INFO: u32 = 2;
const JIT_CODE_CLOSE: u32 = 3;
const JIT_CODE_UNWINDING_INFO: u32 = 4;
const JITDUMP_FLAGS_ARCH_TIMESTAMP: u32 = 1 << 0;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
	pub tv_sec: c_long,
	pub tv_nsec: c_long,
}

#[repr(C)]
pub struct tm {
	pub tm_sec: c_int,
	pub tm_min: c_int,
	pub tm_hour: c_int,
	pub tm_mday: c_int,
	pub tm_mon: c_int,
	pub tm_year: c_int,
	pub tm_wday: c_int,
	pub tm_yday: c_int,
	pub tm_isdst: c_int,
	pub tm_gmtoff: c_long,
	pub tm_zone: *const c_char,
}

pub type pid_t = c_int;
pub type time_t = c_long;
pub type ssize_t = isize;

#[repr(C)]
pub struct jr_prefix {
	pub id: u32,
	pub total_size: u32,
	pub timestamp: u64,
}

#[repr(C)]
pub struct jitheader {
	pub magic: u32,
	pub version: u32,
	pub total_size: u32,
	pub elf_mach: u32,
	pub pad1: u32,
	pub pid: u32,
	pub timestamp: u64,
	pub flags: u64,
}

#[repr(C)]
pub struct jr_code_load {
	pub p: jr_prefix,
	pub pid: u32,
	pub tid: u32,
	pub vma: u64,
	pub code_addr: u64,
	pub code_size: u64,
	pub code_index: u64,
}

#[repr(C)]
pub struct jr_code_close {
	pub p: jr_prefix,
}

#[repr(C)]
pub struct jr_code_debug_info {
	pub p: jr_prefix,
	pub code_addr: u64,
	pub nr_entry: u64,
}

#[repr(C)]
pub struct debug_entry {
	pub addr: u64,
	pub lineno: c_int,
	pub discrim: c_int,
}

#[repr(C)]
pub struct jvmti_line_info_t {
	pub pc: *const c_void,
	pub line_number: c_int,
	pub discrim: c_int,
}

unsafe extern "C" {
	fn syscall(num: c_long, ...) -> c_long;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
	fn close(fd: c_int) -> c_int;
	fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
	fn time(tloc: *mut time_t) -> time_t;
	fn localtime_r(timep: *const time_t, result: *mut tm) -> *mut tm;
	fn getenv(name: *const c_char) -> *mut c_char;
	fn strftime(s: *mut c_char, max: usize, format: *const c_char, tm: *const tm) -> usize;
	fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
	fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
	fn mkdtemp(template: *mut c_char) -> *mut c_char;
	fn sysconf(name: c_int) -> c_long;
	fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
	fn munmap(addr: *mut c_void, length: usize) -> c_int;
	fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
	fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
	fn fwrite_unlocked(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
	fn fclose(stream: *mut FILE) -> c_int;
	fn flockfile(filehandle: *mut FILE);
	fn funlockfile(filehandle: *mut FILE);
	fn getpid() -> pid_t;
	fn strlen(s: *const c_char) -> usize;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn warn(fmt: *const c_char, ...);
	fn warnx(fmt: *const c_char, ...);
	fn __errno_location() -> *mut c_int;
}

static mut JIT_PATH: [c_char; PATH_MAX] = [0; PATH_MAX];
static mut MARKER_ADDR: *mut c_void = ptr::null_mut();
static mut USE_ARCH_TIMESTAMP: c_int = 0;
static mut PERF_CLK_ID: c_int = CLOCK_MONOTONIC;

unsafe fn errno() -> c_int {
	*__errno_location()
}

unsafe fn gettid() -> pid_t {
	syscall(__NR_GETTID) as pid_t
}

unsafe fn get_e_machine(hdr: *mut jitheader) -> c_int {
	let mut sret: ssize_t;
	let mut id: [c_char; 16] = [0; 16];
	let mut ret: c_int = -1;
	#[repr(C)]
	struct Info {
		e_type: u16,
		e_machine: u16,
	}
	let mut info = Info {
		e_type: 0,
		e_machine: 0,
	};

	let fd = open(c"/proc/self/exe".as_ptr(), O_RDONLY);
	if fd == -1 {
		return -1;
	}

	sret = read(fd, id.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>());
	if sret != size_of::<[c_char; 16]>() as ssize_t {
		close(fd);
		return ret;
	}

	/* check ELF signature */
	if id[0] != 0x7f || id[1] != b'E' as c_char || id[2] != b'L' as c_char || id[3] != b'F' as c_char {
		close(fd);
		return ret;
	}

	sret = read(fd, &mut info as *mut Info as *mut c_void, size_of::<Info>());
	if sret != size_of::<Info>() as ssize_t {
		close(fd);
		return ret;
	}

	(*hdr).elf_mach = info.e_machine as u32;
	ret = 0;
	close(fd);
	ret
}

unsafe fn get_arch_timestamp() -> u64 {
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	{
		let low: u32;
		let high: u32;

		core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));

		(low as u64) | ((high as u64) << 32)
	}
	#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
	{
		0
	}
}

const NSEC_PER_SEC: u64 = 1000000000;

unsafe fn timespec_to_ns(ts: *const timespec) -> u64 {
	((*ts).tv_sec as u64 * NSEC_PER_SEC) + (*ts).tv_nsec as u64
}

unsafe fn perf_get_timestamp() -> u64 {
	let mut ts = timespec {
		tv_sec: 0,
		tv_nsec: 0,
	};
	let ret: c_int;

	if USE_ARCH_TIMESTAMP != 0 {
		return get_arch_timestamp();
	}

	ret = clock_gettime(PERF_CLK_ID, &mut ts);
	if ret != 0 {
		return 0;
	}

	timespec_to_ns(&ts)
}

unsafe fn create_jit_cache_dir() -> c_int {
	let mut str_: [c_char; 32] = [0; 32];
	let mut base: *mut c_char;
	let mut p: *mut c_char;
	let mut tm_: tm = core::mem::zeroed();
	let mut t: time_t = 0;
	let mut ret: c_int;

	time(&mut t);
	localtime_r(&t, &mut tm_);

	base = getenv(c"JITDUMPDIR".as_ptr());
	if base.is_null() {
		base = getenv(c"HOME".as_ptr());
	}
	if base.is_null() {
		base = c".".as_ptr() as *mut c_char;
	}

	strftime(str_.as_mut_ptr(), size_of::<[c_char; 32]>(), c"java-jit-%Y%m%d".as_ptr(), &tm_);

	ret = snprintf(JIT_PATH.as_mut_ptr(), PATH_MAX, c"%s/.debug/".as_ptr(), base);
	if ret >= PATH_MAX as c_int {
		warnx(
			c"jvmti: cannot generate jit cache dir because %s/.debug/ is too long, please check the cwd, JITDUMPDIR, and HOME variables".as_ptr(),
			base,
		);
		return -1;
	}
	ret = mkdir(JIT_PATH.as_ptr(), 0o755);
	if ret == -1 {
		if errno() != EEXIST {
			warn(c"jvmti: cannot create jit cache dir %s".as_ptr(), JIT_PATH.as_ptr());
			return -1;
		}
	}

	ret = snprintf(JIT_PATH.as_mut_ptr(), PATH_MAX, c"%s/.debug/jit".as_ptr(), base);
	if ret >= PATH_MAX as c_int {
		warnx(
			c"jvmti: cannot generate jit cache dir because %s/.debug/jit is too long, please check the cwd, JITDUMPDIR, and HOME variables".as_ptr(),
			base,
		);
		return -1;
	}
	ret = mkdir(JIT_PATH.as_ptr(), 0o755);
	if ret == -1 {
		if errno() != EEXIST {
			warn(c"jvmti: cannot create jit cache dir %s".as_ptr(), JIT_PATH.as_ptr());
			return -1;
		}
	}

	ret = snprintf(JIT_PATH.as_mut_ptr(), PATH_MAX, c"%s/.debug/jit/%s.XXXXXXXX".as_ptr(), base, str_.as_ptr());
	if ret >= PATH_MAX as c_int {
		warnx(
			c"jvmti: cannot generate jit cache dir because %s/.debug/jit/%s.XXXXXXXX is too long, please check the cwd, JITDUMPDIR, and HOME variables".as_ptr(),
			base,
			str_.as_ptr(),
		);
		return -1;
	}
	p = mkdtemp(JIT_PATH.as_mut_ptr());
	if p != JIT_PATH.as_mut_ptr() {
		warn(c"jvmti: cannot create jit cache dir %s".as_ptr(), JIT_PATH.as_ptr());
		return -1;
	}

	0
}

unsafe fn perf_open_marker_file(fd: c_int) -> c_int {
	let pgsz: c_long;

	pgsz = sysconf(_SC_PAGESIZE);
	if pgsz == -1 {
		return -1;
	}

	/*
	 * we mmap the jitdump to create an MMAP RECORD in perf.data file.
	 * The mmap is captured either live (perf record running when we mmap)
	 * or  in deferred mode, via /proc/PID/maps
	 * the MMAP record is used as a marker of a jitdump file for more meta
	 * data info about the jitted code. Perf report/annotate detect this
	 * special filename and process the jitdump file.
	 *
	 * mapping must be PROT_EXEC to ensure it is captured by perf record
	 * even when not using -d option
	 */
	MARKER_ADDR = mmap(ptr::null_mut(), pgsz as usize, PROT_READ | PROT_EXEC, MAP_PRIVATE, fd, 0);
	if MARKER_ADDR == (-1isize as *mut c_void) { -1 } else { 0 }
}

unsafe fn perf_close_marker_file() {
	let pgsz: c_long;

	if MARKER_ADDR.is_null() {
		return;
	}

	pgsz = sysconf(_SC_PAGESIZE);
	if pgsz == -1 {
		return;
	}

	munmap(MARKER_ADDR, pgsz as usize);
}

unsafe fn init_arch_timestamp() {
	let str_ = getenv(c"JITDUMP_USE_ARCH_TIMESTAMP".as_ptr());

	if str_.is_null() || *str_ == 0 || strcmp(str_, c"0".as_ptr()) == 0 {
		return;
	}

	USE_ARCH_TIMESTAMP = 1;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jvmti_open() -> *mut c_void {
	let mut dump_path: [c_char; PATH_MAX] = [0; PATH_MAX];
	let mut header: jitheader = core::mem::zeroed();
	let fd: c_int;
	let mut ret: c_int;
	let fp: *mut FILE;

	init_arch_timestamp();

	/*
	 * check if clockid is supported
	 */
	if perf_get_timestamp() == 0 {
		if USE_ARCH_TIMESTAMP != 0 {
			warnx(c"jvmti: arch timestamp not supported".as_ptr());
		} else {
			warnx(c"jvmti: kernel does not support %d clock id".as_ptr(), PERF_CLK_ID);
		}
	}

	memset(&mut header as *mut jitheader as *mut c_void, 0, size_of::<jitheader>());

	/*
	 * jitdump file dir
	 */
	if create_jit_cache_dir() < 0 {
		return ptr::null_mut();
	}

	/*
	 * jitdump file name
	 */
	ret = snprintf(dump_path.as_mut_ptr(), PATH_MAX, c"%s/jit-%i.dump".as_ptr(), JIT_PATH.as_ptr(), getpid());
	if ret >= PATH_MAX as c_int {
		warnx(
			c"jvmti: cannot generate jitdump file full path because %s/jit-%i.dump is too long, please check the cwd, JITDUMPDIR, and HOME variables".as_ptr(),
			JIT_PATH.as_ptr(),
			getpid(),
		);
		return ptr::null_mut();
	}

	fd = open(dump_path.as_ptr(), O_CREAT | O_TRUNC | O_RDWR, 0o666);
	if fd == -1 {
		return ptr::null_mut();
	}

	/*
	 * create perf.data maker for the jitdump file
	 */
	if perf_open_marker_file(fd) != 0 {
		warnx(c"jvmti: failed to create marker file".as_ptr());
		return ptr::null_mut();
	}

	fp = fdopen(fd, c"w+".as_ptr());
	if fp.is_null() {
		warn(c"jvmti: cannot create %s".as_ptr(), dump_path.as_ptr());
		close(fd);
		fclose(fp);
		return ptr::null_mut();
	}

	warnx(c"jvmti: jitdump in %s".as_ptr(), dump_path.as_ptr());

	if get_e_machine(&mut header) != 0 {
		warn(c"get_e_machine failed\n".as_ptr());
		fclose(fp);
		return ptr::null_mut();
	}

	header.magic = JITHEADER_MAGIC;
	header.version = JITHEADER_VERSION;
	header.total_size = size_of::<jitheader>() as u32;
	header.pid = getpid() as u32;

	header.timestamp = perf_get_timestamp();

	if USE_ARCH_TIMESTAMP != 0 {
		header.flags |= JITDUMP_FLAGS_ARCH_TIMESTAMP as u64;
	}

	if fwrite(&header as *const jitheader as *const c_void, size_of::<jitheader>(), 1, fp) == 0 {
		warn(c"jvmti: cannot write dumpfile header".as_ptr());
		fclose(fp);
		return ptr::null_mut();
	}
	fp as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jvmti_close(agent: *mut c_void) -> c_int {
	let mut rec: jr_code_close = core::mem::zeroed();
	let mut fp = agent as *mut FILE;

	if fp.is_null() {
		warnx(c"jvmti: invalid fd in close_agent".as_ptr());
		return -1;
	}

	rec.p.id = JIT_CODE_CLOSE;
	rec.p.total_size = size_of::<jr_code_close>() as u32;

	rec.p.timestamp = perf_get_timestamp();

	if fwrite(&rec as *const jr_code_close as *const c_void, size_of::<jr_code_close>(), 1, fp) == 0 {
		return -1;
	}

	fclose(fp);

	fp = ptr::null_mut();

	perf_close_marker_file();

	0
}

static mut CODE_GENERATION: c_int = 1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jvmti_write_code(
	agent: *mut c_void,
	sym: *const c_char,
	vma: u64,
	code: *const c_void,
	size: c_uint,
) -> c_int {
	let mut rec: jr_code_load = core::mem::zeroed();
	let sym_len: usize;
	let fp = agent as *mut FILE;
	let mut ret: c_int = -1;

	/* don't care about 0 length function, no samples */
	if size == 0 {
		return 0;
	}

	if fp.is_null() {
		warnx(c"jvmti: invalid fd in write_native_code".as_ptr());
		return -1;
	}

	sym_len = strlen(sym) + 1;

	rec.p.id = JIT_CODE_LOAD;
	rec.p.total_size = (size_of::<jr_code_load>() + sym_len) as u32;
	rec.p.timestamp = perf_get_timestamp();

	rec.code_size = size as u64;
	rec.vma = vma;
	rec.code_addr = vma;
	rec.pid = getpid() as u32;
	rec.tid = gettid() as u32;

	if !code.is_null() {
		rec.p.total_size += size;
	}

	/*
	 * If JVM is multi-threaded, multiple concurrent calls to agent
	 * may be possible, so protect file writes
	 */
	flockfile(fp);

	/*
	 * get code index inside lock to avoid race condition
	 */
	rec.code_index = CODE_GENERATION as u64;
	CODE_GENERATION += 1;

	ret = fwrite_unlocked(&rec as *const jr_code_load as *const c_void, size_of::<jr_code_load>(), 1, fp) as c_int;
	fwrite_unlocked(sym as *const c_void, sym_len, 1, fp);

	if !code.is_null() {
		fwrite_unlocked(code, size as usize, 1, fp);
	}

	funlockfile(fp);

	ret = 0;

	ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jvmti_write_debug_info(
	agent: *mut c_void,
	code: u64,
	nr_lines: c_int,
	li: *mut jvmti_line_info_t,
	file_names: *const *const c_char,
) -> c_int {
	let mut rec: jr_code_debug_info = core::mem::zeroed();
	let mut sret: usize;
	let mut len: usize;
	let mut size: usize;
	let mut flen: usize = 0;
	let mut addr: u64;
	let fp = agent as *mut FILE;
	let mut i: c_int;

	/*
	 * no entry to write
	 */
	if nr_lines == 0 {
		return 0;
	}

	if fp.is_null() {
		warnx(c"jvmti: invalid fd in write_debug_info".as_ptr());
		return -1;
	}

	i = 0;
	while i < nr_lines {
		flen += strlen(*file_names.offset(i as isize)) + 1;
		i += 1;
	}

	rec.p.id = JIT_CODE_DEBUG_INFO;
	size = size_of::<jr_code_debug_info>();
	rec.p.timestamp = perf_get_timestamp();
	rec.code_addr = code as usize as u64;
	rec.nr_entry = nr_lines as u64;

	/*
	 * on disk source line info layout:
	 * uint64_t : addr
	 * int      : line number
	 * int      : column discriminator
	 * file[]   : source file name
	 */
	size += nr_lines as usize * size_of::<debug_entry>();
	size += flen;
	rec.p.total_size = size as u32;

	/*
	 * If JVM is multi-threaded, multiple concurrent calls to agent
	 * may be possible, so protect file writes
	 */
	flockfile(fp);

	sret = fwrite_unlocked(&rec as *const jr_code_debug_info as *const c_void, size_of::<jr_code_debug_info>(), 1, fp);
	if sret != 1 {
		funlockfile(fp);
		return -1;
	}

	i = 0;
	while i < nr_lines {
		addr = (*li.offset(i as isize)).pc as u64;
		len = size_of::<u64>();
		sret = fwrite_unlocked(&addr as *const u64 as *const c_void, len, 1, fp);
		if sret != 1 {
			funlockfile(fp);
			return -1;
		}

		len = size_of::<c_int>();
		sret = fwrite_unlocked(&(*li.offset(i as isize)).line_number as *const c_int as *const c_void, len, 1, fp);
		if sret != 1 {
			funlockfile(fp);
			return -1;
		}

		len = size_of::<c_int>();
		sret = fwrite_unlocked(&(*li.offset(i as isize)).discrim as *const c_int as *const c_void, len, 1, fp);
		if sret != 1 {
			funlockfile(fp);
			return -1;
		}

		sret = fwrite_unlocked(
			*file_names.offset(i as isize) as *const c_void,
			strlen(*file_names.offset(i as isize)) + 1,
			1,
			fp,
		);
		if sret != 1 {
			funlockfile(fp);
			return -1;
		}
		i += 1;
	}
	funlockfile(fp);
	0
}
