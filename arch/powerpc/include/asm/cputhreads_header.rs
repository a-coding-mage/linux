/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Mapping of threads to cores
 *
 * Note: This implementation is limited to a power of 2 number of
 * threads per core and the same number for each core in the system
 * (though it would work if some processors had less threads as long
 * as the CPU numbers are still allocated, just not brought online).
 *
 * However, the API allows for a different implementation in the future
 * if needed, as long as you only use the functions and not the variables
 * directly.
 */

#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut threads_per_core: kernel::ffi::c_int;
    pub static mut threads_per_subcore: kernel::ffi::c_int;
    pub static mut threads_shift: kernel::ffi::c_int;
    pub static mut threads_core_mask: cpumask_t;
}

#[cfg(not(CONFIG_SMP))]
pub const threads_per_core: kernel::ffi::c_int = 1;
#[cfg(not(CONFIG_SMP))]
pub const threads_per_subcore: kernel::ffi::c_int = 1;
#[cfg(not(CONFIG_SMP))]
pub const threads_shift: kernel::ffi::c_int = 0;
#[cfg(not(CONFIG_SMP))]
pub const has_big_cores: kernel::ffi::c_int = 0;

/* Non-SMP builds use (*get_cpu_mask(0)) for threads_core_mask. */

extern "C" {
    pub static mut nr_cpu_ids: kernel::ffi::c_int;
    pub fn cpu_has_feature(feature: kernel::ffi::c_int) -> bool;
    pub fn mfspr(spr: kernel::ffi::c_int) -> u32;
}

pub const CPU_FTR_ARCH_300: kernel::ffi::c_int = 0;
pub const CPU_FTR_SMT: kernel::ffi::c_int = 0;
pub const SPRN_TENSR: kernel::ffi::c_int = 0;

pub type u32 = kernel::ffi::c_uint;
pub type cpumask_t = kernel::ffi::c_ulong;

pub const INVALID_THREAD_HWID: kernel::ffi::c_int = 0x0fff;

#[inline]
pub unsafe fn cpu_nr_cores() -> kernel::ffi::c_int {
    nr_cpu_ids >> threads_shift
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn cpu_core_index_of_thread(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn cpu_first_thread_of_core(core: kernel::ffi::c_int) -> kernel::ffi::c_int;
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub const fn cpu_core_index_of_thread(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    cpu
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub const fn cpu_first_thread_of_core(core: kernel::ffi::c_int) -> kernel::ffi::c_int {
    core
}

#[inline]
pub unsafe fn cpu_thread_in_core(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    cpu & (threads_per_core - 1)
}

#[inline]
pub unsafe fn cpu_thread_in_subcore(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    cpu & (threads_per_subcore - 1)
}

#[inline]
pub unsafe fn cpu_first_thread_sibling(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    cpu & !(threads_per_core - 1)
}

#[inline]
pub unsafe fn cpu_last_thread_sibling(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    cpu | (threads_per_core - 1)
}

/*
 * tlb_thread_siblings are siblings which share a TLB. This is not
 * architected, is not something a hypervisor could emulate and a future
 * CPU may change behaviour even in compat mode, so this should only be
 * used on PowerNV, and only with care.
 */
#[inline]
pub unsafe fn cpu_first_tlb_thread_sibling(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    if cpu_has_feature(CPU_FTR_ARCH_300) && threads_per_core == 8 {
        cpu & !0x6 /* Big Core */
    } else {
        cpu_first_thread_sibling(cpu)
    }
}

#[inline]
pub unsafe fn cpu_last_tlb_thread_sibling(cpu: kernel::ffi::c_int) -> kernel::ffi::c_int {
    if cpu_has_feature(CPU_FTR_ARCH_300) && threads_per_core == 8 {
        cpu | 0x6 /* Big Core */
    } else {
        cpu_last_thread_sibling(cpu)
    }
}

#[inline]
pub unsafe fn cpu_tlb_thread_sibling_step() -> kernel::ffi::c_int {
    if cpu_has_feature(CPU_FTR_ARCH_300) && threads_per_core == 8 {
        2 /* Big Core */
    } else {
        1
    }
}

#[inline]
pub unsafe fn get_tensr() -> u32 {
    #[cfg(CONFIG_BOOKE)]
    if cpu_has_feature(CPU_FTR_SMT) {
        return mfspr(SPRN_TENSR);
    }
    1
}

extern "C" {
    pub fn book3e_start_thread(thread: kernel::ffi::c_int, addr: kernel::ffi::c_ulong);
    pub fn book3e_stop_thread(thread: kernel::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
