/*
 * Timer device implementation for SGI UV platform.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2009 Silicon Graphics, Inc.  All rights reserved.
 */

// Linux kernel dependencies supplied by other translation units.

pub const UV_MMTIMER_NAME: &[u8] = b"mmtimer\0";
pub const UV_MMTIMER_DESC: &[u8] = b"SGI UV Memory Mapped RTC Timer\0";
pub const UV_MMTIMER_VERSION: &[u8] = b"1.0\0";

extern "C" {
    static mut uv_mmtimer_femtoperiod: ::core::ffi::c_ulong;
    static mut sn_rtc_cycles_per_second: ::core::ffi::c_ulong;

    fn uv_get_min_hub_revision_id() -> ::core::ffi::c_int;
    fn uv_blade_processor_id() -> ::core::ffi::c_ulong;
    fn uv_local_mmr_address(reg: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    fn is_uv_system() -> bool;
    fn copy_to_user(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        n: usize,
    ) -> ::core::ffi::c_ulong;
    fn hweight64(x: u64) -> ::core::ffi::c_int;
    fn pgprot_noncached(prot: usize) -> usize;
    fn remap_pfn_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        pfn: ::core::ffi::c_ulong,
        size: usize,
        prot: usize,
    ) -> ::core::ffi::c_int;
    fn misc_register(dev: *mut miscdevice) -> ::core::ffi::c_int;
    fn printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: ::core::ffi::c_ulong,
    pub vm_end: ::core::ffi::c_ulong,
    pub vm_flags: ::core::ffi::c_ulong,
    pub vm_page_prot: usize,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut ::core::ffi::c_void,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> ::core::ffi::c_int>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, u32, ::core::ffi::c_ulong) -> ::core::ffi::c_long>,
    pub llseek: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct miscdevice {
    pub minor: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub fops: *const file_operations,
}

pub const MMTIMER_GETOFFSET: u32 = 0;
pub const MMTIMER_GETRES: u32 = 1;
pub const MMTIMER_GETFREQ: u32 = 2;
pub const MMTIMER_GETBITS: u32 = 3;
pub const MMTIMER_MMAPAVAIL: u32 = 4;
pub const MMTIMER_GETCOUNTER: u32 = 5;
pub const UVH_RTC: ::core::ffi::c_ulong = 0;
pub const UVH_RTC_REAL_TIME_CLOCK_MASK: u64 = 0xffff_ffff_ffff_ffff;
pub const UV_LOCAL_MMR_BASE: ::core::ffi::c_ulong = 0;
pub const L1_CACHE_BYTES: ::core::ffi::c_ulong = 64;
pub const PAGE_SIZE: ::core::ffi::c_ulong = 4096;
pub const VM_WRITE: ::core::ffi::c_ulong = 0x0000_0002;

static UV_MMTIMER_FOPS: file_operations = file_operations {
    owner: core::ptr::null_mut(),
    mmap: Some(uv_mmtimer_mmap),
    unlocked_ioctl: Some(uv_mmtimer_ioctl),
    llseek: core::ptr::null_mut(),
};

unsafe extern "C" fn uv_mmtimer_ioctl(
    _file: *mut file,
    cmd: u32,
    arg: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    let mut ret: ::core::ffi::c_long = 0;

    match cmd {
        MMTIMER_GETOFFSET => {
            if uv_get_min_hub_revision_id() == 1 {
                ret = 0;
            } else {
                ret = (((uv_blade_processor_id() * L1_CACHE_BYTES) % PAGE_SIZE) / 8) as _;
            }
        }
        MMTIMER_GETRES => {
            if copy_to_user(
                arg as *mut ::core::ffi::c_void,
                &uv_mmtimer_femtoperiod as *const _ as *const ::core::ffi::c_void,
                core::mem::size_of::<::core::ffi::c_ulong>(),
            ) != 0 { ret = -14; }
        }
        MMTIMER_GETFREQ => {
            if copy_to_user(
                arg as *mut ::core::ffi::c_void,
                &sn_rtc_cycles_per_second as *const _ as *const ::core::ffi::c_void,
                core::mem::size_of::<::core::ffi::c_ulong>(),
            ) != 0 { ret = -14; }
        }
        MMTIMER_GETBITS => ret = hweight64(UVH_RTC_REAL_TIME_CLOCK_MASK) as _,
        MMTIMER_MMAPAVAIL => ret = 1,
        MMTIMER_GETCOUNTER => {
            if copy_to_user(
                arg as *mut ::core::ffi::c_void,
                uv_local_mmr_address(UVH_RTC) as *const ::core::ffi::c_void,
                core::mem::size_of::<::core::ffi::c_ulong>(),
            ) != 0 { ret = -14; }
        }
        _ => ret = -25,
    }
    ret
}

unsafe extern "C" fn uv_mmtimer_mmap(
    _file: *mut file,
    vma: *mut vm_area_struct,
) -> ::core::ffi::c_int {
    if (*vma).vm_end - (*vma).vm_start != PAGE_SIZE { return -22; }
    if (*vma).vm_flags & VM_WRITE != 0 { return -1; }
    if PAGE_SIZE > (1 << 16) { return -38; }

    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    let mut uv_mmtimer_addr = UV_LOCAL_MMR_BASE | UVH_RTC;
    uv_mmtimer_addr &= !(PAGE_SIZE - 1);
    uv_mmtimer_addr &= 0x0fff_ffff_ffff_ffff;

    if remap_pfn_range(vma, (*vma).vm_start, uv_mmtimer_addr >> 12,
                      PAGE_SIZE as usize, (*vma).vm_page_prot) != 0 {
        return -11;
    }
    0
}

static mut UV_MMTIMER_MISCDEV: miscdevice = miscdevice {
    minor: 255,
    name: UV_MMTIMER_NAME.as_ptr() as *const _,
    fops: &UV_MMTIMER_FOPS,
};

unsafe extern "C" fn uv_mmtimer_init() -> ::core::ffi::c_int {
    if !is_uv_system() { return -1; }
    if sn_rtc_cycles_per_second < 100000 { return -1; }

    uv_mmtimer_femtoperiod = ((1_000_000_000_000_000u64
        + sn_rtc_cycles_per_second as u64 / 2)
        / sn_rtc_cycles_per_second as u64) as _;

    if misc_register(&mut UV_MMTIMER_MISCDEV) != 0 { return -1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
