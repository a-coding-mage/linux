// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Memory Protection Keys management
 * Copyright (c) 2015, Intel Corporation.
 */

// Kernel headers and build-time configuration provide the following symbols.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct mm_context_t {
    pub execute_only_pkey: c_int,
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context_t,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_flags: usize,
    pub vm_mm: *mut mm_struct,
}

#[repr(C)]
pub struct file;

type SizeT = usize;
type SSizeT = isize;
type LoFF = i64;
type U32 = u32;

extern "C" {
    fn mm_pkey_alloc(mm: *mut mm_struct) -> c_int;
    fn mm_set_pkey_free(mm: *mut mm_struct, pkey: c_int);
    fn read_pkru() -> U32;
    fn __pkru_allows_read(pkru: U32, pkey: c_int) -> bool;
    fn arch_set_user_pkey_access(pkey: c_int, init_val: U32) -> c_int;
    fn vma_pkey(vma: *mut vm_area_struct) -> c_int;
    fn simple_read_from_buffer(
        user_buf: *mut c_void,
        count: SizeT,
        ppos: *mut LoFF,
        buf: *const c_void,
        len: SizeT,
    ) -> SSizeT;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: SizeT) -> usize;
    fn kstrtouint(s: *const c_char, base: c_uint, res: *mut U32) -> c_int;
    fn cpu_feature_enabled(feature: c_int) -> bool;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut c_void,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut c_void;
    fn default_llseek(file: *mut file, offset: LoFF, whence: c_int) -> LoFF;
}

type c_uint = u32;

const VM_ACCESS_FLAGS: usize = 0x7;
const VM_EXEC: usize = 0x4;
const ARCH_DEFAULT_PKEY: c_int = 0;
const PKEY_DISABLE_ACCESS: U32 = 0x1;
const PKRU_AD_BIT: U32 = 0x1;
const PKRU_WD_BIT: U32 = 0x2;
const PKRU_BITS_PER_PKEY: u32 = 2;
const EFAULT: SSizeT = 14;
const EINVAL: SSizeT = 22;
const S_IRUSR: c_uint = 0o400;
const S_IWUSR: c_uint = 0o200;
const X86_FEATURE_OSPKE: c_int = 0;

const fn pkru_ad_mask(pkey: U32) -> U32 {
    PKRU_AD_BIT << (pkey * PKRU_BITS_PER_PKEY)
}

pub unsafe fn __execute_only_pkey(mm: *mut mm_struct) -> c_int {
    let mut need_to_set_mm_pkey = false;
    let mut execute_only_pkey = (*mm).context.execute_only_pkey;
    let ret: c_int;

    /* Do we need to assign a pkey for mm's execute-only maps? */
    if execute_only_pkey == -1 {
        /* Go allocate one to use, which might fail */
        execute_only_pkey = mm_pkey_alloc(mm);
        if execute_only_pkey < 0 { return -1; }
        need_to_set_mm_pkey = true;
    }

    if !need_to_set_mm_pkey && !__pkru_allows_read(read_pkru(), execute_only_pkey) {
        return execute_only_pkey;
    }

    ret = arch_set_user_pkey_access(execute_only_pkey, PKEY_DISABLE_ACCESS);
    if ret != 0 {
        mm_set_pkey_free(mm, execute_only_pkey);
        return -1;
    }

    if need_to_set_mm_pkey { (*mm).context.execute_only_pkey = execute_only_pkey; }
    execute_only_pkey
}

unsafe fn vma_is_pkey_exec_only(vma: *mut vm_area_struct) -> bool {
    /* Do this check first since the vm_flags should be hot */
    if ((*vma).vm_flags & VM_ACCESS_FLAGS) != VM_EXEC { return false; }
    if vma_pkey(vma) != (*(*vma).vm_mm).context.execute_only_pkey { return false; }
    true
}

/* This is only called for *plain* mprotect calls. */
pub unsafe fn __arch_override_mprotect_pkey(vma: *mut vm_area_struct, prot: c_int, mut pkey: c_int) -> c_int {
    if pkey != -1 { return pkey; }
    if prot == 0x4 {
        pkey = __execute_only_pkey((*vma).vm_mm);
        if pkey > 0 { return pkey; }
    } else if vma_is_pkey_exec_only(vma) {
        return ARCH_DEFAULT_PKEY;
    }
    vma_pkey(vma)
}

pub static mut init_pkru_value: U32 =
    pkru_ad_mask(1) | pkru_ad_mask(2) | pkru_ad_mask(3) | pkru_ad_mask(4) |
    pkru_ad_mask(5) | pkru_ad_mask(6) | pkru_ad_mask(7) | pkru_ad_mask(8) |
    pkru_ad_mask(9) | pkru_ad_mask(10) | pkru_ad_mask(11) | pkru_ad_mask(12) |
    pkru_ad_mask(13) | pkru_ad_mask(14) | pkru_ad_mask(15);

#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, SizeT, *mut LoFF) -> SSizeT>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, SizeT, *mut LoFF) -> SSizeT>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, LoFF, c_int) -> LoFF>,
}

unsafe extern "C" fn init_pkru_read_file(_file: *mut file, user_buf: *mut c_char, count: SizeT, ppos: *mut LoFF) -> SSizeT {
    let mut buf = [0u8; 32];
    let len = sprintf_hex(&mut buf, init_pkru_value);
    simple_read_from_buffer(user_buf as *mut c_void, count, ppos, buf.as_ptr() as *const c_void, len)
}

unsafe extern "C" fn init_pkru_write_file(_file: *mut file, user_buf: *const c_char, count: SizeT, _ppos: *mut LoFF) -> SSizeT {
    let len = core::cmp::min(count, 31);
    let mut buf = [0u8; 32];
    if copy_from_user(buf.as_mut_ptr() as *mut c_void, user_buf as *const c_void, len) != 0 { return -EFAULT; }
    buf[len] = 0;
    let mut new_init_pkru = 0;
    if kstrtouint(buf.as_ptr() as *const c_char, 0, &mut new_init_pkru) != 0 { return -EINVAL; }
    if new_init_pkru & (PKRU_AD_BIT | PKRU_WD_BIT) != 0 { return -EINVAL; }
    core::ptr::write_volatile(&mut init_pkru_value, new_init_pkru);
    count as SSizeT
}

static fops_init_pkru: file_operations = file_operations { read: Some(init_pkru_read_file), write: Some(init_pkru_write_file), llseek: Some(default_llseek) };

unsafe fn sprintf_hex(buf: &mut [u8; 32], value: U32) -> SizeT {
    let s = alloc::format!("0x{:x}\n", value);
    let n = s.len(); buf[..n].copy_from_slice(s.as_bytes()); n
}

unsafe fn create_init_pkru_value() -> c_int {
    /* Do not expose the file if pkeys are not supported. */
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) { return 0; }
    let name = b"init_pkru\0";
    debugfs_create_file(name.as_ptr() as *const c_char, S_IRUSR | S_IWUSR,
        core::ptr::null_mut(), core::ptr::null_mut(), &fops_init_pkru);
    0
}

unsafe fn setup_init_pkru(opt: *mut c_char) -> c_int {
    let mut new_init_pkru = 0;
    if kstrtouint(opt as *const c_char, 0, &mut new_init_pkru) != 0 { return 1; }
    core::ptr::write_volatile(&mut init_pkru_value, new_init_pkru);
    1
}

// late_initcall(create_init_pkru_value);
// __setup("init_pkru=", setup_init_pkru);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
