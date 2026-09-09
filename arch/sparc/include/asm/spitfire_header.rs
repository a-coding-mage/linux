/* SPDX-License-Identifier: GPL-2.0 */
/* spitfire.h: SpitFire/BlackBird/Cheetah inline MMU operations. */

/* CONFIG_SPARC64 */
/* Dependency supplied by the surrounding translation: asm/asi.h */

pub const TSB_TAG_TARGET: u64 = 0x0000_0000_0000_0000;
pub const TLB_SFSR: u64 = 0x0000_0000_0000_0018;
pub const TSB_REG: u64 = 0x0000_0000_0000_0028;
pub const TLB_TAG_ACCESS: u64 = 0x0000_0000_0000_0030;
pub const VIRT_WATCHPOINT: u64 = 0x0000_0000_0000_0038;
pub const PHYS_WATCHPOINT: u64 = 0x0000_0000_0000_0040;
pub const TSB_EXTENSION_P: u64 = 0x0000_0000_0000_0048;
pub const TSB_EXTENSION_S: u64 = 0x0000_0000_0000_0050;
pub const TSB_EXTENSION_N: u64 = 0x0000_0000_0000_0058;
pub const TLB_TAG_ACCESS_EXT: u64 = 0x0000_0000_0000_0060;
pub const PRIMARY_CONTEXT: u64 = 0x0000_0000_0000_0008;
pub const SECONDARY_CONTEXT: u64 = 0x0000_0000_0000_0010;
pub const DMMU_SFAR: u64 = 0x0000_0000_0000_0020;

pub const SPITFIRE_HIGHEST_LOCKED_TLBENT: i32 = 64 - 1;
pub const CHEETAH_HIGHEST_LOCKED_TLBENT: i32 = 16 - 1;
pub const L1DCACHE_SIZE: u32 = 0x4000;
pub const SUN4V_CHIP_INVALID: u32 = 0x00;
pub const SUN4V_CHIP_NIAGARA1: u32 = 0x01;
pub const SUN4V_CHIP_NIAGARA2: u32 = 0x02;
pub const SUN4V_CHIP_NIAGARA3: u32 = 0x03;
pub const SUN4V_CHIP_NIAGARA4: u32 = 0x04;
pub const SUN4V_CHIP_NIAGARA5: u32 = 0x05;
pub const SUN4V_CHIP_SPARC_M6: u32 = 0x06;
pub const SUN4V_CHIP_SPARC_M7: u32 = 0x07;
pub const SUN4V_CHIP_SPARC_M8: u32 = 0x08;
pub const SUN4V_CHIP_SPARC64X: u32 = 0x8a;
pub const SUN4V_CHIP_SPARC_SN: u32 = 0x8b;
pub const SUN4V_CHIP_UNKNOWN: u32 = 0xff;

pub const CPU_ID_NIAGARA1: u8 = b'1';
pub const CPU_ID_NIAGARA2: u8 = b'2';
pub const CPU_ID_NIAGARA3: u8 = b'3';
pub const CPU_ID_NIAGARA4: u8 = b'4';
pub const CPU_ID_NIAGARA5: u8 = b'5';
pub const CPU_ID_M6: u8 = b'6';
pub const CPU_ID_M7: u8 = b'7';
pub const CPU_ID_M8: u8 = b'8';
pub const CPU_ID_SONOMA1: u8 = b'N';

#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum ultra_tlb_layout {
    spitfire = 0,
    cheetah = 1,
    cheetah_plus = 2,
    hypervisor = 3,
}

unsafe extern "C" {
    pub static mut tlb_type: ultra_tlb_layout;
    pub static mut sun4v_chip_type: i32;
    pub static mut cheetah_pcache_forced_on: i32;
    pub fn cheetah_enable_pcache();
    pub static mut num_kernel_image_mappings: i32;
}

#[inline]
pub unsafe fn sparc64_highest_locked_tlbent() -> i32 {
    if tlb_type == ultra_tlb_layout::spitfire {
        SPITFIRE_HIGHEST_LOCKED_TLBENT
    } else {
        CHEETAH_HIGHEST_LOCKED_TLBENT
    }
}

#[inline]
pub unsafe fn spitfire_put_dcache_tag(addr: u64, tag: u64) {
    core::arch::asm!("stxa {tag}, [{addr}] {asi}\n\tmembar #Sync", tag = in(reg) tag, addr = in(reg) addr, asi = const ASI_DCACHE_TAG);
}

#[inline]
pub unsafe fn spitfire_put_icache_tag(addr: u64, tag: u64) {
    core::arch::asm!("stxa {tag}, [{addr}] {asi}\n\tmembar #Sync", tag = in(reg) tag, addr = in(reg) addr, asi = const ASI_IC_TAG);
}

#[inline]
pub unsafe fn spitfire_get_dtlb_data(entry: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {d}", a = in(reg) entry << 3, asi = const ASI_DTLB_DATA_ACCESS, d = out(reg) data);
    data &= !0x0003_fe00_0000_0000u64;
    data
}

#[inline]
pub unsafe fn spitfire_get_dtlb_tag(entry: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) entry << 3, asi = const ASI_DTLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn spitfire_put_dtlb_data(entry: i32, data: u64) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) entry << 3, asi = const ASI_DTLB_DATA_ACCESS);
}

#[inline]
pub unsafe fn spitfire_get_itlb_data(entry: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {d}", a = in(reg) entry << 3, asi = const ASI_ITLB_DATA_ACCESS, d = out(reg) data);
    data &= !0x0003_fe00_0000_0000u64;
    data
}

#[inline]
pub unsafe fn spitfire_get_itlb_tag(entry: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) entry << 3, asi = const ASI_ITLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn spitfire_put_itlb_data(entry: i32, data: u64) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) entry << 3, asi = const ASI_ITLB_DATA_ACCESS);
}

#[inline]
pub unsafe fn spitfire_flush_dtlb_nucleus_page(page: u64) {
    core::arch::asm!("stxa %g0, [{p}] {asi}\n\tmembar #Sync", p = in(reg) page | 0x20, asi = const ASI_DMMU_DEMAP);
}

#[inline]
pub unsafe fn spitfire_flush_itlb_nucleus_page(page: u64) {
    core::arch::asm!("stxa %g0, [{p}] {asi}\n\tmembar #Sync", p = in(reg) page | 0x20, asi = const ASI_IMMU_DEMAP);
}

#[inline]
pub unsafe fn cheetah_flush_dtlb_all() {
    core::arch::asm!("stxa %g0, [a] {asi}\n\tmembar #Sync", a = in(reg) 0x80u64, asi = const ASI_DMMU_DEMAP);
}

#[inline]
pub unsafe fn cheetah_flush_itlb_all() {
    core::arch::asm!("stxa %g0, [a] {asi}\n\tmembar #Sync", a = in(reg) 0x80u64, asi = const ASI_IMMU_DEMAP);
}

#[inline]
pub unsafe fn cheetah_get_ldtlb_data(entry: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, %g0\n\tldxa [{a}] {asi}, {d}", a = in(reg) entry << 3, asi = const ASI_DTLB_DATA_ACCESS, d = out(reg) data);
    data
}

#[inline]
pub unsafe fn cheetah_get_litlb_data(entry: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, %g0\n\tldxa [{a}] {asi}, {d}", a = in(reg) entry << 3, asi = const ASI_ITLB_DATA_ACCESS, d = out(reg) data);
    data
}

#[inline]
pub unsafe fn cheetah_get_ldtlb_tag(entry: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) entry << 3, asi = const ASI_DTLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn cheetah_get_litlb_tag(entry: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) entry << 3, asi = const ASI_ITLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn cheetah_put_ldtlb_data(entry: i32, data: u64) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) entry << 3, asi = const ASI_DTLB_DATA_ACCESS);
}

#[inline]
pub unsafe fn cheetah_put_litlb_data(entry: i32, data: u64) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) entry << 3, asi = const ASI_ITLB_DATA_ACCESS);
}

#[inline]
pub unsafe fn cheetah_get_dtlb_data(entry: i32, tlb: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, %g0\n\tldxa [{a}] {asi}, {d}", a = in(reg) (tlb << 16) | (entry << 3), asi = const ASI_DTLB_DATA_ACCESS, d = out(reg) data);
    data
}

#[inline]
pub unsafe fn cheetah_get_dtlb_tag(entry: i32, tlb: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) (tlb << 16) | (entry << 3), asi = const ASI_DTLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn cheetah_put_dtlb_data(entry: i32, data: u64, tlb: i32) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) (tlb << 16) | (entry << 3), asi = const ASI_DTLB_DATA_ACCESS);
}

#[inline]
pub unsafe fn cheetah_get_itlb_data(entry: i32) -> u64 {
    let mut data: u64;
    core::arch::asm!("ldxa [{a}] {asi}, %g0\n\tldxa [{a}] {asi}, {d}", a = in(reg) (2 << 16) | (entry << 3), asi = const ASI_ITLB_DATA_ACCESS, d = out(reg) data);
    data
}

#[inline]
pub unsafe fn cheetah_get_itlb_tag(entry: i32) -> u64 {
    let mut tag: u64;
    core::arch::asm!("ldxa [{a}] {asi}, {t}", a = in(reg) (2 << 16) | (entry << 3), asi = const ASI_ITLB_TAG_READ, t = out(reg) tag);
    tag
}

#[inline]
pub unsafe fn cheetah_put_itlb_data(entry: i32, data: u64) {
    core::arch::asm!("stxa {d}, [{a}] {asi}\n\tmembar #Sync", d = in(reg) data, a = in(reg) (2 << 16) | (entry << 3), asi = const ASI_ITLB_DATA_ACCESS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
