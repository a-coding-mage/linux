/*
 * Copyright (C) 2006 - 2008 Lemote Inc. & Institute of Computing Technology
 * Author: Yanhua, yanh@lemote.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies: linux/cpufreq.h, linux/errno.h, linux/export.h,
// asm/mach-loongson2ef/loongson.h

#[repr(C)]
pub struct CpufreqFrequencyTable {
    pub driver_data: u32,
    pub frequency: u32,
    pub flags: u32,
}

extern "C" {
    pub static LOONGSON_CHIPCFG: *mut core::ffi::c_void;
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn writel(value: u32, addr: *mut core::ffi::c_void);
}

const CPUFREQ_ENTRY_INVALID: u32 = 0x8000_0000;
const CPUFREQ_TABLE_END: u32 = 0xFFFF_FFFF;
const ENOTSUPP: i32 = 524;

pub const DC_ZERO: u32 = 0;
pub const DC_25PT: u32 = 2;
pub const DC_37PT: u32 = 3;
pub const DC_50PT: u32 = 4;
pub const DC_62PT: u32 = 5;
pub const DC_75PT: u32 = 6;
pub const DC_87PT: u32 = 7;
pub const DC_DISABLE: u32 = 8;
pub const DC_RESV: u32 = 9;

#[no_mangle]
pub static mut loongson2_clockmod_table: [CpufreqFrequencyTable; 10] = [
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_RESV, flags: CPUFREQ_ENTRY_INVALID },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_ZERO, flags: CPUFREQ_ENTRY_INVALID },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_25PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_37PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_50PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_62PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_75PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_87PT, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_DISABLE, flags: 0 },
    CpufreqFrequencyTable { driver_data: 0, frequency: DC_RESV, flags: CPUFREQ_TABLE_END },
];

#[no_mangle]
pub unsafe extern "C" fn loongson2_cpu_set_rate(rate_khz: u64) -> i32 {
    let mut pos: *mut CpufreqFrequencyTable = core::ptr::null_mut();
    let table = loongson2_clockmod_table.as_mut_ptr();
    let mut index = 0usize;

    // Equivalent to cpufreq_for_each_valid_entry(pos, loongson2_clockmod_table).
    while index < loongson2_clockmod_table.len() {
        let candidate = table.add(index);
        if (*candidate).flags != CPUFREQ_ENTRY_INVALID
            && (*candidate).flags != CPUFREQ_TABLE_END
        {
            pos = candidate;
            if rate_khz == (*pos).frequency as u64 {
                break;
            }
        }
        index += 1;
    }
    if pos.is_null() || rate_khz != (*pos).frequency as u64 {
        return -ENOTSUPP;
    }

    let mut regval = readl(LOONGSON_CHIPCFG as *const core::ffi::c_void);
    regval = (regval & !0x7) | ((*pos).driver_data.wrapping_sub(1));
    writel(regval, LOONGSON_CHIPCFG);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
