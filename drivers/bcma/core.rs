/*
 * Broadcom specific AMBA
 * Core ops
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies supplied by bcma_private.h and the Linux BCMA/kernel headers.

unsafe fn bcma_core_wait_value(
    core: *mut bcma_device,
    reg: u16,
    mask: u32,
    value: u32,
    timeout: i32,
) -> bool {
    let deadline = jiffies.wrapping_add(timeout as _);
    let mut val: u32;

    loop {
        val = bcma_aread32(core, reg);
        if (val & mask) == value {
            return true;
        }
        cpu_relax();
        udelay(10);
        if time_after_eq(jiffies, deadline) {
            break;
        }
    }

    bcma_warn((*core).bus, "Timeout waiting for register 0x%04X!\n", reg);

    false
}

pub unsafe fn bcma_core_is_enabled(core: *mut bcma_device) -> bool {
    if (bcma_aread32(core, BCMA_IOCTL) & (BCMA_IOCTL_CLK | BCMA_IOCTL_FGC))
        != BCMA_IOCTL_CLK
    {
        return false;
    }
    if bcma_aread32(core, BCMA_RESET_CTL) & BCMA_RESET_CTL_RESET != 0 {
        return false;
    }
    true
}

pub unsafe fn bcma_core_disable(core: *mut bcma_device, flags: u32) {
    if bcma_aread32(core, BCMA_RESET_CTL) & BCMA_RESET_CTL_RESET != 0 {
        return;
    }

    bcma_core_wait_value(core, BCMA_RESET_ST, !0u32, 0, 300);

    bcma_awrite32(core, BCMA_RESET_CTL, BCMA_RESET_CTL_RESET);
    bcma_aread32(core, BCMA_RESET_CTL);
    udelay(1);

    bcma_awrite32(core, BCMA_IOCTL, flags);
    bcma_aread32(core, BCMA_IOCTL);
    udelay(10);
}

pub unsafe fn bcma_core_enable(core: *mut bcma_device, flags: u32) -> i32 {
    bcma_core_disable(core, flags);

    bcma_awrite32(core, BCMA_IOCTL, BCMA_IOCTL_CLK | BCMA_IOCTL_FGC | flags);
    bcma_aread32(core, BCMA_IOCTL);

    bcma_awrite32(core, BCMA_RESET_CTL, 0);
    bcma_aread32(core, BCMA_RESET_CTL);
    udelay(1);

    bcma_awrite32(core, BCMA_IOCTL, BCMA_IOCTL_CLK | flags);
    bcma_aread32(core, BCMA_IOCTL);
    udelay(1);

    0
}

pub unsafe fn bcma_core_set_clockmode(core: *mut bcma_device, clkmode: bcma_clkmode) {
    let mut i: u16;

    WARN_ON((*core).id.id != BCMA_CORE_CHIPCOMMON
        && (*core).id.id != BCMA_CORE_PCIE
        && (*core).id.id != BCMA_CORE_80211);

    match clkmode {
        BCMA_CLKMODE_FAST => {
            bcma_set32(core, BCMA_CLKCTLST, BCMA_CLKCTLST_FORCEHT);
            usleep_range(64, 300);
            i = 0;
            while i < 1500 {
                if bcma_read32(core, BCMA_CLKCTLST) & BCMA_CLKCTLST_HAVEHT != 0 {
                    i = 0;
                    break;
                }
                i = i.wrapping_add(1);
                udelay(10);
            }
            if i != 0 {
                bcma_err((*core).bus, "HT force timeout\n");
            }
        }
        BCMA_CLKMODE_DYNAMIC => {
            bcma_set32(core, BCMA_CLKCTLST, !BCMA_CLKCTLST_FORCEHT);
        }
    }
}

pub unsafe fn bcma_core_pll_ctl(core: *mut bcma_device, req: u32, status: u32, on: bool) {
    let mut i: u16;

    WARN_ON(req & !BCMA_CLKCTLST_EXTRESREQ != 0);
    WARN_ON(status & !BCMA_CLKCTLST_EXTRESST != 0);

    if on {
        bcma_set32(core, BCMA_CLKCTLST, req);
        i = 0;
        while i < 10000 {
            if (bcma_read32(core, BCMA_CLKCTLST) & status) == status {
                i = 0;
                break;
            }
            i = i.wrapping_add(1);
            udelay(10);
        }
        if i != 0 {
            bcma_err((*core).bus, "PLL enable timeout\n");
        }
    } else {
        /*
         * Mask the PLL but don't wait for it to be disabled. PLL may be
         * shared between cores and will be still up if there is another
         * core using it.
         */
        bcma_mask32(core, BCMA_CLKCTLST, !req);
        bcma_read32(core, BCMA_CLKCTLST);
    }
}

pub unsafe fn bcma_core_dma_translation(core: *mut bcma_device) -> u32 {
    match (*(*core).bus).hosttype {
        BCMA_HOSTTYPE_SOC => 0,
        BCMA_HOSTTYPE_PCI => {
            if bcma_aread32(core, BCMA_IOST) & BCMA_IOST_DMA64 != 0 {
                BCMA_DMA_TRANSLATION_DMA64_CMT
            } else {
                BCMA_DMA_TRANSLATION_DMA32_CMT
            }
        }
        _ => {
            bcma_err(
                (*core).bus,
                "DMA translation unknown for host %d\n",
                (*(*core).bus).hosttype,
            );
            BCMA_DMA_TRANSLATION_NONE
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
