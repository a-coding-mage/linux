/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Annapurna labs cpu-resume register structure.
 *
 * Copyright (C) 2015 Annapurna Labs Ltd.
 */

/* Per-cpu regs */
#[repr(C)]
pub struct al_cpu_resume_regs_per_cpu {
    pub flags: u32,
    pub resume_addr: u32,
}

/* general regs */
#[repr(C)]
pub struct al_cpu_resume_regs {
    /* Watermark for validating the CPU resume struct */
    pub watermark: u32,
    pub flags: u32,
    pub per_cpu: [al_cpu_resume_regs_per_cpu; 0],
}

/* The expected magic number for validating the resume addresses */
pub const AL_CPU_RESUME_MAGIC_NUM: u32 = 0xf0e1d200;
pub const AL_CPU_RESUME_MAGIC_NUM_MASK: u32 = 0xffffff00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
