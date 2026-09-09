/*
 * Broadcom specific AMBA
 * Broadcom MIPS32 74K core driver
 *
 * Copyright 2009, Broadcom Corporation
 * Copyright 2006, 2007, Michael Buesch <mb@bu3sch.de>
 * Copyright 2010, Bernhard Loos <bernhardloos@googlemail.com>
 * Copyright 2011, Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// C dependencies are supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum BcmaBootDev {
    Unk = 0,
    Rom,
    Parallel,
    Serial,
    Nand,
}

#[inline]
unsafe fn bcma_core_mips_bcm47162a0_quirk(dev: *mut bcma_device) -> bool {
    (*(*dev).bus).chipinfo.id == BCMA_CHIP_ID_BCM47162 &&
        (*(*dev).bus).chipinfo.rev == 0 && (*dev).id.id == BCMA_CORE_MIPS_74K
}

#[inline]
unsafe fn bcma_core_mips_bcm5357b0_quirk(dev: *mut bcma_device) -> bool {
    ((*(*dev).bus).chipinfo.id == BCMA_CHIP_ID_BCM5357 ||
        (*(*dev).bus).chipinfo.id == BCMA_CHIP_ID_BCM4749) &&
        (*(*dev).bus).chipinfo.pkg == 11 && (*dev).id.id == BCMA_CORE_USB20_HOST
}

unsafe fn bcma_core_mips_irqflag(dev: *mut bcma_device) -> u32 {
    if bcma_core_mips_bcm47162a0_quirk(dev) || bcma_core_mips_bcm5357b0_quirk(dev) {
        return (*dev).core_index;
    }
    let flag = bcma_aread32(dev, BCMA_MIPS_OOBSELOUTA30);
    if flag != 0 { flag & 0x1f } else { 0x3f }
}

pub unsafe fn bcma_core_mips_irq(dev: *mut bcma_device) -> u32 {
    let mdev = (*(*dev).bus).drv_mips.core;
    let irqflag = bcma_core_mips_irqflag(dev);
    if irqflag == 0x3f { return 6; }
    for irq in 0..=4 {
        if bcma_read32(mdev, BCMA_MIPS_MIPS74K_INTMASK(irq)) & (1u32 << irqflag) != 0 {
            return irq;
        }
    }
    5
}

unsafe fn bcma_core_mips_set_irq(dev: *mut bcma_device, irq: u32) {
    let oldirq = bcma_core_mips_irq(dev);
    let bus = (*dev).bus;
    let mdev = (*bus).drv_mips.core;
    let irqflag = bcma_core_mips_irqflag(dev);
    BUG_ON(oldirq == 6);
    (*dev).irq = irq + 2;
    if oldirq == 0 {
        let r = bcma_read32(mdev, BCMA_MIPS_MIPS74K_INTMASK(0));
        bcma_write32(mdev, BCMA_MIPS_MIPS74K_INTMASK(0), r & !(1u32 << irqflag));
    } else if oldirq != 5 { bcma_write32(mdev, BCMA_MIPS_MIPS74K_INTMASK(oldirq), 0); }
    if irq == 0 {
        let r = bcma_read32(mdev, BCMA_MIPS_MIPS74K_INTMASK(0));
        bcma_write32(mdev, BCMA_MIPS_MIPS74K_INTMASK(0), r | (1u32 << irqflag));
    } else {
        let irqinitmask = bcma_read32(mdev, BCMA_MIPS_MIPS74K_INTMASK(irq));
        if irqinitmask != 0 {
            // C list_for_each_entry over bus->cores; supplied by the surrounding code.
            for core in bcma_core_list(bus) {
                if (1u32 << bcma_core_mips_irqflag(core)) == irqinitmask {
                    bcma_core_mips_set_irq(core, 0);
                    break;
                }
            }
        }
        bcma_write32(mdev, BCMA_MIPS_MIPS74K_INTMASK(irq), 1u32 << irqflag);
    }
    bcma_debug(bus, "set_irq: core 0x%04x, irq %d => %d\n", (*dev).id.id,
               if oldirq <= 4 { oldirq + 2 } else { 0 }, irq + 2);
}

unsafe fn bcma_core_mips_set_irq_name(bus: *mut bcma_bus, irq: u32, coreid: u16, unit: u8) {
    let core = bcma_find_core_unit(bus, coreid, unit);
    if core.is_null() {
        bcma_warn(bus, "Can not find core (id: 0x%x, unit %i) for IRQ configuration.\n", coreid, unit);
        return;
    }
    bcma_core_mips_set_irq(core, irq);
}

unsafe fn bcma_core_mips_print_irq(dev: *mut bcma_device, irq: u32) {
    let names = ["2(S)", "3", "4", "5", "6", "D", "I"];
    let mut interrupts = String::new();
    for (i, name) in names.iter().enumerate() {
        interrupts.push_str(&format!(" {}{}", name, if i as u32 == irq { '*' } else { ' ' }));
    }
    bcma_debug((*dev).bus, "core 0x%04x, irq:%s\n", (*dev).id.id, interrupts);
}

unsafe fn bcma_core_mips_dump_irq(bus: *mut bcma_bus) {
    for core in bcma_core_list(bus) { bcma_core_mips_print_irq(core, bcma_core_mips_irq(core)); }
}

pub unsafe fn bcma_cpu_clock(mcore: *mut bcma_drv_mips) -> u32 {
    let bus = (*(*mcore).core).bus;
    if (*bus).drv_cc.capabilities & BCMA_CC_CAP_PMU != 0 { return bcma_pmu_get_cpu_clock(&mut (*bus).drv_cc); }
    bcma_err(bus, "No PMU available, need this to get the cpu clock\n");
    0
}

unsafe fn bcma_boot_dev(bus: *mut bcma_bus) -> BcmaBootDev {
    let cc = &mut (*bus).drv_cc;
    let cc_rev = cc.core.id.rev;
    if cc_rev == 42 {
        let core = bcma_find_core(bus, BCMA_CORE_NS_ROM);
        if !core.is_null() {
            return match bcma_aread32(core, BCMA_IOST) & BCMA_NS_ROM_IOST_BOOT_DEV_MASK {
                BCMA_NS_ROM_IOST_BOOT_DEV_NOR => BcmaBootDev::Serial,
                BCMA_NS_ROM_IOST_BOOT_DEV_NAND => BcmaBootDev::Nand,
                _ => BcmaBootDev::Rom,
            };
        }
    } else {
        if cc_rev == 38 {
            if cc.status & BCMA_CC_CHIPST_5357_NAND_BOOT != 0 { return BcmaBootDev::Nand; }
            if cc.status & BIT(5) != 0 { return BcmaBootDev::Rom; }
        }
        return if cc.capabilities & BCMA_CC_CAP_FLASHT == BCMA_CC_FLASHT_PARA { BcmaBootDev::Parallel } else { BcmaBootDev::Serial };
    }
    BcmaBootDev::Serial
}

unsafe fn bcma_core_mips_nvram_init(mcore: *mut bcma_drv_mips) {
    let bus = (*(*mcore).core).bus;
    match bcma_boot_dev(bus) {
        BcmaBootDev::Parallel | BcmaBootDev::Serial => {
            #[cfg(CONFIG_BCM47XX)] { bcm47xx_nvram_init_from_mem(BCMA_SOC_FLASH2, BCMA_SOC_FLASH2_SZ); }
        }
        BcmaBootDev::Nand => {
            #[cfg(CONFIG_BCM47XX)] { bcm47xx_nvram_init_from_mem(BCMA_SOC_FLASH1, BCMA_SOC_FLASH1_SZ); }
        }
        BcmaBootDev::Unk | BcmaBootDev::Rom => {}
    }
}

pub unsafe fn bcma_core_mips_early_init(mcore: *mut bcma_drv_mips) {
    let bus = (*(*mcore).core).bus;
    if (*mcore).early_setup_done { return; }
    bcma_chipco_serial_init(&mut (*bus).drv_cc);
    bcma_core_mips_nvram_init(mcore);
    (*mcore).early_setup_done = true;
}

unsafe fn bcma_fix_i2s_irqflag(bus: *mut bcma_bus) {
    if (*bus).chipinfo.id != BCMA_CHIP_ID_BCM4716 && (*bus).chipinfo.id != BCMA_CHIP_ID_BCM4748 { return; }
    let cpu = bcma_find_core(bus, BCMA_CORE_MIPS_74K);
    let pcie = bcma_find_core(bus, BCMA_CORE_PCIE);
    let i2s = bcma_find_core(bus, BCMA_CORE_I2S);
    if !cpu.is_null() && !pcie.is_null() && !i2s.is_null() && bcma_aread32(cpu, BCMA_MIPS_OOBSELINA74) == 0x08060504 && bcma_aread32(pcie, BCMA_MIPS_OOBSELINA74) == 0x08060504 && bcma_aread32(i2s, BCMA_MIPS_OOBSELOUTA30) == 0x88 {
        bcma_awrite32(cpu, BCMA_MIPS_OOBSELINA74, 0x07060504); bcma_awrite32(pcie, BCMA_MIPS_OOBSELINA74, 0x07060504); bcma_awrite32(i2s, BCMA_MIPS_OOBSELOUTA30, 0x87);
        bcma_debug(bus, "Moved i2s interrupt to oob line 7 instead of 8\n");
    }
}

pub unsafe fn bcma_core_mips_init(mcore: *mut bcma_drv_mips) {
    let bus = (*(*mcore).core).bus;
    if (*mcore).setup_done { return; }
    bcma_debug(bus, "Initializing MIPS core...\n");
    bcma_core_mips_early_init(mcore); bcma_fix_i2s_irqflag(bus);
    match (*bus).chipinfo.id {
        BCMA_CHIP_ID_BCM4716 | BCMA_CHIP_ID_BCM4748 => { for &(i,c) in &[(1,BCMA_CORE_80211),(2,BCMA_CORE_MAC_GBIT),(3,BCMA_CORE_USB20_HOST),(4,BCMA_CORE_PCIE),(0,BCMA_CORE_CHIPCOMMON),(0,BCMA_CORE_I2S)] { bcma_core_mips_set_irq_name(bus,i,c,0); } }
        BCMA_CHIP_ID_BCM5356 | BCMA_CHIP_ID_BCM47162 | BCMA_CHIP_ID_BCM53572 => { for &(i,c) in &[(1,BCMA_CORE_80211),(2,BCMA_CORE_MAC_GBIT),(0,BCMA_CORE_CHIPCOMMON)] { bcma_core_mips_set_irq_name(bus,i,c,0); } }
        BCMA_CHIP_ID_BCM5357 | BCMA_CHIP_ID_BCM4749 => { for &(i,c) in &[(1,BCMA_CORE_80211),(2,BCMA_CORE_MAC_GBIT),(3,BCMA_CORE_USB20_HOST),(0,BCMA_CORE_CHIPCOMMON),(0,BCMA_CORE_I2S)] { bcma_core_mips_set_irq_name(bus,i,c,0); } }
        BCMA_CHIP_ID_BCM4706 => { for &(i,c,u) in &[(1,BCMA_CORE_PCIE,0),(2,BCMA_CORE_4706_MAC_GBIT,0),(3,BCMA_CORE_PCIE,1),(4,BCMA_CORE_USB20_HOST,0),(0,BCMA_CORE_4706_CHIPCOMMON,0)] { bcma_core_mips_set_irq_name(bus,i,c,u); } }
        _ => { for core in bcma_core_list(bus) { (*core).irq = bcma_core_irq(core, 0); } bcma_err(bus, "Unknown device (0x%x) found, can not configure IRQs\n", (*bus).chipinfo.id); }
    }
    bcma_debug(bus, "IRQ reconfiguration done\n"); bcma_core_mips_dump_irq(bus); (*mcore).setup_done = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
