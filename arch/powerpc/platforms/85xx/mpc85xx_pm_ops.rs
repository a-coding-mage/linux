// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC85xx PM operators
 *
 * Copyright 2015 Freescale Semiconductor Inc.
 */

// C dependencies supplied by the surrounding kernel translation.

static mut guts: *mut ccsr_guts = core::ptr::null_mut();

#[cfg(feature = "CONFIG_FSL_PMC")]
unsafe fn mpc85xx_irq_mask(_cpu: i32) {
}

#[cfg(feature = "CONFIG_FSL_PMC")]
unsafe fn mpc85xx_irq_unmask(_cpu: i32) {
}

#[cfg(feature = "CONFIG_FSL_PMC")]
unsafe fn mpc85xx_cpu_die(_cpu: i32) {
    let mut tmp: u32;

    tmp = (mfspr(SPRN_HID0) & !(HID0_DOZE | HID0_SLEEP)) | HID0_NAP;
    mtspr(SPRN_HID0, tmp);

    /* Enter NAP mode. */
    tmp = mfmsr();
    tmp |= MSR_WE;
    core::arch::asm!(
        "msync",
        "mtmsr {0}",
        "isync",
        in(reg) tmp,
        options(nostack),
    );
}

#[cfg(feature = "CONFIG_FSL_PMC")]
unsafe fn mpc85xx_cpu_up_prepare(_cpu: i32) {
}

unsafe fn mpc85xx_freeze_time_base(freeze: bool) {
    let mask: u32;

    mask = CCSR_GUTS_DEVDISR_TB0 | CCSR_GUTS_DEVDISR_TB1;
    if freeze {
        setbits32(&mut (*guts).devdisr, mask);
    } else {
        clrbits32(&mut (*guts).devdisr, mask);
    }

    in_be32(&(*guts).devdisr);
}

static mpc85xx_smp_guts_ids: [of_device_id; 8] = [
    of_device_id { compatible: b"fsl,mpc8572-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,p1020-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,p1021-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,p1022-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,p1023-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,p2020-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"fsl,bsc9132-guts\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

static mpc85xx_pm_ops: fsl_pm_ops = fsl_pm_ops {
    freeze_time_base: Some(mpc85xx_freeze_time_base),
    #[cfg(feature = "CONFIG_FSL_PMC")]
    irq_mask: Some(mpc85xx_irq_mask),
    #[cfg(feature = "CONFIG_FSL_PMC")]
    irq_unmask: Some(mpc85xx_irq_unmask),
    #[cfg(feature = "CONFIG_FSL_PMC")]
    cpu_die: Some(mpc85xx_cpu_die),
    #[cfg(feature = "CONFIG_FSL_PMC")]
    cpu_up_prepare: Some(mpc85xx_cpu_up_prepare),
};

unsafe fn mpc85xx_setup_pmc() -> i32 {
    let mut np: *mut device_node;

    np = of_find_matching_node(core::ptr::null_mut(), mpc85xx_smp_guts_ids.as_ptr());
    if !np.is_null() {
        guts = of_iomap(np, 0);
        of_node_put(np);
        if guts.is_null() {
            pr_err!("Could not map guts node address\n");
            return -ENOMEM;
        }
        qoriq_pm_ops = &mpc85xx_pm_ops;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
