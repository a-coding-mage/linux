/*
 * CPU frequency scaling for Broadcom BMIPS SoCs
 *
 * Copyright (c) 2017 Broadcom
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation version 2.
 *
 * This program is distributed "as is" WITHOUT ANY WARRANTY of any
 * kind, whether express or implied; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// Linux kernel dependencies supplied by other translation units.

pub const BMIPS_CPUFREQ_PREFIX: &[u8] = b"bmips\0";
pub const BMIPS_CPUFREQ_NAME: &[u8] = b"bmips-cpufreq\0";
pub const TRANSITION_LATENCY: u32 = 25 * 1000;
pub const BMIPS5_CLK_DIV_SET_SHIFT: u32 = 0x7;
pub const BMIPS5_CLK_DIV_SHIFT: u32 = 0x4;
pub const BMIPS5_CLK_DIV_MASK: u32 = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BmipsType {
    BMIPS5000,
    BMIPS5200,
}

#[repr(C)]
pub struct CpufreqCompat {
    pub compatible: *const u8,
    pub bmips_type: u32,
    pub clk_mult: u32,
    pub max_freqs: u32,
}

#[repr(C)]
pub struct CpufreqFrequencyTable {
    pub frequency: u32,
    pub driver_data: u32,
}

#[repr(C)]
pub struct CpufreqPolicy {
    pub freq_table: *mut CpufreqFrequencyTable,
}

#[repr(C)]
pub struct CpufreqDriver {
    pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    pub target_index: Option<unsafe extern "C" fn(*mut CpufreqPolicy, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(u32) -> u32>,
    pub init: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut CpufreqPolicy)>,
    pub name: *const u8,
}

extern "C" {
    static mut mips_hpt_frequency: u32;
    static mut priv_: *mut CpufreqCompat;
    fn read_c0_brcm_mode() -> u32;
    fn change_c0_brcm_mode(mask: u32, val: u32);
    fn of_find_compatible_node(from: *mut core::ffi::c_void, ty: *const u8, compatible: *const u8) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn cpufreq_generic_frequency_table_verify(policy: *mut CpufreqPolicy) -> i32;
    fn cpufreq_generic_init(policy: *mut CpufreqPolicy, table: *mut CpufreqFrequencyTable, latency: u32);
    fn cpufreq_register_driver(driver: *mut CpufreqDriver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut CpufreqDriver);
}

pub const CPUFREQ_TABLE_END: u32 = u32::MAX;
pub const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1;
pub const ENOMEM: i32 = 12;
pub const ENOTSUPP: i32 = 524;
pub const ENODEV: i32 = 19;

static mut BMIPS_CPUFREQ_COMPAT: [CpufreqCompat; 3] = [
    CpufreqCompat { compatible: b"brcm,bmips5000\0".as_ptr(), bmips_type: 0, clk_mult: 8, max_freqs: 4 },
    CpufreqCompat { compatible: b"brcm,bmips5200\0".as_ptr(), bmips_type: 1, clk_mult: 8, max_freqs: 4 },
    CpufreqCompat { compatible: core::ptr::null(), bmips_type: 0, clk_mult: 0, max_freqs: 0 },
];

unsafe fn htp_freq_to_cpu_freq(clk_mult: u32) -> u32 {
    mips_hpt_frequency.wrapping_mul(clk_mult) / 1000
}

unsafe fn bmips_cpufreq_get_freq_table(_policy: *const CpufreqPolicy) -> *mut CpufreqFrequencyTable {
    let cpu_freq = htp_freq_to_cpu_freq((*priv_).clk_mult);
    let count = (*priv_).max_freqs.wrapping_add(1) as usize;
    let layout = core::alloc::Layout::array::<CpufreqFrequencyTable>(count).unwrap();
    let table = std::alloc::alloc(layout) as *mut CpufreqFrequencyTable;
    if table.is_null() { return (-ENOMEM as isize) as *mut CpufreqFrequencyTable; }
    for i in 0..(*priv_).max_freqs as usize {
        (*table.add(i)).frequency = cpu_freq / (1u32 << i);
        (*table.add(i)).driver_data = i as u32;
    }
    (*table.add((*priv_).max_freqs as usize)).frequency = CPUFREQ_TABLE_END;
    table
}

unsafe fn bmips_cpufreq_get(_cpu: u32) -> u32 {
    let div = match (*priv_).bmips_type {
        1 | 0 => (read_c0_brcm_mode() >> BMIPS5_CLK_DIV_SHIFT) & BMIPS5_CLK_DIV_MASK,
        _ => 0,
    };
    htp_freq_to_cpu_freq((*priv_).clk_mult) / (1u32 << div)
}

unsafe fn bmips_cpufreq_target_index(policy: *mut CpufreqPolicy, index: u32) -> i32 {
    let div = (*(*policy).freq_table.add(index as usize)).driver_data;
    match (*priv_).bmips_type {
        1 | 0 => change_c0_brcm_mode(BMIPS5_CLK_DIV_MASK << BMIPS5_CLK_DIV_SHIFT,
            (1 << BMIPS5_CLK_DIV_SET_SHIFT) | (div << BMIPS5_CLK_DIV_SHIFT)),
        _ => return -ENOTSUPP,
    }
    0
}

unsafe fn bmips_cpufreq_exit(policy: *mut CpufreqPolicy) {
    std::alloc::dealloc((*policy).freq_table as *mut u8,
        core::alloc::Layout::array::<CpufreqFrequencyTable>(((*priv_).max_freqs + 1) as usize).unwrap());
}

unsafe fn bmips_cpufreq_init(policy: *mut CpufreqPolicy) -> i32 {
    let freq_table = bmips_cpufreq_get_freq_table(policy);
    if (freq_table as isize) < 0 {
        return freq_table as isize as i32;
    }
    cpufreq_generic_init(policy, freq_table, TRANSITION_LATENCY);
    0
}

static mut BMIPS_CPUFREQ_DRIVER: CpufreqDriver = CpufreqDriver {
    flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(bmips_cpufreq_target_index),
    get: Some(bmips_cpufreq_get),
    init: Some(bmips_cpufreq_init),
    exit: Some(bmips_cpufreq_exit),
    name: BMIPS_CPUFREQ_PREFIX.as_ptr(),
};

unsafe fn bmips_cpufreq_driver_init() -> i32 {
    let mut cc = BMIPS_CPUFREQ_COMPAT.as_mut_ptr();
    loop {
        if (*cc).compatible.is_null() { return -ENODEV; }
        let np = of_find_compatible_node(core::ptr::null_mut(), b"cpu\0".as_ptr(), (*cc).compatible);
        if !np.is_null() { of_node_put(np); priv_ = cc; break; }
        cc = cc.add(1);
    }
    cpufreq_register_driver(&mut BMIPS_CPUFREQ_DRIVER)
}

unsafe fn bmips_cpufreq_driver_exit() {
    cpufreq_unregister_driver(&mut BMIPS_CPUFREQ_DRIVER);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
