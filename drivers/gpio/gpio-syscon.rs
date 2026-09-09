// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  SYSCON GPIO driver
 *
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

const GPIO_SYSCON_FEAT_IN: u32 = 1 << 0;
const GPIO_SYSCON_FEAT_OUT: u32 = 1 << 1;
const GPIO_SYSCON_FEAT_DIR: u32 = 1 << 2;
const SYSCON_REG_SIZE: u32 = 4;
const SYSCON_REG_BITS: u32 = SYSCON_REG_SIZE * 8;

#[repr(C)]
pub struct syscon_gpio_data {
    pub flags: u32,
    pub bit_count: u32,
    pub dat_bit_offset: u32,
    pub dir_bit_offset: u32,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
}

#[repr(C)]
pub struct syscon_gpio_priv {
    pub chip: gpio_chip,
    pub syscon: *mut regmap,
    pub data: *const syscon_gpio_data,
    pub dreg_offset: u32,
    pub dir_reg_offset: u32,
}

#[repr(C)]
pub struct gpio_chip {
    pub parent: *mut device,
    pub owner: *mut core::ffi::c_void,
    pub label: *const core::ffi::c_char,
    pub base: i32,
    pub ngpio: u32,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
}

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub parent: *mut device_node }
#[repr(C)] pub struct platform_device { pub dev: device }

unsafe extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut syscon_gpio_priv;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const syscon_gpio_data;
    fn syscon_regmap_lookup_by_phandle(np: *mut device_node, name: *const core::ffi::c_char) -> *mut regmap;
    fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn of_property_read_u32_index(np: *mut device_node, name: *const core::ffi::c_char, index: u32, out: *mut u32) -> i32;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut syscon_gpio_priv) -> i32;
}

unsafe extern "C" fn syscon_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(chip);
    let offs = (*priv_).dreg_offset + (*(*priv_).data).dat_bit_offset + offset;
    let mut val = 0;
    let ret = regmap_read((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE, &mut val);
    if ret != 0 { return ret; }
    if val & (1u32 << (offs % SYSCON_REG_BITS)) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn syscon_gpio_set(chip: *mut gpio_chip, offset: u32, val: i32) {
    let priv_ = gpiochip_get_data(chip);
    let offs = (*priv_).dreg_offset + (*(*priv_).data).dat_bit_offset + offset;
    let bit = 1u32 << (offs % SYSCON_REG_BITS);
    let _ = regmap_update_bits((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE,
                               bit, if val != 0 { bit } else { 0 });
}

unsafe extern "C" fn syscon_gpio_dir_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(chip);
    if (*(*priv_).data).flags & GPIO_SYSCON_FEAT_DIR != 0 {
        let offs = (*priv_).dir_reg_offset + (*(*priv_).data).dir_bit_offset + offset;
        let bit = 1u32 << (offs % SYSCON_REG_BITS);
        let _ = regmap_update_bits((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE, bit, 0);
    }
    0
}

unsafe extern "C" fn syscon_gpio_dir_out(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let priv_ = gpiochip_get_data(chip);
    if (*(*priv_).data).flags & GPIO_SYSCON_FEAT_DIR != 0 {
        let offs = (*priv_).dir_reg_offset + (*(*priv_).data).dir_bit_offset + offset;
        let bit = 1u32 << (offs % SYSCON_REG_BITS);
        let _ = regmap_update_bits((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE, bit, bit);
    }
    if let Some(set) = (*chip).set { set(chip, offset, val); }
    0
}

static clps711x_mctrl_gpio: syscon_gpio_data = syscon_gpio_data { flags: GPIO_SYSCON_FEAT_IN, bit_count: 3, dat_bit_offset: 0x40 * 8 + 8, dir_bit_offset: 0, set: None };

unsafe extern "C" fn rockchip_gpio_set(chip: *mut gpio_chip, offset: u32, val: i32) {
    let priv_ = gpiochip_get_data(chip);
    let offs = (*priv_).dreg_offset + (*(*priv_).data).dat_bit_offset + offset;
    let bit = offs % SYSCON_REG_BITS;
    let data = (if val != 0 { 1u32 << bit } else { 0 }) | (1u32 << (bit + 16));
    let ret = regmap_write((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE, data);
    if ret < 0 { dev_err((*chip).parent, c"gpio write failed ret(%d)\n".as_ptr(), ret); }
}

static rockchip_rk3328_gpio_mute: syscon_gpio_data = syscon_gpio_data { flags: GPIO_SYSCON_FEAT_OUT, bit_count: 1, dat_bit_offset: 0x0428 * 8 + 1, dir_bit_offset: 0, set: Some(rockchip_gpio_set) };
const KEYSTONE_LOCK_BIT: u32 = 1;

unsafe extern "C" fn keystone_gpio_set(chip: *mut gpio_chip, offset: u32, val: i32) {
    let priv_ = gpiochip_get_data(chip);
    let offs = (*priv_).dreg_offset + (*(*priv_).data).dat_bit_offset + offset;
    if val == 0 { return; }
    let bit = 1u32 << (offs % SYSCON_REG_BITS);
    let ret = regmap_update_bits((*priv_).syscon, (offs / SYSCON_REG_BITS) * SYSCON_REG_SIZE, bit | KEYSTONE_LOCK_BIT, bit | KEYSTONE_LOCK_BIT);
    if ret < 0 { dev_err((*chip).parent, c"gpio write failed ret(%d)\n".as_ptr(), ret); }
}

static keystone_dsp_gpio: syscon_gpio_data = syscon_gpio_data { flags: GPIO_SYSCON_FEAT_OUT, bit_count: 28, dat_bit_offset: 4, dir_bit_offset: 0, set: Some(keystone_gpio_set) };

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char, data: *const syscon_gpio_data }

static syscon_gpio_ids: [of_device_id; 4] = [
    of_device_id { compatible: c"cirrus,ep7209-mctrl-gpio".as_ptr(), data: &clps711x_mctrl_gpio },
    of_device_id { compatible: c"ti,keystone-dsp-gpio".as_ptr(), data: &keystone_dsp_gpio },
    of_device_id { compatible: c"rockchip,rk3328-grf-gpio".as_ptr(), data: &rockchip_rk3328_gpio_mute },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn syscon_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = dev.of_node;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<syscon_gpio_priv>(), 0) as *mut syscon_gpio_priv;
    if priv_.is_null() { return -12; }
    (*priv_).data = of_device_get_match_data(dev);
    let mut use_parent_regmap = false;
    (*priv_).syscon = syscon_regmap_lookup_by_phandle(np, c"gpio,syscon-dev".as_ptr());
    if (*priv_).syscon.is_null() && !(*np).parent.is_null() {
        (*priv_).syscon = syscon_node_to_regmap((*np).parent);
        use_parent_regmap = true;
    }
    if (*priv_).syscon.is_null() { return -6; }
    if !use_parent_regmap {
        let ret = of_property_read_u32_index(np, c"gpio,syscon-dev".as_ptr(), 1, &mut (*priv_).dreg_offset);
        if ret != 0 { dev_err(dev, c"can't read the data register offset!\n".as_ptr()); }
        (*priv_).dreg_offset <<= 3;
        let ret = of_property_read_u32_index(np, c"gpio,syscon-dev".as_ptr(), 2, &mut (*priv_).dir_reg_offset);
        if ret != 0 { dev_dbg(dev, c"can't read the dir register offset!\n".as_ptr()); }
        (*priv_).dir_reg_offset <<= 3;
    }
    (*priv_).chip.parent = dev;
    (*priv_).chip.owner = core::ptr::null_mut();
    (*priv_).chip.label = dev_name(dev);
    (*priv_).chip.base = -1;
    (*priv_).chip.ngpio = (*(*priv_).data).bit_count;
    (*priv_).chip.get = Some(syscon_gpio_get);
    if (*(*priv_).data).flags & GPIO_SYSCON_FEAT_IN != 0 { (*priv_).chip.direction_input = Some(syscon_gpio_dir_in); }
    if (*(*priv_).data).flags & GPIO_SYSCON_FEAT_OUT != 0 {
        (*priv_).chip.set = (*(*priv_).data).set.or(Some(syscon_gpio_set));
        (*priv_).chip.direction_output = Some(syscon_gpio_dir_out);
    }
    devm_gpiochip_add_data(dev, &mut (*priv_).chip, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
