/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The switch_tls_* definitions in the C header are ARM assembly macros and
 * are intentionally retained here as documentation of the assembly-side
 * interface.  Their behavior is selected by the build configuration.
 */

#[cfg(CONFIG_TLS_REG_EMUL)]
pub const TLS_EMU: usize = 1;
#[cfg(CONFIG_TLS_REG_EMUL)]
pub const HAS_TLS_REG: usize = 1;
#[cfg(CONFIG_TLS_REG_EMUL)]
pub const DEFER_TLS_REG_UPDATE: usize = 0;

#[cfg(all(not(CONFIG_TLS_REG_EMUL), CONFIG_CPU_V6))]
pub const TLS_EMU: usize = 0;
#[cfg(all(not(CONFIG_TLS_REG_EMUL), CONFIG_CPU_V6))]
pub const HAS_TLS_REG: usize = 0; // (elf_hwcap & HWCAP_TLS)
#[cfg(all(not(CONFIG_TLS_REG_EMUL), CONFIG_CPU_V6))]
pub const DEFER_TLS_REG_UPDATE: usize = 0; // is_smp()

#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), CONFIG_CPU_32V6K))]
pub const TLS_EMU: usize = 0;
#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), CONFIG_CPU_32V6K))]
pub const HAS_TLS_REG: usize = 1;
#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), CONFIG_CPU_32V6K))]
pub const DEFER_TLS_REG_UPDATE: usize = 1;

#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), not(CONFIG_CPU_32V6K)))]
pub const TLS_EMU: usize = 0;
#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), not(CONFIG_CPU_32V6K)))]
pub const HAS_TLS_REG: usize = 0;
#[cfg(all(not(CONFIG_TLS_REG_EMUL), not(CONFIG_CPU_V6), not(CONFIG_CPU_32V6K)))]
pub const DEFER_TLS_REG_UPDATE: usize = 0;

#[inline]
pub unsafe fn set_tls(val: usize) {
    let thread = current_thread_info();
    (*thread).tp_value[0] = val;

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    if TLS_EMU == 0 {
        if HAS_TLS_REG != 0 && DEFER_TLS_REG_UPDATE == 0 {
            core::arch::asm!("mcr p15, 0, {0}, c13, c0, 3", in(reg) val);
        } else if HAS_TLS_REG == 0 {
            #[cfg(CONFIG_KUSER_HELPERS)]
            {
                /* User space must use the helper at 0xffff0fe0. */
                core::ptr::write_volatile(0xffff0ff0 as *mut u32, val as u32);
            }
        }
    }
}

#[inline]
pub unsafe fn get_tpuser() -> usize {
    let mut reg: usize = 0;

    if HAS_TLS_REG != 0 && TLS_EMU == 0 {
        core::arch::asm!("mrc p15, 0, {0}, c13, c0, 2", out(reg) reg);
    }

    reg
}

#[inline]
pub unsafe fn set_tpuser(val: usize) {
    /* TPIDRURW is fully context-switched, so thread_info need not be updated. */
    if HAS_TLS_REG != 0 && TLS_EMU == 0 {
        core::arch::asm!("mcr p15, 0, {0}, c13, c0, 2", in(reg) val);
    }
}

#[inline]
pub unsafe fn flush_tls() {
    set_tls(0);
    set_tpuser(0);
}

/* Supplied by asm/thread_info.h. */
extern "C" {
    fn current_thread_info() -> *mut ThreadInfo;
}

#[repr(C)]
pub struct ThreadInfo {
    pub tp_value: [usize; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
