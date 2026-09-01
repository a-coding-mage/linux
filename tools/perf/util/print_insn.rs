// SPDX-License-Identifier: GPL-2.0
/*
 * Instruction binary disassembler based on capstone.
 *
 * Author(s): Changbin Du <changbin.du@huawei.com>
 */

use std::ffi::{c_int, c_uchar, c_void};

pub type u8 = c_uchar;
pub type uint8_t = c_uchar;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;

pub const EM_S390: uint16_t = 22;
pub const EM_X86_64: uint16_t = 62;
pub const EM_AARCH64: uint16_t = 183;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
	pub insn_len: c_int,
	pub insn: *mut i8,
	pub cpumode: u8,
	pub ip: uint64_t,
}

#[repr(C)]
pub struct machine {
	pub env: *mut perf_env,
}

#[repr(C)]
pub struct addr_location {
	pub map: *mut map,
}

#[repr(C)]
pub struct thread {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dso {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
	_private: [u8; 0],
}

#[repr(C)]
pub struct map {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn fprintf(fp: *mut FILE, format: *const i8, ...) -> c_int;
	fn map__dso(map: *mut map) -> *const dso;
	fn dso__is_64_bit(dso: *const dso) -> bool;
	fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut c_void) -> uint16_t;
	fn capstone__fprintf_insn_asm(
		machine: *mut machine,
		thread: *mut thread,
		cpumode: u8,
		is64bit: bool,
		code: *const uint8_t,
		code_size: size_t,
		ip: uint64_t,
		lenp: *mut c_int,
		print_opts: c_int,
		fp: *mut FILE,
	) -> ssize_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sample__fprintf_insn_raw(sample: *mut perf_sample, fp: *mut FILE) -> size_t {
	let mut printed: c_int = 0;

	let mut i: c_int = 0;
	while i < (*sample).insn_len {
		printed += fprintf(
			fp,
			c"%02x".as_ptr(),
			*(*sample).insn.offset(i as isize) as c_uchar as c_int,
		);
		if (*sample).insn_len - i > 1 {
			printed += fprintf(fp, c" ".as_ptr());
		}
		i += 1;
	}
	printed as size_t
}

unsafe fn is64bitip(machine: *mut machine, al: *mut addr_location) -> bool {
	let dso: *const dso = if !(*al).map.is_null() {
		map__dso((*al).map)
	} else {
		std::ptr::null()
	};
	let e_machine: uint16_t;

	if !dso.is_null() {
		return dso__is_64_bit(dso);
	}

	e_machine = perf_env__e_machine((*machine).env, std::ptr::null_mut()); /*e_flags=*/ /* NULL */
	e_machine == EM_X86_64 || e_machine == EM_AARCH64 || e_machine == EM_S390
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf_insn_asm(
	machine: *mut machine,
	thread: *mut thread,
	cpumode: u8,
	is64bit: bool,
	code: *const uint8_t,
	code_size: size_t,
	ip: uint64_t,
	lenp: *mut c_int,
	print_opts: c_int,
	fp: *mut FILE,
) -> ssize_t {
	capstone__fprintf_insn_asm(
		machine, thread, cpumode, is64bit, code, code_size, ip, lenp, print_opts, fp,
	)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sample__fprintf_insn_asm(
	sample: *mut perf_sample,
	thread: *mut thread,
	machine: *mut machine,
	fp: *mut FILE,
	al: *mut addr_location,
) -> size_t {
	let is64bit: bool = is64bitip(machine, al);
	let printed: ssize_t;

	printed = fprintf_insn_asm(
		machine,
		thread,
		(*sample).cpumode,
		is64bit,
		(*sample).insn as *mut uint8_t,
		(*sample).insn_len as size_t,
		(*sample).ip,
		std::ptr::null_mut(),
		0,
		fp,
	);
	if printed < 0 {
		return sample__fprintf_insn_raw(sample, fp);
	}

	printed as size_t
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
