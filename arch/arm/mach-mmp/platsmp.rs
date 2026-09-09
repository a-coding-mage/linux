// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019 Lubomir Rintel <lkundrak@v3.sk>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn __raw_writel(value: u32, address: usize);
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> u32;
    fn scu_enable(base: usize);
    static SCU_VIRT_BASE: usize;
    static secondary_startup: unsafe extern "C" fn();
}

// Opaque declaration supplied by the surrounding kernel translation.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> i32>,
}

// C macro: CIU_REG(0x24), supplied by addr-map.h.
extern "C" {
    fn CIU_REG(offset: usize) -> usize;
}

const SW_BRANCH_VIRT_ADDR: usize = unsafe { CIU_REG(0x24) };

unsafe extern "C" fn mmp3_boot_secondary(_cpu: u32, _idle: *mut task_struct) -> i32 {
    /*
     * Apparently, the boot ROM on the second core spins on this
     * register becoming non-zero and then jumps to the address written
     * there. No IPIs involved.
     */
    __raw_writel(__pa_symbol(secondary_startup), SW_BRANCH_VIRT_ADDR);
    0
}

unsafe extern "C" fn mmp3_smp_prepare_cpus(_max_cpus: u32) {
    scu_enable(SCU_VIRT_BASE);
}

// __initconst
static mmp3_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(mmp3_smp_prepare_cpus),
    smp_boot_secondary: Some(mmp3_boot_secondary),
};

// CPU_METHOD_OF_DECLARE(mmp3_smp, "marvell,mmp3-smp", &mmp3_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
