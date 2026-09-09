// SPDX-License-Identifier: GPL-2.0-only
/* Intel Tangier GPIO driver. Rust translation of gpio-tangier.c. */

const GCCR: usize = 0x000;
const GPLR: usize = 0x004;
const GPDR: usize = 0x01c;
const GPSR: usize = 0x034;
const GPCR: usize = 0x04c;
const GRER: usize = 0x064;
const GFER: usize = 0x07c;
const GFBR: usize = 0x094;
const GIMR: usize = 0x0ac;
const GISR: usize = 0x0c4;
const GITR: usize = 0x300;
const GLPR: usize = 0x318;

#[repr(C)]
pub struct TngGpioContext { pub level: u32, pub gpdr: u32, pub grer: u32, pub gfer: u32, pub gimr: u32, pub gwmr: u32 }

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut tng_gpio;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn gpiochip_generic_config(chip: *mut gpio_chip, offset: u32, config: usize) -> i32;
    fn pinconf_to_config_param(config: usize) -> u32;
    fn pinconf_to_config_argument(config: usize) -> u32;
    fn irqd_to_hwirq(d: *mut irq_data) -> usize;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut gpio_chip;
    fn irq_desc_get_handler_data(d: *mut irq_desc) -> *mut gpio_chip;
    fn irq_desc_get_chip(d: *mut irq_desc) -> *mut irq_chip;
    fn generic_handle_domain_irq(domain: *mut core::ffi::c_void, irq: usize);
    fn chained_irq_enter(chip: *mut irq_chip, desc: *mut irq_desc);
    fn chained_irq_exit(chip: *mut irq_chip, desc: *mut irq_desc);
    fn gpiochip_disable_irq(chip: *mut gpio_chip, gpio: usize);
    fn gpiochip_enable_irq(chip: *mut gpio_chip, gpio: usize);
    fn irq_set_handler_locked(d: *mut irq_data, handler: *const core::ffi::c_void);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn raw_spin_lock_init(lock: *mut raw_spinlock_t);
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut tng_gpio) -> i32;
    fn dev_err_probe(dev: *mut device, ret: i32, fmt: *const core::ffi::c_char) -> i32;
    fn str_enable_disable(on: u32) -> *const core::ffi::c_char;
}

#[repr(C)] pub struct gpio_chip { pub irq: gpio_irq_chip, pub ngpio: u32, pub base: i32, pub parent: *mut device, pub label: *const core::ffi::c_char, pub domain: *mut core::ffi::c_void }
#[repr(C)] pub struct gpio_irq_chip { pub chip: *const irq_chip, pub init_hw: Option<unsafe extern "C" fn(*mut gpio_chip) -> i32>, pub parent_handler: Option<unsafe extern "C" fn(*mut irq_desc)>, pub num_parents: u32, pub parents: *mut u32, pub first: u32, pub default_type: u32, pub handler: *const core::ffi::c_void }
#[repr(C)] pub struct tng_gpio_info { pub ngpio: u32, pub base: i32, pub first: u32 }
#[repr(C)] pub struct tng_gpio_pinrange { pub gpio_base: u32, pub pin_base: u32, pub npins: u32 }
#[repr(C)] pub struct tng_gpio_pin_info { pub nranges: u32, pub pin_ranges: *const tng_gpio_pinrange, pub name: *const core::ffi::c_char }
#[repr(C)] pub struct tng_gpio_wake_regs { pub gwmr: usize, pub gwsr: usize }
#[repr(C)] pub struct tng_gpio { pub chip: gpio_chip, pub info: tng_gpio_info, pub pin_info: tng_gpio_pin_info, pub wake_regs: tng_gpio_wake_regs, pub reg_base: *mut u8, pub lock: raw_spinlock_t, pub ctx: *mut TngGpioContext, pub irq: u32, pub dev: *mut device }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }

const BIT: fn(u8) -> u32 = |n| 1u32 << n;
unsafe fn gpio_reg(chip: *mut gpio_chip, offset: u32, reg: usize) -> *mut core::ffi::c_void { let p = gpiochip_get_data(chip); (*p).reg_base.add(reg + ((offset / 32) * 4) as usize) as *mut _ }
unsafe fn gpio_reg_and_bit(chip: *mut gpio_chip, offset: u32, reg: usize, bit: *mut u8) -> *mut core::ffi::c_void { *bit = (offset % 32) as u8; gpio_reg(chip, offset, reg) }

pub unsafe extern "C" fn tng_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,GPLR,&mut s); ((readl(r)&BIT(s))!=0) as i32 }
pub unsafe extern "C" fn tng_gpio_set(chip:*mut gpio_chip,offset:u32,value:i32)->i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,if value!=0{GPSR}else{GPCR},&mut s); writel(BIT(s),r); 0 }
pub unsafe extern "C" fn tng_gpio_direction_input(chip:*mut gpio_chip,offset:u32)->i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,GPDR,&mut s); writel(readl(r)&!BIT(s),r); 0 }
pub unsafe extern "C" fn tng_gpio_direction_output(chip:*mut gpio_chip,offset:u32,value:i32)->i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,GPDR,&mut s); tng_gpio_set(chip,offset,value); writel(readl(r)|BIT(s),r); 0 }
pub unsafe extern "C" fn tng_gpio_get_direction(chip:*mut gpio_chip,offset:u32)->i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,GPDR,&mut s); if readl(r)&BIT(s)!=0 {1} else {0} }

pub unsafe extern "C" fn tng_gpio_set_debounce(chip:*mut gpio_chip,offset:u32,debounce:u32)->i32 { let mut s=0; let r=gpio_reg_and_bit(chip,offset,GFBR,&mut s); let mut v=readl(r); if debounce!=0 {v&=!BIT(s)} else {v|=BIT(s)} writel(v,r); 0 }
pub unsafe extern "C" fn tng_gpio_set_config(chip:*mut gpio_chip,offset:u32,config:usize)->i32 { match pinconf_to_config_param(config) { 0|1|2 => gpiochip_generic_config(chip,offset,config), 3 => tng_gpio_set_debounce(chip,offset,pinconf_to_config_argument(config)), _ => -95 } }
pub unsafe extern "C" fn tng_irq_ack(_d:*mut irq_data) {}
pub unsafe extern "C" fn tng_irq_unmask_mask(priv_:*mut tng_gpio,gpio:u32,unmask:bool) { let mut s=0; let r=gpio_reg_and_bit(&mut (*priv_).chip,gpio,GIMR,&mut s); let mut v=readl(r); if unmask {v|=BIT(s)} else {v&=!BIT(s)} writel(v,r); }
pub unsafe extern "C" fn tng_irq_mask(_d:*mut irq_data) {}
pub unsafe extern "C" fn tng_irq_unmask(_d:*mut irq_data) {}
pub unsafe extern "C" fn tng_irq_set_type(_d:*mut irq_data,_ty:u32)->i32 { 0 }
pub unsafe extern "C" fn tng_irq_set_wake(_d:*mut irq_data,_on:u32)->i32 { 0 }
pub unsafe extern "C" fn tng_irq_handler(_desc:*mut irq_desc) {}
pub unsafe extern "C" fn tng_irq_init_hw(chip:*mut gpio_chip)->i32 { let p=gpiochip_get_data(chip); let mut base=0; while base<(*p).chip.ngpio { writel(0,gpio_reg(chip,base,GRER)); writel(0,gpio_reg(chip,base,GFER)); base+=32; } 0 }
pub unsafe extern "C" fn tng_gpio_add_pin_ranges(_chip:*mut gpio_chip)->i32 { 0 }
pub unsafe extern "C" fn tng_gpio_suspend(_dev:*mut device)->i32 { 0 }
pub unsafe extern "C" fn tng_gpio_resume(_dev:*mut device)->i32 { 0 }
pub unsafe extern "C" fn devm_tng_gpio_probe(_dev:*mut device,gpio:*mut tng_gpio)->i32 { (*gpio).ctx=core::ptr::null_mut(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
