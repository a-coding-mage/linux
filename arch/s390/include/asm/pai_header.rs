/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Processor Activity Instrumentation support for cryptography counters
 *
 *  Copyright IBM Corp. 2022
 *  Author(s): Thomas Richter <tmricht@linux.ibm.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct QpaciInfoBlock {
    pub header: u64,
    /* C bit-fields: reserved:8, num_cc:8, reserved:9,
     * num_nnpa:7, reserved:32. */
    pub counters: u64,
}

#[inline]
pub unsafe fn qpaci(info: *mut QpaciInfoBlock) -> i32 {
    /* Size of info (in double words minus one). */
    let mut size: usize = core::mem::size_of::<QpaciInfoBlock>()
        / core::mem::size_of::<u64>()
        - 1;
    let mut cc: i32;

    core::arch::asm!(
        "lgr 0,{size}",
        ".insn s,0xb28f0000,{info}",
        "lgr {size},0",
        /* CC_IPM(cc), CC_OUT(cc, cc), and CC_CLOBBER_LIST are supplied by
         * the target kernel's assembly interface. */
        size = inout(reg) size,
        info = inout(reg) *info,
        lateout("0") cc,
        options(nostack)
    );
    if cc != 0 {
        ((size + 1) * core::mem::size_of::<u64>()) as i32
    } else {
        0
    }
}

pub const PAI_CRYPTO_BASE: u32 = 0x1000; /* First event number */
pub const PAI_CRYPTO_MAXCTR: u32 = 256; /* Max # of event counters */
pub const PAI_CRYPTO_KERNEL_OFFSET: u32 = 2048;
pub const PAI_NNPA_BASE: u32 = 0x1800; /* First event number */
pub const PAI_NNPA_MAXCTR: u32 = 128; /* Max # of event counters */

/* DECLARE_STATIC_KEY_FALSE(pai_key); */
extern "C" {
    pub static mut pai_key: core::ffi::c_uchar;
}

#[inline(always)]
pub unsafe fn pai_kernel_enter(regs: *mut crate::pt_regs) {
    if !crate::IS_ENABLED(crate::CONFIG_PERF_EVENTS) {
        return;
    }
    if !crate::static_branch_unlikely(&pai_key) {
        return;
    }
    if !(*crate::get_lowcore()).ccd {
        return;
    }
    if !crate::user_mode(regs) {
        return;
    }
    crate::WRITE_ONCE(
        &mut (*crate::get_lowcore()).ccd,
        (*crate::get_lowcore()).ccd | PAI_CRYPTO_KERNEL_OFFSET,
    );
}

#[inline(always)]
pub unsafe fn pai_kernel_exit(regs: *mut crate::pt_regs) {
    if !crate::IS_ENABLED(crate::CONFIG_PERF_EVENTS) {
        return;
    }
    if !crate::static_branch_unlikely(&pai_key) {
        return;
    }
    if !(*crate::get_lowcore()).ccd {
        return;
    }
    if !crate::user_mode(regs) {
        return;
    }
    crate::WRITE_ONCE(
        &mut (*crate::get_lowcore()).ccd,
        (*crate::get_lowcore()).ccd & !PAI_CRYPTO_KERNEL_OFFSET,
    );
}

/* PAI_SAVE_AREA(x)  => (&mut (*x).hw).event_base */
/* PAI_CPU_MASK(x)   => (&mut (*x).hw).addr_filters */
/* PAI_PMU_IDX(x)    => (&mut (*x).hw).last_tag */
/* PAI_SWLIST(x)     => (&mut (*x).hw).tp_list */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
