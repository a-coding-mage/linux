/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2010 Loongson Inc. & Lemote Inc. &
 *                    Institute of Computing Technology
 * Author:  Xiang Gao, gaoxiang@ict.ac.cn
 *          Huacai Chen, chenhc@lemote.com
 *          Xiaofu Meng, Shuangshuang Zhang
 */

pub const NODE_ADDRSPACE_SHIFT: u32 = 44;

#[inline]
pub const fn pa_to_nid(addr: usize) -> usize {
    (addr & 0xf00000000000usize) >> NODE_ADDRSPACE_SHIFT
}

#[inline]
pub const fn nid_to_addrbase(nid: usize) -> usize {
    nid << NODE_ADDRSPACE_SHIFT
}

// C declaration with the kernel's __init annotation.
unsafe extern "C" {
    pub fn prom_init_numa_memory();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
