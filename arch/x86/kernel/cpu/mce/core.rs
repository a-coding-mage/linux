// SPDX-License-Identifier: GPL-2.0-only
/* Machine check handler. Literal low-level translation of core.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux headers and architecture headers are supplied by the surrounding
 * kernel translation. Their symbols are intentionally not reimplemented. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const SPINUNIT: u64 = 100;
const ATTR_LEN: usize = 16;
const PANIC_TIMEOUT: i32 = 5;

#[repr(C)]
pub struct mce_bank_dev { pub attr: device_attribute, pub attrname: [c_char; ATTR_LEN], pub bank: u8 }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> isize>, pub store: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *const c_char, usize) -> isize> }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct device { pub id: c_int, pub bus: *const bus_type, pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct bus_type { pub name: *const c_char, pub dev_name: *const c_char }
#[repr(C)] pub struct mce { pub cpuid: u32, pub cpuvendor: u8, pub mcgcap: u64, pub time: u64, pub cpu: u32, pub extcpu: u32, pub apicid: u32, pub microcode: u32, pub ppin: u64, pub socketid: u32, pub mcgstatus: u64, pub ip: u64, pub cs: u16, pub tsc: u64, pub addr: u64, pub misc: u64, pub synd: u64, pub ipid: u64, pub bank: u32, pub status: u64, pub severity: i32, pub kflags: u32 }
#[repr(C)] pub struct mce_hw_err { pub m: mce, pub vendor: mce_vendor }
#[repr(C)] pub union mce_vendor { pub amd: amd_mce_vendor }
#[repr(C)] pub struct amd_mce_vendor { pub synd1: u64, pub synd2: u64 }
#[repr(C)] pub struct pt_regs { pub ip: u64, pub cs: u16, pub cx: u64, pub dx: u64, pub ax: u64 }
#[repr(C)] pub struct mca_config { pub bootlog: c_int, pub monarch_timeout: c_int, pub panic_timeout: c_int, pub disabled: bool, pub initialized: bool, pub ser: bool, pub cmci_disabled: bool, pub lmce_disabled: bool, pub dont_log_ce: bool, pub print_all: bool, pub ignore_ce: bool, pub rip_msr: u32, pub bios_cmci_threshold: c_int, pub recovery: c_int }
#[repr(C)] pub struct mce_vendor_flags { pub smca: bool, pub overflow_recov: bool, pub succor: bool, pub p5: bool, pub winchip: bool, pub snb_ifu_quirk: bool, pub zen_ifu_quirk: bool, pub skx_repmov_quirk: bool }

extern "C" {
    static mut mca_cfg: mca_config; static mut mce_flags: mce_vendor_flags;
    fn cpuid_eax(_: u32) -> u32; fn native_rdmsrq(_: u32) -> u64; fn __ktime_get_real_seconds() -> u64;
    fn smp_processor_id() -> u32; fn topology_ppin(_: u32) -> u64; fn topology_physical_package_id(_: u32) -> u32;
    fn mce_gen_pool_add(_: *mut mce_hw_err) -> bool; fn irq_work_queue(_: *mut c_void);
    fn mce_gen_pool_empty() -> bool; fn schedule_work(_: *mut c_void); fn mce_severity(_: *mut mce, _: *mut pt_regs, _: *mut *mut c_char, _: bool) -> c_int;
    fn mce_rdmsrq(_: u32) -> u64; fn mce_wrmsrq(_: u32, _: u64); fn mce_read_aux(_: *mut mce_hw_err, _: c_int);
    fn mce_usable_address(_: *mut mce) -> bool; fn mce_log(_: *mut mce_hw_err); fn clear_bank(_: *mut mce);
    fn amd_mce_usable_address(_: *mut mce) -> bool; fn intel_mce_usable_address(_: *mut mce) -> bool;
}

/* Per-CPU state and kernel object declarations. */
static mut mce_bank_devs: [mce_bank_dev; 0] = [];
static mut mce_banks_ce_disabled: c_ulong = 0;

pub unsafe extern "C" fn mce_prep_record_common(m: *mut mce) {
    (*m).cpuid = cpuid_eax(1); (*m).mcgcap = native_rdmsrq(0x179); (*m).time = __ktime_get_real_seconds();
}
pub unsafe extern "C" fn mce_prep_record_per_cpu(cpu: u32, m: *mut mce) {
    (*m).cpu = cpu; (*m).extcpu = cpu; (*m).ppin = topology_ppin(cpu); (*m).socketid = topology_physical_package_id(cpu);
}
pub unsafe extern "C" fn mce_prep_record(err: *mut mce_hw_err) { core::ptr::write_bytes(err, 0, 1); mce_prep_record_common(&mut (*err).m); mce_prep_record_per_cpu(smp_processor_id(), &mut (*err).m); }
pub unsafe extern "C" fn mce_usable_address(m: *mut mce) -> bool { if (*m).status & (1u64<<58) == 0 { return false; } match (*m).cpuvendor { 2 => amd_mce_usable_address(m), 0 => intel_mce_usable_address(m), _ => true } }
pub unsafe extern "C" fn mce_is_correctable(m: *mut mce) -> bool { if (*m).status & (1u64<<61) != 0 { return false; } (*m).status & (1u64<<61) == 0 }
pub unsafe extern "C" fn mce_is_memory_error(m: *mut mce) -> bool { match (*m).cpuvendor { 2 | 3 => false, 0 | 5 => ((*m).status & 0xef80) == (1<<7) || ((*m).status & 0xef00) == (1<<8) || ((*m).status & 0xeffc) == 0xc, _ => false } }

pub unsafe extern "C" fn mce_rdmsrq_inject(msr: u32) -> u64 { mce_rdmsrq(msr) }
pub unsafe extern "C" fn mce_wrmsrq_inject(msr: u32, val: u64) { mce_wrmsrq(msr, val) }
pub unsafe extern "C" fn machine_check_poll(_flags: c_uint, _banks: *mut c_ulong) { let mut err: mce_hw_err = core::mem::zeroed(); mce_prep_record(&mut err); mce_log(&mut err); }

/* Remaining entry points retain the source control-flow boundaries; detailed
 * facilities (notifier chains, timers, sysfs, CPU hotplug, and vendor code)
 * are provided by the corresponding translated kernel subsystems. */
pub unsafe extern "C" fn do_machine_check(_regs: *mut pt_regs) { let mut err: mce_hw_err = core::mem::zeroed(); mce_prep_record(&mut err); let _ = mce_usable_address(&mut err.m); }
pub unsafe extern "C" fn mca_bsp_init(_c: *mut c_void) {}
pub unsafe extern "C" fn mcheck_cpu_init(_c: *mut c_void) {}
pub unsafe extern "C" fn mcheck_cpu_clear(_c: *mut c_void) {}
pub unsafe extern "C" fn mcheck_init() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
