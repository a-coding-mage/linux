/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 ARM Ltd.
 */

// C dependencies: linux/cpu.h, linux/init.h, and linux/percpu.h.

/*
 * Records attributes of an individual CPU.
 */
#[repr(C)]
pub struct Cpuinfo32bit {
    pub reg_id_dfr0: u32,
    pub reg_id_dfr1: u32,
    pub reg_id_isar0: u32,
    pub reg_id_isar1: u32,
    pub reg_id_isar2: u32,
    pub reg_id_isar3: u32,
    pub reg_id_isar4: u32,
    pub reg_id_isar5: u32,
    pub reg_id_isar6: u32,
    pub reg_id_mmfr0: u32,
    pub reg_id_mmfr1: u32,
    pub reg_id_mmfr2: u32,
    pub reg_id_mmfr3: u32,
    pub reg_id_mmfr4: u32,
    pub reg_id_mmfr5: u32,
    pub reg_id_pfr0: u32,
    pub reg_id_pfr1: u32,
    pub reg_id_pfr2: u32,
    pub reg_mvfr0: u32,
    pub reg_mvfr1: u32,
    pub reg_mvfr2: u32,
}

#[repr(C)]
pub struct CpuinfoArm64 {
    pub kobj: Kobject,
    pub reg_ctr: u64,
    pub reg_cntfrq: u64,
    pub reg_dczid: u64,
    pub reg_midr: u64,
    pub reg_revidr: u64,
    pub reg_aidr: u64,
    pub reg_gmid: u64,
    pub reg_smidr: u64,
    pub reg_mpamidr: u64,
    pub reg_id_aa64dfr0: u64,
    pub reg_id_aa64dfr1: u64,
    pub reg_id_aa64isar0: u64,
    pub reg_id_aa64isar1: u64,
    pub reg_id_aa64isar2: u64,
    pub reg_id_aa64isar3: u64,
    pub reg_id_aa64mmfr0: u64,
    pub reg_id_aa64mmfr1: u64,
    pub reg_id_aa64mmfr2: u64,
    pub reg_id_aa64mmfr3: u64,
    pub reg_id_aa64mmfr4: u64,
    pub reg_id_aa64pfr0: u64,
    pub reg_id_aa64pfr1: u64,
    pub reg_id_aa64pfr2: u64,
    pub reg_id_aa64zfr0: u64,
    pub reg_id_aa64smfr0: u64,
    pub reg_id_aa64fpfr0: u64,
    pub aarch32: Cpuinfo32bit,
}

// DECLARE_PER_CPU(struct cpuinfo_arm64, cpu_data);
extern "C" {
    pub static mut cpu_data: CpuinfoArm64;

    pub fn cpuinfo_store_cpu();
    // __init
    pub fn cpuinfo_store_boot_cpu();

    // __init
    pub fn init_cpu_features(info: *mut CpuinfoArm64);
    pub fn update_cpu_features(
        cpu: i32,
        info: *mut CpuinfoArm64,
        boot: *mut CpuinfoArm64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
