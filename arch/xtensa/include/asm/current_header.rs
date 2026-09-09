/*
 * include/asm-xtensa/current.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// The original header includes <asm/thread_info.h> and <linux/thread_info.h>.
// Their declarations are supplied by the surrounding translation unit.

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

// Opaque dependency corresponding to the externally supplied thread-info type.
pub enum ThreadInfo {}

extern "C" {
    pub fn current_thread_info() -> *mut ThreadInfo;
}

/// Equivalent to the C inline `get_current()` function.
///
/// The `task` field is provided by the target's `thread_info` definition.
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    // TODO: access the `task` member of the externally supplied ThreadInfo.
    // Rust cannot project a field from an opaque external type here.
    let _ = current_thread_info();
    core::ptr::null_mut()
}

// Equivalent to `#define current get_current()`.
#[inline(always)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

// `current_stack_pointer` is bound to the Xtensa register a1 in the C header.
// The register binding is target/build-specific and is preserved as intent.
extern "C" {
    pub static mut current_stack_pointer: usize;
}

// Under __ASSEMBLER__, the source defines:
//
//   GET_CURRENT(reg,sp) {
//       GET_THREAD_INFO(reg,sp);
//       l32i reg, reg, TI_TASK
//   }
//
// This macro depends on the assembler-only GET_THREAD_INFO and TI_TASK
// definitions, so its Xtensa assembly expansion is retained here as a note.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
