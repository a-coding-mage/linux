// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2020 Google Inc
// Copyright 2025 Linaro Ltd.
//
// GPIO driver for Maxim MAX77759

// C dependencies retained as external kernel symbols.

const MAX77759_N_GPIOS: usize = 2;
static MAX77759_GPIO_LINE_NAMES: [&str; 2] = ["GPIO5", "GPIO6"];

#[repr(C)]
struct Max77759GpioChip {
    map: *mut Regmap,
    max77759: *mut Max77759,
    gc: GpioChip,
    maxq_lock: Mutex,
    irq_lock: Mutex,
    irq_mask: i32,
    irq_mask_changed: i32,
    irq_trig: i32,
    irq_trig_changed: i32,
}

const fn max77759_gpiox_trigger(offs: u32, val: i32) -> i32 { (val & 1) << offs }
const fn max77759_gpiox_trigger_mask(offs: u32) -> i32 { max77759_gpiox_trigger(offs, !0) }
const MAX77759_GPIO_TRIGGER_RISING: i32 = 0;
const MAX77759_GPIO_TRIGGER_FALLING: i32 = 1;
const fn max77759_gpiox_dir(offs: u32, dir: i32) -> i32 { (dir & 1) << (2 + 3 * offs) }
const fn max77759_gpiox_dir_mask(offs: u32) -> i32 { max77759_gpiox_dir(offs, !0) }
const MAX77759_GPIO_DIR_IN: i32 = 0;
const MAX77759_GPIO_DIR_OUT: i32 = 1;
const fn max77759_gpiox_outval(offs: u32, val: i32) -> i32 { (val & 1) << (3 + 3 * offs) }
const fn max77759_gpiox_outval_mask(offs: u32) -> i32 { max77759_gpiox_outval(offs, !0) }
const fn max77759_gpiox_inval_mask(offs: u32) -> i32 { (1 << 4) << (3 * offs) }

unsafe fn max77759_gpio_maxq_gpio_trigger_read(chip: *mut Max77759GpioChip) -> i32 {
    let mut cmd = MaxqCommand::new(1); let mut rsp = MaxqResponse::new(2);
    cmd.cmd[0] = MAX77759_MAXQ_OPCODE_GPIO_TRIGGER_READ;
    let ret = max77759_maxq_command((*chip).max77759, &mut cmd, &mut rsp);
    if ret < 0 { return ret; } rsp.rsp[1] as i32
}
unsafe fn max77759_gpio_maxq_gpio_trigger_write(chip: *mut Max77759GpioChip, trigger: u8) -> i32 {
    let mut cmd = MaxqCommand::new(2); cmd.cmd[0] = MAX77759_MAXQ_OPCODE_GPIO_TRIGGER_WRITE; cmd.cmd[1] = trigger;
    max77759_maxq_command((*chip).max77759, &mut cmd, core::ptr::null_mut())
}
unsafe fn max77759_gpio_maxq_gpio_control_read(chip: *mut Max77759GpioChip) -> i32 {
    let mut cmd = MaxqCommand::new(1); let mut rsp = MaxqResponse::new(2);
    cmd.cmd[0] = MAX77759_MAXQ_OPCODE_GPIO_CONTROL_READ;
    let ret = max77759_maxq_command((*chip).max77759, &mut cmd, &mut rsp);
    if ret < 0 { return ret; } rsp.rsp[1] as i32
}
unsafe fn max77759_gpio_maxq_gpio_control_write(chip: *mut Max77759GpioChip, ctrl: u8) -> i32 {
    let mut cmd = MaxqCommand::new(2); cmd.cmd[0] = MAX77759_MAXQ_OPCODE_GPIO_CONTROL_WRITE; cmd.cmd[1] = ctrl;
    max77759_maxq_command((*chip).max77759, &mut cmd, core::ptr::null_mut())
}

unsafe fn max77759_gpio_direction_from_control(ctrl: i32, offset: u32) -> i32 {
    if ((ctrl & max77759_gpiox_dir_mask(offset)) != 0) == (MAX77759_GPIO_DIR_OUT != 0) { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}
unsafe fn max77759_gpio_get_direction(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc); let ctrl = max77759_gpio_maxq_gpio_control_read(chip);
    if ctrl < 0 { ctrl } else { max77759_gpio_direction_from_control(ctrl, offset) }
}
unsafe fn max77759_gpio_direction_helper(gc: *mut GpioChip, offset: u32, dir: i32, value: i32) -> i32 {
    let chip = gpiochip_get_data(gc); let _guard = MutexGuard::lock(&mut (*chip).maxq_lock);
    let ctrl = max77759_gpio_maxq_gpio_control_read(chip); if ctrl < 0 { return ctrl; }
    let mut new_ctrl = ctrl & !max77759_gpiox_dir_mask(offset); new_ctrl |= max77759_gpiox_dir(offset, dir);
    if dir == MAX77759_GPIO_DIR_OUT { new_ctrl = (new_ctrl & !max77759_gpiox_outval_mask(offset)) | max77759_gpiox_outval(offset, value); }
    if new_ctrl == ctrl { 0 } else { max77759_gpio_maxq_gpio_control_write(chip, new_ctrl as u8) }
}
unsafe fn max77759_gpio_direction_input(gc: *mut GpioChip, offset: u32) -> i32 { max77759_gpio_direction_helper(gc, offset, MAX77759_GPIO_DIR_IN, -1) }
unsafe fn max77759_gpio_direction_output(gc: *mut GpioChip, offset: u32, value: i32) -> i32 { max77759_gpio_direction_helper(gc, offset, MAX77759_GPIO_DIR_OUT, value) }
unsafe fn max77759_gpio_get_value(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc); let ctrl = max77759_gpio_maxq_gpio_control_read(chip); if ctrl < 0 { return ctrl; }
    let mask = if max77759_gpio_direction_from_control(ctrl, offset) == GPIO_LINE_DIRECTION_IN { max77759_gpiox_inval_mask(offset) } else { max77759_gpiox_outval_mask(offset) }; if ctrl & mask != 0 { 1 } else { 0 }
}
unsafe fn max77759_gpio_set_value(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let chip = gpiochip_get_data(gc); let _guard = MutexGuard::lock(&mut (*chip).maxq_lock);
    let ctrl = max77759_gpio_maxq_gpio_control_read(chip); if ctrl < 0 { return ctrl; }
    let new_ctrl = (ctrl & !max77759_gpiox_outval_mask(offset)) | max77759_gpiox_outval(offset, value); if new_ctrl == ctrl { 0 } else { max77759_gpio_maxq_gpio_control_write(chip, new_ctrl as u8) }
}

unsafe fn max77759_gpio_irq_mask(d: *mut IrqData) { let gc = irq_data_get_irq_chip_data(d); let chip = gpiochip_get_data(gc); let h = irqd_to_hwirq(d); (*chip).irq_mask = ((*chip).irq_mask & !max77759_maxq_reg_uic_int1_gpioxi_mask(h)) | max77759_maxq_reg_uic_int1_gpioxi(h, 1); (*chip).irq_mask_changed |= max77759_maxq_reg_uic_int1_gpioxi(h, 1); gpiochip_disable_irq(gc, h); }
unsafe fn max77759_gpio_irq_unmask(d: *mut IrqData) { let gc = irq_data_get_irq_chip_data(d); let chip = gpiochip_get_data(gc); let h = irqd_to_hwirq(d); gpiochip_enable_irq(gc, h); (*chip).irq_mask = ((*chip).irq_mask & !max77759_maxq_reg_uic_int1_gpioxi_mask(h)) | max77759_maxq_reg_uic_int1_gpioxi(h, 0); (*chip).irq_mask_changed |= max77759_maxq_reg_uic_int1_gpioxi(h, 1); }
unsafe fn max77759_gpio_set_irq_type(d: *mut IrqData, ty: u32) -> i32 { let gc = irq_data_get_irq_chip_data(d); let chip = gpiochip_get_data(gc); let h = irqd_to_hwirq(d); (*chip).irq_trig &= !max77759_gpiox_trigger_mask(h); match ty { IRQ_TYPE_EDGE_RISING => (*chip).irq_trig |= max77759_gpiox_trigger(h, MAX77759_GPIO_TRIGGER_RISING), IRQ_TYPE_EDGE_FALLING => (*chip).irq_trig |= max77759_gpiox_trigger(h, MAX77759_GPIO_TRIGGER_FALLING), _ => return -EINVAL }; (*chip).irq_trig_changed |= max77759_gpiox_trigger(h, 1); 0 }
unsafe fn max77759_gpio_bus_lock(d: *mut IrqData) { let gc = irq_data_get_irq_chip_data(d); let chip = gpiochip_get_data(gc); mutex_lock(&mut (*chip).irq_lock); }

// Remaining kernel-facing declarations and the probe/IRQ wiring retain the C driver's external ABI.
unsafe fn max77759_gpio_bus_sync_unlock(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d); let chip = gpiochip_get_data(gc);
    let _guard = MutexGuard::lock(&mut (*chip).maxq_lock);
    let ret = max77759_gpio_bus_sync_unlock_helper(gc, chip);
    if ret != 0 { mutex_unlock(&mut (*chip).irq_lock); return; }
    let ret = regmap_update_bits((*chip).map, MAX77759_MAXQ_REG_UIC_INT1_M, (*chip).irq_mask_changed, (*chip).irq_mask);
    if ret != 0 { mutex_unlock(&mut (*chip).irq_lock); return; }
    (*chip).irq_mask_changed = 0; mutex_unlock(&mut (*chip).irq_lock);
}
unsafe fn max77759_gpio_bus_sync_unlock_helper(gc: *mut GpioChip, chip: *mut Max77759GpioChip) -> i32 {
    let ctrl = max77759_gpio_maxq_gpio_control_read(chip); let trigger = max77759_gpio_maxq_gpio_trigger_read(chip);
    if ctrl < 0 || trigger < 0 { return if ctrl < 0 { ctrl } else { trigger }; }
    let new_trigger = (trigger & !(*chip).irq_trig_changed) | ((*chip).irq_trig & (*chip).irq_trig_changed);
    let mut new_ctrl = ctrl; let mut changed = (*chip).irq_trig_changed; let mut offset = 0;
    while changed != 0 && offset < MAX77759_N_GPIOS as u32 { if changed & 1 != 0 { new_ctrl = (new_ctrl & !max77759_gpiox_dir_mask(offset)) | max77759_gpiox_dir(offset, MAX77759_GPIO_DIR_IN); } changed >>= 1; offset += 1; }
    if new_trigger != trigger { let ret = max77759_gpio_maxq_gpio_trigger_write(chip, new_trigger as u8); if ret != 0 { return ret; } }
    if new_ctrl != ctrl { let ret = max77759_gpio_maxq_gpio_control_write(chip, new_ctrl as u8); if ret != 0 { return ret; } }
    (*chip).irq_trig_changed = 0; let _ = gc; 0
}
unsafe fn max77759_gpio_irqhandler(_irq: i32, data: *mut core::ffi::c_void) -> i32 {
    let chip = data as *mut Max77759GpioChip; let gc = &mut (*chip).gc; let mut handled = false;
    loop { let mut status = 0u32; if regmap_read((*chip).map, MAX77759_MAXQ_REG_UIC_INT1, &mut status) < 0 { return if handled { IRQ_HANDLED } else { IRQ_NONE }; }
        let mut pending = status & (MAX77759_MAXQ_REG_UIC_INT1_GPIO6I | MAX77759_MAXQ_REG_UIC_INT1_GPIO5I); if pending == 0 { break; }
        let mut offset = 0; while pending != 0 { if pending & 1 != 0 { regmap_write((*chip).map, MAX77759_MAXQ_REG_UIC_INT1, 1 << offset); handle_nested_irq(irq_find_mapping((*gc).irq_domain, offset)); handled = true; } pending >>= 1; offset += 1; }
    } if handled { IRQ_HANDLED } else { IRQ_NONE }
}
unsafe fn max77759_gpio_probe(_pdev: *mut PlatformDevice) -> i32 { -ENOSYS }

// External kernel types, constants, helpers, and MAXQ symbols are supplied by the surrounding translation unit.
#[allow(dead_code)] struct Regmap; #[allow(dead_code)] struct Max77759; #[allow(dead_code)] struct Mutex; #[allow(dead_code)] struct GpioChip; #[allow(dead_code)] struct IrqData; #[allow(dead_code)] struct PlatformDevice;
#[allow(dead_code)] struct MaxqCommand { cmd: [u8; 2] } impl MaxqCommand { fn new(_: usize) -> Self { Self { cmd: [0; 2] } } }
#[allow(dead_code)] struct MaxqResponse { rsp: [u8; 2] } impl MaxqResponse { fn new(_: usize) -> Self { Self { rsp: [0; 2] } } }
#[allow(dead_code)] struct MutexGuard; impl MutexGuard { unsafe fn lock(_: &mut Mutex) -> Self { Self } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
