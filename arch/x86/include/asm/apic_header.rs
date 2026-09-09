/* SPDX-License-Identifier: GPL-2.0-only */

pub const ARCH_APICTIMER_STOPS_ON_C3: u32 = 1;
pub const APIC_EXTNMI_BSP: u32 = 0;
pub const APIC_EXTNMI_ALL: u32 = 1;
pub const APIC_EXTNMI_NONE: u32 = 2;
pub const APIC_QUIET: u32 = 0;
pub const APIC_VERBOSE: u32 = 1;
pub const APIC_DEBUG: u32 = 2;

// Dependencies supplied by the surrounding kernel translation are intentionally external.
extern "C" {
    pub static mut cpuid_to_apicid: *mut u32;
    pub static mut apic_verbosity: i32;
    pub static mut local_apic_timer_c2_ok: i32;
    pub static mut apic_is_disabled: bool;
    pub static mut lapic_timer_period: u32;
    pub static mut smp_found_config: bool;
    pub fn printk(fmt: *const u8, ...);
    pub fn x86_32_probe_apic();
    pub fn rdmsrq_safe(msr: u32, val: *mut u64) -> i32;
    pub fn wrmsrq(msr: u32, val: u64);
    pub fn native_wrmsrq(msr: u32, val: u64);
    pub fn readl(addr: *mut core::ffi::c_void) -> u32;
    pub fn pi_pending_this_cpu(vector: u32) -> bool;
    pub fn clear_bit(bit: u32, addr: *mut core::ffi::c_void);
    pub fn set_bit(bit: u32, addr: *mut core::ffi::c_void);
    pub fn test_bit(bit: u32, addr: *mut core::ffi::c_void) -> i32;
    pub fn __fls(x: u32) -> i32;
}

pub const CPU_ACPIID_INVALID: u32 = u32::MAX;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ApicIntrModeId { ApicPic, ApicVirtualWire, ApicVirtualWireNoConfig, ApicSymmetricIo, ApicSymmetricIoNoRouting }

pub unsafe fn apic_from_smp_config() -> bool { smp_found_config && !apic_is_disabled }

pub unsafe fn native_apic_mem_write(reg: u32, v: u32) {
    let addr = (APIC_BASE + reg) as *mut u32;
    core::ptr::write_volatile(addr, v);
}
pub unsafe fn native_apic_mem_read(reg: u32) -> u32 { readl((APIC_BASE + reg) as *mut core::ffi::c_void) }
pub unsafe fn native_apic_mem_eoi() { native_apic_mem_write(APIC_EOI, APIC_EOI_ACK); }
pub unsafe fn apic_is_x2apic_enabled() -> bool {
    let mut msr = 0u64;
    if rdmsrq_safe(MSR_IA32_APICBASE, &mut msr) != 0 { false } else { msr & X2APIC_ENABLE != 0 }
}

pub const APIC_BASE: u32 = 0xfee00000;
pub const APIC_BASE_MSR: u32 = 0x800;
pub const APIC_EOI: u32 = 0xB0;
pub const APIC_EOI_ACK: u32 = 0;
pub const APIC_ICR: u32 = 0x300;
pub const APIC_ID: u32 = 0x20;
pub const APIC_DFR: u32 = 0xE0;
pub const APIC_LDR: u32 = 0xD0;
pub const APIC_LVR: u32 = 0x30;
pub const APIC_IRR: u32 = 0x200;
pub const X2APIC_ENABLE: u64 = 1 << 10;
pub const MSR_IA32_APICBASE: u32 = 0x1b;

extern "C" {
    pub fn native_apic_icr_write(low: u32, id: u32);
    pub fn native_apic_icr_read() -> u64;
    pub fn enable_IR_x2apic();
    pub fn lapic_get_maxlvt() -> i32;
    pub fn clear_local_APIC();
    pub fn disconnect_bsp_APIC(virt_wire_setup: i32);
    pub fn disable_local_APIC();
    pub fn apic_soft_disable();
    pub fn lapic_shutdown();
    pub fn sync_Arb_IDs();
    pub fn init_bsp_APIC();
    pub fn apic_intr_mode_select();
    pub fn apic_intr_mode_init();
    pub fn init_apic_mappings();
    pub fn register_lapic_address(address: usize);
    pub fn setup_boot_APIC_clock();
    pub fn setup_secondary_APIC_clock();
    pub fn lapic_update_tsc_freq();
    pub fn apic_ap_setup();
    pub fn setup_APIC_eilvt(lvt_off: u8, vector: u8, msg_type: u8, mask: u8) -> i32;
    pub fn lapic_assign_system_vectors();
    pub fn lapic_assign_legacy_vector(isairq: u32, replace: bool);
    pub fn lapic_update_legacy_vectors();
    pub fn lapic_online();
    pub fn lapic_offline();
    pub fn apic_send_IPI_allbutself(vector: u32);
    pub fn apic_needs_pit() -> bool;
    pub fn topology_register_apic(apic_id: u32, acpi_id: u32, present: bool);
    pub fn topology_register_boot_apic(apic_id: u32);
    pub fn topology_hotplug_apic(apic_id: u32, acpi_id: u32) -> i32;
    pub fn topology_hotunplug_apic(cpu: u32);
    pub fn topology_apply_cmdline_limits_early();
    pub fn topology_init_possible_cpus();
    pub fn topology_reset_possible_cpus_up();
}

#[repr(C)]
pub struct Cpumask { _private: [u8; 0] }
#[repr(C)]
pub struct Apic {
    pub eoi: Option<unsafe extern "C" fn()>, pub native_eoi: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn(u32,u32)>, pub read: Option<unsafe extern "C" fn(u32)->u32>,
    pub wait_icr_idle: Option<unsafe extern "C" fn()>, pub safe_wait_icr_idle: Option<unsafe extern "C" fn()->u32>,
    pub send_IPI: Option<unsafe extern "C" fn(i32,i32)>, pub send_IPI_mask: Option<unsafe extern "C" fn(*const Cpumask,i32)>,
    pub send_IPI_mask_allbutself: Option<unsafe extern "C" fn(*const Cpumask,i32)>, pub send_IPI_allbutself: Option<unsafe extern "C" fn(i32)>,
    pub send_IPI_all: Option<unsafe extern "C" fn(i32)>, pub send_IPI_self: Option<unsafe extern "C" fn(i32)>,
    pub disable_esr: u32, pub dest_mode_logical: u32, pub x2apic_set_max_apicid: u32, pub nmi_to_offline_cpu: u32,
    pub calc_dest_apicid: Option<unsafe extern "C" fn(u32)->u32>, pub icr_read: Option<unsafe extern "C" fn()->u64>, pub icr_write: Option<unsafe extern "C" fn(u32,u32)>,
    pub max_apic_id: u32, pub probe: Option<unsafe extern "C" fn()->i32>, pub setup: Option<unsafe extern "C" fn()>, pub teardown: Option<unsafe extern "C" fn()>,
    pub acpi_madt_oem_check: Option<unsafe extern "C" fn(*mut u8,*mut u8)->i32>, pub init_apic_ldr: Option<unsafe extern "C" fn()>, pub cpu_present_to_apicid: Option<unsafe extern "C" fn(i32)->u32>,
    pub get_apic_id: Option<unsafe extern "C" fn(u32)->u32>, pub wakeup_secondary_cpu: Option<unsafe extern "C" fn(u32,usize,u32)->i32>, pub wakeup_secondary_cpu_64: Option<unsafe extern "C" fn(u32,usize,u32)->i32>,
    pub update_vector: Option<unsafe extern "C" fn(u32,u32,bool)>, pub name: *mut u8,
}
#[repr(C)]
pub struct ApicOverride {
    pub eoi: Option<unsafe extern "C" fn()>, pub native_eoi: Option<unsafe extern "C" fn()>, pub write: Option<unsafe extern "C" fn(u32,u32)>, pub read: Option<unsafe extern "C" fn(u32)->u32>,
    pub send_IPI: Option<unsafe extern "C" fn(i32,i32)>, pub send_IPI_mask: Option<unsafe extern "C" fn(*const Cpumask,i32)>, pub send_IPI_mask_allbutself: Option<unsafe extern "C" fn(*const Cpumask,i32)>,
    pub send_IPI_allbutself: Option<unsafe extern "C" fn(i32)>, pub send_IPI_all: Option<unsafe extern "C" fn(i32)>, pub send_IPI_self: Option<unsafe extern "C" fn(i32)>, pub icr_read: Option<unsafe extern "C" fn()->u64>, pub icr_write: Option<unsafe extern "C" fn(u32,u32)>,
    pub wakeup_secondary_cpu: Option<unsafe extern "C" fn(u32,usize,u32)->i32>, pub wakeup_secondary_cpu_64: Option<unsafe extern "C" fn(u32,usize,u32)->i32>,
}
extern "C" { pub static mut apic: *mut Apic; pub static mut __x86_apic_override: ApicOverride; }

pub const MAX_APIC_VECTOR: u32 = 256;
pub const APIC_VECTORS_PER_REG: u32 = 32;
pub const TRAMPOLINE_PHYS_LOW: u32 = 0x467;
pub const TRAMPOLINE_PHYS_HIGH: u32 = 0x469;
pub const fn apic_vector_to_bit_number(v: u32) -> u32 { v % 32 }
pub const fn apic_vector_to_reg_offset(v: u32) -> u32 { v / 32 * 0x10 }
pub unsafe fn apic_read(_reg: u32) -> u32 { 0 }
pub unsafe fn apic_write(_reg: u32, _val: u32) {}
pub unsafe fn apic_eoi() {}
pub unsafe fn apic_native_eoi() {}
pub unsafe fn apic_icr_read() -> u64 { 0 }
pub unsafe fn apic_icr_write(_low: u32, _high: u32) {}
pub unsafe fn apic_wait_icr_idle() {}
pub unsafe fn safe_apic_wait_icr_idle() -> u32 { 0 }
pub unsafe fn apic_update_vector(_cpu: u32, _vector: u32, _set: bool) {}
pub unsafe fn lapic_vector_set_in_irr(vector: u32) -> bool { let irr = apic_read(APIC_IRR + apic_vector_to_reg_offset(vector)); irr & (1u32 << apic_vector_to_bit_number(vector)) != 0 }
pub unsafe fn is_vector_pending(vector: u32) -> bool { lapic_vector_set_in_irr(vector) || pi_pending_this_cpu(vector) }
pub unsafe fn apic_find_highest_vector(bitmap: *mut u8) -> i32 { let mut vec = 224i32; while vec >= 0 { let reg = bitmap.add(apic_vector_to_reg_offset(vec as u32) as usize) as *mut u32; if *reg != 0 { return __fls(*reg) + vec; } vec -= 32; } -1 }
pub unsafe fn apic_get_reg(regs: *mut u8, reg: i32) -> u32 { *((regs.offset(reg as isize)) as *mut u32) }
pub unsafe fn apic_set_reg(regs: *mut u8, reg: i32, val: u32) { *((regs.offset(reg as isize)) as *mut u32) = val; }
pub unsafe fn apic_get_reg64(regs: *mut u8, reg: i32) -> u64 { debug_assert!(reg as u32 == APIC_ICR); *((regs.offset(reg as isize)) as *mut u64) }
pub unsafe fn apic_set_reg64(regs: *mut u8, reg: i32, val: u64) { debug_assert!(reg as u32 == APIC_ICR); *((regs.offset(reg as isize)) as *mut u64) = val; }
pub unsafe fn apic_clear_vector(vec: i32, bitmap: *mut u8) { clear_bit(apic_vector_to_bit_number(vec as u32), bitmap.add(apic_vector_to_reg_offset(vec as u32) as usize) as *mut _); }
pub unsafe fn apic_set_vector(vec: i32, bitmap: *mut u8) { set_bit(apic_vector_to_bit_number(vec as u32), bitmap.add(apic_vector_to_reg_offset(vec as u32) as usize) as *mut _); }
pub unsafe fn apic_test_vector(vec: i32, bitmap: *mut u8) -> i32 { test_bit(apic_vector_to_bit_number(vec as u32), bitmap.add(apic_vector_to_reg_offset(vec as u32) as usize) as *mut _) }

extern "C" {
    pub fn apic_ack_irq(data: *mut core::ffi::c_void);
    pub fn read_apic_id() -> u32;
    pub fn apic_smt_update();
    pub fn __irq_msi_compose_msg(cfg: *mut core::ffi::c_void, msg: *mut core::ffi::c_void, dmar: bool);
    pub fn ioapic_zap_locks();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
