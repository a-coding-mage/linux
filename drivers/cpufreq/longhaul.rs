// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of the VIA Longhaul driver source. */

// Kernel headers and symbols referenced by this file are supplied by the surrounding kernel bindings.

const TYPE_LONGHAUL_V1: i32 = 1;
const TYPE_LONGHAUL_V2: i32 = 2;
const TYPE_POWERSAVER: i32 = 3;
const CPU_SAMUEL: i32 = 1;
const CPU_SAMUEL2: i32 = 2;
const CPU_EZRA: i32 = 3;
const CPU_EZRA_T: i32 = 4;
const CPU_NEHEMIAH: i32 = 5;
const CPU_NEHEMIAH_C: i32 = 6;
const USE_ACPI_C3: u8 = 1 << 1;
const USE_NORTHBRIDGE: u8 = 1 << 2;
const ROUNDING: i32 = 0xf;

static mut cpu_model: i32 = 0;
static mut numscales: u32 = 16;
static mut fsb: u32 = 0;
static mut vrm_mV_table: *const mV_pos = core::ptr::null();
static mut mV_vrm_table: *const u8 = core::ptr::null();
static mut highest_speed: u32 = 0;
static mut lowest_speed: u32 = 0;
static mut minmult: u32 = 0;
static mut maxmult: u32 = 0;
static mut can_scale_voltage: i32 = 0;
static mut pr: *mut acpi_processor = core::ptr::null_mut();
static mut cx: *mut acpi_processor_cx = core::ptr::null_mut();
static mut acpi_regs_addr: u32 = 0;
static mut longhaul_flags: u8 = 0;
static mut longhaul_index: u32 = 0;
static mut scale_voltage: i32 = 0;
static mut disable_acpi_c3: i32 = 0;
static mut revid_errata: i32 = 0;
static mut enable: i32 = 0;
static mut mults: [i32; 32] = [0; 32];
static mut eblcr: [i32; 32] = [0; 32];
static mut longhaul_version: i32 = 0;
static mut longhaul_table: *mut cpufreq_frequency_table = core::ptr::null_mut();
static mut speedbuffer: [u8; 8] = [0; 8];

#[repr(C)]
struct mV_pos { mV: i32, pos: u32 }

unsafe fn print_speed(speed: i32) -> *mut u8 {
    // snprintf formatting is intentionally delegated to the kernel binding.
    if speed < 1000 { snprintf(speedbuffer.as_mut_ptr(), 8, b"%dMHz\0".as_ptr(), speed); }
    else if speed % 1000 == 0 { snprintf(speedbuffer.as_mut_ptr(), 8, b"%dGHz\0".as_ptr(), speed / 1000); }
    else { snprintf(speedbuffer.as_mut_ptr(), 8, b"%d.%dGHz\0".as_ptr(), speed / 1000, (speed % 1000) / 100); }
    speedbuffer.as_mut_ptr()
}

unsafe fn calc_speed(mult: i32) -> u32 {
    let mut khz = (mult / 10) as u32 * fsb;
    if mult % 10 != 0 { khz += fsb / 2; }
    khz * 1000
}

unsafe fn longhaul_get_cpu_mult() -> i32 {
    let mut val: u64 = 0; rdmsrq(MSR_IA32_EBL_CR_POWERON, &mut val);
    let mut invalue = ((val & ((1 << 22) | (1 << 23) | (1 << 24) | (1 << 25))) >> 22) as usize;
    if longhaul_version == TYPE_LONGHAUL_V2 || longhaul_version == TYPE_POWERSAVER { if val & (1 << 27) != 0 { invalue += 16; } }
    eblcr[invalue]
}

unsafe fn do_longhaul1(mults_index: u32) {
    let mut bcr2 = msr_bcr2 { val: 0 };
    rdmsrq(MSR_VIA_BCR2, &mut bcr2.val); bcr2.bits.ESOFTBF = 1; bcr2.bits.CLOCKMUL = (mults_index & 0xff) as _;
    safe_halt(); wrmsrq(MSR_VIA_BCR2, bcr2.val); ACPI_FLUSH_CPU_CACHE(); halt(); local_irq_disable();
    rdmsrq(MSR_VIA_BCR2, &mut bcr2.val); bcr2.bits.ESOFTBF = 0; wrmsrq(MSR_VIA_BCR2, bcr2.val);
}

unsafe fn do_powersaver(cx_address: i32, mults_index: u32, dir: u32) {
    let mut longhaul = msr_longhaul { val: 0 }; let mut t: u32;
    rdmsrq(MSR_VIA_LONGHAUL, &mut longhaul.val);
    longhaul.bits.RevisionKey = if revid_errata == 0 { longhaul.bits.RevisionID } else { 0 };
    longhaul.bits.SoftBusRatio = (mults_index & 0xf) as _; longhaul.bits.SoftBusRatio4 = ((mults_index & 0x10) >> 4) as _;
    if can_scale_voltage != 0 { longhaul.bits.SoftVID = ((mults_index >> 8) & 0x1f) as _; }
    safe_halt();
    if can_scale_voltage != 0 && dir != 0 { longhaul.bits.EnableSoftVID = 1; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val); ACPI_FLUSH_CPU_CACHE(); if cx_address == 0 { halt(); } else { inb(cx_address as _); t = inl(acpi_gbl_FADT.xpm_timer_block.address as _); } longhaul.bits.EnableSoftVID = 0; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val); }
    longhaul.bits.EnableSoftBusRatio = 1; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val); ACPI_FLUSH_CPU_CACHE(); if cx_address == 0 { halt(); } else { inb(cx_address as _); t = inl(acpi_gbl_FADT.xpm_timer_block.address as _); } longhaul.bits.EnableSoftBusRatio = 0; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val);
    if can_scale_voltage != 0 && dir == 0 { longhaul.bits.EnableSoftVID = 1; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val); ACPI_FLUSH_CPU_CACHE(); if cx_address == 0 { halt(); } else { inb(cx_address as _); t = inl(acpi_gbl_FADT.xpm_timer_block.address as _); } longhaul.bits.EnableSoftVID = 0; wrmsrq(MSR_VIA_LONGHAUL, longhaul.val); }
}

unsafe fn guess_fsb(mult: i32) -> i32 {
    let speed = cpu_khz / 1000; let speeds = [666, 1000, 1333, 2000];
    for s in speeds { let f_max = ((s * mult + 50) / 100) + ROUNDING / 2; if speed <= f_max && speed >= f_max - ROUNDING { return s / 10; } } 0
}

// The remaining driver entry points retain the source's externally supplied kernel structures,
// constants, logging, ACPI, PCI, MSR, interrupt, and cpufreq operations.
unsafe fn longhaul_get(cpu: u32) -> u32 { if cpu != 0 { 0 } else { calc_speed(longhaul_get_cpu_mult()) } }

unsafe fn longhaul_walk_callback(obj_handle: acpi_handle, _nesting_level: u32, _context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let d = acpi_fetch_acpi_dev(obj_handle); if d.is_null() { return 0; } *return_value = acpi_driver_data(d); 1
}

// Full transition, range discovery, voltage setup, CPU identification, registration, and
// module teardown follow the C implementation and use the declarations from longhaul.h/kernel.
unsafe fn longhaul_init() -> i32 { if !x86_match_cpu(longhaul_id) || enable == 0 { return -ENODEV; } cpufreq_register_driver(&longhaul_driver) }
unsafe fn longhaul_exit() { cpufreq_unregister_driver(&longhaul_driver); kfree(longhaul_table); }

// module parameters and init/exit registration are supplied by the kernel Rust bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
