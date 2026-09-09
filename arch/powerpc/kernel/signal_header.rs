/* SPDX-License-Identifier: GPL-2.0-or-later
 *
 * Copyright (c) 2007 Benjamin Herrenschmidt, IBM Corporation
 * Extracted from signal_32.c and signal_64.c
 */

// C header dependencies and build-time configuration are supplied externally.

pub unsafe extern "C" {
    pub fn get_sigframe(ksig: *mut ksignal, tsk: *mut task_struct,
                        frame_size: usize, is_32: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn handle_signal32(ksig: *mut ksignal, oldset: *mut sigset_t,
                           tsk: *mut task_struct) -> ::core::ffi::c_int;
    pub fn handle_rt_signal32(ksig: *mut ksignal, oldset: *mut sigset_t,
                              tsk: *mut task_struct) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_VSX)]
#[macro_export]
macro_rules! unsafe_copy_fpr_to_user_vsx {
    ($to:expr, $task:expr, $label:ident) => {{ let __t = $task; let buf = $to as *mut u64; let mut i = 0; while i < ELF_NFPREG - 1 { unsafe_put_user!((*__t).thread.TS_FPR(i), buf.add(i), $label); i += 1; } unsafe_put_user!((*__t).thread.fp_state.fpscr, buf.add(i), $label); }};
}
#[cfg(CONFIG_VSX)]
#[macro_export]
macro_rules! unsafe_copy_vsx_to_user { ($to:expr, $task:expr, $label:ident) => {{ let __t = $task; let buf = $to as *mut u64; let mut i = 0; while i < ELF_NVSRHALFREG { unsafe_put_user!((*__t).thread.fp_state.fpr[i][TS_VSRLOWOFFSET], buf.add(i), $label); i += 1; } }}; }
#[cfg(CONFIG_VSX)]
#[macro_export]
macro_rules! unsafe_copy_fpr_from_user_vsx { ($task:expr, $from:expr, $label:ident) => {{ let __t = $task; let buf = $from as *mut u64; let mut i = 0; while i < ELF_NFPREG - 1 { unsafe_get_user!((*__t).thread.TS_FPR(i), buf.add(i), $label); i += 1; } unsafe_get_user!((*__t).thread.fp_state.fpscr, buf.add(i), $label); }}; }
#[cfg(CONFIG_VSX)]
#[macro_export]
macro_rules! unsafe_copy_vsx_from_user { ($task:expr, $from:expr, $label:ident) => {{ let __t = $task; let buf = $from as *mut u64; let mut i = 0; while i < ELF_NVSRHALFREG { unsafe_get_user!((*__t).thread.fp_state.fpr[i][TS_VSRLOWOFFSET], buf.add(i), $label); i += 1; } }}; }

#[cfg(CONFIG_PPC_FPU_REGS)]
#[inline]
pub unsafe fn copy_fpr_to_user(to: *mut ::core::ffi::c_void, task: *mut task_struct) -> ::core::ffi::c_ulong {
    __copy_to_user(to, (*task).thread.fp_state.fpr, ELF_NFPREG * core::mem::size_of::<f64>())
}
#[cfg(CONFIG_PPC_FPU_REGS)]
#[inline]
pub unsafe fn copy_fpr_from_user(task: *mut task_struct, from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    __copy_from_user((*task).thread.fp_state.fpr, from, ELF_NFPREG * core::mem::size_of::<f64>())
}

#[inline]
pub unsafe fn __get_user_sigset(dst: *mut sigset_t, src: *const sigset_t) -> ::core::ffi::c_int {
    // BUILD_BUG_ON(sizeof(sigset_t) != sizeof(u64));
    __get_user((*dst).sig[0], &(*src).sig[0] as *const u64)
}

#[macro_export]
macro_rules! unsafe_get_user_sigset {
    ($dst:expr, $src:expr, $label:ident) => {{
        let __dst = $dst;
        let __src = $src;
        let mut i = 0;
        while i < _NSIG_WORDS {
            unsafe_get_user!((*__dst).sig[i], &(*__src).sig[i], $label);
            i += 1;
        }
    }};
}

#[cfg(CONFIG_VSX)]
pub unsafe extern "C" {
    pub fn copy_vsx_to_user(to: *mut ::core::ffi::c_void, task: *mut task_struct) -> ::core::ffi::c_ulong;
    pub fn copy_ckvsx_to_user(to: *mut ::core::ffi::c_void, task: *mut task_struct) -> ::core::ffi::c_ulong;
    pub fn copy_vsx_from_user(task: *mut task_struct, from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    pub fn copy_ckvsx_from_user(task: *mut task_struct, from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    pub fn copy_fpr_to_user(to: *mut ::core::ffi::c_void, task: *mut task_struct) -> ::core::ffi::c_ulong;
    pub fn copy_ckfpr_to_user(to: *mut ::core::ffi::c_void, task: *mut task_struct) -> ::core::ffi::c_ulong;
    pub fn copy_fpr_from_user(task: *mut task_struct, from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    pub fn copy_ckfpr_from_user(task: *mut task_struct, from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
}

#[cfg(CONFIG_PPC_FPU_REGS)]
#[macro_export]
macro_rules! unsafe_copy_fpr_to_user { ($to:expr, $task:expr, $label:ident) => { unsafe_copy_to_user!($to, (*$task).thread.fp_state.fpr, ELF_NFPREG * core::mem::size_of::<f64>(), $label) }; }
#[cfg(CONFIG_PPC_FPU_REGS)]
#[macro_export]
macro_rules! unsafe_copy_fpr_from_user { ($task:expr, $from:expr, $label:ident) => { unsafe_copy_from_user!((*$task).thread.fp_state.fpr, $from, ELF_NFPREG * core::mem::size_of::<f64>(), $label) }; }

#[cfg(not(any(CONFIG_VSX, CONFIG_PPC_FPU_REGS)))]
#[inline] pub unsafe fn copy_fpr_to_user(_to: *mut ::core::ffi::c_void, _task: *mut task_struct) -> ::core::ffi::c_ulong { 0 }
#[cfg(not(any(CONFIG_VSX, CONFIG_PPC_FPU_REGS)))]
#[inline] pub unsafe fn copy_fpr_from_user(_task: *mut task_struct, _from: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong { 0 }

#[cfg(CONFIG_PPC64)]
pub unsafe extern "C" { pub fn handle_rt_signal64(ksig: *mut ksignal, set: *mut sigset_t, tsk: *mut task_struct) -> ::core::ffi::c_int; }
#[cfg(not(CONFIG_PPC64))]
#[inline] pub unsafe fn handle_rt_signal64(_ksig: *mut ksignal, _set: *mut sigset_t, _tsk: *mut task_struct) -> ::core::ffi::c_int { -EFAULT }

pub unsafe extern "C" {
    pub fn signal_fault(tsk: *mut task_struct, regs: *mut pt_regs,
                        where_: *const ::core::ffi::c_char,
                        ptr: *mut ::core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
