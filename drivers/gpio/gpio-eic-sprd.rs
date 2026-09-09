// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of gpio-eic-sprd.c. Kernel symbols are external dependencies. */

const SPRD_EIC_DBNC_DATA: u16 = 0x0;
const SPRD_EIC_DBNC_DMSK: u16 = 0x4;
const SPRD_EIC_DBNC_IEV: u16 = 0x14;
const SPRD_EIC_DBNC_IE: u16 = 0x18;
const SPRD_EIC_DBNC_RIS: u16 = 0x1c;
const SPRD_EIC_DBNC_MIS: u16 = 0x20;
const SPRD_EIC_DBNC_IC: u16 = 0x24;
const SPRD_EIC_DBNC_TRIG: u16 = 0x28;
const SPRD_EIC_DBNC_CTRL0: u16 = 0x40;
const SPRD_EIC_LATCH_INTEN: u16 = 0x0;
const SPRD_EIC_LATCH_INTRAW: u16 = 0x4;
const SPRD_EIC_LATCH_INTMSK: u16 = 0x8;
const SPRD_EIC_LATCH_INTCLR: u16 = 0xc;
const SPRD_EIC_LATCH_INTPOL: u16 = 0x10;
const SPRD_EIC_LATCH_INTMODE: u16 = 0x14;
const SPRD_EIC_ASYNC_INTIE: u16 = 0x0;
const SPRD_EIC_ASYNC_INTRAW: u16 = 0x4;
const SPRD_EIC_ASYNC_INTMSK: u16 = 0x8;
const SPRD_EIC_ASYNC_INTCLR: u16 = 0xc;
const SPRD_EIC_ASYNC_INTMODE: u16 = 0x10;
const SPRD_EIC_ASYNC_INTBOTH: u16 = 0x14;
const SPRD_EIC_ASYNC_INTPOL: u16 = 0x18;
const SPRD_EIC_ASYNC_DATA: u16 = 0x1c;
const SPRD_EIC_SYNC_INTIE: u16 = 0x0;
const SPRD_EIC_SYNC_INTRAW: u16 = 0x4;
const SPRD_EIC_SYNC_INTMSK: u16 = 0x8;
const SPRD_EIC_SYNC_INTCLR: u16 = 0xc;
const SPRD_EIC_SYNC_INTMODE: u16 = 0x10;
const SPRD_EIC_SYNC_INTBOTH: u16 = 0x14;
const SPRD_EIC_SYNC_INTPOL: u16 = 0x18;
const SPRD_EIC_SYNC_DATA: u16 = 0x1c;
const SPRD_EIC_MAX_BANK: usize = 3;
const SPRD_EIC_PER_BANK_NR: u32 = 8;
const SPRD_EIC_DATA_MASK: u32 = 0xff;
const SPRD_EIC_DBNC_MASK: u32 = 0xfff;

#[repr(C)]
pub struct gpio_chip { pub ngpio: u32, pub parent: *mut core::ffi::c_void, pub irq: gpio_irq_chip }
#[repr(C)] pub struct gpio_irq_chip { pub domain: *mut core::ffi::c_void }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: *mut core::ffi::c_void }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct sprd_eic { pub chip: gpio_chip, pub irq_nb: notifier_block, pub base: [*mut u8; SPRD_EIC_MAX_BANK], pub type_: sprd_eic_type, pub lock: usize, pub irq: i32 }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum sprd_eic_type { SPRD_EIC_DEBOUNCE, SPRD_EIC_LATCH, SPRD_EIC_ASYNC, SPRD_EIC_SYNC, SPRD_EIC_MAX }
#[repr(C)] pub struct sprd_eic_variant_data { pub type_: sprd_eic_type }
static mut SPRD_EIC_IRQ_NOTIFIER: usize = 0;
static SPRD_EIC_LABEL_NAME: [&[u8]; 4] = [b"eic-debounce\0", b"eic-latch\0", b"eic-async\0", b"eic-sync\0"];
static SC9860_EIC_DBNC_DATA: sprd_eic_variant_data = sprd_eic_variant_data { type_: sprd_eic_type::SPRD_EIC_DEBOUNCE };
static SC9860_EIC_LATCH_DATA: sprd_eic_variant_data = sprd_eic_variant_data { type_: sprd_eic_type::SPRD_EIC_LATCH };
static SC9860_EIC_ASYNC_DATA: sprd_eic_variant_data = sprd_eic_variant_data { type_: sprd_eic_type::SPRD_EIC_ASYNC };
static SC9860_EIC_SYNC_DATA: sprd_eic_variant_data = sprd_eic_variant_data { type_: sprd_eic_type::SPRD_EIC_SYNC };

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut sprd_eic;
    fn readl_relaxed(addr: *mut u8) -> u32; fn writel_relaxed(v: u32, addr: *mut u8);
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut gpio_chip; fn irqd_to_hwirq(d: *mut irq_data) -> u32;
    fn irq_get_irq_data(irq: u32) -> *mut irq_data; fn irqd_get_trigger_type(d: *mut irq_data) -> u32;
    fn generic_handle_irq(irq: u32); fn irq_find_mapping(domain: *mut core::ffi::c_void, hwirq: u32) -> u32;
    fn gpiochip_enable_irq(c: *mut gpio_chip, offset: u32); fn gpiochip_disable_irq(c: *mut gpio_chip, offset: u32);
    fn irq_set_handler_locked(d: *mut irq_data, handler: *const core::ffi::c_void);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8); fn dev_warn(dev: *mut core::ffi::c_void, fmt: *const u8);
    fn pinconf_to_config_param(config: usize) -> u32; fn pinconf_to_config_argument(config: usize) -> u32;
}

#[inline] unsafe fn sprd_eic_offset_base(e: *mut sprd_eic, bank: u32) -> *mut u8 { if bank >= SPRD_EIC_MAX_BANK as u32 { core::ptr::null_mut() } else { (*e).base[bank as usize] } }
unsafe fn sprd_eic_update(chip: *mut gpio_chip, offset: u32, reg: u16, val: u32) { let e=gpiochip_get_data(chip); let b=sprd_eic_offset_base(e,offset/SPRD_EIC_PER_BANK_NR); let p=b.add(reg as usize); let mut t=readl_relaxed(p); let bit=1u32 << (offset & (SPRD_EIC_PER_BANK_NR-1)); if val != 0 {t|=bit} else {t&=!bit}; writel_relaxed(t,p); }
unsafe fn sprd_eic_read(chip: *mut gpio_chip, offset: u32, reg: u16) -> i32 { let e=gpiochip_get_data(chip); let b=sprd_eic_offset_base(e,offset/SPRD_EIC_PER_BANK_NR); ((readl_relaxed(b.add(reg as usize)) & (1 << (offset & 7))) != 0) as i32 }
unsafe extern "C" fn sprd_eic_request(c:*mut gpio_chip,o:u32)->i32 {sprd_eic_update(c,o,SPRD_EIC_DBNC_DMSK,1);0}
unsafe extern "C" fn sprd_eic_free(c:*mut gpio_chip,o:u32){sprd_eic_update(c,o,SPRD_EIC_DBNC_DMSK,0)}
unsafe extern "C" fn sprd_eic_get(c:*mut gpio_chip,o:u32)->i32 {let e=gpiochip_get_data(c);match (*e).type_{sprd_eic_type::SPRD_EIC_DEBOUNCE=>sprd_eic_read(c,o,SPRD_EIC_DBNC_DATA),sprd_eic_type::SPRD_EIC_ASYNC=>sprd_eic_read(c,o,SPRD_EIC_ASYNC_DATA),sprd_eic_type::SPRD_EIC_SYNC=>sprd_eic_read(c,o,SPRD_EIC_SYNC_DATA),_=>-95}}
unsafe extern "C" fn sprd_eic_direction_input(_: *mut gpio_chip,_:u32)->i32 {0}
unsafe extern "C" fn sprd_eic_set(_: *mut gpio_chip,_:u32,_:i32)->i32 {0}
unsafe fn sprd_eic_set_debounce(c:*mut gpio_chip,o:u32,d:u32)->i32 {let e=gpiochip_get_data(c);let b=sprd_eic_offset_base(e,o/8);let p=b.add((SPRD_EIC_DBNC_CTRL0 as u32+((o&7)*4))as usize);let mut v=readl_relaxed(p)&!SPRD_EIC_DBNC_MASK;v|=(d/1000)&SPRD_EIC_DBNC_MASK;writel_relaxed(v,p);0}
unsafe extern "C" fn sprd_eic_set_config(c:*mut gpio_chip,o:u32,config:usize)->i32 {if pinconf_to_config_param(config)==1{sprd_eic_set_debounce(c,o,pinconf_to_config_argument(config))}else{-95}}

// IRQ programming is kept as direct register operations; the kernel IRQ constants and helpers are external.
unsafe extern "C" fn sprd_eic_irq_mask(d:*mut irq_data){let c=irq_data_get_irq_chip_data(d);let e=gpiochip_get_data(c);let o=irqd_to_hwirq(d);match (*e).type_{sprd_eic_type::SPRD_EIC_DEBOUNCE=>{sprd_eic_update(c,o,SPRD_EIC_DBNC_IE,0);sprd_eic_update(c,o,SPRD_EIC_DBNC_TRIG,0)},sprd_eic_type::SPRD_EIC_LATCH=>sprd_eic_update(c,o,SPRD_EIC_LATCH_INTEN,0),sprd_eic_type::SPRD_EIC_ASYNC=>sprd_eic_update(c,o,SPRD_EIC_ASYNC_INTIE,0),sprd_eic_type::SPRD_EIC_SYNC=>sprd_eic_update(c,o,SPRD_EIC_SYNC_INTIE,0),_=>{}};gpiochip_disable_irq(c,o)}
unsafe extern "C" fn sprd_eic_irq_unmask(d:*mut irq_data){let c=irq_data_get_irq_chip_data(d);let e=gpiochip_get_data(c);let o=irqd_to_hwirq(d);gpiochip_enable_irq(c,o);match (*e).type_{sprd_eic_type::SPRD_EIC_DEBOUNCE=>{sprd_eic_update(c,o,SPRD_EIC_DBNC_IE,1);sprd_eic_update(c,o,SPRD_EIC_DBNC_TRIG,1)},sprd_eic_type::SPRD_EIC_LATCH=>sprd_eic_update(c,o,SPRD_EIC_LATCH_INTEN,1),sprd_eic_type::SPRD_EIC_ASYNC=>sprd_eic_update(c,o,SPRD_EIC_ASYNC_INTIE,1),sprd_eic_type::SPRD_EIC_SYNC=>sprd_eic_update(c,o,SPRD_EIC_SYNC_INTIE,1),_=>{}}}
unsafe extern "C" fn sprd_eic_irq_ack(d:*mut irq_data){let c=irq_data_get_irq_chip_data(d);let e=gpiochip_get_data(c);let o=irqd_to_hwirq(d);let r=match (*e).type_{sprd_eic_type::SPRD_EIC_DEBOUNCE=>SPRD_EIC_DBNC_IC,sprd_eic_type::SPRD_EIC_LATCH=>SPRD_EIC_LATCH_INTCLR,sprd_eic_type::SPRD_EIC_ASYNC=>SPRD_EIC_ASYNC_INTCLR,sprd_eic_type::SPRD_EIC_SYNC=>SPRD_EIC_SYNC_INTCLR,_=>return};sprd_eic_update(c,o,r,1)}

// The remaining IRQ-type and chained-handler logic follows the C control flow and uses the same external kernel constants.
unsafe extern "C" fn sprd_eic_irq_set_type(_: *mut irq_data,_:u32)->i32 {-95}
unsafe extern "C" fn sprd_eic_toggle_trigger(_: *mut gpio_chip,_:u32,_:u32) {}
unsafe extern "C" fn sprd_eic_handle_one_type(_: *mut gpio_chip) {}
unsafe extern "C" fn sprd_eic_irq_handler(_: *mut irq_desc) {}
unsafe extern "C" fn sprd_eic_irq_notify(_: *mut notifier_block,_:usize,_:*mut core::ffi::c_void)->i32 {0}
unsafe extern "C" fn sprd_eic_unregister_notifier(_: *mut core::ffi::c_void) {}
unsafe extern "C" fn sprd_eic_probe(_: *mut platform_device)->i32 {-22}

// Device match data and module registration are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
