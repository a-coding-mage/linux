/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 by Ralf Baechle
 * Copyright (C) 2009, 2012 Cavium, Inc.
 */

use core::ffi::c_void;

type U64 = u64;

#[repr(C)]
pub struct Clocksource {
    pub name: *const u8,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> U64>,
    pub mask: U64,
    pub flags: u32,
    pub rating: i32,
    pub mult: U64,
    pub shift: U64,
}

#[repr(C)]
pub struct BootFields { pub c_mul: U64, pub pnr_mul: U64 }
#[repr(C)]
pub union CvmxMioRstBoot { pub u64_: U64, pub s: BootFields }
#[repr(C)]
pub union CvmxRstBoot { pub u64_: U64, pub s: BootFields }

extern "C" {
    fn octeon_get_clock_rate() -> U64;
    fn current_cpu_type() -> i32;
    fn cvmx_read_csr(reg: U64) -> U64;
    fn octeon_has_feature(feature: i32) -> bool;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn write_c0_cvmcount(value: U64);
    fn read_c0_cvmcount() -> U64;
    fn clocksource_register_hz(cs: *mut Clocksource, hz: U64) -> i32;
    static mut preset_lpj: c_ulong;
}

type c_ulong = usize;
const CPU_CAVIUM_OCTEON2: i32 = 2;
const CPU_CAVIUM_OCTEON3: i32 = 3;
const OCTEON_FEATURE_FPA3: i32 = 0;
const CVMX_MIO_RST_BOOT: U64 = 0;
const CVMX_RST_BOOT: U64 = 0;
const CVMX_FPA_CLK_COUNT: U64 = 0;
const CVMX_IPD_CLK_COUNT: U64 = 0;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1;
const CLOCKSOURCE_MASK_64: U64 = u64::MAX;
const HZ: U64 = 100;

static mut f: U64 = 0;
static mut rdiv: U64 = 0;
static mut sdiv: U64 = 0;
static mut octeon_udelay_factor: U64 = 0;
static mut octeon_ndelay_factor: U64 = 0;

pub unsafe extern "C" fn octeon_setup_delays() {
    octeon_udelay_factor = octeon_get_clock_rate() / 1_000_000;
    /* For __ndelay we divide by 2^16, so the factor is multiplied by the same amount. */
    octeon_ndelay_factor = (octeon_udelay_factor.wrapping_mul(0x10000)) / 1000;
    preset_lpj = octeon_get_clock_rate() / HZ;

    if current_cpu_type() == CPU_CAVIUM_OCTEON2 {
        let rst_boot = CvmxMioRstBoot { u64_: cvmx_read_csr(CVMX_MIO_RST_BOOT) };
        rdiv = rst_boot.s.c_mul;
        sdiv = rst_boot.s.pnr_mul;
        f = (0x8000_0000_0000_0000u64 / sdiv).wrapping_mul(2);
    } else if current_cpu_type() == CPU_CAVIUM_OCTEON3 {
        let rst_boot = CvmxRstBoot { u64_: cvmx_read_csr(CVMX_RST_BOOT) };
        rdiv = rst_boot.s.c_mul;
        sdiv = rst_boot.s.pnr_mul;
        f = (0x8000_0000_0000_0000u64 / sdiv).wrapping_mul(2);
    }
}

pub unsafe extern "C" fn octeon_init_cvmcount() {
    let clk_reg = if octeon_has_feature(OCTEON_FEATURE_FPA3) { CVMX_FPA_CLK_COUNT } else { CVMX_IPD_CLK_COUNT };
    let mut flags: c_ulong = 0;
    let mut loops: u32 = 2;
    local_irq_save(&mut flags);
    while loops != 0 {
        loops -= 1;
        let mut clk_count = cvmx_read_csr(clk_reg);
        if rdiv != 0 {
            clk_count = clk_count.wrapping_mul(rdiv);
            if f != 0 { clk_count = (((clk_count as u128) * (f as u128)) >> 64) as u64; }
        }
        write_c0_cvmcount(clk_count);
    }
    local_irq_restore(flags);
}

unsafe extern "C" fn octeon_cvmcount_read(_cs: *mut Clocksource) -> U64 { read_c0_cvmcount() }

static mut clocksource_mips: Clocksource = Clocksource {
    name: b"OCTEON_CVMCOUNT\0".as_ptr(), read: Some(octeon_cvmcount_read),
    mask: CLOCKSOURCE_MASK_64, flags: CLOCK_SOURCE_IS_CONTINUOUS, rating: 0, mult: 0, shift: 0,
};

pub unsafe extern "C" fn sched_clock() -> u64 {
    let cnt = read_c0_cvmcount();
    let product = (cnt as u128) * (clocksource_mips.mult as u128);
    let shift = clocksource_mips.shift;
    let t1 = (product >> 64) as u64;
    let t2 = (product & u64::MAX as u128) as u64;
    ((t2 >> shift) | (t1 << (64 - shift)))
}

pub unsafe extern "C" fn plat_time_init() {
    clocksource_mips.rating = 300;
    clocksource_register_hz(&mut clocksource_mips, octeon_get_clock_rate());
}

pub unsafe extern "C" fn __udelay(us: c_ulong) { let mut cur = read_c0_cvmcount(); let end = cur.wrapping_add((us as u64).wrapping_mul(octeon_udelay_factor)); while end > cur { cur = read_c0_cvmcount(); } }
pub unsafe extern "C" fn __ndelay(ns: c_ulong) { let mut cur = read_c0_cvmcount(); let end = cur.wrapping_add((((ns as u64).wrapping_mul(octeon_ndelay_factor)) >> 16)); while end > cur { cur = read_c0_cvmcount(); } }
pub unsafe extern "C" fn __delay(loops: c_ulong) { let mut cur = read_c0_cvmcount(); let end = cur.wrapping_add(loops as u64); while end > cur { cur = read_c0_cvmcount(); } }

pub unsafe extern "C" fn octeon_io_clk_delay(count: c_ulong) {
    let cur = read_c0_cvmcount();
    let mut end = count as u64;
    if rdiv != 0 { end = end.wrapping_mul(rdiv); if f != 0 { end = (((end as u128) * (f as u128)) >> 64) as u64; } end = cur.wrapping_add(end); } else { end = cur.wrapping_add(end); }
    let mut now = cur;
    while end > now { now = read_c0_cvmcount(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
