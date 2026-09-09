// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/init.c
 *
 * CPU init code
 *
 * Copyright (C) 2002 - 2009  Paul Mundt
 * Copyright (C) 2003  Richard Curnow
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[cfg(feature = "CONFIG_SH_FPU")]
const CPU_HAS_FPU: i32 = 1;
#[cfg(not(feature = "CONFIG_SH_FPU"))]
const CPU_HAS_FPU: i32 = 0;
#[cfg(feature = "CONFIG_SH_DSP")]
const CPU_HAS_DSP: i32 = 1;
#[cfg(not(feature = "CONFIG_SH_DSP"))]
const CPU_HAS_DSP: i32 = 0;

#[cfg(feature = "CONFIG_SPECULATIVE_EXECUTION")]
const CPUOPM: u32 = 0xff2f0000;
#[cfg(feature = "CONFIG_SPECULATIVE_EXECUTION")]
const CPUOPM_RABD: u32 = 1 << 5;

#[cfg(feature = "CONFIG_SPECULATIVE_EXECUTION")]
unsafe fn speculative_execution_init() {
    __raw_writel(__raw_readl(CPUOPM) & !CPUOPM_RABD, CPUOPM);
    let _ = __raw_readl(CPUOPM);
    ctrl_barrier();
}
#[cfg(not(feature = "CONFIG_SPECULATIVE_EXECUTION"))]
unsafe fn speculative_execution_init() {}

#[cfg(feature = "CONFIG_CPU_SH4A")]
const EXPMASK: u32 = 0xff2f0004;
#[cfg(feature = "CONFIG_CPU_SH4A")]
const EXPMASK_RTEDS: u32 = 1 << 0;
#[cfg(feature = "CONFIG_CPU_SH4A")]
const EXPMASK_BRDSSLP: u32 = 1 << 1;
#[cfg(feature = "CONFIG_CPU_SH4A")]
const EXPMASK_MMCAW: u32 = 1 << 4;

#[cfg(feature = "CONFIG_CPU_SH4A")]
unsafe fn expmask_init() {
    let mut expmask = __raw_readl(EXPMASK);
    expmask &= !(EXPMASK_RTEDS | EXPMASK_BRDSSLP | EXPMASK_MMCAW);
    __raw_writel(expmask, EXPMASK);
    ctrl_barrier();
}
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
unsafe fn expmask_init() {}

#[no_mangle]
pub unsafe extern "C" fn l2_cache_init() {}

#[cfg(not(feature = "CONFIG_CPU_J2"))]
unsafe fn cache_init() {
    jump_to_uncached();
    let ccr = __raw_readl(SH_CCR);

    if ccr & CCR_CACHE_ENABLE != 0 {
        let mut waysize = current_cpu_data.dcache.sets;
        #[cfg(feature = "CCR_CACHE_ORA")]
        if ccr & CCR_CACHE_ORA != 0 { waysize >>= 1; }
        waysize <<= current_cpu_data.dcache.entry_shift;

        let ways = {
            #[cfg(feature = "CCR_CACHE_EMODE")]
            if ccr & CCR_CACHE_EMODE == 0 { 1 } else { current_cpu_data.dcache.ways }
            #[cfg(not(feature = "CCR_CACHE_EMODE"))]
            { current_cpu_data.dcache.ways }
        };
        let mut addrstart = CACHE_OC_ADDRESS_ARRAY;
        let mut remaining = ways;
        loop {
            let mut addr = addrstart;
            while addr < addrstart + waysize {
                __raw_writel(0, addr);
                addr += current_cpu_data.dcache.linesz;
            }
            addrstart += current_cpu_data.dcache.way_incr;
            remaining -= 1;
            if remaining == 0 { break; }
        }
    }

    let mut flags = CCR_CACHE_ENABLE | CCR_CACHE_INVALIDATE;
    #[cfg(feature = "CCR_CACHE_EMODE")]
    if current_cpu_data.dcache.ways > 1 { flags |= CCR_CACHE_EMODE; }
    #[cfg(feature = "CCR_CACHE_EMODE")]
    if current_cpu_data.dcache.ways <= 1 { flags &= !CCR_CACHE_EMODE; }
    #[cfg(feature = "CONFIG_CACHE_WRITETHROUGH")]
    { flags |= CCR_CACHE_WT; }
    #[cfg(feature = "CONFIG_CACHE_WRITEBACK")]
    { flags |= CCR_CACHE_CB; }
    #[cfg(not(any(feature = "CONFIG_CACHE_WRITETHROUGH", feature = "CONFIG_CACHE_WRITEBACK")))]
    { flags &= !CCR_CACHE_ENABLE; }
    l2_cache_init();
    __raw_writel(flags, SH_CCR);
    back_to_cached();
}
#[cfg(feature = "CONFIG_CPU_J2")]
unsafe fn cache_init() {}

const fn cshape(totalsize: u32, linesize: u32, assoc: u32) -> u32 {
    (totalsize & !0xff) | (linesize << 4) | assoc
}

unsafe fn detect_cache_shape() {
    l1d_cache_shape = cshape(current_cpu_data.dcache.way_size * current_cpu_data.dcache.ways, ilog2(current_cpu_data.dcache.linesz), current_cpu_data.dcache.ways);
    if current_cpu_data.dcache.flags & SH_CACHE_COMBINED != 0 { l1i_cache_shape = l1d_cache_shape; }
    else { l1i_cache_shape = cshape(current_cpu_data.icache.way_size * current_cpu_data.icache.ways, ilog2(current_cpu_data.icache.linesz), current_cpu_data.icache.ways); }
    if current_cpu_data.flags & CPU_HAS_L2_CACHE != 0 { l2_cache_shape = cshape(current_cpu_data.scache.way_size * current_cpu_data.scache.ways, ilog2(current_cpu_data.scache.linesz), current_cpu_data.scache.ways); }
    else { l2_cache_shape = -1; }
}

unsafe fn fpu_init() {
    if fpu_disabled && current_cpu_data.flags & CPU_HAS_FPU != 0 {
        printk("FPU Disabled\n");
        current_cpu_data.flags &= !CPU_HAS_FPU;
    }
    disable_fpu();
    clear_used_math();
}

#[cfg(feature = "CONFIG_SH_DSP")]
unsafe fn release_dsp() {
    let mut sr: usize;
    core::arch::asm!("stc sr, {0}", "and {1}, {0}", "ldc {0}, sr", out(reg) sr, in(reg) (!SR_DSP));
}
#[cfg(feature = "CONFIG_SH_DSP")]
unsafe fn dsp_init() {
    let mut sr: usize;
    core::arch::asm!("stc sr, {0}", "or {1}, {0}", "ldc {0}, sr", "nop", "stc sr, {0}", out(reg) sr, in(reg) SR_DSP);
    if sr & SR_DSP != 0 { current_cpu_data.flags |= CPU_HAS_DSP; }
    if dsp_disabled && current_cpu_data.flags & CPU_HAS_DSP != 0 { printk("DSP Disabled\n"); current_cpu_data.flags &= !CPU_HAS_DSP; }
    release_dsp();
}
#[cfg(not(feature = "CONFIG_SH_DSP"))]
unsafe fn dsp_init() {}

#[no_mangle]
pub unsafe extern "C" fn cpu_init() {
    (*current_thread_info()).cpu = hard_smp_processor_id();
    cpu_probe();
    if current_cpu_data.type_ == CPU_SH_NONE { panic!("Unknown CPU"); }
    current_cpu_data.icache.entry_mask = current_cpu_data.icache.way_incr - current_cpu_data.icache.linesz;
    current_cpu_data.icache.way_size = current_cpu_data.icache.sets * current_cpu_data.icache.linesz;
    current_cpu_data.dcache.entry_mask = current_cpu_data.dcache.way_incr - current_cpu_data.dcache.linesz;
    current_cpu_data.dcache.way_size = current_cpu_data.dcache.sets * current_cpu_data.dcache.linesz;
    cache_init();
    if raw_smp_processor_id() == 0 {
        #[cfg(feature = "CONFIG_MMU")]
        { shm_align_mask = core::cmp::max(current_cpu_data.dcache.way_size - 1, PAGE_SIZE - 1); }
        #[cfg(not(feature = "CONFIG_MMU"))]
        { shm_align_mask = PAGE_SIZE - 1; }
        detect_cache_shape();
    }
    fpu_init();
    dsp_init();
    current_cpu_data.asid_cache = NO_CONTEXT;
    current_cpu_data.phys_bits = if __in_29bit_mode() { 29 } else { 32 };
    speculative_execution_init();
    expmask_init();
    if raw_smp_processor_id() == 0 {
        sh_bios_vbr_init();
        per_cpu_trap_init();
        init_thread_xstate();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
