// SPDX-License-Identifier: GPL-2.0-only
/* Kontron PLD GPIO driver */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

const KEMPLD_GPIO_MAX_NUM: u32 = 16;
const KEMPLD_GPIO_DIR: u8 = 0x40;
const KEMPLD_GPIO_LVL: u8 = 0x42;
const KEMPLD_GPIO_STS: u8 = 0x44;
const KEMPLD_GPIO_EVT_LVL_EDGE: u8 = 0x46;
const KEMPLD_GPIO_EVT_LOW_HIGH: u8 = 0x48;
const KEMPLD_GPIO_IEN: u8 = 0x4a;
const KEMPLD_GPIO_OUT_LVL: u8 = 0x4e;

static mut gpio_irq: u32 = 0;

#[repr(C)]
pub struct kempld_gpio_data {
    pub chip: gpio_chip,
    pub pld: *mut kempld_device_data,
    pub out_lvl_reg: u8,
    pub irq_lock: mutex,
    pub ien: u16,
    pub evt_low_high: u16,
    pub evt_lvl_edge: u16,
}

#[repr(C)] pub struct gpio_chip { pub irq: gpio_irq_chip, pub ngpio: u32, pub label: *const u8, pub owner: *mut c_void, pub parent: *mut device, pub can_sleep: bool, pub base: i32 }
#[repr(C)] pub struct gpio_irq_chip { _unused: [u8; 0] }
#[repr(C)] pub struct irq_data { pub hwirq: u32 }
#[repr(C)] pub struct irq_chip { _unused: [u8; 0] }
#[repr(C)] pub struct mutex { _unused: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct kempld_device_data { pub info: kempld_info, pub dev: *mut device }
#[repr(C)] pub struct kempld_info { pub spec_major: u8, pub spec_minor: u8 }
#[repr(C)] pub struct kempld_platform_data { pub gpio_base: i32 }

extern "C" {
    fn kempld_read8(p: *mut kempld_device_data, reg: u8) -> u8;
    fn kempld_read16(p: *mut kempld_device_data, reg: u8) -> u16;
    fn kempld_write8(p: *mut kempld_device_data, reg: u8, val: u8);
    fn kempld_write16(p: *mut kempld_device_data, reg: u8, val: u16);
    fn kempld_get_mutex(p: *mut kempld_device_data);
    fn kempld_release_mutex(p: *mut kempld_device_data);
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut kempld_gpio_data;
    fn gpiochip_disable_irq(chip: *mut gpio_chip, irq: u32);
    fn gpiochip_enable_irq(chip: *mut gpio_chip, irq: u32);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

#[inline] fn gpio_mask(bit: u32) -> u8 { 1u8 << (bit % 8) }

unsafe fn kempld_gpio_bitop(pld: *mut kempld_device_data, reg: u8, bit: u32, val: bool) {
    let mut status = kempld_read8(pld, reg + (bit / 8) as u8);
    if val { status |= gpio_mask(bit); } else { status &= !gpio_mask(bit); }
    kempld_write8(pld, reg + (bit / 8) as u8, status);
}

unsafe fn kempld_gpio_get_bit(pld: *mut kempld_device_data, reg: u8, bit: u32) -> i32 {
    kempld_get_mutex(pld);
    let status = kempld_read8(pld, reg + (bit / 8) as u8);
    kempld_release_mutex(pld);
    if status & gpio_mask(bit) != 0 { 1 } else { 0 }
}

unsafe fn kempld_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); kempld_gpio_get_bit((*gpio).pld, KEMPLD_GPIO_LVL, offset)
}

unsafe fn kempld_gpio_get_multiple(chip: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let gpio = gpiochip_get_data(chip); let pld = (*gpio).pld; let mut reg = KEMPLD_GPIO_LVL;
    *bits &= !*mask; kempld_get_mutex(pld);
    let mut shift = 0; while shift < (*gpio).chip.ngpio { let msk = (*mask >> shift) & 0xff;
        if msk != 0 { *bits |= ((kempld_read8(pld, reg) as usize) & msk) << shift; }
        shift += 8; reg += 1;
    } kempld_release_mutex(pld); 0
}

unsafe fn kempld_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); kempld_get_mutex((*gpio).pld); kempld_gpio_bitop((*gpio).pld, (*gpio).out_lvl_reg, offset, value != 0); kempld_release_mutex((*gpio).pld); 0
}

unsafe fn kempld_gpio_set_multiple(chip: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let gpio = gpiochip_get_data(chip); let pld = (*gpio).pld; let mut reg = (*gpio).out_lvl_reg; kempld_get_mutex(pld); let mut shift = 0;
    while shift < (*gpio).chip.ngpio { let msk = ((*mask >> shift) & 0xff) as u8; if msk != 0 { let mut val = if msk != 0xff { kempld_read8(pld, reg) & !msk } else { 0 }; val |= ((*bits >> shift) as u8) & msk; kempld_write8(pld, reg, val); } shift += 8; reg += 1; } kempld_release_mutex(pld); 0
}

unsafe fn kempld_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 { let gpio = gpiochip_get_data(chip); kempld_get_mutex((*gpio).pld); kempld_gpio_bitop((*gpio).pld, KEMPLD_GPIO_DIR, offset, false); kempld_release_mutex((*gpio).pld); 0 }
unsafe fn kempld_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 { let gpio = gpiochip_get_data(chip); kempld_get_mutex((*gpio).pld); kempld_gpio_bitop((*gpio).pld, (*gpio).out_lvl_reg, offset, value != 0); kempld_gpio_bitop((*gpio).pld, KEMPLD_GPIO_DIR, offset, true); kempld_release_mutex((*gpio).pld); 0 }
unsafe fn kempld_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 { let gpio = gpiochip_get_data(chip); if kempld_gpio_get_bit((*gpio).pld, KEMPLD_GPIO_DIR, offset) != 0 { 1 } else { 0 } }

unsafe fn kempld_gpio_pincount(pld: *mut kempld_device_data) -> u32 { kempld_get_mutex(pld); let back = kempld_read16(pld, KEMPLD_GPIO_EVT_LVL_EDGE); kempld_write16(pld, KEMPLD_GPIO_EVT_LVL_EDGE, 0); let evt = kempld_read16(pld, KEMPLD_GPIO_EVT_LVL_EDGE); kempld_write16(pld, KEMPLD_GPIO_EVT_LVL_EDGE, back); kempld_release_mutex(pld); if evt != 0 { evt.trailing_zeros() } else { 16 } }

const IRQ_TYPE_EDGE_RISING: u32 = 1; const IRQ_TYPE_EDGE_FALLING: u32 = 2;
const IRQ_TYPE_LEVEL_HIGH: u32 = 4; const IRQ_TYPE_LEVEL_LOW: u32 = 8;
extern "C" {
    fn irq_data_get_irq_chip_data(data: *mut irq_data) -> *mut gpio_chip;
    fn irq_find_mapping(domain: *mut c_void, pin: u32) -> u32;
    fn handle_nested_irq(irq: u32);
}

unsafe fn kempld_irq_mask(data: *mut irq_data) { let chip = irq_data_get_irq_chip_data(data); let gpio = gpiochip_get_data(chip); (*gpio).ien &= !(1u16 << (*data).hwirq); gpiochip_disable_irq(chip, (*data).hwirq); }
unsafe fn kempld_irq_unmask(data: *mut irq_data) { let chip = irq_data_get_irq_chip_data(data); let gpio = gpiochip_get_data(chip); gpiochip_enable_irq(chip, (*data).hwirq); (*gpio).ien |= 1u16 << (*data).hwirq; }
unsafe fn kempld_irq_set_type(data: *mut irq_data, typ: u32) -> i32 { let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)); let bit = 1u16 << (*data).hwirq; match typ { IRQ_TYPE_EDGE_RISING => { (*gpio).evt_low_high |= bit; (*gpio).evt_lvl_edge |= bit; }, IRQ_TYPE_EDGE_FALLING => { (*gpio).evt_low_high &= !bit; (*gpio).evt_lvl_edge |= bit; }, IRQ_TYPE_LEVEL_HIGH => { (*gpio).evt_low_high |= bit; (*gpio).evt_lvl_edge &= !bit; }, IRQ_TYPE_LEVEL_LOW => { (*gpio).evt_low_high &= !bit; (*gpio).evt_lvl_edge &= !bit; }, _ => return -22 }; 0 }
unsafe fn kempld_irq_bus_lock(data: *mut irq_data) { let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)); mutex_lock(&mut (*gpio).irq_lock); }
unsafe fn kempld_irq_bus_sync_unlock(data: *mut irq_data) { let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)); kempld_get_mutex((*gpio).pld); kempld_write16((*gpio).pld, KEMPLD_GPIO_EVT_LVL_EDGE, (*gpio).evt_lvl_edge); kempld_write16((*gpio).pld, KEMPLD_GPIO_EVT_LOW_HIGH, (*gpio).evt_low_high); kempld_write16((*gpio).pld, KEMPLD_GPIO_IEN, (*gpio).ien); kempld_release_mutex((*gpio).pld); mutex_unlock(&mut (*gpio).irq_lock); }

unsafe fn kempld_gpio_irq_handler(_irq: i32, data: *mut c_void) -> i32 { let gpio = data as *mut kempld_gpio_data; kempld_get_mutex((*gpio).pld); let mut status = kempld_read16((*gpio).pld, KEMPLD_GPIO_STS); if status != 0 { kempld_write16((*gpio).pld, KEMPLD_GPIO_STS, status); } kempld_release_mutex((*gpio).pld); status &= (*gpio).ien; if status == 0 { return 0; } let mut pin = 0; while pin < (*gpio).chip.ngpio { if status & (1 << pin) != 0 { let child = irq_find_mapping(core::ptr::null_mut(), pin); handle_nested_irq(child); } pin += 1; } 1 }

unsafe fn kempld_gpio_irq_init(_dev: *mut device, gpio: *mut kempld_gpio_data) -> i32 { let pld = (*gpio).pld; kempld_get_mutex(pld); let mut irq = kempld_read8(pld, 0); kempld_release_mutex(pld); if irq == 0xff { return 0; } if gpio_irq > 0 { irq = (irq & !0x0f) | (gpio_irq as u8 & 0x0f); } if irq & 0x0f == 0 { return 0; } kempld_get_mutex(pld); (*gpio).evt_low_high = kempld_read16(pld, KEMPLD_GPIO_EVT_LOW_HIGH); (*gpio).evt_lvl_edge = kempld_read16(pld, KEMPLD_GPIO_EVT_LVL_EDGE); kempld_write16(pld, KEMPLD_GPIO_IEN, 0); kempld_write16(pld, KEMPLD_GPIO_STS, 0xffff); kempld_write8(pld, 0, irq); kempld_release_mutex(pld); 0 }

// The platform probe and module registration use kernel allocation and registration
// helpers supplied by the surrounding Linux translation.
unsafe fn kempld_gpio_probe(_pdev: *mut platform_device) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
