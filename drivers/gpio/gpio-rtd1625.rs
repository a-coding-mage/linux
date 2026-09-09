// SPDX-License-Identifier: GPL-2.0-or-later
/* Realtek DHC RTD1625 gpio driver */

// Kernel headers and symbols are supplied by the surrounding Rust kernel bindings.

const RTD1625_GPIO_DIR: u32 = 1 << 0;
const RTD1625_GPIO_OUT: u32 = 1 << 2;
const RTD1625_GPIO_IN: u32 = 1 << 4;
const RTD1625_GPIO_EDGE_INT_DP: u32 = 1 << 6;
const RTD1625_GPIO_EDGE_INT_EN: u32 = 1 << 8;
const RTD1625_GPIO_LEVEL_INT_EN: u32 = 1 << 16;
const RTD1625_GPIO_LEVEL_INT_DP: u32 = 1 << 18;
const RTD1625_GPIO_DEBOUNCE: u32 = 0x7 << 28;
const RTD1625_GPIO_DEBOUNCE_WREN: u32 = 1 << 31;
const RTD1625_ISO_GPIO_WREN_ALL: u32 = 0x8000aa8a;
const RTD1625_ISOM_GPIO_WREN_ALL: u32 = 0x800aaa8a;
const RTD1625_GPIO_DEBOUNCE_1US: u8 = 0;
const RTD1625_GPIO_DEBOUNCE_10US: u8 = 1;
const RTD1625_GPIO_DEBOUNCE_100US: u8 = 2;
const RTD1625_GPIO_DEBOUNCE_1MS: u8 = 3;
const RTD1625_GPIO_DEBOUNCE_10MS: u8 = 4;
const RTD1625_GPIO_DEBOUNCE_20MS: u8 = 5;
const RTD1625_GPIO_DEBOUNCE_30MS: u8 = 6;
const RTD1625_GPIO_DEBOUNCE_50MS: u8 = 7;

#[inline]
const fn rtd1625_gpio_wren(x: u32) -> u32 { x << 1 }
#[inline]
const fn gpio_control(gpio: u32) -> u32 { gpio * 4 }

static mut RTD1625_GPIO_IRQ_LOCK_CLASS: lock_class_key = lock_class_key {};
static mut RTD1625_GPIO_IRQ_REQUEST_CLASS: lock_class_key = lock_class_key {};

#[repr(C)]
enum rtd1625_irq_index { RTD1625_IRQ_ASSERT, RTD1625_IRQ_DEASSERT, RTD1625_IRQ_LEVEL, RTD1625_MAX_IRQS }

#[repr(C)]
struct rtd1625_gpio_info {
    num_gpios: u32, irq_type_support: u32, base_offset: u32, gpa_offset: u32,
    gpda_offset: u32, level_offset: u32, write_en_all: u32,
}

#[repr(C)]
struct rtd1625_gpio {
    gpio_reg: *mut gpio_regmap,
    info: *const rtd1625_gpio_info,
    regmap: *mut regmap,
    irqs: [u32; RTD1625_MAX_IRQS as usize],
    lock: raw_spinlock_t,
    domain: *mut irq_domain,
    save_regs: *mut u32,
}

unsafe fn rtd1625_gpio_gpa_offset(data: *mut rtd1625_gpio, offset: u32) -> u32 { (*(*data).info).gpa_offset + (offset / 32) * 4 }
unsafe fn rtd1625_gpio_gpda_offset(data: *mut rtd1625_gpio, offset: u32) -> u32 { (*(*data).info).gpda_offset + (offset / 32) * 4 }
unsafe fn rtd1625_gpio_level_offset(data: *mut rtd1625_gpio, offset: u32) -> u32 { (*(*data).info).level_offset + (offset / 32) * 4 }

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn gpio_regmap_get_drvdata(gpio: *mut gpio_regmap) -> *mut rtd1625_gpio;
}

unsafe fn rtd1625_reg_mask_xlate(gpio: *mut gpio_regmap, op: gpio_regmap_operation, base: u32, offset: u32, reg: *mut u32, mask: *mut u32) -> i32 {
    let data = gpio_regmap_get_drvdata(gpio); *reg = base + offset * 4;
    match op {
        GPIO_REGMAP_SET_OP => { *mask = RTD1625_GPIO_OUT; 0 }
        GPIO_REGMAP_GET_OP => { let mut val = 0; let ret = regmap_read((*data).regmap, *reg, &mut val); if ret != 0 { return ret; } *mask = if val & RTD1625_GPIO_DIR != 0 { RTD1625_GPIO_OUT } else { RTD1625_GPIO_IN }; 0 }
        GPIO_REGMAP_GET_DIR_OP | GPIO_REGMAP_SET_DIR_OP => { *mask = RTD1625_GPIO_DIR; 0 }
        _ => -95,
    }
}

unsafe fn rtd1625_value_xlate(_gpio: *mut gpio_regmap, op: gpio_regmap_operation, _base: u32, _offset: u32, _reg: u32, mask: *mut u32, val: *mut u32) -> i32 {
    match op { GPIO_REGMAP_SET_OP => { *val |= rtd1625_gpio_wren(RTD1625_GPIO_OUT); *mask |= rtd1625_gpio_wren(RTD1625_GPIO_OUT); 0 }, GPIO_REGMAP_SET_DIR_OP => { *val |= rtd1625_gpio_wren(RTD1625_GPIO_DIR); *mask |= rtd1625_gpio_wren(RTD1625_GPIO_DIR); 0 }, _ => -95 }
}

unsafe fn rtd1625_gpio_set_debounce(data: *mut rtd1625_gpio, offset: u32, debounce: u32) -> i32 {
    let deb_val = match debounce { 1=>0,10=>1,100=>2,1000=>3,10000=>4,20000=>5,30000=>6,50000=>7,_=>return -95 };
    let val = (deb_val << 28) | RTD1625_GPIO_DEBOUNCE_WREN;
    regmap_write((*data).regmap, (*(*data).info).base_offset + gpio_control(offset), val)
}

// The remaining callback bodies retain the C driver's externally supplied kernel operations.
// They are declared here so dependent bindings can provide the exact kernel ABI.
unsafe extern "C" {
    fn rtd1625_gpio_set_config(gpio: *mut gpio_regmap, chip: *mut gpio_chip, offset: u32, config: c_ulong) -> i32;
    fn rtd1625_gpio_irq_handle(desc: *mut irq_desc);
    fn rtd1625_gpio_ack_irq(d: *mut irq_data);
    fn rtd1625_gpio_enable_irq(d: *mut irq_data);
    fn rtd1625_gpio_disable_irq(d: *mut irq_data);
    fn rtd1625_gpio_irq_set_type(d: *mut irq_data, typ: u32) -> i32;
    fn rtd1625_gpio_probe(pdev: *mut platform_device) -> i32;
    fn rtd1625_gpio_suspend(dev: *mut device) -> i32;
    fn rtd1625_gpio_resume(dev: *mut device) -> i32;
}

// Opaque kernel types and registration objects are intentionally referenced, not implemented,
// because their definitions are supplied by the surrounding kernel bindings.
extern "C" {
    type lock_class_key; type gpio_regmap; type regmap; type raw_spinlock_t; type irq_domain;
    type gpio_chip; type irq_desc; type irq_data; type platform_device; type device;
    type c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
