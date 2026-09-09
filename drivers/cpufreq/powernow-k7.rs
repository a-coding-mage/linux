// SPDX-License-Identifier: GPL-2.0-only
/*
 *  AMD K7 Powernow driver.
 *  (C) 2003 Dave Jones on behalf of SuSE Labs.
 *
 *  Based upon datasheets & sample CPUs kindly provided by AMD.
 *
 * Errata 5:
 *  CPU may fail to execute a FID/VID change in presence of interrupt.
 *  - We cli/sti on stepping A0 CPUs around the FID/VID transition.
 * Errata 15:
 *  CPU with half frequency multipliers may hang upon wakeup from disconnect.
 *  - We disable half multipliers if ACPI is used on A0 stepping CPUs.
 */

// C headers and build-time configuration supplied by the surrounding kernel.

#[repr(C)]
struct PsB { signature: [u8; 10], tableversion: u8, flags: u8, settlingtime: u16, reserved1: u8, numpst: u8 }

#[repr(C)]
struct Pst { cpuid: u32, fsbspeed: u8, maxfid: u8, startvid: u8, numpstates: u8 }

/* divide by 1000 to get VCore voltage in V. */
static MOBILE_VID_TABLE: [i32; 32] = [
    2000, 1950, 1900, 1850, 1800, 1750, 1700, 1650,
    1600, 1550, 1500, 1450, 1400, 1350, 1300, 0,
    1275, 1250, 1225, 1200, 1175, 1150, 1125, 1100,
    1075, 1050, 1025, 1000, 975, 950, 925, 0,
];

/* divide by 10 to get FID. */
static FID_CODES: [i32; 32] = [
    110, 115, 120, 125, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105,
    30, 190, 40, 200, 130, 135, 140, 210, 150, 225, 160, 165, 170, 180, -1, -1,
];

/* This parameter is used in order to force ACPI instead of legacy method for configuration purpose. */
static mut ACPI_FORCE: i32 = 0;
static mut POWERNow_TABLE: *mut cpufreq_frequency_table = core::ptr::null_mut();
static mut CAN_SCALE_BUS: u32 = 0;
static mut CAN_SCALE_VID: u32 = 0;
static mut MINIMUM_SPEED: u32 = u32::MAX;
static mut MAXIMUM_SPEED: u32 = 0;
static mut NUMBER_SCALES: u32 = 0;
static mut FSB: u32 = 0;
static mut LATENCY: u32 = 0;
static mut HAVE_A0: i8 = 0;

unsafe fn check_fsb(fsbspeed: u32) -> i32 {
    let f = FSB / 1000;
    let delta = if fsbspeed > f { fsbspeed - f } else { f - fsbspeed };
    (delta < 5) as i32
}

unsafe fn get_ranges(mut pst: *mut u8) -> i32 {
    POWERNow_TABLE = kzalloc(core::mem::size_of::<cpufreq_frequency_table>() * (NUMBER_SCALES as usize + 1), GFP_KERNEL) as *mut _;
    if POWERNow_TABLE.is_null() { return -ENOMEM; }
    for j in 0..NUMBER_SCALES as usize {
        let fid = *pst; pst = pst.add(1);
        (*POWERNow_TABLE.add(j)).frequency = FSB * FID_CODES[fid as usize] as u32 / 10;
        (*POWERNow_TABLE.add(j)).driver_data = fid as u32;
        let speed = (*POWERNow_TABLE.add(j)).frequency;
        if FID_CODES[fid as usize] % 10 == 5 && HAVE_A0 == 1 { /* CONFIG_X86_POWERNOW_K7_ACPI */ }
        if speed < MINIMUM_SPEED { MINIMUM_SPEED = speed; }
        if speed > MAXIMUM_SPEED { MAXIMUM_SPEED = speed; }
        let vid = *pst; pst = pst.add(1);
        (*POWERNow_TABLE.add(j)).driver_data |= (vid as u32) << 8;
        pr_debug!("   FID: 0x%x (%d.%dx [%dMHz])  VID: 0x%x (%d.%03dV)\n", fid, FID_CODES[fid as usize] / 10, FID_CODES[fid as usize] % 10, speed / 1000, vid, MOBILE_VID_TABLE[vid as usize] / 1000, MOBILE_VID_TABLE[vid as usize] % 1000);
    }
    (*POWERNow_TABLE.add(NUMBER_SCALES as usize)).frequency = CPUFREQ_TABLE_END;
    (*POWERNow_TABLE.add(NUMBER_SCALES as usize)).driver_data = 0;
    0
}

unsafe fn change_fid(fid: i32) {
    let mut ctl: msr_fidvidctl = core::mem::zeroed();
    rdmsrq(MSR_K7_FID_VID_CTL, &mut ctl.val);
    if ctl.bits.FID != fid { ctl.bits.SGTC = LATENCY; ctl.bits.FID = fid; ctl.bits.VIDC = 0; ctl.bits.FIDC = 1; wrmsrq(MSR_K7_FID_VID_CTL, ctl.val); }
}

unsafe fn change_vid(vid: i32) {
    let mut ctl: msr_fidvidctl = core::mem::zeroed();
    rdmsrq(MSR_K7_FID_VID_CTL, &mut ctl.val);
    if ctl.bits.VID != vid { ctl.bits.SGTC = LATENCY; ctl.bits.VID = vid; ctl.bits.FIDC = 0; ctl.bits.VIDC = 1; wrmsrq(MSR_K7_FID_VID_CTL, ctl.val); }
}

unsafe fn powernow_target(_policy: *mut cpufreq_policy, index: u32) -> i32 {
    let entry = &*POWERNow_TABLE.add(index as usize);
    let fid = (entry.driver_data & 0xff) as i32;
    let vid = ((entry.driver_data & 0xff00) >> 8) as i32;
    let mut status: msr_fidvidstatus = core::mem::zeroed();
    rdmsrq(MSR_K7_FID_VID_STATUS, &mut status.val);
    let old = FSB * FID_CODES[status.bits.CFID as usize] as u32 / 10;
    if HAVE_A0 == 1 { local_irq_disable(); }
    if old > entry.frequency { change_fid(fid); change_vid(vid); } else { change_vid(vid); change_fid(fid); }
    if HAVE_A0 == 1 { local_irq_enable(); }
    0
}

unsafe fn powernow_get(cpu: u32) -> u32 {
    if cpu != 0 { return 0; }
    let mut status: msr_fidvidstatus = core::mem::zeroed();
    rdmsrq(MSR_K7_FID_VID_STATUS, &mut status.val);
    FSB * FID_CODES[status.bits.CFID as usize] as u32 / 10
}

unsafe fn invalidate_entry(entry: usize) { (*POWERNow_TABLE.add(entry)).frequency = CPUFREQ_ENTRY_INVALID; }

unsafe fn powernow_acpi_init() -> i32 {
    // CONFIG_X86_POWERNOW_K7_ACPI supplies the ACPI performance structures and registration calls.
    pr_info!("no support for ACPI processor found - please recompile your kernel with ACPI processor\n");
    -EINVAL
}

unsafe fn print_pst_entry(pst: *mut Pst, j: u32) {
    pr_debug!("PST:%d (@%p)\n", j, pst);
    pr_debug!(" cpuid: 0x%x  fsb: %d  maxFID: 0x%x  startvid: 0x%x\n", (*pst).cpuid, (*pst).fsbspeed, (*pst).maxfid, (*pst).startvid);
}

unsafe fn powernow_decode_bios(maxfid: i32, startvid: i32) -> i32 {
    let etuple = cpuid_eax(0x80000001);
    let mut i = 0xC0000usize;
    while i < 0xffff0 {
        let p = phys_to_virt(i as u64) as *mut u8;
        if core::slice::from_raw_parts(p, 10) == b"AMDK7PNOW!" {
            let psb = p as *mut PsB;
            if (*psb).tableversion != 0x12 { return -ENODEV; }
            LATENCY = (*psb).settlingtime as u32;
            if LATENCY < 100 { LATENCY = 100; }
            let mut q = p.add(core::mem::size_of::<PsB>());
            for j in 0..(*psb).numpst as u32 {
                let pst = q as *mut Pst;
                NUMBER_SCALES = (*pst).numpstates as u32;
                if etuple == (*pst).cpuid && check_fsb((*pst).fsbspeed as u32) != 0 && maxfid == (*pst).maxfid as i32 && startvid == (*pst).startvid as i32 {
                    print_pst_entry(pst, j);
                    return get_ranges(q.add(core::mem::size_of::<Pst>()) as *mut u8);
                }
                q = q.add(core::mem::size_of::<Pst>() + NUMBER_SCALES as usize * 2);
            }
            return -EINVAL;
        }
        i += 1;
    }
    -ENODEV
}

unsafe fn fixup_sgtc() -> u32 {
    let mut m = FSB / 3333;
    if m % 10 >= 5 { m += 5; }
    let mut sgtc = 100 * (m / 10) * LATENCY / 3;
    if sgtc > 0xfffff { sgtc = 0xfffff; }
    sgtc
}

unsafe fn powernow_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    if (*policy).cpu != 0 { return -ENODEV; }
    recalibrate_cpu_khz();
    FSB = 10 * cpu_khz / FID_CODES[0] as u32;
    if FSB == 0 { return -EINVAL; }
    let mut result = powernow_decode_bios(0, 0);
    if result != 0 { result = powernow_acpi_init(); } else { LATENCY = fixup_sgtc(); }
    if result != 0 { return result; }
    (*policy).freq_table = POWERNow_TABLE;
    (*policy).cpuinfo.transition_latency = cpufreq_scale(2000000, FSB, LATENCY);
    0
}

unsafe fn powernow_cpu_exit(_policy: *mut cpufreq_policy) { kfree(POWERNow_TABLE as *mut core::ffi::c_void); }

static mut POWERNOW_DRIVER: cpufreq_driver = cpufreq_driver {
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(powernow_target), get: Some(powernow_get),
    init: Some(powernow_cpu_init), exit: Some(powernow_cpu_exit), name: b"powernow-k7\0".as_ptr() as *const i8,
};

unsafe fn powernow_init() -> i32 { if check_powernow() == 0 { return -ENODEV; } cpufreq_register_driver(&mut POWERNOW_DRIVER) }
unsafe fn powernow_exit() { cpufreq_unregister_driver(&mut POWERNOW_DRIVER); }

// module_param(acpi_force, int, 0444); MODULE_PARM_DESC, MODULE_AUTHOR, MODULE_DESCRIPTION,
// MODULE_LICENSE, late_initcall(powernow_init), and module_exit(powernow_exit) are kernel metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
