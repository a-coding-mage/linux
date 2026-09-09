// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

extern "C" {
    fn current_thread_info() -> *mut thread_info;
    fn copy_to_user(to: *mut u8, from: *const u8, n: usize) -> i32;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> i32;
    fn fprs_write(value: u64);
    fn set_thread_wsaved(value: i32);
    fn synchronize_user_stack();
    fn get_thread_wsaved() -> i32;
}

extern "C" {
    fn __put_user_u64(value: u64, to: *mut u64) -> i32;
    fn __put_user_i32(value: i32, to: *mut i32) -> i32;
    fn __get_user_u64(from: *const u64, value: *mut u64) -> i32;
    fn __get_user_i32(from: *const i32, value: *mut i32) -> i32;
}

#[repr(C)]
pub struct pt_regs {
    pub tstate: u64,
}

#[repr(C)]
pub struct thread_info {
    pub fpregs: *mut u64,
    pub fpsaved: [u64; 1],
    pub xfsr: [u64; 1],
    pub gsr: [u64; 1],
    pub reg_window: [reg_window; NSWINS as usize],
    pub rwbuf_stkptrs: [u64; NSWINS as usize],
}

#[repr(C)]
pub struct reg_window {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct __siginfo_fpu_t {
    pub si_float_regs: [u32; 64],
    pub si_fsr: u64,
    pub si_gsr: u64,
    pub si_fprs: u64,
}

#[repr(C)]
pub struct __siginfo_rwin_t {
    pub wsaved: i32,
    pub reg_window: [reg_window; NSWINS as usize],
    pub rwbuf_stkptrs: [u64; NSWINS as usize],
}

pub const FPRS_DL: u64 = 1;
pub const FPRS_DU: u64 = 2;
pub const TSTATE_PEF: u64 = 1 << 32;
pub const NSWINS: i32 = 31;
pub const EFAULT: i32 = 14;

pub unsafe fn save_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32 {
    let ti = current_thread_info();
    let fpregs = (*ti).fpregs;
    let fprs: u64;
    let mut err: i32 = 0;

    let _ = regs;
    fprs = (*ti).fpsaved[0];
    if fprs & FPRS_DL != 0 {
        err |= copy_to_user(
            (*fpu).si_float_regs.as_mut_ptr() as *mut u8,
            fpregs as *const u8,
            core::mem::size_of::<u32>() * 32,
        );
    }
    if fprs & FPRS_DU != 0 {
        err |= copy_to_user(
            (*fpu).si_float_regs[32..].as_mut_ptr() as *mut u8,
            fpregs.add(16) as *const u8,
            core::mem::size_of::<u32>() * 32,
        );
    }
    err |= __put_user_u64((*ti).xfsr[0], &mut (*fpu).si_fsr);
    err |= __put_user_u64((*ti).gsr[0], &mut (*fpu).si_gsr);
    err |= __put_user_u64(fprs, &mut (*fpu).si_fprs);

    err
}

pub unsafe fn restore_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32 {
    let ti = current_thread_info();
    let fpregs = (*ti).fpregs;
    let mut fprs: u64 = 0;
    let mut err: i32;

    if (fpu as usize) & 7 != 0 {
        return -EFAULT;
    }

    err = __get_user_u64(&(*fpu).si_fprs, &mut fprs);
    fprs_write(0);
    (*regs).tstate &= !TSTATE_PEF;
    if fprs & FPRS_DL != 0 {
        err |= copy_from_user(
            fpregs as *mut u8,
            (*fpu).si_float_regs.as_ptr() as *const u8,
            core::mem::size_of::<u32>() * 32,
        );
    }
    if fprs & FPRS_DU != 0 {
        err |= copy_from_user(
            fpregs.add(16) as *mut u8,
            (*fpu).si_float_regs[32..].as_ptr() as *const u8,
            core::mem::size_of::<u32>() * 32,
        );
    }
    err |= __get_user_u64(&(*fpu).si_fsr, &mut (*ti).xfsr[0]);
    err |= __get_user_u64(&(*fpu).si_gsr, &mut (*ti).gsr[0]);
    (*ti).fpsaved[0] |= fprs;
    err
}

pub unsafe fn save_rwin_state(wsaved: i32, rwin: *mut __siginfo_rwin_t) -> i32 {
    let mut err = __put_user_i32(wsaved, &mut (*rwin).wsaved);
    let mut i = 0;
    while i < wsaved {
        let ti = current_thread_info();
        let rp = &(*ti).reg_window[i as usize] as *const reg_window;
        let fp = (*ti).rwbuf_stkptrs[i as usize];
        err |= copy_to_user(
            &mut (*rwin).reg_window[i as usize] as *mut reg_window as *mut u8,
            rp as *const u8,
            core::mem::size_of::<reg_window>(),
        );
        err |= __put_user_u64(fp, &mut (*rwin).rwbuf_stkptrs[i as usize]);
        i += 1;
    }
    err
}

pub unsafe fn restore_rwin_state(rp: *mut __siginfo_rwin_t) -> i32 {
    let t = current_thread_info();
    let mut wsaved: i32 = 0;

    if (rp as usize) & 7 != 0 {
        return -EFAULT;
    }
    let _ = __get_user_i32(&(*rp).wsaved, &mut wsaved);
    if wsaved > NSWINS {
        return -EFAULT;
    }

    let mut err = 0;
    let mut i = 0;
    while i < wsaved {
        err |= copy_from_user(
            &mut (*t).reg_window[i as usize] as *mut reg_window as *mut u8,
            &(*rp).reg_window[i as usize] as *const reg_window as *const u8,
            core::mem::size_of::<reg_window>(),
        );
        err |= __get_user_u64(
            &(*rp).rwbuf_stkptrs[i as usize],
            &mut (*t).rwbuf_stkptrs[i as usize],
        );
        i += 1;
    }
    if err != 0 {
        return err;
    }

    set_thread_wsaved(wsaved);
    synchronize_user_stack();
    if get_thread_wsaved() != 0 {
        return -EFAULT;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
