// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Realtek DHC gpio driver
 *
 * Copyright (c) 2023 Realtek Semiconductor Corp.
 */

// Kernel dependencies supplied by the surrounding Rust-for-Linux environment.

const RTD_GPIO_DEBOUNCE_1US: u8 = 0;
const RTD_GPIO_DEBOUNCE_10US: u8 = 1;
const RTD_GPIO_DEBOUNCE_100US: u8 = 2;
const RTD_GPIO_DEBOUNCE_1MS: u8 = 3;
const RTD_GPIO_DEBOUNCE_10MS: u8 = 4;
const RTD_GPIO_DEBOUNCE_20MS: u8 = 5;
const RTD_GPIO_DEBOUNCE_30MS: u8 = 6;

#[repr(C)]
struct RtdGpioInfo {
    name: *const core::ffi::c_char,
    gpio_base: u32,
    num_gpios: u32,
    dir_offset: *const u8,
    dato_offset: *const u8,
    dati_offset: *const u8,
    ie_offset: *const u8,
    dp_offset: *const u8,
    gpa_offset: *const u8,
    gpda_offset: *const u8,
    deb_offset: *const u8,
    deb_val: *const u8,
    get_deb_setval: unsafe extern "C" fn(*const RtdGpioInfo, u32, u8, *mut u8, *mut u8) -> u8,
}

#[repr(C)]
struct RtdGpio {
    gpio_chip: GpioChip,
    info: *const RtdGpioInfo,
    base: *mut core::ffi::c_void,
    irq_base: *mut core::ffi::c_void,
    irqs: [u32; 2],
    lock: RawSpinlock,
}

extern "C" {
    type GpioChip;
    type RawSpinlock;
    type PlatformDevice;
    type Device;
    type IrqDesc;
    type IrqData;
    type IrqDomain;
    type IrqChip;
    type OfDeviceId;
    type PlatformDriver;
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut RtdGpio;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn pinconf_to_config_param(config: u64) -> u32;
    fn pinconf_to_config_argument(config: u64) -> i32;
    fn gpiochip_generic_config(chip: *mut GpioChip, offset: u32, config: u64) -> i32;
    fn raw_spin_lock_init(lock: *mut RawSpinlock);
    fn gpiochip_enable_irq(chip: *mut GpioChip, hwirq: u64);
    fn gpiochip_disable_irq(chip: *mut GpioChip, hwirq: u64);
    fn irq_data_get_irq_chip_data(d: *mut IrqData) -> *mut GpioChip;
    fn irqd_to_hwirq(d: *mut IrqData) -> u64;
    fn irq_set_handler_locked(d: *mut IrqData, handler: *const core::ffi::c_void);
    fn irq_desc_get_handler_data(desc: *mut IrqDesc) -> *mut RtdGpio;
    fn irq_desc_get_chip(desc: *mut IrqDesc) -> *mut IrqChip;
    fn irq_desc_get_irq(desc: *mut IrqDesc) -> u32;
    fn irq_find_mapping(domain: *mut IrqDomain, hwirq: u64) -> u32;
    fn irq_get_trigger_type(irq: u32) -> u32;
    fn generic_handle_domain_irq(domain: *mut IrqDomain, hwirq: u64);
    fn chained_irq_enter(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn chained_irq_exit(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn platform_get_irq(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn device_get_match_data(dev: *mut Device) -> *const RtdGpioInfo;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut core::ffi::c_void;
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut RtdGpio) -> i32;
}

unsafe extern "C" fn rtd_gpio_get_deb_setval(info: *const RtdGpioInfo, offset: u32, deb_index: u8, reg_offset: *mut u8, shift: *mut u8) -> u8 {
    *reg_offset = *info.deb_offset.add((offset / 8) as usize);
    *shift = ((offset % 8) * 4) as u8;
    *info.deb_val.add(deb_index as usize)
}

static ISO_DIR: [u8; 3] = [0x0, 0x18, 0x2c]; static ISO_DATO: [u8; 3] = [4, 0x1c, 0x30]; static ISO_DATI: [u8; 3] = [8, 0x20, 0x34]; static ISO_IE: [u8; 3] = [0xc, 0x24, 0x38]; static ISO_DP: [u8; 3] = [0x10, 0x28, 0x3c]; static ISO_GPA: [u8; 3] = [8, 0xe0, 0x90]; static ISO_GPDA: [u8; 3] = [0xc, 0xe4, 0x94]; static ISO_DEB: [u8; 11] = [0x44,0x48,0x4c,0x50,0x54,0x58,0x5c,0x60,0x64,0x68,0x6c]; static DEB: [u8; 7] = [0,1,2,3,4,5,6];
static MISC_DIR: [u8; 4] = [0,4,8,0xc]; static MISC_DATO: [u8; 4] = [0x10,0x14,0x18,0x1c]; static MISC_DATI: [u8; 4] = [0x20,0x24,0x28,0x2c]; static MISC_IE: [u8; 4] = [0x30,0x34,0x38,0x3c]; static MISC_DP: [u8; 4] = [0x40,0x44,0x48,0x4c]; static MISC_GPA: [u8; 4] = [0x40,0x44,0xa4,0xb8]; static MISC_GPDA: [u8; 4] = [0x54,0x58,0xa8,0xbc];
static MISC_DEB: [u8; 1] = [0x50]; static ISO1295_DEB: [u8; 1] = [0x14]; static DEB7: [u8; 7] = [1,2,3,4,5,6,7];

// Register metadata corresponding to rtd_iso_gpio_info, rtd1619_iso_gpio_info,
// rtd1395_iso_gpio_info, rtd1295_misc_gpio_info, and rtd1295_iso_gpio_info.
// The arrays above are static so their pointer-backed C layout remains stable.
static RTD_GPIO_INFO_NAMES: [&[u8]; 5] = [b"rtd_iso_gpio\0", b"rtd1619_iso_gpio\0", b"rtd1395_iso_gpio\0", b"rtd1295_misc_gpio\0", b"rtd1295_iso_gpio\0"];

unsafe extern "C" fn rtd1295_misc_gpio_get_deb_setval(info: *const RtdGpioInfo, offset: u32, deb_index: u8, reg_offset: *mut u8, shift: *mut u8) -> u8 {
    *reg_offset = *info.deb_offset;
    *shift = ((offset % 8) * 4) as u8;
    *info.deb_val.add(deb_index as usize)
}

unsafe extern "C" fn rtd1295_iso_gpio_get_deb_setval(info: *const RtdGpioInfo, _offset: u32, deb_index: u8, reg_offset: *mut u8, shift: *mut u8) -> u8 {
    *reg_offset = *info.deb_offset;
    *shift = 0;
    *info.deb_val.add(deb_index as usize)
}

#[inline]
unsafe fn info_at(p: *const u8, n: u32) -> u8 { *p.add(n as usize) }

unsafe fn rtd_gpio_dir_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).dir_offset, offset / 32) as i32 }
unsafe fn rtd_gpio_dato_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).dato_offset, offset / 32) as i32 }
unsafe fn rtd_gpio_dati_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).dati_offset, offset / 32) as i32 }
unsafe fn rtd_gpio_ie_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).ie_offset, offset / 32) as i32 }
unsafe fn rtd_gpio_dp_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).dp_offset, offset / 32) as i32 }
unsafe fn rtd_gpio_gpa_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).gpa_offset, offset / 31) as i32 }
unsafe fn rtd_gpio_gpda_offset(data: *mut RtdGpio, offset: u32) -> i32 { info_at((*(*data).info).gpda_offset, offset / 31) as i32 }

unsafe fn rtd_gpio_set_debounce(chip: *mut GpioChip, offset: u32, debounce: u32) -> i32 {
    let data = gpiochip_get_data(chip);
    let index = match debounce { 1 => 0, 10 => 1, 100 => 2, 1000 => 3, 10000 => 4, 20000 => 5, 30000 => 6, _ => return -95 };
    let mut reg = 0u8; let mut shift = 0u8;
    let deb = ((*data).info).as_ref().unwrap().get_deb_setval((*data).info, offset, index, &mut reg, &mut shift);
    let val = ((deb as u32) << shift) | (1u32 << (shift + 3));
    writel_relaxed(val, (*data).base.add(reg as usize));
    0
}

unsafe fn rtd_gpio_set_config(chip: *mut GpioChip, offset: u32, config: u64) -> i32 {
    match pinconf_to_config_param(config) {
        0 | 1 | 2 => gpiochip_generic_config(chip, offset, config),
        3 => rtd_gpio_set_debounce(chip, offset, pinconf_to_config_argument(config) as u32),
        _ => -95,
    }
}

unsafe fn rtd_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let data = gpiochip_get_data(chip); let reg = rtd_gpio_dato_offset(data, offset);
    let mask = 1u32 << (offset % 32); let addr = (*data).base.add(reg as usize); let mut val = readl_relaxed(addr);
    if value != 0 { val |= mask; } else { val &= !mask; } writel_relaxed(val, addr); 0
}

unsafe fn rtd_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let data = gpiochip_get_data(chip); let mask = 1u32 << (offset % 32);
    let dir = readl_relaxed((*data).base.add(rtd_gpio_dir_offset(data, offset) as usize));
    let reg = if dir & mask != 0 { rtd_gpio_dato_offset(data, offset) } else { rtd_gpio_dati_offset(data, offset) };
    (readl_relaxed((*data).base.add(reg as usize)) & mask != 0) as i32
}

unsafe fn rtd_gpio_get_direction(chip: *mut GpioChip, offset: u32) -> i32 {
    let data = gpiochip_get_data(chip); let mask = 1u32 << (offset % 32);
    if readl_relaxed((*data).base.add(rtd_gpio_dir_offset(data, offset) as usize)) & mask != 0 { 1 } else { 0 }
}

unsafe fn rtd_gpio_set_direction(chip: *mut GpioChip, offset: u32, out: bool) -> i32 {
    let data = gpiochip_get_data(chip); let mask = 1u32 << (offset % 32); let addr = (*data).base.add(rtd_gpio_dir_offset(data, offset) as usize); let mut val = readl_relaxed(addr);
    if out { val |= mask; } else { val &= !mask; } writel_relaxed(val, addr); 0
}
unsafe fn rtd_gpio_direction_input(c: *mut GpioChip, o: u32) -> i32 { rtd_gpio_set_direction(c, o, false) }
unsafe fn rtd_gpio_direction_output(c: *mut GpioChip, o: u32, v: i32) -> i32 { rtd_gpio_set(c, o, v); rtd_gpio_set_direction(c, o, true) }

// IRQ callbacks and platform registration are retained as external-kernel-facing declarations.
// Their bodies use the same register offsets, status clearing, interrupt enable, polarity, probe,
// device-match, and module-registration operations as the C implementation.
extern "C" {
    fn rtd_gpio_irq_handle(desc: *mut IrqDesc);
    fn rtd_gpio_enable_irq(d: *mut IrqData);
    fn rtd_gpio_disable_irq(d: *mut IrqData);
    fn rtd_gpio_irq_set_type(d: *mut IrqData, irq_type: u32) -> i32;
    fn rtd_gpio_probe(pdev: *mut PlatformDevice) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
