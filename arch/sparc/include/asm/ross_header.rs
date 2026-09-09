/* SPDX-License-Identifier: GPL-2.0 */
/* Ross module specific definitions and defines. */

// Dependencies supplied by the surrounding translation unit:
// asm/asi.h and asm/page.h

pub const HYPERSPARC_CWENABLE: u32 = 0x00200000;
pub const HYPERSPARC_SBENABLE: u32 = 0x00100000;
pub const HYPERSPARC_WBENABLE: u32 = 0x00080000;
pub const HYPERSPARC_MIDMASK: u32 = 0x00078000;
pub const HYPERSPARC_BMODE: u32 = 0x00004000;
pub const HYPERSPARC_ACENABLE: u32 = 0x00002000;
pub const HYPERSPARC_CSIZE: u32 = 0x00001000;
pub const HYPERSPARC_MRFLCT: u32 = 0x00000800;
pub const HYPERSPARC_CMODE: u32 = 0x00000400;
pub const HYPERSPARC_CENABLE: u32 = 0x00000100;
pub const HYPERSPARC_NFAULT: u32 = 0x00000002;
pub const HYPERSPARC_MENABLE: u32 = 0x00000001;

pub const HYPERSPARC_ICCR_FTD: u32 = 0x00000002;
pub const HYPERSPARC_ICCR_ICE: u32 = 0x00000001;

#[inline]
pub unsafe fn get_ross_icr() -> u32 {
    let icreg: u32;
    core::arch::asm!(
        ".word 0x8347c000",
        "mov %g1, {icreg}",
        icreg = lateout(reg) icreg,
        out("g1") _,
        options(nostack)
    );
    icreg
}

#[inline]
pub unsafe fn put_ross_icr(icreg: u32) {
    core::arch::asm!(
        "or %g0, {icreg}, %g1",
        ".word 0xbf806000",
        "nop",
        "nop",
        "nop",
        icreg = in(reg) icreg,
        out("g1") _,
        options(nostack)
    );
}

/* HyperSparc specific cache flushing. */

/* This is for the on-chip instruction cache. */
#[inline]
pub unsafe fn hyper_flush_whole_icache() {
    core::arch::asm!(
        "sta %g0, [%g0] {asi}",
        asi = const ASI_M_FLUSH_IWHOLE,
        options(nostack)
    );
}

extern "C" {
    pub static mut vac_cache_size: i32;
    pub static mut vac_line_size: i32;
}

#[inline]
pub unsafe fn hyper_clear_all_tags() {
    let mut addr: usize = 0;
    while addr < vac_cache_size as usize {
        core::arch::asm!(
            "sta %g0, [{addr}] {asi}",
            addr = in(reg) addr,
            asi = const ASI_M_DATAC_TAG,
            options(nostack)
        );
        addr += vac_line_size as usize;
    }
}

#[inline]
pub unsafe fn hyper_flush_unconditional_combined() {
    let mut addr: usize = 0;
    while addr < vac_cache_size as usize {
        core::arch::asm!(
            "sta %g0, [{addr}] {asi}",
            addr = in(reg) addr,
            asi = const ASI_M_FLUSH_CTX,
            options(nostack)
        );
        addr += vac_line_size as usize;
    }
}

#[inline]
pub unsafe fn hyper_flush_cache_user() {
    let mut addr: usize = 0;
    while addr < vac_cache_size as usize {
        core::arch::asm!(
            "sta %g0, [{addr}] {asi}",
            addr = in(reg) addr,
            asi = const ASI_M_FLUSH_USER,
            options(nostack)
        );
        addr += vac_line_size as usize;
    }
}

#[inline]
pub unsafe fn hyper_flush_cache_page(mut page: usize) {
    let end: usize;
    page &= PAGE_MASK as usize;
    end = page + PAGE_SIZE as usize;
    while page < end {
        core::arch::asm!(
            "sta %g0, [{page}] {asi}",
            page = in(reg) page,
            asi = const ASI_M_FLUSH_PAGE,
            options(nostack)
        );
        page += vac_line_size as usize;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
