// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[repr(C)]
pub struct regex_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regmatch_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct e_machine_and_e_flags {
	_private: [u8; 0],
}

#[repr(C)]
pub struct ins_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct objdump {
	pub comment_char: c_char,
	pub skip_functions_char: c_char,
}

#[repr(C)]
pub struct arch {
	pub name: *const c_char,
	pub id: e_machine_and_e_flags,
	pub objdump: objdump,
	pub associate_instruction_ops:
		Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
}

#[repr(C)]
pub struct arch_arm {
	pub arch: arch,
	pub call_insn: regex_t,
	pub jump_insn: regex_t,
}

extern "C" {
	static call_ops: ins_ops;
	static jump_ops: ins_ops;

	fn zalloc(size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
	fn regexec(
		preg: *const regex_t,
		string: *const c_char,
		nmatch: usize,
		pmatch: *mut regmatch_t,
		eflags: c_int,
	) -> c_int;
	fn regfree(preg: *mut regex_t);
	fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

extern "Rust" {
	static mut errno: c_int;
}

const REG_EXTENDED: c_int = 1;
const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP: c_int = 1;

unsafe extern "C" fn arm__associate_instruction_ops(
	arch: *mut arch,
	name: *const c_char,
) -> *const ins_ops {
	let arm = arch as *mut arch_arm;
	let ops: *const ins_ops;
	let mut match_: [MaybeUninit<regmatch_t>; 2] = MaybeUninit::uninit().assume_init();

	if regexec(
		&mut (*arm).call_insn,
		name,
		2,
		match_.as_mut_ptr() as *mut regmatch_t,
		0,
	) == 0
	{
		ops = &call_ops;
	} else if regexec(
		&mut (*arm).jump_insn,
		name,
		2,
		match_.as_mut_ptr() as *mut regmatch_t,
		0,
	) == 0
	{
		ops = &jump_ops;
	} else {
		return ptr::null();
	}

	arch__associate_ins_ops(arch, name, ops);
	ops
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_arm(
	id: *const e_machine_and_e_flags,
	_cpuid: *const c_char,
) -> *const arch {
	let mut err: c_int;
	let arm = zalloc(core::mem::size_of::<arch_arm>()) as *mut arch_arm;
	let arch: *mut arch;

	if arm.is_null() {
		return ptr::null();
	}

	arch = &mut (*arm).arch;
	(*arch).name = c"arm".as_ptr();
	ptr::copy_nonoverlapping(id, &mut (*arch).id, 1);
	(*arch).objdump.comment_char = b';' as c_char;
	(*arch).objdump.skip_functions_char = b'+' as c_char;
	(*arch).associate_instruction_ops = Some(arm__associate_instruction_ops);

	const ARM_CONDS: &str = "(cc|cs|eq|ge|gt|hi|le|ls|lt|mi|ne|pl|vc|vs)";
	err = regcomp(
		&mut (*arm).call_insn,
		c"^blx?(cc|cs|eq|ge|gt|hi|le|ls|lt|mi|ne|pl|vc|vs)?$".as_ptr(),
		REG_EXTENDED,
	);
	if err != 0 {
		goto_out_free_arm(arm);
		return ptr::null();
	}

	err = regcomp(
		&mut (*arm).jump_insn,
		c"^bx?(cc|cs|eq|ge|gt|hi|le|ls|lt|mi|ne|pl|vc|vs)?$".as_ptr(),
		REG_EXTENDED,
	);
	if err != 0 {
		regfree(&mut (*arm).call_insn);
		goto_out_free_arm(arm);
		return ptr::null();
	}
	let _ = ARM_CONDS;

	arch
}

unsafe fn goto_out_free_arm(arm: *mut arch_arm) {
	free(arm as *mut c_void);
	errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP;
}
