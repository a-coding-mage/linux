// SPDX-License-Identifier: GPL-2.0

/*
 * Original C++ dependency intent:
 *
 * Must come before the linux/compiler.h include, which defines several
 * macros (e.g. noinline) that conflict with compiler builtins used
 * by LLVM.
 *
 * #pragma GCC diagnostic push
 * #pragma GCC diagnostic ignored "-Wunused-parameter"  Needed for LLVM <= 15
 * #include <llvm/DebugInfo/Symbolize/Symbolize.h>
 * #include <llvm/Support/TargetSelect.h>
 * #pragma GCC diagnostic pop
 *
 * #include <inttypes.h>
 * #include <stdio.h>
 * #include <sys/types.h>
 * #include <linux/compiler.h>
 * extern "C" {
 * #include <linux/zalloc.h>
 * }
 * #include "llvm-c-helpers.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u64 = u64;

#[repr(C)]
pub struct dso {
	_private: [u8; 0],
}

#[repr(C)]
pub struct llvm_a2l_frame {
	pub filename: *mut c_char,
	pub funcname: *mut c_char,
	pub line: c_uint,
}

#[repr(C)]
struct LLVMSymbolizer {
	_private: [u8; 0],
}

#[repr(C)]
struct LLVMSymbolizerOptions {
	Demangle: bool,
}

#[repr(C)]
struct SectionedAddress {
	Address: u64,
	SectionIndex: u64,
}

#[repr(C)]
struct DILineInfo {
	FileName: CxxString,
	FunctionName: CxxString,
	Line: c_uint,
	StartAddress: OptionalU64,
}

#[repr(C)]
struct DIInliningInfo {
	_private: [u8; 0],
}

#[repr(C)]
struct DIGlobal {
	Name: CxxString,
	Start: u64,
}

#[repr(C)]
struct CxxString {
	_private: [u8; 0],
}

#[repr(C)]
struct OptionalU64 {
	_private: [u8; 0],
}

#[repr(C)]
struct ExpectedDILineInfo {
	_private: [u8; 0],
}

#[repr(C)]
struct ExpectedDIInliningInfo {
	_private: [u8; 0],
}

#[repr(C)]
struct ExpectedDIGlobal {
	_private: [u8; 0],
}

const UNDEF_SECTION: u64 = !0u64;

extern "C" {
	fn dso__demangle_sym(dso: *mut dso, kmodule: c_int, elf_name: *const c_char) -> *mut c_char;

	fn calloc(nmemb: usize, size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strdup(s: *const c_char) -> *mut c_char;
	fn zfree(ptr: *mut c_void);

	/* External C++/LLVM bindings required to model LLVMSymbolizer use. */
	fn LLVMSymbolizerOptions_new() -> LLVMSymbolizerOptions;
	fn LLVMSymbolizer_new(opts: *const LLVMSymbolizerOptions) -> *mut LLVMSymbolizer;
	fn LLVMSymbolizer_symbolizeInlinedCode(
		symbolizer: *mut LLVMSymbolizer,
		dso_name: *const c_char,
		addr: SectionedAddress,
	) -> ExpectedDIInliningInfo;
	fn LLVMSymbolizer_symbolizeCode(
		symbolizer: *mut LLVMSymbolizer,
		dso_name: *const c_char,
		addr: SectionedAddress,
	) -> ExpectedDILineInfo;
	fn LLVMSymbolizer_symbolizeData(
		symbolizer: *mut LLVMSymbolizer,
		dso_name: *const c_char,
		addr: SectionedAddress,
	) -> ExpectedDIGlobal;

	fn ExpectedDIInliningInfo_is_valid(res: *const ExpectedDIInliningInfo) -> bool;
	fn ExpectedDIInliningInfo_get(res: *mut ExpectedDIInliningInfo) -> *mut DIInliningInfo;
	fn ExpectedDILineInfo_is_valid(res: *const ExpectedDILineInfo) -> bool;
	fn ExpectedDILineInfo_get(res: *mut ExpectedDILineInfo) -> *mut DILineInfo;
	fn ExpectedDIGlobal_is_valid(res: *const ExpectedDIGlobal) -> bool;
	fn ExpectedDIGlobal_get(res: *mut ExpectedDIGlobal) -> *mut DIGlobal;

	fn DIInliningInfo_getNumberOfFrames(info: *const DIInliningInfo) -> c_uint;
	fn DIInliningInfo_getFrame(info: *const DIInliningInfo, index: c_uint) -> *const DILineInfo;

	fn CxxString_eq_cstr(s: *const CxxString, cstr: *const c_char) -> bool;
	fn CxxString_c_str(s: *const CxxString) -> *const c_char;
	fn OptionalU64_has_value(v: *const OptionalU64) -> bool;
	fn OptionalU64_value(v: *const OptionalU64) -> u64;
}

/*
 * Allocate a static LLVMSymbolizer, which will live to the end of the program.
 * Unlike the bfd paths, LLVMSymbolizer has its own cache, so we do not need
 * to store anything in the dso struct.
 */
unsafe fn get_symbolizer() -> *mut LLVMSymbolizer {
	static mut INSTANCE: *mut LLVMSymbolizer = ptr::null_mut();

	if INSTANCE.is_null() {
		let mut opts = LLVMSymbolizerOptions_new();
		/*
		 * LLVM sometimes demangles slightly different from the rest
		 * of the code, and this mismatch can cause new_inline_sym()
		 * to get confused and mark non-inline symbol as inlined
		 * (since the name does not properly match up with base_sym).
		 * Thus, disable the demangling and let the rest of the code
		 * handle it.
		 */
		opts.Demangle = false;
		INSTANCE = LLVMSymbolizer_new(&opts);
	}
	INSTANCE
}

/* Returns 0 on error, 1 on success. */
unsafe fn extract_file_and_line(
	line_info: *const DILineInfo,
	file: *mut *mut c_char,
	line: *mut c_uint,
) -> c_int {
	if !file.is_null() {
		if CxxString_eq_cstr(&(*line_info).FileName, c"<invalid>".as_ptr()) {
			/* Match the convention of libbfd. */
			*file = ptr::null_mut();
		} else {
			/* The caller expects to get something it can free(). */
			*file = strdup(CxxString_c_str(&(*line_info).FileName));
			if (*file).is_null() {
				return 0;
			}
		}
	}
	if !line.is_null() {
		*line = (*line_info).Line;
	}
	1
}

#[no_mangle]
pub unsafe extern "C" fn llvm_addr2line(
	dso_name: *const c_char,
	addr: u64,
	file: *mut *mut c_char,
	line: *mut c_uint,
	unwind_inlines: bool,
	inline_frames: *mut *mut llvm_a2l_frame,
) -> c_int {
	let symbolizer = get_symbolizer();
	let sectioned_addr = SectionedAddress {
		Address: addr,
		SectionIndex: UNDEF_SECTION,
	};

	if unwind_inlines {
		let mut res_or_err = LLVMSymbolizer_symbolizeInlinedCode(symbolizer, dso_name, sectioned_addr);
		if !ExpectedDIInliningInfo_is_valid(&res_or_err) {
			return 0;
		}
		let res = ExpectedDIInliningInfo_get(&mut res_or_err);
		let num_frames = DIInliningInfo_getNumberOfFrames(res);
		if num_frames == 0 {
			return 0;
		}

		if extract_file_and_line(DIInliningInfo_getFrame(res, 0), file, line) == 0 {
			return 0;
		}

		*inline_frames = calloc(
			num_frames as usize,
			core::mem::size_of::<llvm_a2l_frame>(),
		) as *mut llvm_a2l_frame;
		if (*inline_frames).is_null() {
			return 0;
		}

		let mut i: c_uint = 0;
		while i < num_frames {
			let src = DIInliningInfo_getFrame(res, i);
			let dst = (*inline_frames).add(i as usize);
			if CxxString_eq_cstr(&(*src).FileName, c"<invalid>".as_ptr()) {
				/* Match the convention of libbfd. */
				(*dst).filename = ptr::null_mut();
			} else {
				(*dst).filename = strdup(CxxString_c_str(&(*src).FileName));
			}
			(*dst).funcname = strdup(CxxString_c_str(&(*src).FunctionName));
			(*dst).line = (*src).Line;

			if (*dst).filename.is_null() || (*dst).funcname.is_null() {
				let mut j: c_uint = 0;
				while j <= i {
					zfree(&mut (*(*inline_frames).add(j as usize)).filename as *mut *mut c_char as *mut c_void);
					zfree(&mut (*(*inline_frames).add(j as usize)).funcname as *mut *mut c_char as *mut c_void);
					j += 1;
				}
				zfree(inline_frames as *mut c_void);
				return 0;
			}
			i += 1;
		}

		num_frames as c_int
	} else {
		if !inline_frames.is_null() {
			*inline_frames = ptr::null_mut();
		}

		let mut res_or_err = LLVMSymbolizer_symbolizeCode(symbolizer, dso_name, sectioned_addr);
		if !ExpectedDILineInfo_is_valid(&res_or_err) {
			return 0;
		}
		extract_file_and_line(ExpectedDILineInfo_get(&mut res_or_err), file, line)
	}
}

unsafe fn make_symbol_relative_string(
	dso: *mut dso,
	sym_name: *const c_char,
	addr: u64,
	base_addr: u64,
) -> *mut c_char {
	if strcmp(sym_name, c"<invalid>".as_ptr()) == 0 {
		return ptr::null_mut();
	}

	let demangled = dso__demangle_sym(dso, 0, sym_name);
	if base_addr != 0 && base_addr != addr {
		let mut buf = [0 as c_char; 256];
		snprintf(
			buf.as_mut_ptr(),
			buf.len(),
			c"%s+0x%lx".as_ptr(),
			if !demangled.is_null() { demangled as *const c_char } else { sym_name },
			addr.wrapping_sub(base_addr),
		);
		free(demangled as *mut c_void);
		strdup(buf.as_ptr())
	} else {
		if !demangled.is_null() {
			demangled
		} else {
			strdup(sym_name)
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn llvm_name_for_code(
	dso: *mut dso,
	dso_name: *const c_char,
	addr: u64,
) -> *mut c_char {
	let symbolizer = get_symbolizer();
	let sectioned_addr = SectionedAddress {
		Address: addr,
		SectionIndex: UNDEF_SECTION,
	};
	let mut res_or_err = LLVMSymbolizer_symbolizeCode(symbolizer, dso_name, sectioned_addr);
	if !ExpectedDILineInfo_is_valid(&res_or_err) {
		return ptr::null_mut();
	}
	let res = ExpectedDILineInfo_get(&mut res_or_err);
	make_symbol_relative_string(
		dso,
		CxxString_c_str(&(*res).FunctionName),
		addr,
		if OptionalU64_has_value(&(*res).StartAddress) {
			OptionalU64_value(&(*res).StartAddress)
		} else {
			0
		},
	)
}

#[no_mangle]
pub unsafe extern "C" fn llvm_name_for_data(
	dso: *mut dso,
	dso_name: *const c_char,
	addr: u64,
) -> *mut c_char {
	let symbolizer = get_symbolizer();
	let sectioned_addr = SectionedAddress {
		Address: addr,
		SectionIndex: UNDEF_SECTION,
	};
	let mut res_or_err = LLVMSymbolizer_symbolizeData(symbolizer, dso_name, sectioned_addr);
	if !ExpectedDIGlobal_is_valid(&res_or_err) {
		return ptr::null_mut();
	}
	let res = ExpectedDIGlobal_get(&mut res_or_err);
	make_symbol_relative_string(dso, CxxString_c_str(&(*res).Name), addr, (*res).Start)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
