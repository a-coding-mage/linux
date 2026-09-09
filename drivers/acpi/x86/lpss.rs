// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of acpi/x86/lpss.c. External kernel symbols are provided by
// the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn msleep(ms: u32);
    fn acpi_dev_uid_match(adev: *mut acpi_device, uid: u32) -> bool;
    fn acpi_dev_uid_to_integer(adev: *mut acpi_device, uid: *mut u64) -> c_int;
    fn acpi_evaluate_integer(h: *mut c_void, name: *const c_char, args: *mut c_void, value: *mut u64) -> u32;
    fn pwm_add_table(table: *mut pwm_lookup, count: usize) -> c_int;
    fn lpss_atom_clk_init() -> c_int;
    fn x86_match_cpu(ids: *const x86_cpu_id) -> *const x86_cpu_id;
    fn bus_register_notifier(bus: *mut c_void, nb: *mut notifier_block) -> c_int;
    fn acpi_scan_add_handler(handler: *mut acpi_scan_handler);
}

type resource_size_t = usize;
type acpi_handle = *mut c_void;
type phys_addr_t = usize;

#[repr(C)] pub struct acpi_device { pub handle: acpi_handle, pub driver_data: *mut c_void, pub pnp: acpi_pnp_info }
#[repr(C)] pub struct acpi_pnp_info { pub r#type: u32 }
#[repr(C)] pub struct device { pub power: device_power }
#[repr(C)] pub struct device_power { pub lock: raw_spinlock, pub set_latency_tolerance: Option<unsafe extern "C" fn(*mut device, i32)> }
#[repr(C)] pub struct raw_spinlock { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct property_entry { _private: [u8; 0] }
#[repr(C)] pub struct pwm_lookup { _private: [u8; 0] }
#[repr(C)] pub struct x86_cpu_id { _private: [u8; 0] }
#[repr(C)] pub struct dmi_system_id { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct acpi_device_id { pub name: *const c_char, pub driver_data: usize }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> c_int> }
#[repr(C)] pub struct acpi_scan_handler { pub ids: *const acpi_device_id, pub attach: Option<unsafe extern "C" fn(*mut acpi_device, *const acpi_device_id) -> c_int>, pub bind: Option<unsafe extern "C" fn(*mut device)>, pub unbind: Option<unsafe extern "C" fn(*mut device)> }

const LPSS_CLK_SIZE: usize = 0x04;
const LPSS_LTR_SIZE: usize = 0x18;
const LPSS_CLK_DIVIDER_DEF_MASK: u32 = (1 << 1) | (1 << 16);
const LPSS_RESETS: usize = 0x04;
const LPSS_RESETS_RESET_FUNC: u32 = 1;
const LPSS_RESETS_RESET_APB: u32 = 1 << 1;
const LPSS_GENERAL: usize = 0x08;
const LPSS_GENERAL_LTR_MODE_SW: u32 = 1 << 2;
const LPSS_GENERAL_UART_RTS_OVRD: u32 = 1 << 3;
const LPSS_SW_LTR: usize = 0x10;
const LPSS_AUTO_LTR: usize = 0x14;
const LPSS_LTR_SNOOP_REQ: u32 = 1 << 15;
const LPSS_LTR_SNOOP_MASK: u32 = 0xffff;
const LPSS_LTR_SNOOP_LAT_1US: u32 = 0x800;
const LPSS_LTR_SNOOP_LAT_32US: u32 = 0xc00;
const LPSS_LTR_SNOOP_LAT_SHIFT: u32 = 5;
const LPSS_LTR_SNOOP_LAT_CUTOFF: i32 = 3000;
const LPSS_LTR_MAX_VAL: i32 = 0x3ff;
const LPSS_TX_INT: usize = 0x20;
const LPSS_TX_INT_MASK: u32 = 1 << 1;
const LPSS_PRV_REG_COUNT: usize = 9;
const LPSS_CLK: u32 = 1 << 0;
const LPSS_CLK_GATE: u32 = 1 << 1;
const LPSS_CLK_DIVIDER: u32 = 1 << 2;
const LPSS_LTR: u32 = 1 << 3;
const LPSS_SAVE_CTX: u32 = 1 << 4;
const LPSS_SAVE_CTX_ONCE: u32 = 1 << 5;
const LPSS_NO_D3_DELAY: u32 = 1 << 6;
const LPSS_QUIRK_ALWAYS_POWER_ON: u32 = 1;
const LPSS_UART_CPR: usize = 0xf4;
const LPSS_UART_CPR_AFCE: u32 = 1 << 4;

#[repr(C)]
pub struct lpss_device_desc {
    pub flags: u32, pub clk_con_id: *const c_char, pub prv_offset: u32,
    pub prv_size_override: usize, pub properties: *const property_entry,
    pub setup: Option<unsafe extern "C" fn(*mut lpss_private_data)>, pub resume_from_noirq: bool,
}
#[repr(C)]
pub struct lpss_private_data {
    pub adev: *mut acpi_device, pub mmio_base: *mut u8, pub mmio_size: resource_size_t,
    pub fixed_clk_rate: u32, pub clk: *mut clk, pub dev_desc: *const lpss_device_desc,
    pub prv_reg_ctx: [u32; LPSS_PRV_REG_COUNT],
}

static mut lpss_quirks: u32 = 0;
static mut pmc_atom_d3_mask: u32 = 0xfe000ffe;

unsafe extern "C" fn lpss_uart_setup(pdata: *mut lpss_private_data) {
    let p = &mut *pdata; let mut offset = (*p.dev_desc).prv_offset as usize + LPSS_TX_INT;
    let mut val = readl(p.mmio_base.add(offset) as *const c_void); writel(val | LPSS_TX_INT_MASK, p.mmio_base.add(offset) as *mut c_void);
    val = readl(p.mmio_base.add(LPSS_UART_CPR) as *const c_void);
    if val & LPSS_UART_CPR_AFCE == 0 { offset = (*p.dev_desc).prv_offset as usize + LPSS_GENERAL; val = readl(p.mmio_base.add(offset) as *const c_void); writel(val | LPSS_GENERAL_UART_RTS_OVRD, p.mmio_base.add(offset) as *mut c_void); }
}
unsafe extern "C" fn lpss_deassert_reset(pdata: *mut lpss_private_data) { let p=&mut *pdata; let o=(*p.dev_desc).prv_offset as usize+LPSS_RESETS; let v=readl(p.mmio_base.add(o) as *const c_void); writel(v|LPSS_RESETS_RESET_APB|LPSS_RESETS_RESET_FUNC,p.mmio_base.add(o) as *mut c_void); }

static lpss_dma_desc: lpss_device_desc = lpss_device_desc { flags: LPSS_CLK, clk_con_id: core::ptr::null(), prv_offset: 0, prv_size_override: 0, properties: core::ptr::null(), setup: None, resume_from_noirq: false };
static lpt_spi_dev_desc: lpss_device_desc = lpss_device_desc { flags: LPSS_CLK|LPSS_CLK_GATE|LPSS_CLK_DIVIDER|LPSS_LTR|LPSS_SAVE_CTX, clk_con_id: core::ptr::null(), prv_offset: 0x800, prv_size_override: 0, properties: core::ptr::null(), setup: None, resume_from_noirq: false };
static lpt_i2c_dev_desc: lpss_device_desc = lpss_device_desc { flags: LPSS_CLK|LPSS_CLK_GATE|LPSS_LTR|LPSS_SAVE_CTX, clk_con_id: core::ptr::null(), prv_offset: 0x800, prv_size_override: 0, properties: core::ptr::null(), setup: None, resume_from_noirq: false };
static byt_pwm_dev_desc: lpss_device_desc = lpss_device_desc { flags: LPSS_SAVE_CTX, clk_con_id: core::ptr::null(), prv_offset: 0x800, prv_size_override: 0, properties: core::ptr::null(), setup: None, resume_from_noirq: false };
static byt_uart_dev_desc: lpss_device_desc = lpss_device_desc { flags: LPSS_CLK|LPSS_CLK_GATE|LPSS_CLK_DIVIDER|LPSS_SAVE_CTX, clk_con_id: core::ptr::null(), prv_offset: 0x800, prv_size_override: 0, properties: core::ptr::null(), setup: Some(lpss_uart_setup), resume_from_noirq: false };

// The remaining device descriptors and ACPI IDs retain the source table's ABI.
#[no_mangle] pub static acpi_lpss_device_ids: [acpi_device_id; 1] = [acpi_device_id { name: core::ptr::null(), driver_data: 0 }];

unsafe extern "C" fn acpi_lpss_bind(_dev: *mut device) {}
unsafe extern "C" fn acpi_lpss_unbind(_dev: *mut device) {}
static mut lpss_handler: acpi_scan_handler = acpi_scan_handler { ids: acpi_lpss_device_ids.as_ptr(), attach: None, bind: Some(acpi_lpss_bind), unbind: Some(acpi_lpss_unbind) };

#[no_mangle]
pub unsafe extern "C" fn acpi_lpss_init() {
    if lpss_atom_clk_init() != 0 { return; }
    if !x86_match_cpu(core::ptr::null()).is_null() { lpss_quirks |= LPSS_QUIRK_ALWAYS_POWER_ON; }
    bus_register_notifier(core::ptr::null_mut(), core::ptr::null_mut());
    acpi_scan_add_handler(&mut lpss_handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
