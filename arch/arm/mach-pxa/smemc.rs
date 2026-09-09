// SPDX-License-Identifier: GPL-2.0
/*
 * Static Memory Controller
 */

// C dependencies supplied by other translation units/headers:
// linux/module.h, linux/kernel.h, linux/init.h, linux/io.h,
// linux/syscore_ops.h, linux/soc/pxa/cpu.h, smemc.h,
// linux/soc/pxa/smemc.h

#[cfg(CONFIG_PM)]
static mut MSC: [core::ffi::c_ulong; 2] = [0; 2];
#[cfg(CONFIG_PM)]
static mut SXCNFG: core::ffi::c_ulong = 0;
#[cfg(CONFIG_PM)]
static mut MEMCLKCFG_SAVED: core::ffi::c_ulong = 0;
#[cfg(CONFIG_PM)]
static mut CSADRCFG: [core::ffi::c_ulong; 4] = [0; 4];

#[cfg(CONFIG_PM)]
unsafe fn pxa3xx_smemc_suspend(_data: *mut core::ffi::c_void) -> i32 {
    MSC[0] = __raw_readl(MSC0) as core::ffi::c_ulong;
    MSC[1] = __raw_readl(MSC1) as core::ffi::c_ulong;
    SXCNFG = __raw_readl(SXCNFG_REG) as core::ffi::c_ulong;
    MEMCLKCFG_SAVED = __raw_readl(MEMCLKCFG) as core::ffi::c_ulong;
    CSADRCFG[0] = __raw_readl(CSADRCFG0) as core::ffi::c_ulong;
    CSADRCFG[1] = __raw_readl(CSADRCFG1) as core::ffi::c_ulong;
    CSADRCFG[2] = __raw_readl(CSADRCFG2) as core::ffi::c_ulong;
    CSADRCFG[3] = __raw_readl(CSADRCFG3) as core::ffi::c_ulong;

    0
}

#[cfg(CONFIG_PM)]
unsafe fn pxa3xx_smemc_resume(_data: *mut core::ffi::c_void) {
    __raw_writel(MSC[0] as u32, MSC0);
    __raw_writel(MSC[1] as u32, MSC1);
    __raw_writel(SXCNFG as u32, SXCNFG_REG);
    __raw_writel(MEMCLKCFG_SAVED as u32, MEMCLKCFG);
    __raw_writel(CSADRCFG[0] as u32, CSADRCFG0);
    __raw_writel(CSADRCFG[1] as u32, CSADRCFG1);
    __raw_writel(CSADRCFG[2] as u32, CSADRCFG2);
    __raw_writel(CSADRCFG[3] as u32, CSADRCFG3);
    /* CSMSADRCFG wakes up in its default state (0), so we need to set it */
    __raw_writel(0x2, CSMSADRCFG);
}

#[cfg(CONFIG_PM)]
static SMEMC_SYSCORE_OPS: syscore_ops = syscore_ops {
    suspend: Some(pxa3xx_smemc_suspend),
    resume: Some(pxa3xx_smemc_resume),
};

#[cfg(CONFIG_PM)]
static mut SMEMC_SYSCORE: syscore = syscore {
    ops: &SMEMC_SYSCORE_OPS,
};

#[cfg(CONFIG_PM)]
unsafe fn smemc_init() -> i32 {
    if cpu_is_pxa3xx() {
        /*
         * The only documentation we have on the
         * Chip Select Configuration Register (CSMSADRCFG) is that
         * it must be programmed to 0x2.
         * Moreover, in the bit definitions, the second bit
         * (CSMSADRCFG[1]) is called "SETALWAYS".
         * Other bits are reserved in this register.
         */
        __raw_writel(0x2, CSMSADRCFG);

        register_syscore(&SMEMC_SYSCORE);
    }

    0
}

// Equivalent of subsys_initcall(smemc_init); registration is supplied by the kernel runtime.

static DF_CLKDIV: [u32; 4] = [1, 2, 4, 1];

pub unsafe fn pxa3xx_smemc_get_memclkdiv() -> u32 {
    let memclkcfg = __raw_readl(MEMCLKCFG);

    DF_CLKDIV[((memclkcfg >> 16) & 0x3) as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
