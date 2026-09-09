// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Based on Ocelot Linux port, which is
 * Copyright 2001 MontaVista Software Inc.
 * Author: jsun@mvista.com or jsun@junsun.net
 *
 * Copyright 2003 ICT CAS
 * Author: Michael Guo <guoyi@ict.ac.cn>
 *
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// The declarations below are supplied by the corresponding kernel and
// firmware dependencies in the surrounding translation unit.
#[repr(C)]
pub struct CpuData {
    pub processor_id: u32,
}

extern "C" {
    pub static mut memsize: u32;
    pub static mut highmemsize: u32;
    pub static mut current_cpu_data: CpuData;

    pub fn fw_getenvl(name: *const u8) -> u32;
    pub fn pr_info(fmt: *const i8, ...);
}

pub const PRID_REV_MASK: u32 = 0x3f;
pub const PRID_REV_LOONGSON2E: u32 = 0x02;
pub const PRID_REV_LOONGSON2F: u32 = 0x03;

#[no_mangle]
pub static mut cpu_clock_freq: u32 = 0;

// __init is a kernel build-time section attribute in the C source.
#[no_mangle]
pub unsafe extern "C" fn prom_init_env() {
    /* pmon passes arguments in 32bit pointers */
    let processor_id: u32;

    cpu_clock_freq = fw_getenvl(b"cpuclock\0".as_ptr());
    memsize = fw_getenvl(b"memsize\0".as_ptr());
    highmemsize = fw_getenvl(b"highmemsize\0".as_ptr());

    if memsize == 0 {
        memsize = 256;
    }

    pr_info(
        b"memsize=%u, highmemsize=%u\n\0".as_ptr() as *const i8,
        memsize,
        highmemsize,
    );

    if cpu_clock_freq == 0 {
        processor_id = (&current_cpu_data).processor_id;
        match processor_id & PRID_REV_MASK {
            PRID_REV_LOONGSON2E => {
                cpu_clock_freq = 533080000;
            }
            PRID_REV_LOONGSON2F => {
                cpu_clock_freq = 797000000;
            }
            _ => {
                cpu_clock_freq = 100000000;
            }
        }
    }
    pr_info(
        b"CpuClock = %u\n\0".as_ptr() as *const i8,
        cpu_clock_freq,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
