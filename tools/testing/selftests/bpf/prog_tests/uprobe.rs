// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Hengqi Chen */

/* Translated from:
 * #include <test_progs.h>
 * #include <asm/ptrace.h>
 * #include "test_uprobe.skel.h"
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_uprobe_opts {
	pub sz: usize,
	pub retprobe: bool,
	pub bpf_cookie: u64,
	pub ref_ctr_offset: usize,
	pub func_name: *const c_char,
}

#[repr(C)]
pub struct test_uprobe_bss {
	pub my_pid: c_int,
	pub test1_result: c_int,
	pub test2_result: c_int,
	pub test3_result: c_int,
	pub test4_result: c_int,
	pub regs: pt_regs,
	pub ip: c_ulong,
}

#[repr(C)]
pub struct test_uprobe_progs {
	pub test4: *mut bpf_program,
	pub test_regs_change: *mut bpf_program,
	pub test_regs_change_ip: *mut bpf_program,
}

#[repr(C)]
pub struct test_uprobe_links {
	pub test4: *mut bpf_link,
	pub test_regs_change: *mut bpf_link,
	pub test_regs_change_ip: *mut bpf_link,
}

#[repr(C)]
pub struct test_uprobe {
	pub bss: *mut test_uprobe_bss,
	pub progs: test_uprobe_progs,
	pub links: test_uprobe_links,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
	pub r15: c_ulong,
	pub r14: c_ulong,
	pub r13: c_ulong,
	pub r12: c_ulong,
	pub rbp: c_ulong,
	pub rbx: c_ulong,
	pub r11: c_ulong,
	pub r10: c_ulong,
	pub r9: c_ulong,
	pub r8: c_ulong,
	pub rax: c_ulong,
	pub rcx: c_ulong,
	pub rdx: c_ulong,
	pub rsi: c_ulong,
	pub rdi: c_ulong,
	pub orig_rax: c_ulong,
	pub rip: c_ulong,
	pub cs: c_ulong,
	pub eflags: c_ulong,
	pub rsp: c_ulong,
	pub ss: c_ulong,
}

impl Default for pt_regs {
	fn default() -> Self {
		unsafe { MaybeUninit::<Self>::zeroed().assume_init() }
	}
}

unsafe extern "C" {
	static mut errno: c_int;

	fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
	fn pclose(stream: *mut FILE) -> c_int;
	fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn getpid() -> c_int;

	fn test_uprobe__open_and_load() -> *mut test_uprobe;
	fn test_uprobe__attach(skel: *mut test_uprobe) -> c_int;
	fn test_uprobe__destroy(skel: *mut test_uprobe);
	fn bpf_program__attach_uprobe_opts(
		prog: *mut bpf_program,
		pid: c_int,
		binary_path: *const c_char,
		func_offset: usize,
		opts: *const bpf_uprobe_opts,
	) -> *mut bpf_link;

	fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_ulong, expected: c_ulong, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
}

const EINVAL: c_int = 22;

unsafe fn libbpf_opts_bpf_uprobe_opts() -> bpf_uprobe_opts {
	MaybeUninit::<bpf_uprobe_opts>::zeroed().assume_init()
}

unsafe fn urand_spawn(pid: *mut c_int) -> *mut FILE {
	let f: *mut FILE;

	/* urandom_read's stdout is wired into f */
	f = popen(c"./urandom_read 1 report-pid".as_ptr(), c"r".as_ptr());
	if f.is_null() {
		return ptr::null_mut();
	}

	if fscanf(f, c"%d".as_ptr(), pid) != 1 {
		pclose(f);
		errno = EINVAL;
		return ptr::null_mut();
	}

	f
}

unsafe fn urand_trigger(urand_pipe: *mut *mut FILE) -> c_int {
	let exit_code: c_int;

	/* pclose() waits for child process to exit and returns their exit code */
	exit_code = pclose(*urand_pipe);
	*urand_pipe = ptr::null_mut();

	exit_code
}

unsafe fn test_uprobe_attach() {
	let mut uprobe_opts: bpf_uprobe_opts = libbpf_opts_bpf_uprobe_opts();
	let skel: *mut test_uprobe;
	let mut urand_pipe: *mut FILE = ptr::null_mut();
	let mut urand_pid: c_int = 0;
	let err: c_int;

	skel = test_uprobe__open_and_load();
	if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open".as_ptr()) {
		return;
	}

	urand_pipe = urand_spawn(&mut urand_pid);
	if !ASSERT_OK_PTR(urand_pipe as *mut c_void, c"urand_spawn".as_ptr()) {
		goto_cleanup(urand_pipe, skel);
		return;
	}

	(*(*skel).bss).my_pid = urand_pid;

	/* Manual attach uprobe to urandlib_api
	 * There are two `urandlib_api` symbols in .dynsym section:
	 *   - urandlib_api@LIBURANDOM_READ_1.0.0
	 *   - urandlib_api@@LIBURANDOM_READ_2.0.0
	 * Both are global bind and would cause a conflict if user
	 * specify the symbol name without a version suffix
	 */
	uprobe_opts.func_name = c"urandlib_api".as_ptr();
	(*skel).links.test4 = bpf_program__attach_uprobe_opts(
		(*skel).progs.test4,
		urand_pid,
		c"./liburandom_read.so".as_ptr(),
		0, /* offset */
		&uprobe_opts,
	);
	if !ASSERT_ERR_PTR((*skel).links.test4 as *mut c_void, c"urandlib_api_attach_conflict".as_ptr()) {
		goto_cleanup(urand_pipe, skel);
		return;
	}

	uprobe_opts.func_name = c"urandlib_api@LIBURANDOM_READ_1.0.0".as_ptr();
	(*skel).links.test4 = bpf_program__attach_uprobe_opts(
		(*skel).progs.test4,
		urand_pid,
		c"./liburandom_read.so".as_ptr(),
		0, /* offset */
		&uprobe_opts,
	);
	if !ASSERT_OK_PTR((*skel).links.test4 as *mut c_void, c"urandlib_api_attach_ok".as_ptr()) {
		goto_cleanup(urand_pipe, skel);
		return;
	}

	/* Auto attach 3 u[ret]probes to urandlib_api_sameoffset */
	err = test_uprobe__attach(skel);
	if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
		goto_cleanup(urand_pipe, skel);
		return;
	}

	/* trigger urandom_read */
	ASSERT_OK(urand_trigger(&mut urand_pipe), c"urand_exit_code".as_ptr());

	ASSERT_EQ((*(*skel).bss).test1_result as c_ulong, 1, c"urandlib_api_sameoffset".as_ptr());
	ASSERT_EQ((*(*skel).bss).test2_result as c_ulong, 1, c"urandlib_api_sameoffset@v1".as_ptr());
	ASSERT_EQ((*(*skel).bss).test3_result as c_ulong, 3, c"urandlib_api_sameoffset@@v2".as_ptr());
	ASSERT_EQ((*(*skel).bss).test4_result as c_ulong, 1, c"urandlib_api".as_ptr());

	goto_cleanup(urand_pipe, skel);
}

unsafe fn goto_cleanup(urand_pipe: *mut FILE, skel: *mut test_uprobe) {
	if !urand_pipe.is_null() {
		pclose(urand_pipe);
	}
	test_uprobe__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_regs_change_trigger() -> c_ulong {
	core::arch::naked_asm!("ret");
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn uprobe_regs_change(_before: *mut pt_regs, _after: *mut pt_regs) {
	core::arch::naked_asm!(
		"movq %r11,  48(%rdi)",
		"movq %r10,  56(%rdi)",
		"movq  %r9,  64(%rdi)",
		"movq  %r8,  72(%rdi)",
		"movq %rax,  80(%rdi)",
		"movq %rcx,  88(%rdi)",
		"movq %rdx,  96(%rdi)",
		"movq %rsi, 104(%rdi)",
		"movq %rdi, 112(%rdi)",

		/* save 2nd argument */
		"pushq %rsi",
		"call uprobe_regs_change_trigger",

		/* save  return value and load 2nd argument pointer to rax */
		"pushq %rax",
		"movq 8(%rsp), %rax",

		"movq %r11,  48(%rax)",
		"movq %r10,  56(%rax)",
		"movq  %r9,  64(%rax)",
		"movq  %r8,  72(%rax)",
		"movq %rcx,  88(%rax)",
		"movq %rdx,  96(%rax)",
		"movq %rsi, 104(%rax)",
		"movq %rdi, 112(%rax)",

		/* restore return value and 2nd argument */
		"pop %rax",
		"pop %rsi",

		"movq %rax,  80(%rsi)",
		"ret",
	);
}

#[cfg(target_arch = "x86_64")]
unsafe fn regs_common() {
	let mut before: pt_regs = pt_regs::default();
	let mut after: pt_regs = pt_regs::default();
	let expected: pt_regs = pt_regs {
		rax: 0xc0ffe,
		rcx: 0xbad,
		rdx: 0xdead,
		r8: 0x8,
		r9: 0x9,
		r10: 0x10,
		r11: 0x11,
		rdi: 0x12,
		rsi: 0x13,
		..pt_regs::default()
	};
	let mut uprobe_opts: bpf_uprobe_opts = libbpf_opts_bpf_uprobe_opts();
	let skel: *mut test_uprobe;

	skel = test_uprobe__open_and_load();
	if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open".as_ptr()) {
		return;
	}

	(*(*skel).bss).my_pid = getpid();
	(*(*skel).bss).regs = expected;

	uprobe_opts.func_name = c"uprobe_regs_change_trigger".as_ptr();
	(*skel).links.test_regs_change = bpf_program__attach_uprobe_opts(
		(*skel).progs.test_regs_change,
		-1,
		c"/proc/self/exe".as_ptr(),
		0, /* offset */
		&uprobe_opts,
	);
	if !ASSERT_OK_PTR(
		(*skel).links.test_regs_change as *mut c_void,
		c"bpf_program__attach_uprobe_opts".as_ptr(),
	) {
		test_uprobe__destroy(skel);
		return;
	}

	uprobe_regs_change(&mut before, &mut after);

	ASSERT_EQ(after.rax, expected.rax, c"ax".as_ptr());
	ASSERT_EQ(after.rcx, expected.rcx, c"cx".as_ptr());
	ASSERT_EQ(after.rdx, expected.rdx, c"dx".as_ptr());
	ASSERT_EQ(after.r8, expected.r8, c"r8".as_ptr());
	ASSERT_EQ(after.r9, expected.r9, c"r9".as_ptr());
	ASSERT_EQ(after.r10, expected.r10, c"r10".as_ptr());
	ASSERT_EQ(after.r11, expected.r11, c"r11".as_ptr());
	ASSERT_EQ(after.rdi, expected.rdi, c"rdi".as_ptr());
	ASSERT_EQ(after.rsi, expected.rsi, c"rsi".as_ptr());

	test_uprobe__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
unsafe fn uprobe_regs_change_ip_1() -> c_ulong {
	0xc0ffee
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
unsafe fn uprobe_regs_change_ip_2() -> c_ulong {
	0xdeadbeef
}

#[cfg(target_arch = "x86_64")]
unsafe fn regs_ip() {
	let mut uprobe_opts: bpf_uprobe_opts = libbpf_opts_bpf_uprobe_opts();
	let skel: *mut test_uprobe;
	let ret: c_ulong;

	skel = test_uprobe__open_and_load();
	if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open".as_ptr()) {
		return;
	}

	(*(*skel).bss).my_pid = getpid();
	(*(*skel).bss).ip = uprobe_regs_change_ip_2 as usize as c_ulong;

	uprobe_opts.func_name = c"uprobe_regs_change_ip_1".as_ptr();
	(*skel).links.test_regs_change_ip = bpf_program__attach_uprobe_opts(
		(*skel).progs.test_regs_change_ip,
		-1,
		c"/proc/self/exe".as_ptr(),
		0, /* offset */
		&uprobe_opts,
	);
	if !ASSERT_OK_PTR(
		(*skel).links.test_regs_change_ip as *mut c_void,
		c"bpf_program__attach_uprobe_opts".as_ptr(),
	) {
		test_uprobe__destroy(skel);
		return;
	}

	ret = uprobe_regs_change_ip_1();
	ASSERT_EQ(ret, 0xdeadbeef, c"ret".as_ptr());

	test_uprobe__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_regs_change() {
	if test__start_subtest(c"regs_change_common".as_ptr()) {
		regs_common();
	}
	if test__start_subtest(c"regs_change_ip".as_ptr()) {
		regs_ip();
	}
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn test_uprobe_regs_change() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_uprobe() {
	if test__start_subtest(c"attach".as_ptr()) {
		test_uprobe_attach();
	}
	test_uprobe_regs_change();
}
