// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MMP PMU power island support
 *
 * Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
 */

// Dependencies supplied by the Linux power-domain, slab, I/O, and clock code.

#[repr(C)]
pub struct MmpPmDomain {
    pub genpd: GenericPmDomain,
    pub reg: *mut core::ffi::c_void,
    pub lock: *mut Spinlock,
    pub power_on: u32,
    pub reset: u32,
    pub clock_enable: u32,
    pub flags: core::ffi::c_uint,
}

// C build-time definitions supplied by the surrounding source tree.
extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: core::ffi::c_ulong);
    fn pm_genpd_init(
        genpd: *mut GenericPmDomain,
        gov: *mut core::ffi::c_void,
        is_off: bool,
    );
    fn kzalloc_obj<T>() -> *mut T;
    fn err_ptr(error: core::ffi::c_long) -> *mut GenericPmDomain;
}

#[repr(C)]
pub struct GenericPmDomain {
    pub name: *const core::ffi::c_char,
    pub power_on: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> core::ffi::c_int>,
    pub power_off: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

pub const MMP_PM_DOMAIN_NO_DISABLE: core::ffi::c_uint = 1 << 0;

unsafe fn to_mmp_pm_domain(genpd: *mut GenericPmDomain) -> *mut MmpPmDomain {
    genpd.cast::<u8>().sub(core::mem::offset_of!(MmpPmDomain, genpd)).cast()
}

unsafe extern "C" fn mmp_pm_domain_power_on(
    genpd: *mut GenericPmDomain,
) -> core::ffi::c_int {
    let pm_domain = to_mmp_pm_domain(genpd);
    let mut flags: core::ffi::c_ulong = 0;
    let mut val: u32;

    if !(*pm_domain).lock.is_null() {
        spin_lock_irqsave((*pm_domain).lock, &mut flags);
    }

    val = readl((*pm_domain).reg);

    /* Turn on the power island */
    val |= (*pm_domain).power_on;
    writel(val, (*pm_domain).reg);

    /* Disable isolation */
    val |= 0x100;
    writel(val, (*pm_domain).reg);

    /* Some blocks need to be reset after a power up */
    if (*pm_domain).reset != 0 || (*pm_domain).clock_enable != 0 {
        let after_power_on = val;

        val &= !(*pm_domain).reset;
        writel(val, (*pm_domain).reg);

        val |= (*pm_domain).clock_enable;
        writel(val, (*pm_domain).reg);

        val |= (*pm_domain).reset;
        writel(val, (*pm_domain).reg);

        writel(after_power_on, (*pm_domain).reg);
    }

    if !(*pm_domain).lock.is_null() {
        spin_unlock_irqrestore((*pm_domain).lock, flags);
    }

    0
}

unsafe extern "C" fn mmp_pm_domain_power_off(
    genpd: *mut GenericPmDomain,
) -> core::ffi::c_int {
    let pm_domain = to_mmp_pm_domain(genpd);
    let mut flags: core::ffi::c_ulong = 0;
    let mut val: u32;

    if (*pm_domain).flags & MMP_PM_DOMAIN_NO_DISABLE != 0 {
        return 0;
    }

    if !(*pm_domain).lock.is_null() {
        spin_lock_irqsave((*pm_domain).lock, &mut flags);
    }

    /* Turn off and isolate the power island. */
    val = readl((*pm_domain).reg);
    val &= !(*pm_domain).power_on;
    val &= !0x100;
    writel(val, (*pm_domain).reg);

    if !(*pm_domain).lock.is_null() {
        spin_unlock_irqrestore((*pm_domain).lock, flags);
    }

    0
}

pub unsafe extern "C" fn mmp_pm_domain_register(
    name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
    power_on: u32,
    reset: u32,
    clock_enable: u32,
    flags: core::ffi::c_uint,
    lock: *mut Spinlock,
) -> *mut GenericPmDomain {
    let pm_domain = kzalloc_obj::<MmpPmDomain>();
    if pm_domain.is_null() {
        return err_ptr(-12);
    }

    (*pm_domain).reg = reg;
    (*pm_domain).power_on = power_on;
    (*pm_domain).reset = reset;
    (*pm_domain).clock_enable = clock_enable;
    (*pm_domain).flags = flags;
    (*pm_domain).lock = lock;

    (*pm_domain).genpd.name = name;
    (*pm_domain).genpd.power_on = Some(mmp_pm_domain_power_on);
    (*pm_domain).genpd.power_off = Some(mmp_pm_domain_power_off);
    pm_genpd_init(&mut (*pm_domain).genpd, core::ptr::null_mut(), true);

    &mut (*pm_domain).genpd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
