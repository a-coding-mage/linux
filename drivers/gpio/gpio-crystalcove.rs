// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Crystal Cove GPIO Driver
 *
 * Copyright (C) 2012, 2014 Intel Corporation. All rights reserved.
 *
 * Author: Yang, Bin <bin.yang@intel.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

const CRYSTALCOVE_GPIO_NUM: i32 = 16;
const CRYSTALCOVE_VGPIO_NUM: i32 = 95;

const UPDATE_IRQ_TYPE: i32 = 1 << 0;
const UPDATE_IRQ_MASK: i32 = 1 << 1;

const GPIO0IRQ: i32 = 0x0b;
const GPIO1IRQ: i32 = 0x0c;
const MGPIO0IRQS0: i32 = 0x19;
const MGPIO1IRQS0: i32 = 0x1a;
const MGPIO0IRQSX: i32 = 0x1b;
const MGPIO1IRQSX: i32 = 0x1c;
const GPIO0P0CTLO: i32 = 0x2b;
const GPIO0P0CTLI: i32 = 0x33;
const GPIO1P0CTLO: i32 = 0x3b;
const GPIO1P0CTLI: i32 = 0x43;
const GPIOPANELCTL: i32 = 0x52;

const CTLI_INTCNT_DIS: i32 = 0;
const CTLI_INTCNT_NE: i32 = 1 << 1;
const CTLI_INTCNT_PE: i32 = 2 << 1;
const CTLI_INTCNT_BE: i32 = 3 << 1;

const CTLO_DIR_IN: i32 = 0;
const CTLO_DIR_OUT: i32 = 1 << 5;
const CTLO_DRV_CMOS: i32 = 0;
const CTLO_DRV_OD: i32 = 1 << 4;
const CTLO_DRV_REN: i32 = 1 << 3;
const CTLO_RVAL_2KDW: i32 = 0;
const CTLO_RVAL_2KUP: i32 = 1 << 1;
const CTLO_RVAL_50KDW: i32 = 2 << 1;
const CTLO_RVAL_50KUP: i32 = 3 << 1;
const CTLO_INPUT_SET: i32 = CTLO_DRV_CMOS | CTLO_DRV_REN | CTLO_RVAL_2KUP;
const CTLO_OUTPUT_SET: i32 = CTLO_DIR_OUT | CTLO_INPUT_SET;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum CtrlRegister {
    CTRL_IN,
    CTRL_OUT,
}

#[repr(C)]
pub struct CrystalcoveGpio {
    pub buslock: Mutex,
    pub chip: GpioChip,
    pub regmap: *mut Regmap,
    pub update: i32,
    pub intcnt_value: i32,
    pub set_irq_mask: bool,
}

#[inline]
unsafe fn to_reg(gpio: i32, reg_type: CtrlRegister) -> i32 {
    let reg: i32;
    if gpio >= CRYSTALCOVE_GPIO_NUM {
        match gpio {
            0x5e => return GPIOPANELCTL,
            _ => return -ENOTSUPP,
        }
    }
    if reg_type == CtrlRegister::CTRL_IN {
        reg = if gpio < 8 { GPIO0P0CTLI } else { GPIO1P0CTLI };
    } else {
        reg = if gpio < 8 { GPIO0P0CTLO } else { GPIO1P0CTLO };
    }
    reg + gpio % 8
}

unsafe fn crystalcove_update_irq_mask(cg: *mut CrystalcoveGpio, gpio: i32) {
    let mirqs0 = if gpio < 8 { MGPIO0IRQS0 } else { MGPIO1IRQS0 };
    let mask = 1 << (gpio % 8);
    if (*cg).set_irq_mask {
        regmap_update_bits((*cg).regmap, mirqs0, mask, mask);
    } else {
        regmap_update_bits((*cg).regmap, mirqs0, mask, 0);
    }
}

unsafe fn crystalcove_update_irq_ctrl(cg: *mut CrystalcoveGpio, gpio: i32) {
    let reg = to_reg(gpio, CtrlRegister::CTRL_IN);
    regmap_update_bits((*cg).regmap, reg, CTLI_INTCNT_BE, (*cg).intcnt_value);
}

unsafe fn crystalcove_gpio_dir_in(chip: *mut GpioChip, gpio: u32) -> i32 {
    let cg = gpiochip_get_data(chip) as *mut CrystalcoveGpio;
    let reg = to_reg(gpio as i32, CtrlRegister::CTRL_OUT);
    if reg < 0 { return 0; }
    regmap_write((*cg).regmap, reg, CTLO_INPUT_SET)
}

unsafe fn crystalcove_gpio_dir_out(chip: *mut GpioChip, gpio: u32, value: i32) -> i32 {
    let cg = gpiochip_get_data(chip) as *mut CrystalcoveGpio;
    let reg = to_reg(gpio as i32, CtrlRegister::CTRL_OUT);
    if reg < 0 { return 0; }
    regmap_write((*cg).regmap, reg, CTLO_OUTPUT_SET | value)
}

unsafe fn crystalcove_gpio_get(chip: *mut GpioChip, gpio: u32) -> i32 {
    let cg = gpiochip_get_data(chip) as *mut CrystalcoveGpio;
    let reg = to_reg(gpio as i32, CtrlRegister::CTRL_IN);
    if reg < 0 { return 0; }
    let mut val: u32 = 0;
    let ret = regmap_read((*cg).regmap, reg, &mut val);
    if ret != 0 { return ret; }
    (val & 0x1) as i32
}

unsafe fn crystalcove_gpio_set(chip: *mut GpioChip, gpio: u32, value: i32) -> i32 {
    let cg = gpiochip_get_data(chip) as *mut CrystalcoveGpio;
    let reg = to_reg(gpio as i32, CtrlRegister::CTRL_OUT);
    if reg < 0 { return 0; }
    if value != 0 {
        regmap_update_bits((*cg).regmap, reg, 1, 1)
    } else {
        regmap_update_bits((*cg).regmap, reg, 1, 0)
    }
}

unsafe fn crystalcove_irq_type(data: *mut IrqData, irq_type: u32) -> i32 {
    let cg = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut CrystalcoveGpio;
    let hwirq = irqd_to_hwirq(data);
    if hwirq >= CRYSTALCOVE_GPIO_NUM as u64 { return 0; }
    (*cg).intcnt_value = match irq_type {
        IRQ_TYPE_NONE => CTLI_INTCNT_DIS,
        IRQ_TYPE_EDGE_BOTH => CTLI_INTCNT_BE,
        IRQ_TYPE_EDGE_RISING => CTLI_INTCNT_PE,
        IRQ_TYPE_EDGE_FALLING => CTLI_INTCNT_NE,
        _ => return -EINVAL,
    };
    (*cg).update |= UPDATE_IRQ_TYPE;
    0
}

unsafe fn crystalcove_bus_lock(data: *mut IrqData) {
    let cg = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut CrystalcoveGpio;
    mutex_lock(&mut (*cg).buslock);
}

unsafe fn crystalcove_bus_sync_unlock(data: *mut IrqData) {
    let cg = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut CrystalcoveGpio;
    let hwirq = irqd_to_hwirq(data) as i32;
    if (*cg).update & UPDATE_IRQ_TYPE != 0 { crystalcove_update_irq_ctrl(cg, hwirq); }
    if (*cg).update & UPDATE_IRQ_MASK != 0 { crystalcove_update_irq_mask(cg, hwirq); }
    (*cg).update = 0;
    mutex_unlock(&mut (*cg).buslock);
}

unsafe fn crystalcove_irq_unmask(data: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(data);
    let cg = gpiochip_get_data(gc) as *mut CrystalcoveGpio;
    let hwirq = irqd_to_hwirq(data);
    if hwirq >= CRYSTALCOVE_GPIO_NUM as u64 { return; }
    gpiochip_enable_irq(gc, hwirq);
    (*cg).set_irq_mask = false;
    (*cg).update |= UPDATE_IRQ_MASK;
}

unsafe fn crystalcove_irq_mask(data: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(data);
    let cg = gpiochip_get_data(gc) as *mut CrystalcoveGpio;
    let hwirq = irqd_to_hwirq(data);
    if hwirq >= CRYSTALCOVE_GPIO_NUM as u64 { return; }
    (*cg).set_irq_mask = true;
    (*cg).update |= UPDATE_IRQ_MASK;
    gpiochip_disable_irq(gc, hwirq);
}

static crystalcove_irqchip: IrqChip = IrqChip {
    name: "Crystal Cove",
    irq_mask: Some(crystalcove_irq_mask),
    irq_unmask: Some(crystalcove_irq_unmask),
    irq_set_type: Some(crystalcove_irq_type),
    irq_bus_lock: Some(crystalcove_bus_lock),
    irq_bus_sync_unlock: Some(crystalcove_bus_sync_unlock),
    flags: IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
};

unsafe fn crystalcove_gpio_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> IrqReturn {
    let cg = data as *mut CrystalcoveGpio;
    let mut p0: u32 = 0;
    let mut p1: u32 = 0;
    if regmap_read((*cg).regmap, GPIO0IRQ, &mut p0) != 0 ||
       regmap_read((*cg).regmap, GPIO1IRQ, &mut p1) != 0 { return IRQ_NONE; }
    regmap_write((*cg).regmap, GPIO0IRQ, p0);
    regmap_write((*cg).regmap, GPIO1IRQ, p1);
    let pending = p0 | (p1 << 8);
    for gpio in 0..CRYSTALCOVE_GPIO_NUM {
        if pending & (1u32 << gpio) != 0 {
            let virq = irq_find_mapping((*cg).chip.irq.domain, gpio as u32);
            handle_nested_irq(virq);
        }
    }
    IRQ_HANDLED
}

unsafe fn crystalcove_gpio_dbg_show(s: *mut SeqFile, chip: *mut GpioChip) {
    let cg = gpiochip_get_data(chip) as *mut CrystalcoveGpio;
    for gpio in 0..CRYSTALCOVE_GPIO_NUM {
        let mut ctlo = 0u32; let mut ctli = 0u32; let mut mirqs0 = 0u32;
        let mut mirqsx = 0u32; let mut irq = 0u32;
        regmap_read((*cg).regmap, to_reg(gpio, CtrlRegister::CTRL_OUT), &mut ctlo);
        regmap_read((*cg).regmap, to_reg(gpio, CtrlRegister::CTRL_IN), &mut ctli);
        regmap_read((*cg).regmap, if gpio < 8 { MGPIO0IRQS0 } else { MGPIO1IRQS0 }, &mut mirqs0);
        regmap_read((*cg).regmap, if gpio < 8 { MGPIO0IRQSX } else { MGPIO1IRQSX }, &mut mirqsx);
        regmap_read((*cg).regmap, if gpio < 8 { GPIO0IRQ } else { GPIO1IRQ }, &mut irq);
        seq_printf(s, " gpio-%-2d %s %s %s %s ctlo=%2x,%s %s %s\n", gpio,
            if ctlo & CTLO_DIR_OUT as u32 != 0 { "out" } else { "in " },
            str_hi_lo(ctli & 0x1), if ctli & CTLI_INTCNT_NE as u32 != 0 { "fall" } else { "    " },
            if ctli & CTLI_INTCNT_PE as u32 != 0 { "rise" } else { "    " }, ctlo,
            if mirqs0 & (1 << (gpio % 8)) != 0 { "s0 mask  " } else { "s0 unmask" },
            if mirqsx & (1 << (gpio % 8)) != 0 { "sx mask  " } else { "sx unmask" },
            if irq & (1 << (gpio % 8)) != 0 { "pending" } else { "       " });
    }
}

unsafe fn crystalcove_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let cg = devm_kzalloc((*pdev).dev, core::mem::size_of::<CrystalcoveGpio>(), GFP_KERNEL) as *mut CrystalcoveGpio;
    if cg.is_null() { return -ENOMEM; }
    mutex_init(&mut (*cg).buslock);
    (*cg).chip.label = KBUILD_MODNAME;
    (*cg).chip.direction_input = Some(crystalcove_gpio_dir_in);
    (*cg).chip.direction_output = Some(crystalcove_gpio_dir_out);
    (*cg).chip.get = Some(crystalcove_gpio_get); (*cg).chip.set = Some(crystalcove_gpio_set);
    (*cg).chip.base = -1; (*cg).chip.ngpio = CRYSTALCOVE_VGPIO_NUM as u32; (*cg).chip.can_sleep = true;
    (*cg).chip.dbg_show = Some(crystalcove_gpio_dbg_show);
    let ret = devm_request_threaded_irq((*pdev).dev, irq, None, Some(crystalcove_gpio_irq_handler), IRQF_ONESHOT, KBUILD_MODNAME, cg as *mut _);
    if ret != 0 { dev_warn((*pdev).dev, "request irq failed: %d\n", ret); return ret; }
    let ret = devm_gpiochip_add_data((*pdev).dev, &mut (*cg).chip, cg as *mut _);
    if ret != 0 { return ret; }
    irq_domain_update_bus_token((*cg).chip.irq.domain, DOMAIN_BUS_WIRED);
    0
}

static crystalcove_gpio_driver: PlatformDriver = PlatformDriver {
    probe: Some(crystalcove_gpio_probe),
    name: "crystal_cove_gpio",
};

module_platform_driver!(crystalcove_gpio_driver);
module_author!("Yang, Bin <bin.yang@intel.com>");
module_description!("Intel Crystal Cove GPIO Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
