// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale LBC and UPM routines.
 *
 * Copyright © 2007-2008  MontaVista Software, Inc.
 * Copyright © 2010 Freescale Semiconductor
 *
 * Author: Anton Vorontsov <avorontsov@ru.mvista.com>
 * Author: Jack Lan <Jack.Lan@freescale.com>
 * Author: Roy Zang <tie-fei.zang@freescale.com>
 */

// Linux dependencies and asm/fsl_lbc.h are supplied by other translation units.

static mut FSL_LBC_LOCK: Spinlock = DEFINE_SPINLOCK!();
pub static mut fsl_lbc_ctrl_dev: *mut fsl_lbc_ctrl = core::ptr::null_mut();

pub unsafe fn fsl_lbc_addr(addr_base: phys_addr_t) -> u32 {
    let np = (*(*fsl_lbc_ctrl_dev).dev).of_node;
    let addr = (addr_base & 0xffff8000) as u32;
    if of_device_is_compatible(np, c"fsl,elbc") {
        return addr;
    }
    addr | (((addr_base & 0x300000000u64) >> 19) as u32)
}

pub unsafe fn fsl_lbc_find(addr_base: phys_addr_t) -> i32 {
    let mut i: i32 = 0;
    let lbc: *mut fsl_lbc_regs;
    if fsl_lbc_ctrl_dev.is_null() || (*fsl_lbc_ctrl_dev).regs.is_null() {
        return -ENODEV;
    }
    lbc = (*fsl_lbc_ctrl_dev).regs;
    while i < (*lbc).bank.len() as i32 {
        let br = in_be32(&(*lbc).bank[i as usize].br);
        let or = in_be32(&(*lbc).bank[i as usize].or_);
        if br & BR_V != 0 && (br & or & BR_BA) == fsl_lbc_addr(addr_base) {
            return i;
        }
        i += 1;
    }
    -ENOENT
}

pub unsafe fn fsl_upm_find(addr_base: phys_addr_t, upm: *mut fsl_upm) -> i32 {
    let bank = fsl_lbc_find(addr_base);
    if bank < 0 { return bank; }
    if fsl_lbc_ctrl_dev.is_null() || (*fsl_lbc_ctrl_dev).regs.is_null() {
        return -ENODEV;
    }
    let lbc = (*fsl_lbc_ctrl_dev).regs;
    let br = in_be32(&(*lbc).bank[bank as usize].br);
    (*upm).mxmr = match br & BR_MSEL {
        BR_MS_UPMA => &mut (*lbc).mamr,
        BR_MS_UPMB => &mut (*lbc).mbmr,
        BR_MS_UPMC => &mut (*lbc).mcmr,
        _ => return -EINVAL,
    };
    (*upm).width = match br & BR_PS {
        BR_PS_8 => 8,
        BR_PS_16 => 16,
        BR_PS_32 => 32,
        _ => return -EINVAL,
    };
    0
}

pub unsafe fn fsl_upm_run_pattern(upm: *mut fsl_upm, io_base: *mut core::ffi::c_void, mar: u32) -> i32 {
    let mut ret = 0;
    let mut flags: c_ulong = 0;
    if fsl_lbc_ctrl_dev.is_null() || (*fsl_lbc_ctrl_dev).regs.is_null() { return -ENODEV; }
    spin_lock_irqsave(&raw mut FSL_LBC_LOCK, &mut flags);
    out_be32(&mut (*(*fsl_lbc_ctrl_dev).regs).mar, mar);
    match (*upm).width {
        8 => out_8(io_base, 0),
        16 => out_be16(io_base, 0),
        32 => out_be32(io_base, 0),
        _ => ret = -EINVAL,
    }
    spin_unlock_irqrestore(&raw mut FSL_LBC_LOCK, flags);
    ret
}

unsafe fn fsl_lbc_ctrl_init(ctrl: *mut fsl_lbc_ctrl, node: *mut device_node) -> i32 {
    let lbc = (*ctrl).regs;
    setbits32(&mut (*lbc).ltesr, LTESR_CLEAR);
    out_be32(&mut (*lbc).lteatr, 0);
    out_be32(&mut (*lbc).ltear, 0);
    out_be32(&mut (*lbc).lteccr, LTECCR_CLEAR);
    out_be32(&mut (*lbc).ltedr, LTEDR_ENABLE);
    if of_device_is_compatible(node, c"fsl,elbc") { clrsetbits_be32(&mut (*lbc).lbcr, LBCR_BMT, LBCR_BMTPS); }
    0
}

// This interrupt reports localbus events of various kinds, such as transaction errors.
unsafe extern "C" fn fsl_lbc_ctrl_irq(_irqno: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let ctrl = data as *mut fsl_lbc_ctrl;
    let lbc = (*ctrl).regs;
    let mut flags = 0;
    spin_lock_irqsave(&raw mut FSL_LBC_LOCK, &mut flags);
    let status = in_be32(&(*lbc).ltesr);
    if status == 0 { spin_unlock_irqrestore(&raw mut FSL_LBC_LOCK, flags); return IRQ_NONE; }
    out_be32(&mut (*lbc).ltesr, LTESR_CLEAR);
    out_be32(&mut (*lbc).lteatr, 0);
    out_be32(&mut (*lbc).ltear, 0);
    (*ctrl).irq_status = status;
    if status & LTESR_BM != 0 { dev_err((*ctrl).dev, "Local bus monitor time-out: LTESR 0x%08X\n", status); }
    if status & LTESR_WP != 0 { dev_err((*ctrl).dev, "Write protect error: LTESR 0x%08X\n", status); }
    if status & LTESR_ATMW != 0 { dev_err((*ctrl).dev, "Atomic write error: LTESR 0x%08X\n", status); }
    if status & LTESR_ATMR != 0 { dev_err((*ctrl).dev, "Atomic read error: LTESR 0x%08X\n", status); }
    if status & LTESR_CS != 0 { dev_err((*ctrl).dev, "Chip select error: LTESR 0x%08X\n", status); }
    if status & LTESR_FCT != 0 { dev_err((*ctrl).dev, "FCM command time-out: LTESR 0x%08X\n", status); smp_wmb(); wake_up(&mut (*ctrl).irq_wait); }
    if status & LTESR_PAR != 0 { dev_err((*ctrl).dev, "Parity or Uncorrectable ECC error: LTESR 0x%08X\n", status); smp_wmb(); wake_up(&mut (*ctrl).irq_wait); }
    if status & LTESR_CC != 0 { smp_wmb(); wake_up(&mut (*ctrl).irq_wait); }
    if status & !LTESR_MASK != 0 { dev_err((*ctrl).dev, "Unknown error: LTESR 0x%08X\n", status); }
    spin_unlock_irqrestore(&raw mut FSL_LBC_LOCK, flags);
    IRQ_HANDLED
}

unsafe fn fsl_lbc_ctrl_probe(dev: *mut platform_device) -> i32 {
    let mut ret;
    if (*dev).dev.of_node.is_null() { dev_err(&mut (*dev).dev, "Device OF-Node is NULL"); return -EFAULT; }
    fsl_lbc_ctrl_dev = kzalloc_obj::<fsl_lbc_ctrl>();
    if fsl_lbc_ctrl_dev.is_null() { return -ENOMEM; }
    dev_set_drvdata(&mut (*dev).dev, fsl_lbc_ctrl_dev);
    spin_lock_init(&mut (*fsl_lbc_ctrl_dev).lock);
    init_waitqueue_head(&mut (*fsl_lbc_ctrl_dev).irq_wait);
    (*fsl_lbc_ctrl_dev).regs = of_iomap((*dev).dev.of_node, 0);
    if (*fsl_lbc_ctrl_dev).regs.is_null() { dev_err(&mut (*dev).dev, "failed to get memory region\n"); ret = -ENODEV; goto_err!(err); }
    (*fsl_lbc_ctrl_dev).irq[0] = irq_of_parse_and_map((*dev).dev.of_node, 0);
    if (*fsl_lbc_ctrl_dev).irq[0] == 0 { dev_err(&mut (*dev).dev, "failed to get irq resource\n"); ret = -ENODEV; goto_err!(err); }
    (*fsl_lbc_ctrl_dev).dev = &mut (*dev).dev;
    ret = fsl_lbc_ctrl_init(fsl_lbc_ctrl_dev, (*dev).dev.of_node);
    if ret < 0 { goto_err!(err); }
    ret = request_irq((*fsl_lbc_ctrl_dev).irq[0], fsl_lbc_ctrl_irq, 0, c"fsl-lbc", fsl_lbc_ctrl_dev);
    if ret != 0 { dev_err(&mut (*dev).dev, "failed to install irq (%d)\n", (*fsl_lbc_ctrl_dev).irq[0]); ret = (*fsl_lbc_ctrl_dev).irq[0]; goto_err!(err); }
    (*fsl_lbc_ctrl_dev).irq[1] = irq_of_parse_and_map((*dev).dev.of_node, 1);
    if (*fsl_lbc_ctrl_dev).irq[1] != 0 {
        ret = request_irq((*fsl_lbc_ctrl_dev).irq[1], fsl_lbc_ctrl_irq, IRQF_SHARED, c"fsl-lbc-err", fsl_lbc_ctrl_dev);
        if ret != 0 { dev_err(&mut (*dev).dev, "failed to install irq (%d)\n", (*fsl_lbc_ctrl_dev).irq[1]); ret = (*fsl_lbc_ctrl_dev).irq[1]; free_irq((*fsl_lbc_ctrl_dev).irq[0], fsl_lbc_ctrl_dev); goto_err!(err); }
    }
    out_be32(&mut (*(*fsl_lbc_ctrl_dev).regs).lteir, LTEIR_ENABLE);
    return 0;
    err: { iounmap((*fsl_lbc_ctrl_dev).regs); kfree(fsl_lbc_ctrl_dev); fsl_lbc_ctrl_dev = core::ptr::null_mut(); return ret; }
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn fsl_lbc_syscore_suspend(_data: *mut core::ffi::c_void) -> i32 {
    let ctrl = fsl_lbc_ctrl_dev;
    if ctrl.is_null() || (*ctrl).regs.is_null() { return 0; }
    (*ctrl).saved_regs = kmalloc_obj::<fsl_lbc_regs>();
    if (*ctrl).saved_regs.is_null() { return -ENOMEM; }
    _memcpy_fromio((*ctrl).saved_regs, (*ctrl).regs, core::mem::size_of::<fsl_lbc_regs>());
    0
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn fsl_lbc_syscore_resume(_data: *mut core::ffi::c_void) {
    let ctrl = fsl_lbc_ctrl_dev;
    if ctrl.is_null() || (*ctrl).regs.is_null() { return; }
    if !(*ctrl).saved_regs.is_null() {
        _memcpy_toio((*ctrl).regs, (*ctrl).saved_regs, core::mem::size_of::<fsl_lbc_regs>());
        kfree((*ctrl).saved_regs);
        (*ctrl).saved_regs = core::ptr::null_mut();
    }
}

static fsl_lbc_match: [of_device_id; 5] = [
    of_device_id { compatible: c"fsl,elbc" },
    of_device_id { compatible: c"fsl,pq3-localbus" },
    of_device_id { compatible: c"fsl,pq2-localbus" },
    of_device_id { compatible: c"fsl,pq2pro-localbus" },
    of_device_id { compatible: core::ptr::null() },
];

static mut fsl_lbc_ctrl_driver: platform_driver = platform_driver {
    driver: driver { name: c"fsl-lbc", of_match_table: fsl_lbc_match.as_ptr() },
    probe: Some(fsl_lbc_ctrl_probe),
};

unsafe fn fsl_lbc_init() -> i32 {
    #[cfg(CONFIG_SUSPEND)]
    register_syscore(&lbc_syscore_pm);
    platform_driver_register(&mut fsl_lbc_ctrl_driver)
}

subsys_initcall!(fsl_lbc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
