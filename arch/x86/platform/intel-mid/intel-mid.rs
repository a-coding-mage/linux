// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel MID platform setup code
 *
 * (C) Copyright 2008, 2012, 2021 Intel Corporation
 * Author: Jacob Pan (jacob.jun.pan@intel.com)
 * Author: Sathyanarayanan Kuppuswamy <sathyanarayanan.kuppuswamy@intel.com>
 */

// C headers provide the declarations used below.

pub const IPCMSG_COLD_OFF: u32 = 0x80; // Only for Tangier
pub const IPCMSG_COLD_RESET: u32 = 0xF1;

extern "C" {
    fn intel_mid_pwr_power_off();
    fn intel_scu_ipc_dev_simple_command(dev: *mut core::ffi::c_void, cmd: u32, sub: u32);
    fn setup_boot_APIC_clock();
    fn setup_secondary_APIC_clock();
    fn x86_init_noop();
    fn intel_mid_pci_init();
    fn set_bit(bit: u32, addr: *mut core::ffi::c_void);

    // External kernel objects and structures supplied by the surrounding tree.
    static mut x86_init: X86Init;
    static mut x86_cpuinit: X86CpuInit;
    static mut x86_platform: X86Platform;
    static mut boot_cpu_data: BootCpuData;
    static mut legacy_pic: *mut LegacyPic;
    static mut null_legacy_pic: LegacyPic;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut machine_ops: MachineOps;
    static mut mp_bus_not_pci: u32;
}

#[repr(C)]
pub struct X86Init {
    pub timers: Timers,
    pub resources: Resources,
    pub irqs: Irqs,
    pub oem: Oem,
    pub pci: Pci,
    pub acpi: Acpi,
    pub mpparse: MpParse,
}
#[repr(C)] pub struct Timers { pub setup_percpu_clockev: Option<unsafe extern "C" fn()>, pub timer_init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Resources { pub probe_roms: Option<unsafe extern "C" fn()>, pub reserve_resources: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Irqs { pub pre_vector_init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Oem { pub arch_setup: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Pci { pub arch_init: Option<unsafe extern "C" fn()>, pub fixup_irqs: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Acpi { pub reduced_hw_early_init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct MpParse { pub find_mptable: Option<unsafe extern "C" fn()>, pub early_parse_smp_cfg: Option<unsafe extern "C" fn()>, pub parse_smp_cfg: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct X86CpuInit { pub setup_percpu_clockev: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct X86Platform { pub legacy: Legacy, pub get_nmi_reason: Option<unsafe extern "C" fn() -> u8> }
#[repr(C)] pub struct Legacy { pub rtc: u32 }
#[repr(C)] pub struct BootCpuData { pub x86_vfm: u32 }
#[repr(C)] pub struct LegacyPic;
#[repr(C)] pub struct MachineOps { pub emergency_restart: Option<unsafe extern "C" fn()> }

pub const INTEL_ATOM_SILVERMONT_MID: u32 = 0; // Supplied by asm headers.
pub const MP_BUS_ISA: u32 = 0; // Supplied by asm headers.

extern "C" { fn regulator_has_full_constraints(); }

unsafe extern "C" fn intel_mid_power_off() {
    // Shut down South Complex via PWRMU
    intel_mid_pwr_power_off();

    // Only for Tangier, the rest will ignore this command
    intel_scu_ipc_dev_simple_command(core::ptr::null_mut(), IPCMSG_COLD_OFF, 1);
}

unsafe extern "C" fn intel_mid_reboot() {
    intel_scu_ipc_dev_simple_command(core::ptr::null_mut(), IPCMSG_COLD_RESET, 0);
}

unsafe extern "C" fn intel_mid_time_init() {
    // Lapic only, no apbt
    x86_init.timers.setup_percpu_clockev = Some(setup_boot_APIC_clock);
    x86_cpuinit.setup_percpu_clockev = Some(setup_secondary_APIC_clock);
}

unsafe extern "C" fn intel_mid_arch_setup() {
    match boot_cpu_data.x86_vfm {
        INTEL_ATOM_SILVERMONT_MID => x86_platform.legacy.rtc = 1,
        _ => {}
    }

    /*
     * Intel MID platforms are using explicitly defined regulators.
     *
     * Let the regulator core know that we do not have any additional
     * regulators left. This lets it substitute unprovided regulators with
     * dummy ones:
     */
    regulator_has_full_constraints();
}

/*
 * Moorestown does not have external NMI source nor port 0x61 to report
 * NMI status. The possible NMI sources are from pmu as a result of NMI
 * watchdog or lock debug. Reading io port 0x61 results in 0xff which
 * misled NMI handler.
 */
unsafe extern "C" fn intel_mid_get_nmi_reason() -> u8 { 0 }

/*
 * Moorestown specific x86_init function overrides and early setup
 * calls.
 */
pub unsafe extern "C" fn x86_intel_mid_early_setup() {
    x86_init.resources.probe_roms = Some(x86_init_noop);
    x86_init.resources.reserve_resources = Some(x86_init_noop);
    x86_init.timers.timer_init = Some(intel_mid_time_init);
    x86_init.timers.setup_percpu_clockev = Some(x86_init_noop);
    x86_init.irqs.pre_vector_init = Some(x86_init_noop);
    x86_init.oem.arch_setup = Some(intel_mid_arch_setup);
    x86_platform.get_nmi_reason = Some(intel_mid_get_nmi_reason);
    x86_init.pci.arch_init = Some(intel_mid_pci_init);
    x86_init.pci.fixup_irqs = Some(x86_init_noop);
    legacy_pic = &mut null_legacy_pic;
    x86_init.acpi.reduced_hw_early_init = Some(x86_init_noop);
    pm_power_off = Some(intel_mid_power_off);
    machine_ops.emergency_restart = Some(intel_mid_reboot);
    x86_init.mpparse.find_mptable = Some(x86_init_noop);
    x86_init.mpparse.early_parse_smp_cfg = Some(x86_init_noop);
    x86_init.mpparse.parse_smp_cfg = Some(x86_init_noop);
    set_bit(MP_BUS_ISA, &mut mp_bus_not_pci as *mut _ as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
