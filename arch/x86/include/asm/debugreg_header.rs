/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Define bits that are always set to 1 in DR7, only bit 10 is
 * architecturally reserved to '1'.
 *
 * This is also the init/reset value for DR7.
 */
pub const DR7_FIXED_1: usize = 0x0000_0400;

// DECLARE_PER_CPU(unsigned long, cpu_dr7);

#[inline(always)]
pub unsafe fn native_get_debugreg(regno: i32) -> usize {
    let mut val: usize;
    match regno {
        0 => core::arch::asm!("mov {}, dr0", out(reg) val),
        1 => core::arch::asm!("mov {}, dr1", out(reg) val),
        2 => core::arch::asm!("mov {}, dr2", out(reg) val),
        3 => core::arch::asm!("mov {}, dr3", out(reg) val),
        6 => core::arch::asm!("mov {}, dr6", out(reg) val),
        7 => {
            /* Use volatile assembly for DR7 reads to forbid re-ordering. */
            core::arch::asm!("mov {}, dr7", out(reg) val, options(nostack, preserves_flags));
        }
        _ => BUG(),
    }
    val
}

#[inline(always)]
pub unsafe fn native_set_debugreg(regno: i32, value: usize) {
    match regno {
        0 => core::arch::asm!("mov dr0, {}", in(reg) value),
        1 => core::arch::asm!("mov dr1, {}", in(reg) value),
        2 => core::arch::asm!("mov dr2, {}", in(reg) value),
        3 => core::arch::asm!("mov dr3, {}", in(reg) value),
        6 => core::arch::asm!("mov dr6, {}", in(reg) value),
        7 => {
            /* Use volatile assembly for DR7 writes to forbid re-ordering. */
            core::arch::asm!("mov dr7, {}", in(reg) value, options(nostack, preserves_flags));
        }
        _ => BUG(),
    }
}

#[inline]
pub unsafe fn hw_breakpoint_disable() {
    /* Reset the control register for HW Breakpoint */
    native_set_debugreg(7, DR7_FIXED_1);

    /* Zero-out the individual HW breakpoint address registers */
    native_set_debugreg(0, 0);
    native_set_debugreg(1, 0);
    native_set_debugreg(2, 0);
    native_set_debugreg(3, 0);
}

#[inline(always)]
pub unsafe fn hw_breakpoint_active() -> bool {
    __this_cpu_read(cpu_dr7) & DR_GLOBAL_ENABLE_MASK != 0
}

pub unsafe extern "C" fn hw_breakpoint_restore();

#[inline(always)]
pub unsafe fn local_db_save() -> usize {
    let mut dr7: usize;

    if cpu_feature_enabled(X86_FEATURE_HYPERVISOR) && !hw_breakpoint_active() {
        return 0;
    }

    dr7 = native_get_debugreg(7);

    /* Architecturally set bit */
    dr7 &= !DR7_FIXED_1;
    if dr7 != 0 {
        native_set_debugreg(7, DR7_FIXED_1);
    }

    /* Ensure the compiler doesn't lower the above statements into the critical section. */
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    dr7
}

#[inline(always)]
pub unsafe fn local_db_restore(dr7: usize) {
    /* Ensure the compiler doesn't raise this statement into the critical section. */
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    if dr7 != 0 {
        native_set_debugreg(7, dr7);
    }
}

#[cfg(feature = "CONFIG_CPU_SUP_AMD")]
pub unsafe extern "C" fn amd_set_dr_addr_mask(mask: usize, dr: u32);
#[cfg(feature = "CONFIG_CPU_SUP_AMD")]
pub unsafe extern "C" fn amd_get_dr_addr_mask(dr: u32) -> usize;

#[cfg(not(feature = "CONFIG_CPU_SUP_AMD"))]
#[inline]
pub unsafe fn amd_set_dr_addr_mask(_mask: usize, _dr: u32) {}

#[cfg(not(feature = "CONFIG_CPU_SUP_AMD"))]
#[inline]
pub unsafe fn amd_get_dr_addr_mask(_dr: u32) -> usize { 0 }

#[inline]
pub unsafe fn get_debugctlmsr() -> usize {
    let mut debugctlmsr: usize = 0;

    #[cfg(not(feature = "CONFIG_X86_DEBUGCTLMSR"))]
    if boot_cpu_data.x86 < 6 {
        return 0;
    }
    rdmsrq(MSR_IA32_DEBUGCTLMSR, &mut debugctlmsr);
    debugctlmsr
}

#[inline]
pub unsafe fn update_debugctlmsr(debugctlmsr: usize) {
    #[cfg(not(feature = "CONFIG_X86_DEBUGCTLMSR"))]
    if boot_cpu_data.x86 < 6 {
        return;
    }
    wrmsrq(MSR_IA32_DEBUGCTLMSR, debugctlmsr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
