// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-sa1100/cpu-sa1110.c
 *
 *  Copyright (C) 2001 Russell King
 *
 * Note: there are two erratas that apply to the SA1110 here:
 *  7 - SDRAM auto-power-up failure (rev A0)
 * 13 - Corruption of internal register reads/writes following
 *      SDRAM reads (rev A0, B0, B1)
 *
 * We ignore rev. A0 and B0 devices; I don't think they're worth supporting.
 *
 * The SDRAM type can be passed on the command line as cpu_sa1110.sdram=type
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct sdram_params {
    name: [core::ffi::c_char; 20],
    rows: u8,
    cas_latency: u8,
    tck: u8,
    trcd: u8,
    trp: u8,
    twr: u8,
    refresh: u16,
}

#[repr(C)]
struct sdram_info {
    mdcnfg: u32,
    mdrefr: u32,
    mdcas: [u32; 3],
}

static mut SDRAM_TBL: [sdram_params; 7] = [
    sdram_params { name: *b"TC59SM716-CL2\0\0\0\0\0\0\0\0\0", rows: 12, tck: 10, trcd: 20, trp: 20, twr: 10, refresh: 64000, cas_latency: 2 },
    sdram_params { name: *b"TC59SM716-CL3\0\0\0\0\0\0\0\0\0", rows: 12, tck: 8, trcd: 20, trp: 20, twr: 8, refresh: 64000, cas_latency: 3 },
    sdram_params { name: *b"K4S641632D\0\0\0\0\0\0\0\0\0\0", rows: 14, tck: 9, trcd: 27, trp: 20, twr: 9, refresh: 64000, cas_latency: 3 },
    sdram_params { name: *b"K4S281632B-1H\0\0\0\0\0\0", rows: 12, tck: 10, trcd: 0, trp: 20, twr: 10, refresh: 64000, cas_latency: 3 },
    sdram_params { name: *b"KM416S4030CT\0\0\0\0\0\0\0\0", rows: 13, tck: 8, trcd: 24, trp: 24, twr: 16, refresh: 64000, cas_latency: 3 },
    sdram_params { name: *b"W982516AH75L\0\0\0\0\0\0\0", rows: 16, tck: 8, trcd: 20, trp: 20, twr: 8, refresh: 64000, cas_latency: 3 },
    sdram_params { name: *b"MT48LC8M16A2TG-75\0\0", rows: 12, tck: 8, trcd: 20, trp: 20, twr: 8, refresh: 64000, cas_latency: 3 },
];

static mut sdram_params_global: sdram_params = sdram_params { name: [0; 20], rows: 0, cas_latency: 0, tck: 0, trcd: 0, trp: 0, twr: 0, refresh: 0 };

#[inline]
unsafe fn ns_to_cycles(ns: u32, khz: u32) -> u32 { (ns.wrapping_mul(khz).wrapping_add(999_999)) / 1_000_000 }

#[inline]
unsafe fn set_mdcas(mdcas: *mut u32, mut delayed: i32, mut rcd: u32) {
    rcd = 2 * rcd - 1;
    let shift = delayed + 1 + rcd as i32;
    *mdcas = (1u32 << rcd) - 1;
    *mdcas |= 0x5555_5555u32 << shift;
    *mdcas.add(1) = 0x5555_5555u32 << (shift as u32 & 1);
    *mdcas.add(2) = *mdcas.add(1);
}

unsafe fn sdram_calculate_timing(sd: *mut sdram_info, cpu_khz: u32, sdram: *mut sdram_params) {
    let mem_khz = cpu_khz / 2;
    let mut sd_khz = mem_khz;
    if ns_to_cycles((*sdram).tck as u32, sd_khz) > 1 || (read_cpuid_revision() < ARM_CPU_REV_SA1110_B2 && sd_khz < 62000) { sd_khz /= 2; }
    (*sd).mdcnfg = MDCNFG & 0x007f007f;
    let twr = ns_to_cycles((*sdram).twr as u32, mem_khz);
    let mut trp = ns_to_cycles((*sdram).trp as u32, mem_khz).wrapping_sub(1);
    if trp < 1 { trp = 1; }
    (*sd).mdcnfg |= trp << 8 | trp << 24 | ((*sdram).cas_latency as u32) << 12 | ((*sdram).cas_latency as u32) << 28 | twr << 14 | twr << 30;
    (*sd).mdrefr = MDREFR & 0xffbf_fff0;
    (*sd).mdrefr |= 7;
    if sd_khz != mem_khz { (*sd).mdrefr |= MDREFR_K1DB2; }
    set_mdcas((*sd).mdcas.as_mut_ptr(), (sd_khz >= 62000) as i32, ns_to_cycles((*sdram).trcd as u32, mem_khz));
}

#[inline]
unsafe fn sdram_set_refresh(dri: u32) { MDREFR = (MDREFR & 0xffff000f) | (dri << 4); let _ = MDREFR; }

unsafe fn sdram_update_refresh(cpu_khz: u32, sdram: *mut sdram_params) {
    let ns_row = ((*sdram).refresh as u32 * 1000) >> (*sdram).rows;
    let dri = ns_to_cycles(ns_row, cpu_khz / 2) / 32;
    sdram_set_refresh(dri);
}

unsafe fn sa1110_target(_policy: *mut cpufreq_policy, ppcr: u32) -> i32 {
    let sdram = &mut sdram_params_global as *mut sdram_params;
    let mut sd = sdram_info { mdcnfg: 0, mdrefr: 0, mdcas: [0; 3] };
    sdram_calculate_timing(&mut sd, sa11x0_freq_table[ppcr as usize].frequency, sdram);
    sdram_set_refresh(2);
    if !irqs_disabled() { msleep(20); } else { mdelay(20); }
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    core::arch::asm!("mcr p15, 0, {zero}, c7, c10, 4", zero = in(reg) 0u32);
    udelay(10);
    // The original contains ARM assembly that writes MDCNFG, MDREFR, MDCAS0-2,
    // and PPCR within a cache line while interrupts are disabled.
    core::arch::asm!("str {mdcnfg}, [{mdcnfgp}]", mdcnfg = in(reg) sd.mdcnfg, mdcnfgp = in(reg) &raw mut MDCNFG);
    local_irq_restore(flags);
    sdram_update_refresh(sa11x0_freq_table[ppcr as usize].frequency, sdram);
    0
}

unsafe fn sa1110_cpu_init(policy: *mut cpufreq_policy) -> i32 { cpufreq_generic_init(policy, sa11x0_freq_table.as_ptr(), 0); 0 }

unsafe fn sa1110_find_sdram(name: *const core::ffi::c_char) -> *mut sdram_params {
    for sdram in SDRAM_TBL.iter_mut() {
        if strcmp(name, sdram.name.as_ptr()) == 0 { return sdram; }
    }
    core::ptr::null_mut()
}

static mut sdram_name: [core::ffi::c_char; 16] = [0; 16];

unsafe fn sa1110_clk_init() -> i32 {
    if !cpu_is_sa1110() { return -ENODEV; }
    let mut name = sdram_name.as_ptr();
    if *name == 0 {
        if machine_is_assabet() { name = b"TC59SM716-CL3\0".as_ptr() as *const _; }
        if machine_is_jornada720() || machine_is_h3600() { name = b"K4S281632B-1H\0".as_ptr() as *const _; }
    }
    let sdram = sa1110_find_sdram(name);
    if !sdram.is_null() {
        memcpy(&mut sdram_params_global as *mut _ as *mut _, sdram as *const _, core::mem::size_of::<sdram_params>());
        return cpufreq_register_driver(&sa1110_driver);
    }
    0
}

// module_param_string(sdram, sdram_name, sizeof(sdram_name), 0);
// arch_initcall(sa1110_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
