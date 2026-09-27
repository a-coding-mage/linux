/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers are intentionally referenced
// but not implemented here.

#[repr(C)]
pub struct kimage;

#[repr(C)]
pub struct crash_mem {
    pub max_nr_ranges: ::kernel::ffi::c_uint,
    pub nr_ranges: ::kernel::ffi::c_uint,
    pub ranges: [range; 0], // __counted_by(max_nr_ranges)
}

// CONFIG_CRASH_DUMP
#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub fn crash_shrink_memory(new_size: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn crash_get_memory_size() -> isize;
}

// Default architecture hooks; architecture code may override these.
#[cfg(CONFIG_CRASH_DUMP)]
#[inline]
pub unsafe fn arch_kexec_protect_crashkres() {}

#[cfg(CONFIG_CRASH_DUMP)]
#[inline]
pub unsafe fn arch_kexec_unprotect_crashkres() {}

#[cfg(CONFIG_CRASH_DUMP)]
#[inline]
pub unsafe fn arch_crash_handle_hotplug_event(_image: *mut kimage, _arg: *mut ::kernel::ffi::c_void) {}

#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub fn crash_check_hotplug_support() -> ::kernel::ffi::c_int;
}

#[cfg(CONFIG_CRASH_DUMP)]
#[inline]
pub unsafe fn arch_crash_hotplug_support(
    _image: *mut kimage,
    _kexec_flags: ::kernel::ffi::c_ulong,
) -> ::kernel::ffi::c_int { 0 }

#[cfg(CONFIG_CRASH_DUMP)]
#[inline]
pub unsafe fn crash_get_elfcorehdr_size() -> ::kernel::ffi::c_uint { 0 }

/* Alignment required for elf header segment. */
pub const ELF_CORE_HEADER_ALIGN: ::kernel::ffi::c_uint = 4096;

#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub fn crash_exclude_mem_range(
        mem: *mut crash_mem,
        mstart: ::kernel::ffi::c_ulonglong,
        mend: ::kernel::ffi::c_ulonglong,
    ) -> ::kernel::ffi::c_int;
    pub fn crash_prepare_elf64_headers(
        mem: *mut crash_mem,
        need_kernel_map: ::kernel::ffi::c_int,
        addr: *mut *mut ::kernel::ffi::c_void,
        sz: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn crash_prepare_headers(
        need_kernel_map: ::kernel::ffi::c_int,
        addr: *mut *mut ::kernel::ffi::c_void,
        sz: *mut ::kernel::ffi::c_ulong,
        nr_mem_ranges: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn crash_exclude_core_ranges(cmem: *mut *mut crash_mem) -> ::kernel::ffi::c_int;
}

#[repr(C)]
pub struct kexec_segment;

pub const KEXEC_CRASH_HP_NONE: ::kernel::ffi::c_uint = 0;
pub const KEXEC_CRASH_HP_ADD_CPU: ::kernel::ffi::c_uint = 1;
pub const KEXEC_CRASH_HP_REMOVE_CPU: ::kernel::ffi::c_uint = 2;
pub const KEXEC_CRASH_HP_ADD_MEMORY: ::kernel::ffi::c_uint = 3;
pub const KEXEC_CRASH_HP_REMOVE_MEMORY: ::kernel::ffi::c_uint = 4;
pub const KEXEC_CRASH_HP_INVALID_CPU: ::kernel::ffi::c_uint = !0;

#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub fn __crash_kexec(regs: *mut pt_regs);
    pub fn crash_kexec(regs: *mut pt_regs);
    pub fn kexec_should_crash(p: *mut task_struct) -> ::kernel::ffi::c_int;
    pub fn kexec_crash_loaded() -> ::kernel::ffi::c_int;
    pub fn crash_save_cpu(regs: *mut pt_regs, cpu: ::kernel::ffi::c_int);
    pub fn kimage_crash_copy_vmcoreinfo(image: *mut kimage) -> ::kernel::ffi::c_int;
    pub fn arch_get_system_nr_ranges() -> ::kernel::ffi::c_uint;
    pub fn arch_crash_populate_cmem(cmem: *mut crash_mem) -> ::kernel::ffi::c_int;
    pub fn arch_crash_exclude_ranges(cmem: *mut crash_mem) -> ::kernel::ffi::c_int;
    pub fn arch_crash_exclude_mem_range(
        mem: *mut *mut crash_mem,
        mstart: ::kernel::ffi::c_ulonglong,
        mend: ::kernel::ffi::c_ulonglong,
    ) -> ::kernel::ffi::c_int;
}

#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn __crash_kexec(_regs: *mut pt_regs) {}
#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn crash_kexec(_regs: *mut pt_regs) {}
#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn kexec_should_crash(_p: *mut task_struct) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn kexec_crash_loaded() -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn crash_save_cpu(_regs: *mut pt_regs, _cpu: ::kernel::ffi::c_int) {}
#[cfg(not(CONFIG_CRASH_DUMP))]
#[inline]
pub unsafe fn kimage_crash_copy_vmcoreinfo(_image: *mut kimage) -> ::kernel::ffi::c_int { 0 }

// CONFIG_CRASH_DM_CRYPT
#[cfg(CONFIG_CRASH_DM_CRYPT)]
extern "C" {
    pub fn crash_load_dm_crypt_keys(image: *mut kimage) -> ::kernel::ffi::c_int;
    pub fn dm_crypt_keys_read(
        buf: *mut ::kernel::ffi::c_char,
        count: usize,
        ppos: *mut u64,
    ) -> isize;
}

#[cfg(not(CONFIG_CRASH_DM_CRYPT))]
#[inline]
pub unsafe fn crash_load_dm_crypt_keys(_image: *mut kimage) -> ::kernel::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
