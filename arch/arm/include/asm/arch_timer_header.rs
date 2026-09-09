/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/barrier.h, asm/errno.h, asm/hwcap.h, linux/clocksource.h,
// linux/init.h, linux/io-64-nonatomic-lo-hi.h, linux/types.h, and
// clocksource/arm_arch_timer.h.

#[cfg(CONFIG_ARM_ARCH_TIMER)]
pub const fn has_erratum_handler<H>(_h: H) -> bool {
    false
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
// C macro: erratum_handler(h) -> arch_timer_##h
// Rust callers should refer to the corresponding arch_timer_* symbol directly.

extern "C" {
    pub fn arch_timer_arch_init() -> core::ffi::c_int;
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline(always)]
pub unsafe fn arch_timer_reg_write_cp15(
    access: core::ffi::c_int,
    reg: enum_arch_timer_reg,
    val: u64,
) {
    if access == ARCH_TIMER_PHYS_ACCESS {
        match reg {
            ARCH_TIMER_REG_CTRL => {
                core::arch::asm!("mcr p15, 0, {0}, c14, c2, 1", in(reg) val as u32);
                isb();
            }
            ARCH_TIMER_REG_CVAL => {
                core::arch::asm!("mcrr p15, 2, {0}, c14", in(reg) val);
            }
            _ => BUILD_BUG(),
        }
    } else if access == ARCH_TIMER_VIRT_ACCESS {
        match reg {
            ARCH_TIMER_REG_CTRL => {
                core::arch::asm!("mcr p15, 0, {0}, c14, c3, 1", in(reg) val as u32);
                isb();
            }
            ARCH_TIMER_REG_CVAL => {
                core::arch::asm!("mcrr p15, 3, {0}, c14", in(reg) val);
            }
            _ => BUILD_BUG(),
        }
    } else {
        BUILD_BUG();
    }
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline(always)]
pub unsafe fn arch_timer_reg_read_cp15(
    access: core::ffi::c_int,
    reg: enum_arch_timer_reg,
) -> u32 {
    let mut val: u32 = 0;

    if access == ARCH_TIMER_PHYS_ACCESS {
        match reg {
            ARCH_TIMER_REG_CTRL => {
                core::arch::asm!("mrc p15, 0, {0}, c14, c2, 1", out(reg) val);
            }
            _ => BUILD_BUG(),
        }
    } else if access == ARCH_TIMER_VIRT_ACCESS {
        match reg {
            ARCH_TIMER_REG_CTRL => {
                core::arch::asm!("mrc p15, 0, {0}, c14, c3, 1", out(reg) val);
            }
            _ => BUILD_BUG(),
        }
    } else {
        BUILD_BUG();
    }

    val
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn arch_timer_get_cntfrq() -> u32 {
    let mut val: u32;
    core::arch::asm!("mrc p15, 0, {0}, c14, c0, 0", out(reg) val);
    val
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn __arch_counter_get_cntpct() -> u64 {
    let mut cval: u64;
    isb();
    core::arch::asm!("mrrc p15, 0, {0}, c14", out(reg) cval);
    cval
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn __arch_counter_get_cntpct_stable() -> u64 {
    __arch_counter_get_cntpct()
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn __arch_counter_get_cntvct() -> u64 {
    let mut cval: u64;
    isb();
    core::arch::asm!("mrrc p15, 1, {0}, c14", out(reg) cval);
    cval
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn __arch_counter_get_cntvct_stable() -> u64 {
    __arch_counter_get_cntvct()
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn arch_timer_get_cntkctl() -> u32 {
    let mut cntkctl: u32;
    core::arch::asm!("mrc p15, 0, {0}, c14, c1, 0", out(reg) cntkctl);
    cntkctl
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn arch_timer_set_cntkctl(cntkctl: u32) {
    core::arch::asm!("mcr p15, 0, {0}, c14, c1, 0", in(reg) cntkctl);
    isb();
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn arch_timer_set_evtstrm_feature() {
    elf_hwcap |= HWCAP_EVTSTRM;
}

#[cfg(CONFIG_ARM_ARCH_TIMER)]
#[inline]
pub unsafe fn arch_timer_have_evtstrm_feature() -> bool {
    (elf_hwcap & HWCAP_EVTSTRM) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
