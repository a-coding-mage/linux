/* SPDX-License-Identifier: GPL-2.0 */
/* cpudata.h: Per-cpu parameters.
 *
 * Copyright (C) 2003, 2005, 2006 David S. Miller (davem@davemloft.net)
 */

/* The C header is excluded for assembler builds. */

#[repr(C)]
pub struct CpuinfoSparc {
    /* Dcache line 1 */
    pub __softirq_pending: u32, /* must be 1st, see rtrap.S */
    pub __nmi_count: u32,
    pub clock_tick: u64, /* %tick's per second */
    pub __pad: u64,
    pub irq0_irqs: u32,
    pub __pad2: u32,

    /* Dcache line 2, rarely used */
    pub dcache_size: u32,
    pub dcache_line_size: u32,
    pub icache_size: u32,
    pub icache_line_size: u32,
    pub ecache_size: u32,
    pub ecache_line_size: u32,
    pub sock_id: u16, /* physical package */
    pub core_id: u16,
    pub max_cache_id: u16, /* groupings of highest shared cache */
    pub proc_id: i16, /* strand (aka HW thread) id */
}

/* DECLARE_PER_CPU(cpuinfo_sparc, __cpu_data); */
extern "C" {
    pub static mut __cpu_data: CpuinfoSparc;
}

/* Supplied by the per-CPU support layer. */
#[macro_export]
macro_rules! cpu_data {
    ($cpu:expr) => {
        per_cpu!(__cpu_data, $cpu)
    };
}

#[macro_export]
macro_rules! local_cpu_data {
    () => {
        *this_cpu_ptr!(&__cpu_data)
    };
}

/* Dependency equivalent to <asm/trap_block.h> is supplied by other files. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
