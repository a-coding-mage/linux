// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of the Nomadik GPIO implementation. Kernel-provided types
// and functions referenced below are intentionally left as external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn clk_enable(clk: *mut clk);
    fn clk_disable(clk: *mut clk);
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut nmk_gpio_chip;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut gpio_chip;
    fn irqd_to_hwirq(d: *mut irq_data) -> c_uint;
    fn gpiochip_disable_irq(gc: *mut gpio_chip, irq: c_uint);
    fn gpiochip_enable_irq(gc: *mut gpio_chip, irq: c_uint);
    fn irqd_irq_disabled(d: *mut irq_data) -> bool;
    fn irqd_is_wakeup_set(d: *mut irq_data) -> bool;
    fn generic_handle_domain_irq_safe(domain: *mut c_void, bit: c_uint);
    fn irq_has_action(irq: c_int) -> bool;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
}

type spinlock_t = c_void;
type clk = c_void;
type irqreturn_t = c_int;

const NMK_GPIO_SLPC: usize = 0;
const NMK_GPIO_DATS: usize = 4;
const NMK_GPIO_DATC: usize = 8;
const NMK_GPIO_DIRS: usize = 12;
const NMK_GPIO_DIRC: usize = 16;
const NMK_GPIO_RIMSC: usize = 20;
const NMK_GPIO_FIMSC: usize = 24;
const NMK_GPIO_RWIMSC: usize = 28;
const NMK_GPIO_FWIMSC: usize = 32;
const NMK_GPIO_IC: usize = 36;
const NMK_GPIO_IS: usize = 40;
const NMK_GPIO_DIR: usize = 44;
const NMK_GPIO_DAT: usize = 48;
const NMK_GPIO_AFSLA: usize = 52;
const NMK_GPIO_AFSLB: usize = 56;
const NMK_GPIO_PDIS: usize = 60;
const NMK_GPIO_LOWEMI: usize = 64;
const NMK_GPIO_PER_CHIP: u32 = 32;
const IRQ_TYPE_LEVEL_HIGH: u32 = 4;
const IRQ_TYPE_LEVEL_LOW: u32 = 8;
const IRQ_TYPE_EDGE_RISING: u32 = 1;
const IRQ_TYPE_EDGE_FALLING: u32 = 2;
const GPIO_LINE_DIRECTION_OUT: c_int = 1;
const GPIO_LINE_DIRECTION_IN: c_int = 0;
const IRQ_NONE: c_int = 0;
const IRQ_HANDLED: c_int = 1;

#[repr(C)] pub struct gpio_chip { pub ngpio: u32, pub base: c_int, pub irq: gpio_irq_chip, pub parent: *mut device, pub gpiodev: *mut c_void }
#[repr(C)] pub struct gpio_irq_chip { pub domain: *mut c_void }
#[repr(C)] pub struct irq_data { pub hwirq: c_uint }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, pub id: c_int }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct pinctrl_dev { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

#[repr(C)] pub struct nmk_gpio_chip {
    pub chip: gpio_chip, pub addr: *mut c_void, pub clk: *mut clk, pub lock: spinlock_t,
    pub rimsc: u32, pub fimsc: u32, pub rwimsc: u32, pub fwimsc: u32,
    pub edge_rising: u32, pub edge_falling: u32, pub real_wake: u32, pub lowemi: u32,
    pub bank: u32, pub sleepmode: bool, pub is_mobileye_soc: bool,
}

#[inline] fn bit(n: c_uint) -> u32 { 1u32.wrapping_shl(n) }

pub unsafe fn __nmk_gpio_set_slpm(c: *mut nmk_gpio_chip, off: c_uint, mode: c_uint) {
    if (*c).is_mobileye_soc { return; }
    let mut v = readl((*c).addr.add(NMK_GPIO_SLPC));
    if mode == 0 { v |= bit(off); } else { v &= !bit(off); }
    writel(v, (*c).addr.add(NMK_GPIO_SLPC));
}
unsafe fn __nmk_gpio_set_output(c: *mut nmk_gpio_chip, off: c_uint, val: c_int) {
    writel(bit(off), (*c).addr.add(if val != 0 { NMK_GPIO_DATS } else { NMK_GPIO_DATC }));
}
pub unsafe fn __nmk_gpio_make_output(c: *mut nmk_gpio_chip, off: c_uint, val: c_int) {
    writel(bit(off), (*c).addr.add(NMK_GPIO_DIRS)); __nmk_gpio_set_output(c, off, val);
}

unsafe fn __nmk_gpio_irq_modify(c: *mut nmk_gpio_chip, off: c_int, wake: bool, enable: bool) {
    if wake && (*c).is_mobileye_soc { return; }
    let (rr, fr, rp, fp) = if wake { (NMK_GPIO_RWIMSC, NMK_GPIO_FWIMSC, &mut (*c).rwimsc, &mut (*c).fwimsc) }
        else { (NMK_GPIO_RIMSC, NMK_GPIO_FIMSC, &mut (*c).rimsc, &mut (*c).fimsc) };
    let b = bit(off as c_uint);
    if (*c).edge_rising & b != 0 { if enable { *rp |= b } else { *rp &= !b }; writel(*rp, (*c).addr.add(rr)); }
    if (*c).edge_falling & b != 0 { if enable { *fp |= b } else { *fp &= !b }; writel(*fp, (*c).addr.add(fr)); }
}
unsafe fn __nmk_gpio_set_wake(c: *mut nmk_gpio_chip, off: c_int, on: bool) {
    if (*c).is_mobileye_soc { return; }
    if (*c).sleepmode && on { __nmk_gpio_set_slpm(c, off as u32, 1); }
    __nmk_gpio_irq_modify(c, off, true, on);
}

pub unsafe fn nmk_gpio_get_dir(chip: *mut gpio_chip, off: c_uint) -> c_int {
    let c = gpiochip_get_data(chip); clk_enable((*c).clk);
    let v = readl((*c).addr.add(NMK_GPIO_DIR)) & bit(off); clk_disable((*c).clk);
    if v != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}
pub unsafe fn nmk_gpio_make_input(chip: *mut gpio_chip, off: c_uint) -> c_int {
    let c = gpiochip_get_data(chip); clk_enable((*c).clk); writel(bit(off), (*c).addr.add(NMK_GPIO_DIRC)); clk_disable((*c).clk); 0
}
pub unsafe fn nmk_gpio_get_input(chip: *mut gpio_chip, off: c_uint) -> c_int {
    let c = gpiochip_get_data(chip); clk_enable((*c).clk); let v = (readl((*c).addr.add(NMK_GPIO_DAT)) & bit(off)) != 0; clk_disable((*c).clk); v as c_int
}
pub unsafe fn nmk_gpio_set_output(chip: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    let c = gpiochip_get_data(chip); clk_enable((*c).clk); __nmk_gpio_set_output(c, off, val); clk_disable((*c).clk); 0
}
pub unsafe fn nmk_gpio_make_output(chip: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    let c = gpiochip_get_data(chip); clk_enable((*c).clk); __nmk_gpio_make_output(c, off, val); clk_disable((*c).clk); 0
}

pub unsafe fn nmk_gpio_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let c = gpiochip_get_data(gc);
    clk_enable((*c).clk); writel(bit((*d).hwirq), (*c).addr.add(NMK_GPIO_IC)); clk_disable((*c).clk);
}
pub unsafe fn nmk_gpio_irq_set_wake(d: *mut irq_data, on: bool) -> c_int {
    let gc = irq_data_get_irq_chip_data(d); let c = gpiochip_get_data(gc);
    if (*c).is_mobileye_soc { return -6; }
    if irqd_irq_disabled(d) { __nmk_gpio_set_wake(c, (*d).hwirq as c_int, on); }
    if on { (*c).real_wake |= bit((*d).hwirq); } else { (*c).real_wake &= !bit((*d).hwirq); } 0
}
pub unsafe fn nmk_gpio_irq_set_type(d: *mut irq_data, ty: u32) -> c_int {
    if ty & (IRQ_TYPE_LEVEL_HIGH | IRQ_TYPE_LEVEL_LOW) != 0 { return -22; }
    let gc = irq_data_get_irq_chip_data(d); let c = gpiochip_get_data(gc); let b = bit((*d).hwirq);
    if (*c).edge_rising & b != 0 { (*c).edge_rising &= !b; }
    if (*c).edge_falling & b != 0 { (*c).edge_falling &= !b; }
    if ty & IRQ_TYPE_EDGE_RISING != 0 { (*c).edge_rising |= b; }
    if ty & IRQ_TYPE_EDGE_FALLING != 0 { (*c).edge_falling |= b; } 0
}
pub unsafe fn nmk_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let c = gpiochip_get_data(gc);
    __nmk_gpio_irq_modify(c, (*d).hwirq as c_int, false, false); gpiochip_disable_irq(gc, (*d).hwirq);
}
pub unsafe fn nmk_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let c = gpiochip_get_data(gc);
    gpiochip_enable_irq(gc, (*d).hwirq); __nmk_gpio_irq_modify(c, (*d).hwirq as c_int, false, true);
}
pub unsafe fn nmk_gpio_irq_startup(d: *mut irq_data) -> c_uint { nmk_gpio_irq_unmask(d); 0 }
pub unsafe fn nmk_gpio_irq_shutdown(d: *mut irq_data) { nmk_gpio_irq_mask(d); }

// Remaining driver registration and debug/IRQ glue retain the C driver's ABI
// through external kernel bindings supplied by the surrounding repository.
extern "C" {
    static mut nmk_irq_chip: c_void;
    static mut nmk_gpio_driver: platform_driver;
}
pub unsafe fn nmk_gpio_init() -> c_int { platform_driver_register(&mut nmk_gpio_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
