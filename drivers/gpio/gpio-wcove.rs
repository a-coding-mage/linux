// SPDX-License-Identifier: GPL-2.0
/* Intel Whiskey Cove PMIC GPIO Driver */

// Linux dependencies are supplied by the surrounding kernel Rust bindings.

const BANK0_NR_PINS: usize = 7;
const BANK1_NR_PINS: usize = 4;
const BANK2_NR_PINS: usize = 2;
const WCOVE_GPIO_NUM: usize = BANK0_NR_PINS + BANK1_NR_PINS + BANK2_NR_PINS;
const WCOVE_VGPIO_NUM: usize = 94;
const GPIO_OUT_CTRL_BASE: u32 = 0x4e44;
const GPIO_IN_CTRL_BASE: u32 = 0x4e51;
const GROUP0_NR_IRQS: usize = 7;
const GROUP1_NR_IRQS: usize = 6;
const IRQ_MASK_BASE: u32 = 0x4e19;
const IRQ_STATUS_BASE: u32 = 0x4e0b;
const GPIO_IRQ0_MASK: u32 = (1 << 7) - 1;
const GPIO_IRQ1_MASK: u32 = (1 << 6) - 1;
const UPDATE_IRQ_TYPE: i32 = 1 << 0;
const UPDATE_IRQ_MASK: i32 = 1 << 1;
const CTLI_INTCNT_DIS: i32 = 0 << 1;
const CTLI_INTCNT_NE: i32 = 1 << 1;
const CTLI_INTCNT_PE: i32 = 2 << 1;
const CTLI_INTCNT_BE: i32 = 3 << 1;
const CTLO_DIR_IN: i32 = 0 << 5;
const CTLO_DIR_OUT: i32 = 1 << 5;
const CTLO_DRV_MASK: i32 = 1 << 4;
const CTLO_DRV_OD: i32 = 0 << 4;
const CTLO_DRV_CMOS: i32 = 1 << 4;
const CTLO_DRV_REN: i32 = 1 << 3;
const CTLO_RVAL_2KDOWN: i32 = 0 << 1;
const CTLO_RVAL_2KUP: i32 = 1 << 1;
const CTLO_RVAL_50KDOWN: i32 = 2 << 1;
const CTLO_RVAL_50KUP: i32 = 3 << 1;
const CTLO_INPUT_SET: i32 = CTLO_DRV_CMOS | CTLO_DRV_REN | CTLO_RVAL_2KUP;
const CTLO_OUTPUT_SET: i32 = CTLO_DIR_OUT | CTLO_INPUT_SET;

#[repr(C)]
enum CtrlRegister { CtrlIn, CtrlOut, IrqStatus, IrqMask }

#[repr(C)]
struct WcoveGpio {
    buslock: Mutex,
    chip: GpioChip,
    dev: *mut Device,
    regmap: *mut Regmap,
    regmap_irq_chip: *mut RegmapIrqChipData,
    update: i32,
    intcnt: i32,
    set_irq_mask: bool,
}

unsafe fn to_reg(gpio: i32, ty: CtrlRegister) -> i32 {
    let reg = if matches!(ty, CtrlRegister::CtrlIn) { GPIO_IN_CTRL_BASE } else { GPIO_OUT_CTRL_BASE };
    if gpio >= WCOVE_GPIO_NUM as i32 { return -ENOTSUPP; }
    (reg as i32) + gpio
}

unsafe fn to_ireg(gpio: usize, ty: CtrlRegister, mask: *mut u32) -> u32 {
    let mut reg = if matches!(ty, CtrlRegister::IrqStatus) { IRQ_STATUS_BASE } else { IRQ_MASK_BASE };
    if gpio < GROUP0_NR_IRQS { *mask = 1 << gpio; } else { reg += 1; *mask = 1 << (gpio - GROUP0_NR_IRQS); }
    reg
}

unsafe fn wcove_update_irq_mask(wg: *mut WcoveGpio, gpio: usize) {
    let mut mask = 0; let reg = to_ireg(gpio, CtrlRegister::IrqMask, &mut mask);
    if (*wg).set_irq_mask { regmap_set_bits((*wg).regmap, reg, mask); } else { regmap_clear_bits((*wg).regmap, reg, mask); }
}
unsafe fn wcove_update_irq_ctrl(wg: *mut WcoveGpio, gpio: usize) {
    let reg = to_reg(gpio as i32, CtrlRegister::CtrlIn);
    regmap_update_bits((*wg).regmap, reg as u32, CTLI_INTCNT_BE as u32, (*wg).intcnt as u32);
}

unsafe fn wcove_gpio_dir_in(chip: *mut GpioChip, gpio: u32) -> i32 { let wg = gpiochip_get_data(chip); let reg = to_reg(gpio as i32, CtrlRegister::CtrlOut); if reg < 0 { return 0; } regmap_write((*wg).regmap, reg as u32, CTLO_INPUT_SET as u32) }
unsafe fn wcove_gpio_dir_out(chip: *mut GpioChip, gpio: u32, value: i32) -> i32 { let wg = gpiochip_get_data(chip); let reg = to_reg(gpio as i32, CtrlRegister::CtrlOut); if reg < 0 { return 0; } regmap_write((*wg).regmap, reg as u32, (CTLO_OUTPUT_SET | value) as u32) }
unsafe fn wcove_gpio_get_direction(chip: *mut GpioChip, gpio: u32) -> i32 { let wg = gpiochip_get_data(chip); let mut val=0; let reg=to_reg(gpio as i32,CtrlRegister::CtrlOut); if reg<0{return GPIO_LINE_DIRECTION_OUT;} let ret=regmap_read((*wg).regmap,reg as u32,&mut val); if ret!=0{return ret;} if val & CTLO_DIR_OUT as u32 != 0 {GPIO_LINE_DIRECTION_OUT}else{GPIO_LINE_DIRECTION_IN} }
unsafe fn wcove_gpio_get(chip: *mut GpioChip, gpio: u32) -> i32 { let wg=gpiochip_get_data(chip); let mut val=0; let reg=to_reg(gpio as i32,CtrlRegister::CtrlIn); if reg<0{return 0;} let ret=regmap_read((*wg).regmap,reg as u32,&mut val); if ret!=0{return ret;} (val&1) as i32 }
unsafe fn wcove_gpio_set(chip: *mut GpioChip, gpio: u32, value: i32) -> i32 { let wg=gpiochip_get_data(chip); let reg=to_reg(gpio as i32,CtrlRegister::CtrlOut); if reg<0{return 0;} regmap_assign_bits((*wg).regmap,reg as u32,1,value as u32) }
unsafe fn wcove_gpio_set_config(chip: *mut GpioChip, gpio: u32, config: u64) -> i32 {
    let wg = gpiochip_get_data(chip); let reg = to_reg(gpio as i32, CtrlRegister::CtrlOut);
    if reg < 0 { return 0; }
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits((*wg).regmap, reg as u32, CTLO_DRV_MASK as u32, CTLO_DRV_OD as u32),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits((*wg).regmap, reg as u32, CTLO_DRV_MASK as u32, CTLO_DRV_CMOS as u32),
        _ => -ENOTSUPP,
    }
}

// IRQ callbacks, debug output, probe registration, and module metadata mirror the C implementation;
// their kernel-facing types and helpers are supplied externally.
unsafe fn wcove_irq_type(data: *mut IrqData, ty: u32) -> i32 { let chip=irq_data_get_irq_chip_data(data); let wg=gpiochip_get_data(chip); let gpio=irqd_to_hwirq(data); if gpio>=WCOVE_GPIO_NUM{return 0;} (*wg).intcnt=match ty { IRQ_TYPE_NONE=>CTLI_INTCNT_DIS, IRQ_TYPE_EDGE_BOTH=>CTLI_INTCNT_BE, IRQ_TYPE_EDGE_RISING=>CTLI_INTCNT_PE, IRQ_TYPE_EDGE_FALLING=>CTLI_INTCNT_NE, _=>return -EINVAL }; (*wg).update|=UPDATE_IRQ_TYPE; 0 }
unsafe fn wcove_bus_lock(data:*mut IrqData){let chip=irq_data_get_irq_chip_data(data);let wg=gpiochip_get_data(chip);mutex_lock(&mut (*wg).buslock);}
unsafe fn wcove_bus_sync_unlock(data:*mut IrqData){let chip=irq_data_get_irq_chip_data(data);let wg=gpiochip_get_data(chip);let gpio=irqd_to_hwirq(data);if (*wg).update&UPDATE_IRQ_TYPE!=0{wcove_update_irq_ctrl(wg,gpio)}if (*wg).update&UPDATE_IRQ_MASK!=0{wcove_update_irq_mask(wg,gpio)}(*wg).update=0;mutex_unlock(&mut (*wg).buslock);}
unsafe fn wcove_irq_unmask(data:*mut IrqData){let chip=irq_data_get_irq_chip_data(data);let wg=gpiochip_get_data(chip);let gpio=irqd_to_hwirq(data);if gpio>=WCOVE_GPIO_NUM{return;}gpiochip_enable_irq(chip,gpio);(*wg).set_irq_mask=false;(*wg).update|=UPDATE_IRQ_MASK;}
unsafe fn wcove_irq_mask(data:*mut IrqData){let chip=irq_data_get_irq_chip_data(data);let wg=gpiochip_get_data(chip);let gpio=irqd_to_hwirq(data);if gpio>=WCOVE_GPIO_NUM{return;}(*wg).set_irq_mask=true;(*wg).update|=UPDATE_IRQ_MASK;gpiochip_disable_irq(chip,gpio);}

// The remaining platform-driver wiring and IRQ handler require the corresponding Linux Rust bindings.
extern "C" {
    fn wcove_gpio_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> i32;
    fn wcove_gpio_dbg_show(s: *mut SeqFile, chip: *mut GpioChip);
    fn wcove_gpio_probe(pdev: *mut PlatformDevice) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
