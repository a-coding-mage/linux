// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/powerpc/kernel/pmc.c
 *
 *  Copyright (C) 2004 David Gibson, IBM Corporation.
 *  Includes code formerly from arch/ppc/kernel/perfmon.c:
 *    Author: Andy Fleming
 *    Copyright (c) 2004 Freescale Semiconductor, Inc
 */

// Linux and PowerPC declarations supplied by the surrounding translation unit.

#[cfg(not(any()))]
const MMCR0_PMAO: u64 = 0;

unsafe fn dummy_perf(regs: *mut pt_regs) {
    let _ = regs;

    #[cfg(CONFIG_FSL_EMB_PERFMON)]
    {
        mtpmr(PMRN_PMGC0, mfpmr(PMRN_PMGC0) & !PMGC0_PMIE);
    }
    #[cfg(any(CONFIG_PPC64, CONFIG_PPC_BOOK3S_32))]
    {
        if (*cur_cpu_spec).pmc_type == PPC_PMC_IBM {
            mtspr(
                SPRN_MMCR0,
                mfspr(SPRN_MMCR0) & !(MMCR0_PMXE | MMCR0_PMAO),
            );
        }
    }
    #[cfg(not(any(CONFIG_FSL_EMB_PERFMON, CONFIG_PPC64, CONFIG_PPC_BOOK3S_32)))]
    {
        mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & !MMCR0_PMXE);
    }
}

static mut pmc_owner_lock: RawSpinLock = RawSpinLock::new();
static mut pmc_owner_caller: *mut core::ffi::c_void = core::ptr::null_mut();
static mut perf_irq: perf_irq_t = dummy_perf;

pub unsafe fn reserve_pmc_hardware(new_perf_irq: perf_irq_t) -> i32 {
    let mut err: i32 = 0;

    raw_spin_lock(&raw mut pmc_owner_lock);

    if !pmc_owner_caller.is_null() {
        printk(
            KERN_WARNING,
            c"reserve_pmc_hardware: PMC hardware busy (reserved by caller %p)\n",
            pmc_owner_caller,
        );
        err = -EBUSY;
        raw_spin_unlock(&raw mut pmc_owner_lock);
        return err;
    } else {
        pmc_owner_caller = __builtin_return_address(0);
        perf_irq = if new_perf_irq != 0 { new_perf_irq } else { dummy_perf };
    }

    raw_spin_unlock(&raw mut pmc_owner_lock);
    err
}

pub unsafe fn release_pmc_hardware() {
    raw_spin_lock(&raw mut pmc_owner_lock);

    WARN_ON(pmc_owner_caller.is_null());

    pmc_owner_caller = core::ptr::null_mut();
    perf_irq = dummy_perf;

    raw_spin_unlock(&raw mut pmc_owner_lock);
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
pub unsafe fn power4_enable_pmcs() {
    let mut hid0: usize;

    hid0 = mfspr(SPRN_HID0);
    hid0 |= 1usize << (63 - 20);

    /* POWER4 requires the following sequence */
    core::arch::asm!(
        "sync",
        "mtspr {spr}, {hid0}",
        "mfspr {hid0}, {spr}",
        "mfspr {hid0}, {spr}",
        "mfspr {hid0}, {spr}",
        "mfspr {hid0}, {spr}",
        "mfspr {hid0}, {spr}",
        "mfspr {hid0}, {spr}",
        "isync",
        spr = const SPRN_HID0,
        hid0 = inout(reg) hid0,
        options(nostack, preserves_flags),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
