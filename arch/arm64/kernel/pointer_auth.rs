// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct ptrauth_key {
    pub value: [u8; 16],
}

#[repr(C)]
pub struct ptrauth_keys_user {
    pub apia: ptrauth_key,
    pub apib: ptrauth_key,
    pub apda: ptrauth_key,
    pub apdb: ptrauth_key,
    pub apga: ptrauth_key,
}

#[repr(C)]
pub struct thread_struct {
    pub keys_user: ptrauth_keys_user,
    pub sctlr_user: u64,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

extern "C" {
    static mut current: *mut task_struct;

    fn system_supports_address_auth() -> bool;
    fn system_supports_generic_auth() -> bool;
    fn task_thread_info(tsk: *mut task_struct) -> *mut core::ffi::c_void;
    fn is_compat_thread(thread_info: *mut core::ffi::c_void) -> bool;
    fn ptrauth_keys_init_user(keys: *mut ptrauth_keys_user);
    fn ptrauth_keys_install_user(keys: *mut ptrauth_keys_user);
    fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
    fn update_sctlr_el1(sctlr: u64);
    fn preempt_disable();
    fn preempt_enable();
    fn warn_on(condition: bool);
}

// Values supplied by asm/pointer_auth.h and the architecture headers.
extern "C" {
    static PR_PAC_APIAKEY: usize;
    static PR_PAC_APIBKEY: usize;
    static PR_PAC_APDAKEY: usize;
    static PR_PAC_APDBKEY: usize;
    static PR_PAC_APGAKEY: usize;
    static PR_PAC_ENABLED_KEYS_MASK: usize;
    static SCTLR_ELx_ENIA: u64;
    static SCTLR_ELx_ENIB: u64;
    static SCTLR_ELx_ENDA: u64;
    static SCTLR_ELx_ENDB: u64;
}

pub unsafe fn ptrauth_prctl_reset_keys(tsk: *mut task_struct, arg: usize) -> i32 {
    let keys = &mut (*tsk).thread.keys_user;
    let addr_key_mask = PR_PAC_APIAKEY
        | PR_PAC_APIBKEY
        | PR_PAC_APDAKEY
        | PR_PAC_APDBKEY;
    let key_mask = addr_key_mask | PR_PAC_APGAKEY;

    if !system_supports_address_auth() && !system_supports_generic_auth() {
        return -22;
    }

    if is_compat_thread(task_thread_info(tsk)) {
        return -22;
    }

    if arg == 0 {
        ptrauth_keys_init_user(keys);
        return 0;
    }

    if arg & !key_mask != 0 {
        return -22;
    }

    if ((arg & addr_key_mask != 0) && !system_supports_address_auth())
        || ((arg & PR_PAC_APGAKEY != 0) && !system_supports_generic_auth())
    {
        return -22;
    }

    if arg & PR_PAC_APIAKEY != 0 {
        get_random_bytes(&mut keys.apia as *mut _ as *mut _, core::mem::size_of_val(&keys.apia));
    }
    if arg & PR_PAC_APIBKEY != 0 {
        get_random_bytes(&mut keys.apib as *mut _ as *mut _, core::mem::size_of_val(&keys.apib));
    }
    if arg & PR_PAC_APDAKEY != 0 {
        get_random_bytes(&mut keys.apda as *mut _ as *mut _, core::mem::size_of_val(&keys.apda));
    }
    if arg & PR_PAC_APDBKEY != 0 {
        get_random_bytes(&mut keys.apdb as *mut _ as *mut _, core::mem::size_of_val(&keys.apdb));
    }
    if arg & PR_PAC_APGAKEY != 0 {
        get_random_bytes(&mut keys.apga as *mut _ as *mut _, core::mem::size_of_val(&keys.apga));
    }
    ptrauth_keys_install_user(keys);

    0
}

unsafe fn arg_to_enxx_mask(arg: usize) -> u64 {
    let mut sctlr_enxx_mask: u64 = 0;

    warn_on(arg & !PR_PAC_ENABLED_KEYS_MASK != 0);
    if arg & PR_PAC_APIAKEY != 0 {
        sctlr_enxx_mask |= SCTLR_ELx_ENIA;
    }
    if arg & PR_PAC_APIBKEY != 0 {
        sctlr_enxx_mask |= SCTLR_ELx_ENIB;
    }
    if arg & PR_PAC_APDAKEY != 0 {
        sctlr_enxx_mask |= SCTLR_ELx_ENDA;
    }
    if arg & PR_PAC_APDBKEY != 0 {
        sctlr_enxx_mask |= SCTLR_ELx_ENDB;
    }
    sctlr_enxx_mask
}

pub unsafe fn ptrauth_set_enabled_keys(
    tsk: *mut task_struct,
    keys: usize,
    enabled: usize,
) -> i32 {
    if !system_supports_address_auth() {
        return -22;
    }
    if is_compat_thread(task_thread_info(tsk)) {
        return -22;
    }
    if keys & !PR_PAC_ENABLED_KEYS_MASK != 0 || enabled & !keys != 0 {
        return -22;
    }

    preempt_disable();
    let mut sctlr = (*tsk).thread.sctlr_user;
    sctlr &= !arg_to_enxx_mask(keys);
    sctlr |= arg_to_enxx_mask(enabled);
    (*tsk).thread.sctlr_user = sctlr;
    if tsk == current {
        update_sctlr_el1(sctlr);
    }
    preempt_enable();

    0
}

pub unsafe fn ptrauth_get_enabled_keys(tsk: *mut task_struct) -> i32 {
    let mut retval: i32 = 0;

    if !system_supports_address_auth() {
        return -22;
    }
    if is_compat_thread(task_thread_info(tsk)) {
        return -22;
    }
    if (*tsk).thread.sctlr_user & SCTLR_ELx_ENIA != 0 {
        retval |= PR_PAC_APIAKEY as i32;
    }
    if (*tsk).thread.sctlr_user & SCTLR_ELx_ENIB != 0 {
        retval |= PR_PAC_APIBKEY as i32;
    }
    if (*tsk).thread.sctlr_user & SCTLR_ELx_ENDA != 0 {
        retval |= PR_PAC_APDAKEY as i32;
    }
    if (*tsk).thread.sctlr_user & SCTLR_ELx_ENDB != 0 {
        retval |= PR_PAC_APDBKEY as i32;
    }

    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
