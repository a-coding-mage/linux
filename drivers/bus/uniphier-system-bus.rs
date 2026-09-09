// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const UNIPHIER_SBC_BASE: usize = 0x100;
const UNIPHIER_SBC_BASE_BE: u32 = 1 << 0;
const UNIPHIER_SBC_CTRL0: usize = 0x200;
const UNIPHIER_SBC_CTRL1: usize = 0x204;
const UNIPHIER_SBC_CTRL2: usize = 0x208;
const UNIPHIER_SBC_CTRL3: usize = 0x20c;
const UNIPHIER_SBC_CTRL4: usize = 0x300;
const UNIPHIER_SBC_STRIDE: usize = 0x10;
const UNIPHIER_SBC_NR_BANKS: usize = 8;
const UNIPHIER_SBC_BASE_DUMMY: u32 = 0xffff_ffff;

#[repr(C)]
struct uniphier_system_bus_bank {
    base: u32,
    end: u32,
}

#[repr(C)]
struct uniphier_system_bus_priv {
    dev: *mut device,
    membase: *mut core::ffi::c_void,
    bank: [uniphier_system_bus_bank; UNIPHIER_SBC_NR_BANKS],
}

unsafe fn uniphier_system_bus_add_bank(
    priv_: *mut uniphier_system_bus_priv,
    bank: i32,
    addr: u32,
    mut paddr: u64,
    size: u32,
) -> i32 {
    let mut end: u64;
    let mut mask: u64;

    dev_dbg((*priv_).dev, "range found: bank = %d, addr = %08x, paddr = %08llx, size = %08x\n", bank, addr, paddr, size);
    if bank < 0 || bank as usize >= (*priv_).bank.len() {
        dev_err((*priv_).dev, "unsupported bank number %d\n", bank);
        return -EINVAL;
    }
    let b = &mut (*priv_).bank[bank as usize];
    if b.base != 0 || b.end != 0 {
        dev_err((*priv_).dev, "range for bank %d has already been specified\n", bank);
        return -EINVAL;
    }
    if paddr > u32::MAX as u64 {
        dev_err((*priv_).dev, "base address %llx is too high\n", paddr);
        return -EINVAL;
    }
    end = paddr.wrapping_add(size as u64);
    if addr as u64 > paddr {
        dev_err((*priv_).dev, "base %08x cannot be mapped to %08llx of parent\n", addr, paddr);
        return -EINVAL;
    }
    paddr -= addr as u64;
    paddr &= !0x1ffff;
    end = (end.wrapping_add(0x1ffff)) & !0x1ffff;
    if end > u32::MAX as u64 {
        dev_err((*priv_).dev, "end address %08llx is too high\n", end);
        return -EINVAL;
    }
    mask = paddr ^ (end - 1);
    mask = mask.next_power_of_two();
    paddr &= !mask.wrapping_sub(1);
    end = (end.wrapping_add(mask.wrapping_sub(1))) & !mask.wrapping_sub(1);
    b.base = paddr as u32;
    b.end = end as u32;
    dev_dbg((*priv_).dev, "range added: bank = %d, addr = %08x, end = %08x\n", bank, b.base, b.end);
    0
}

unsafe fn uniphier_system_bus_check_overlap(priv_: *const uniphier_system_bus_priv) -> i32 {
    for i in 0..(*priv_).bank.len() {
        for j in (i + 1)..(*priv_).bank.len() {
            if (*priv_).bank[i].end > (*priv_).bank[j].base && (*priv_).bank[i].base < (*priv_).bank[j].end {
                dev_err((*priv_).dev, "region overlap between bank%d and bank%d\n", i, j);
                return -EINVAL;
            }
        }
    }
    0
}

unsafe fn uniphier_system_bus_check_boot_swap(priv_: *mut uniphier_system_bus_priv) {
    let base_reg = ((*priv_).membase as *mut u8).add(UNIPHIER_SBC_BASE) as *mut u32;
    let is_swapped = (readl(base_reg) & UNIPHIER_SBC_BASE_BE) == 0;
    dev_dbg((*priv_).dev, "Boot Swap: %s\n", if is_swapped { "on" } else { "off" });
    if is_swapped {
        core::ptr::swap(&mut (*priv_).bank[0], &mut (*priv_).bank[1]);
    }
}

unsafe fn uniphier_system_bus_set_reg(priv_: *const uniphier_system_bus_priv) {
    let base_reg = ((*priv_).membase as *mut u8).add(UNIPHIER_SBC_BASE) as *mut u32;
    for i in 0..(*priv_).bank.len() {
        let base = (*priv_).bank[i].base;
        let end = (*priv_).bank[i].end;
        let val = if base == end {
            if i < 2 { UNIPHIER_SBC_BASE_DUMMY } else { 0 }
        } else {
            let mask = base ^ (end - 1);
            (base & 0xfffe0000) | ((!mask >> 16) & 0xfffe) | UNIPHIER_SBC_BASE_BE
        };
        dev_dbg((*priv_).dev, "SBC_BASE[%d] = 0x%08x\n", i, val);
        writel(val, (base_reg as *mut u8).add(UNIPHIER_SBC_STRIDE * i) as *mut u32);
    }
}

unsafe fn uniphier_system_bus_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<uniphier_system_bus_priv>(), GFP_KERNEL) as *mut uniphier_system_bus_priv;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).membase = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_).membase) { return ptr_err((*priv_).membase); }
    (*priv_).dev = dev;
    let mut parser = of_range_parser { _private: [] };
    let ret = of_range_parser_init(&mut parser, (*dev).of_node);
    if ret != 0 { return ret; }
    let mut range = of_range { _private: [] };
    while for_each_of_range(&mut parser, &mut range) {
        if range.cpu_addr == OF_BAD_ADDR { return -EINVAL; }
        let ret = uniphier_system_bus_add_bank(priv_, (range.bus_addr >> 32) as i32, range.bus_addr as u32, range.cpu_addr, range.size);
        if ret != 0 { return ret; }
    }
    let ret = uniphier_system_bus_check_overlap(priv_);
    if ret != 0 { return ret; }
    uniphier_system_bus_check_boot_swap(priv_);
    uniphier_system_bus_set_reg(priv_);
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    of_platform_default_populate((*dev).of_node, core::ptr::null(), dev)
}

unsafe fn uniphier_system_bus_resume(dev: *mut device) -> i32 {
    uniphier_system_bus_set_reg(dev_get_drvdata(dev) as *const uniphier_system_bus_priv);
    0
}

// Equivalent declarations for the kernel driver registration and power-management tables.
static uniphier_system_bus_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };
static uniphier_system_bus_match: [of_device_id; 2] = [
    of_device_id { compatible: "socionext,uniphier-system-bus", _private: [] },
    of_device_id { compatible: "", _private: [] },
];
static mut uniphier_system_bus_driver: platform_driver = platform_driver { _private: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
