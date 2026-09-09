// SPDX-License-Identifier: GPL-2.0+
/*
 * Originally from efivars.c
 *
 * Copyright (C) 2001,2003,2004 Dell <Matt_Domsch@ dell.com>
 * Copyright (C) 2004 Intel Corporation <matthew.e.tolentino@ intel.com>
 */

// C dependencies supplied by the surrounding kernel tree are intentionally
// referenced here rather than implemented in this translation unit.

type U32 = u32;
type U64 = u64;
type EfiStatus = usize;
type EfiChar16 = u16;

#[repr(C)]
pub struct EfiGuid {
    _opaque: [u8; 16],
}

#[repr(C)]
pub struct Efivars {
    pub ops: *const EfivarOperations,
}

pub type GetVariable = unsafe extern "C" fn(
    *mut EfiChar16, *mut EfiGuid, *mut U32, *mut usize, *mut core::ffi::c_void,
) -> EfiStatus;
pub type GetNextVariable = unsafe extern "C" fn(
    *mut usize, *mut EfiChar16, *mut EfiGuid,
) -> EfiStatus;
pub type SetVariable = unsafe extern "C" fn(
    *mut EfiChar16, *mut EfiGuid, U32, usize, *mut core::ffi::c_void,
) -> EfiStatus;
pub type QueryVariableStore = unsafe extern "C" fn(U32, usize, bool) -> EfiStatus;
pub type QueryVariableInfo = unsafe extern "C" fn(U32, *mut U64, *mut U64, *mut U64) -> EfiStatus;

#[repr(C)]
pub struct EfivarOperations {
    pub query_variable_store: Option<QueryVariableStore>,
    pub get_variable: GetVariable,
    pub get_next_variable: GetNextVariable,
    pub set_variable: SetVariable,
    pub set_variable_nonblocking: Option<SetVariable>,
    pub query_variable_info: Option<QueryVariableInfo>,
}

extern "C" {
    static mut efivars_lock: core::ffi::c_void;
    static mut efivar_ops_nh: core::ffi::c_void;
    fn down_interruptible(lock: *mut core::ffi::c_void) -> i32;
    fn down_trylock(lock: *mut core::ffi::c_void) -> i32;
    fn up(lock: *mut core::ffi::c_void);
    fn blocking_notifier_call_chain(
        chain: *mut core::ffi::c_void,
        event: i32,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn ucs2_strsize(name: *const EfiChar16, max: usize) -> usize;
}

const EFI_SUCCESS: EfiStatus = 0;
const EFI_UNSUPPORTED: EfiStatus = 3;
const EFI_OUT_OF_RESOURCES: EfiStatus = 9;
const EFI_ABORTED: EfiStatus = 0x8000_0000_0000_0015;
const SZ_64K: usize = 64 * 1024;
const EFI_VAR_NAME_LEN: usize = 1024;
const EFIVAR_OPS_RDWR: i32 = 0;
const EFIVAR_OPS_RDONLY: i32 = 1;

static mut __EFIVARS: *mut Efivars = core::ptr::null_mut();

unsafe fn check_var_size(nonblocking: bool, attributes: U32, size: usize) -> EfiStatus {
    let fops = (*__EFIVARS).ops;
    let status = match (*fops).query_variable_store {
        None => EFI_UNSUPPORTED,
        Some(query) => query(attributes, size, nonblocking),
    };
    if status == EFI_UNSUPPORTED {
        if size <= SZ_64K { EFI_SUCCESS } else { EFI_OUT_OF_RESOURCES }
    } else {
        status
    }
}

pub unsafe extern "C" fn efivar_is_available() -> bool {
    !__EFIVARS.is_null()
}

pub unsafe extern "C" fn efivars_register(
    efivars: *mut Efivars,
    ops: *const EfivarOperations,
) -> i32 {
    if down_interruptible(&mut efivars_lock) != 0 { return -4; }
    let rv;
    if !__EFIVARS.is_null() {
        rv = -16;
    } else {
        (*efivars).ops = ops;
        __EFIVARS = efivars;
        let event = if efivar_supports_writes() { EFIVAR_OPS_RDWR } else { EFIVAR_OPS_RDONLY };
        blocking_notifier_call_chain(&mut efivar_ops_nh, event, core::ptr::null_mut());
        rv = 0;
    }
    up(&mut efivars_lock);
    rv
}

pub unsafe extern "C" fn efivars_unregister(efivars: *mut Efivars) -> i32 {
    if down_interruptible(&mut efivars_lock) != 0 { return -4; }
    let rv;
    if __EFIVARS.is_null() || __EFIVARS != efivars { rv = -22; }
    else { __EFIVARS = core::ptr::null_mut(); rv = 0; }
    up(&mut efivars_lock);
    rv
}

pub unsafe extern "C" fn efivar_supports_writes() -> bool {
    !__EFIVARS.is_null() && (*(*__EFIVARS).ops).set_variable as usize != 0
}

pub unsafe extern "C" fn efivar_lock() -> i32 {
    if down_interruptible(&mut efivars_lock) != 0 { return -4; }
    if (*__EFIVARS).ops.is_null() { up(&mut efivars_lock); return -19; }
    0
}

pub unsafe extern "C" fn efivar_trylock() -> i32 {
    if down_trylock(&mut efivars_lock) != 0 { return -16; }
    if (*__EFIVARS).ops.is_null() { up(&mut efivars_lock); return -19; }
    0
}

pub unsafe extern "C" fn efivar_unlock() { up(&mut efivars_lock); }

pub unsafe extern "C" fn efivar_get_variable(name: *mut EfiChar16, vendor: *mut EfiGuid, attr: *mut U32, size: *mut usize, data: *mut core::ffi::c_void) -> EfiStatus {
    ((*(*__EFIVARS).ops).get_variable)(name, vendor, attr, size, data)
}

pub unsafe extern "C" fn efivar_get_next_variable(name_size: *mut usize, name: *mut EfiChar16, vendor: *mut EfiGuid) -> EfiStatus {
    ((*(*__EFIVARS).ops).get_next_variable)(name_size, name, vendor)
}

pub unsafe extern "C" fn efivar_set_variable_locked(name: *mut EfiChar16, vendor: *mut EfiGuid, attr: U32, data_size: usize, data: *mut core::ffi::c_void, nonblocking: bool) -> EfiStatus {
    if data_size > 0 {
        let status = check_var_size(nonblocking, attr, data_size + ucs2_strsize(name, EFI_VAR_NAME_LEN));
        if status != EFI_SUCCESS { return status; }
    }
    let setvar = (*(*__EFIVARS).ops).set_variable_nonblocking
        .filter(|_| nonblocking)
        .unwrap_or((*(*__EFIVARS).ops).set_variable);
    setvar(name, vendor, attr, data_size, data)
}

pub unsafe extern "C" fn efivar_set_variable(name: *mut EfiChar16, vendor: *mut EfiGuid, attr: U32, data_size: usize, data: *mut core::ffi::c_void) -> EfiStatus {
    if efivar_lock() != 0 { return EFI_ABORTED; }
    let status = efivar_set_variable_locked(name, vendor, attr, data_size, data, false);
    efivar_unlock();
    status
}

pub unsafe extern "C" fn efivar_query_variable_info(attr: U32, storage_space: *mut U64, remaining_space: *mut U64, max_variable_size: *mut U64) -> EfiStatus {
    match (*(*__EFIVARS).ops).query_variable_info {
        None => EFI_UNSUPPORTED,
        Some(query) => query(attr, storage_space, remaining_space, max_variable_size),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
