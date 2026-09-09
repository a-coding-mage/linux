// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP IOMMU quirks for various TI SoCs
 *
 * Copyright (C) 2015-2019 Texas Instruments Incorporated - https://www.ti.com/
 *      Suman Anna <s-anna@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pwrdm_link {
    pub dev: *mut device,
    pub pwrdm: *mut powerdomain,
    pub node: list_head,
}

static mut iommu_lock: spinlock_t = spinlock_t::new();
static mut emu_clkdm: *mut clockdomain = core::ptr::null_mut();
static mut emu_count: atomic_t = atomic_t::new(0);

unsafe fn omap_iommu_dra7_emu_swsup_config(
    pdev: *mut platform_device,
    enable: bool,
) {
    let np = (*pdev).dev.of_node;
    let mut flags: c_ulong = 0;

    if !of_device_is_compatible(np, c"ti,dra7-dsp-iommu".as_ptr() as *const c_char) {
        return;
    }

    if emu_clkdm.is_null() {
        emu_clkdm = clkdm_lookup(c"emu_clkdm".as_ptr() as *const c_char);
        if WARN_ON_ONCE(emu_clkdm.is_null()) {
            return;
        }
    }

    spin_lock_irqsave(&raw mut iommu_lock, &mut flags);

    if enable && (atomic_inc_return(&raw mut emu_count) == 1) {
        clkdm_deny_idle(emu_clkdm);
    } else if !enable && (atomic_dec_return(&raw mut emu_count) == 0) {
        clkdm_allow_idle(emu_clkdm);
    }

    spin_unlock_irqrestore(&raw mut iommu_lock, flags);
}

unsafe fn _get_pwrdm(dev: *mut device) -> *mut powerdomain {
    let mut clk: *mut clk;
    let mut hwclk: *mut clk_hw_omap;
    let mut clkdm: *mut clockdomain;
    let mut pwrdm: *mut powerdomain = core::ptr::null_mut();
    let mut entry: *mut pwrdm_link;
    let mut flags: c_ulong = 0;
    static mut cache: list_head = list_head::new();

    spin_lock_irqsave(&raw mut iommu_lock, &mut flags);

    list_for_each_entry!(entry, &raw mut cache, node) {
        if (*entry).dev == dev {
            pwrdm = (*entry).pwrdm;
            break;
        }
    }

    spin_unlock_irqrestore(&raw mut iommu_lock, flags);

    if !pwrdm.is_null() {
        return pwrdm;
    }

    clk = of_clk_get((*(*dev).of_node).parent, 0);
    if IS_ERR(clk) {
        dev_err(dev, "no fck found\n");
        return core::ptr::null_mut();
    }

    hwclk = to_clk_hw_omap(__clk_get_hw(clk));
    clk_put(clk);
    if hwclk.is_null() || (*hwclk).clkdm_name.is_null() {
        dev_err(dev, "no hwclk data\n");
        return core::ptr::null_mut();
    }

    clkdm = clkdm_lookup((*hwclk).clkdm_name);
    if clkdm.is_null() {
        dev_err(dev, "clkdm not found: %s\n", (*hwclk).clkdm_name);
        return core::ptr::null_mut();
    }

    pwrdm = clkdm_get_pwrdm(clkdm);
    if pwrdm.is_null() {
        dev_err(dev, "pwrdm not found: %s\n", (*clkdm).name);
        return core::ptr::null_mut();
    }

    entry = kmalloc_obj::<pwrdm_link>();
    if !entry.is_null() {
        (*entry).dev = dev;
        (*entry).pwrdm = pwrdm;
        spin_lock_irqsave(&raw mut iommu_lock, &mut flags);
        list_add(&mut (*entry).node, &raw mut cache);
        spin_unlock_irqrestore(&raw mut iommu_lock, flags);
    }

    pwrdm
}

pub unsafe fn omap_iommu_set_pwrdm_constraint(
    pdev: *mut platform_device,
    request: bool,
    pwrst: *mut u8,
) -> c_int {
    let pwrdm: *mut powerdomain;
    let next_pwrst: u8;
    let mut ret: c_int = 0;

    pwrdm = _get_pwrdm(&mut (*pdev).dev);
    if pwrdm.is_null() {
        return -ENODEV;
    }

    if request {
        *pwrst = pwrdm_read_next_pwrst(pwrdm);
        omap_iommu_dra7_emu_swsup_config(pdev, true);
    }

    if *pwrst <= PWRDM_POWER_RET {
        next_pwrst = if request { PWRDM_POWER_ON } else { *pwrst };
        ret = pwrdm_set_next_pwrst(pwrdm, next_pwrst);
    }

    if !request {
        omap_iommu_dra7_emu_swsup_config(pdev, false);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
