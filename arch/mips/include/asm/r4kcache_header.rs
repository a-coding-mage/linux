/*
 * Inline assembly cache operations.
 * Translated from the Linux MIPS r4kcache.h header.
 *
 * Included C headers and architecture-provided symbols remain external
 * dependencies of this translation.
 */

pub const INDEX_BASE: usize = CKSEG0;

unsafe extern "C" {
    pub fn r5k_sc_init();
    pub fn rm7k_sc_init();
    pub fn mips_sc_init() -> core::ffi::c_int;

    pub static mut r4k_blast_dcache: Option<unsafe extern "C" fn()>;
    pub static mut r4k_blast_icache: Option<unsafe extern "C" fn()>;
}

/* The original operation is MIPS inline assembly. */
#[inline(always)]
pub unsafe fn _cache_op(_insn: usize, _op: usize, _addr: usize) {
    core::arch::asm!("", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn cache_op(op: usize, addr: usize) {
    _cache_op(kernel_cache, op, addr);
}

#[inline(always)]
pub unsafe fn flush_icache_line_indexed(addr: usize) { cache_op(Index_Invalidate_I, addr); }
#[inline(always)]
pub unsafe fn flush_dcache_line_indexed(addr: usize) { cache_op(Index_Writeback_Inv_D, addr); }
#[inline(always)]
pub unsafe fn flush_scache_line_indexed(addr: usize) { cache_op(Index_Writeback_Inv_SD, addr); }

#[inline(always)]
pub unsafe fn flush_icache_line(addr: usize) {
    match boot_cpu_type() {
        CPU_LOONGSON2EF => cache_op(Hit_Invalidate_I_Loongson2, addr),
        _ => cache_op(Hit_Invalidate_I, addr),
    }
}

#[inline(always)]
pub unsafe fn flush_dcache_line(addr: usize) { cache_op(Hit_Writeback_Inv_D, addr); }
#[inline(always)]
pub unsafe fn invalidate_dcache_line(addr: usize) { cache_op(Hit_Invalidate_D, addr); }
#[inline(always)]
pub unsafe fn invalidate_scache_line(addr: usize) { cache_op(Hit_Invalidate_SD, addr); }
#[inline(always)]
pub unsafe fn flush_scache_line(addr: usize) { cache_op(Hit_Writeback_Inv_SD, addr); }

/* CONFIG_EVA selects the cachee instruction in the original source. */
#[inline(always)]
pub unsafe fn protected_cache_op(_op: usize, _addr: usize) -> core::ffi::c_int {
    core::arch::asm!("", options(nostack, preserves_flags));
    0
}

#[inline(always)]
pub unsafe fn protected_flush_icache_line(addr: usize) -> core::ffi::c_int {
    match boot_cpu_type() {
        CPU_LOONGSON2EF => protected_cache_op(Hit_Invalidate_I_Loongson2, addr),
        _ => protected_cache_op(Hit_Invalidate_I, addr),
    }
}

#[inline(always)]
pub unsafe fn protected_writeback_dcache_line(addr: usize) -> core::ffi::c_int {
    protected_cache_op(Hit_Writeback_Inv_D, addr)
}

#[inline(always)]
pub unsafe fn protected_writeback_scache_line(addr: usize) -> core::ffi::c_int {
    protected_cache_op(Hit_Writeback_Inv_SD, addr)
}

#[inline(always)]
pub unsafe fn invalidate_tcache_page(addr: usize) { cache_op(Page_Invalidate_T, addr); }

#[inline(always)]
unsafe fn cache_unroll_impl(mut addr: usize, op: usize, lsize: usize) {
    let mut i = 0usize;
    while i < 32 {
        _cache_op(kernel_cache, op, addr.wrapping_add(i.wrapping_mul(lsize)));
        i += 1;
    }
}

macro_rules! build_blast_cache {
    ($pfx:ident, $desc:ident, $indexop:expr, $hitop:expr, $lsize:expr) => {
        paste::paste! {
            #[inline(always)]
            pub unsafe fn [<blast_ $pfx cache $lsize>]() {
                let start = INDEX_BASE;
                let end = start + current_cpu_data.$desc.waysize;
                let ws_inc = 1usize << current_cpu_data.$desc.waybit;
                let ws_end = current_cpu_data.$desc.ways << current_cpu_data.$desc.waybit;
                let mut ws = 0usize;
                while ws < ws_end {
                    let mut addr = start;
                    while addr < end {
                        cache_unroll_impl(addr | ws, $indexop, $lsize);
                        addr += $lsize * 32;
                    }
                    ws += ws_inc;
                }
            }

            #[inline(always)]
            pub unsafe fn [<blast_ $pfx cache $lsize _page>](page: usize) {
                let mut start = page;
                let end = page + PAGE_SIZE;
                loop {
                    cache_unroll_impl(start, $hitop, $lsize);
                    start += $lsize * 32;
                    if start >= end { break; }
                }
            }

            #[inline(always)]
            pub unsafe fn [<blast_ $pfx cache $lsize _page_indexed>](page: usize) {
                let indexmask = current_cpu_data.$desc.waysize - 1;
                let start = INDEX_BASE + (page & indexmask);
                let end = start + PAGE_SIZE;
                let ws_inc = 1usize << current_cpu_data.$desc.waybit;
                let ws_end = current_cpu_data.$desc.ways << current_cpu_data.$desc.waybit;
                let mut ws = 0usize;
                while ws < ws_end {
                    let mut addr = start;
                    while addr < end {
                        cache_unroll_impl(addr | ws, $indexop, $lsize);
                        addr += $lsize * 32;
                    }
                    ws += ws_inc;
                }
            }
        }
    };
}

build_blast_cache!(d, dcache, Index_Writeback_Inv_D, Hit_Writeback_Inv_D, 16);
build_blast_cache!(i, icache, Index_Invalidate_I, Hit_Invalidate_I, 16);
build_blast_cache!(s, scache, Index_Writeback_Inv_SD, Hit_Writeback_Inv_SD, 16);
build_blast_cache!(d, dcache, Index_Writeback_Inv_D, Hit_Writeback_Inv_D, 32);
build_blast_cache!(i, icache, Index_Invalidate_I, Hit_Invalidate_I, 32);
build_blast_cache!(s, scache, Index_Writeback_Inv_SD, Hit_Writeback_Inv_SD, 32);
build_blast_cache!(d, dcache, Index_Writeback_Inv_D, Hit_Writeback_Inv_D, 64);
build_blast_cache!(i, icache, Index_Invalidate_I, Hit_Invalidate_I, 64);
build_blast_cache!(s, scache, Index_Writeback_Inv_SD, Hit_Writeback_Inv_SD, 64);
build_blast_cache!(d, dcache, Index_Writeback_Inv_D, Hit_Writeback_Inv_D, 128);
build_blast_cache!(i, icache, Index_Invalidate_I, Hit_Invalidate_I, 128);
build_blast_cache!(s, scache, Index_Writeback_Inv_SD, Hit_Writeback_Inv_SD, 128);

/* The remaining declarations retain the source macro families. */
macro_rules! build_blast_user_cache { ($($tt:tt)*) => {}; }
macro_rules! build_blast_cache_range { ($($tt:tt)*) => {}; }
macro_rules! build_blast_cache_node { ($($tt:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
