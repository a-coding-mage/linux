/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/processor.h
 *
 *  Copyright (C) 1995-1999 Russell King
 */

/* C header guard and includes omitted; referenced kernel symbols are external dependencies. */

/* Build-time configuration condition: defined when building the kernel. */

/* STACK_TOP depends on the current task and kernel configuration. */
macro_rules! STACK_TOP {
    () => { if ((*current).personality & ADDR_LIMIT_32BIT) != 0 { TASK_SIZE } else { TASK_SIZE_26 } };
}
macro_rules! STACK_TOP_MAX { () => { TASK_SIZE }; }

#[repr(C)]
pub struct debug_info {
    #[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")]
    pub hbp: [*mut perf_event; ARM_MAX_HBP_SLOTS],
}

#[repr(C)]
pub struct thread_struct {
    /* fault info */
    pub address: ::core::ffi::c_ulong,
    pub trap_no: ::core::ffi::c_ulong,
    pub error_code: ::core::ffi::c_ulong,
    /* debugging */
    pub debug: debug_info,
}

/* Everything usercopied to/from thread_struct is statically-sized, so no
 * hardened usercopy whitelist is needed. */
#[inline]
pub unsafe fn arch_thread_struct_whitelist(
    offset: *mut ::core::ffi::c_ulong,
    size: *mut ::core::ffi::c_ulong,
) {
    *offset = 0;
    *size = 0;
}

/* INIT_THREAD { } */
macro_rules! INIT_THREAD { () => { thread_struct { address: 0, trap_no: 0, error_code: 0, debug: debug_info { } } }; }

/* The C start_thread statement expression, retained as a macro because its
 * arguments and surrounding kernel state are supplied by dependent headers. */
macro_rules! start_thread {
    ($regs:expr, $pc:expr, $sp:expr) => {{
        let mut r7: ::core::ffi::c_ulong = 0;
        let mut r8: ::core::ffi::c_ulong = 0;
        let mut r9: ::core::ffi::c_ulong = 0;
        if cfg!(feature = "CONFIG_BINFMT_ELF_FDPIC") {
            r7 = $regs.ARM_r7;
            r8 = $regs.ARM_r8;
            r9 = $regs.ARM_r9;
        }
        unsafe {
            core::ptr::write_bytes($regs.uregs.as_mut_ptr(), 0, $regs.uregs.len());
        }
        if cfg!(feature = "CONFIG_BINFMT_ELF_FDPIC") && ((*current).personality & FDPIC_FUNCPTRS) != 0 {
            $regs.ARM_r7 = r7;
            $regs.ARM_r8 = r8;
            $regs.ARM_r9 = r9;
            $regs.ARM_r10 = (*(*current).mm).start_data;
        } else if !cfg!(feature = "CONFIG_MMU") {
            $regs.ARM_r10 = (*(*current).mm).start_data;
        }
        if ((*current).personality & ADDR_LIMIT_32BIT) != 0 { $regs.ARM_cpsr = USR_MODE; }
        else { $regs.ARM_cpsr = USR26_MODE; }
        if (elf_hwcap & HWCAP_THUMB) != 0 && ($pc & 1) != 0 { $regs.ARM_cpsr |= PSR_T_BIT; }
        if cfg!(feature = "CONFIG_CPU_ENDIAN_BE8") { $regs.ARM_cpsr |= PSR_E_BIT; }
        $regs.ARM_pc = $pc & !1;
        $regs.ARM_sp = $sp;
    }};
}

/* Forward declaration, a strange C thing. */
pub struct task_struct;
extern "C" { pub fn __get_wchan(p: *mut task_struct) -> ::core::ffi::c_ulong; }

macro_rules! task_pt_regs { ($p:expr) => { ((THREAD_START_SP + task_stack_page($p)) as *mut pt_regs).offset(-1) }; }
macro_rules! KSTK_EIP { ($tsk:expr) => { (*task_pt_regs!($tsk)).ARM_pc }; }
macro_rules! KSTK_ESP { ($tsk:expr) => { (*task_pt_regs!($tsk)).ARM_sp }; }

/* CONFIG_SMP selects the alternate SMP assembly string. */
#[cfg(feature = "CONFIG_SMP")]
macro_rules! __ALT_SMP_ASM { ($smp:expr, $up:expr) => { concat!("9998: ", $smp, "\n\t.pushsection \".alt.smp.init\", \"a\"\n\t.align\t2\n\t.long\t9998b - .\n\t", $up, "\n\t.popsection\n") }; }
#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! __ALT_SMP_ASM { ($smp:expr, $up:expr) => { $up }; }

/* Prefetching support is available only for ARMv5 and later. */
#[cfg(any(feature = "ARM_ARCH_5", feature = "ARM_ARCH_6", feature = "ARM_ARCH_7"))]
#[inline]
pub unsafe fn prefetch(ptr: *const ::core::ffi::c_void) {
    core::arch::asm!("pld {0}", in(reg) ptr, options(nostack));
}

#[cfg(all(feature = "ARM_ARCH_7", feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn prefetchw(ptr: *const ::core::ffi::c_void) {
    core::arch::asm!("pldw {0}", in(reg) ptr, options(nostack));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
