// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and PowerPC architecture headers.

extern "C" {
    static mut text_mutex: core::ffi::c_void;
    static mut __static_call_return0: *mut core::ffi::c_void;

    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn is_offset_in_branch_range(offset: isize) -> bool;
    fn patch_instruction(site: *mut core::ffi::c_void, instruction: u32) -> i32;
    fn patch_branch(site: *mut core::ffi::c_void, target: usize, flags: i32) -> i32;
    fn patch_ulong(site: *mut core::ffi::c_void, value: usize) -> i32;
    fn panic(format: *const u8, ...);
}

// Architecture-provided constants and instruction constructors.
extern "C" {
    static PPC_SCT_RET0: usize;
    static PPC_SCT_DATA: usize;
    static _R3: i32;
    static BRANCH_SET_LINK: i32;
}

unsafe extern "C" {
    fn PPC_RAW_BLR() -> u32;
    fn PPC_RAW_NOP() -> u32;
    fn PPC_RAW_LI(reg: i32, value: i32) -> u32;
    fn ppc_inst(instruction: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn arch_static_call_transform(
    site: *mut core::ffi::c_void,
    tramp: *mut core::ffi::c_void,
    func: *mut core::ffi::c_void,
    tail: bool,
) {
    let mut err: i32;
    let is_ret0 = func == __static_call_return0;
    let _tramp = tramp as usize;
    let _func = func as usize;
    let _ret0 = _tramp + PPC_SCT_RET0;
    let base = if !site.is_null() { site } else { tramp };
    let is_short = is_offset_in_branch_range(
        (func as isize).wrapping_sub(base as isize),
    );

    mutex_lock(&mut text_mutex as *mut _ as *mut core::ffi::c_void);

    if !site.is_null() && tail {
        if func.is_null() {
            err = patch_instruction(site, ppc_inst(PPC_RAW_BLR()));
        } else if is_ret0 {
            err = patch_branch(site, _ret0, 0);
        } else if is_short {
            err = patch_branch(site, _func, 0);
        } else if !tramp.is_null() {
            err = patch_branch(site, _tramp, 0);
        } else {
            err = 0;
        }
    } else if !site.is_null() {
        if func.is_null() {
            err = patch_instruction(site, ppc_inst(PPC_RAW_NOP()));
        } else if is_ret0 {
            err = patch_instruction(site, ppc_inst(PPC_RAW_LI(_R3, 0)));
        } else if is_short {
            err = patch_branch(site, _func, BRANCH_SET_LINK);
        } else if !tramp.is_null() {
            err = patch_branch(site, _tramp, BRANCH_SET_LINK);
        } else {
            err = 0;
        }
    } else if !tramp.is_null() {
        if !func.is_null() && !is_short {
            err = patch_ulong(
                (tramp as *mut u8).add(PPC_SCT_DATA) as *mut core::ffi::c_void,
                _func,
            );
            if err != 0 {
                goto_out(err, func, tramp);
                return;
            }
        }

        if func.is_null() {
            err = patch_instruction(tramp, ppc_inst(PPC_RAW_BLR()));
        } else if is_ret0 {
            err = patch_branch(tramp, _ret0, 0);
        } else if is_short {
            err = patch_branch(tramp, _func, 0);
        } else {
            err = patch_instruction(tramp, ppc_inst(PPC_RAW_NOP()));
        }
    } else {
        err = 0;
    }

    mutex_unlock(&mut text_mutex as *mut _ as *mut core::ffi::c_void);

    if err != 0 {
        panic(b"%s: patching failed %pS at %pS\0".as_ptr(), func, tramp);
    }
}

unsafe fn goto_out(
    err: i32,
    func: *mut core::ffi::c_void,
    tramp: *mut core::ffi::c_void,
) {
    mutex_unlock(&mut text_mutex as *mut _ as *mut core::ffi::c_void);
    if err != 0 {
        panic(b"%s: patching failed %pS at %pS\0".as_ptr(), func, tramp);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
