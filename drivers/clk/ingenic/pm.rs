// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Paul Cercueil <paul@crapouillou.net>
 */

// Dependencies supplied by cgu.h, pm.h, linux/io.h, and linux/syscore_ops.h.

const CGU_REG_LCR: usize = 0x04;
const LCR_LOW_POWER_MODE: u32 = 1u32 << 0;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn register_syscore(syscore: *mut syscore);
}

#[repr(C)]
pub struct ingenic_cgu {
    pub base: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct syscore_ops {
    pub suspend: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct syscore {
    pub ops: *const syscore_ops,
}

static mut ingenic_cgu_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" fn ingenic_cgu_pm_suspend(_data: *mut core::ffi::c_void) -> i32 {
    let addr = ingenic_cgu_base.add(CGU_REG_LCR);
    let val = readl(addr);

    writel(val | LCR_LOW_POWER_MODE, addr);

    0
}

unsafe extern "C" fn ingenic_cgu_pm_resume(_data: *mut core::ffi::c_void) {
    let addr = ingenic_cgu_base.add(CGU_REG_LCR);
    let val = readl(addr);

    writel(val & !LCR_LOW_POWER_MODE, addr);
}

static ingenic_cgu_pm_ops: syscore_ops = syscore_ops {
    suspend: Some(ingenic_cgu_pm_suspend),
    resume: Some(ingenic_cgu_pm_resume),
};

static mut ingenic_cgu_pm: syscore = syscore {
    ops: &ingenic_cgu_pm_ops,
};

pub unsafe extern "C" fn ingenic_cgu_register_syscore(cgu: *mut ingenic_cgu) {
    // Equivalent to IS_ENABLED(CONFIG_PM_SLEEP); preserve the build-time condition.
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    {
        ingenic_cgu_base = (*cgu).base;
        register_syscore(&raw mut ingenic_cgu_pm);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
