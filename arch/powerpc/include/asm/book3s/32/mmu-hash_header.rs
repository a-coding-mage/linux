/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 32-bit hash table MMU support
 */

/* Block size masks */
pub const BL_128K: u32 = 0x000;
pub const BL_256K: u32 = 0x001;
pub const BL_512K: u32 = 0x003;
pub const BL_1M: u32 = 0x007;
pub const BL_2M: u32 = 0x00f;
pub const BL_4M: u32 = 0x01f;
pub const BL_8M: u32 = 0x03f;
pub const BL_16M: u32 = 0x07f;
pub const BL_32M: u32 = 0x0ff;
pub const BL_64M: u32 = 0x1ff;
pub const BL_128M: u32 = 0x3ff;
pub const BL_256M: u32 = 0x7ff;

/* BAT Access Protection */
pub const BPP_XX: u32 = 0x00; /* No access */
pub const BPP_RX: u32 = 0x01; /* Read only */
pub const BPP_RW: u32 = 0x02; /* Read/write */

/*
 * Contort a phys_addr_t into the right format/bits for a BAT.
 * CONFIG_PHYS_64BIT selects the 64-bit physical-address mapping.
 */
#[inline]
pub const unsafe fn bat_phys_addr(x: u64) -> u32 {
    ((x & 0x00000000fffe0000u64)
        | ((x & 0x0000000e00000000u64) >> 24)
        | ((x & 0x0000000100000000u64) >> 30)) as u32
}

#[inline]
pub const unsafe fn phys_bat_addr(x: u64) -> u64 {
    (x & 0x00000000fffe0000u64)
        | ((x << 24) & 0x0000000e00000000u64)
        | ((x << 30) & 0x0000000100000000u64)
}

#[repr(C)]
pub struct ppc_bat {
    pub batu: u32,
    pub batl: u32,
}

/* Values for PP (assumes Ks=0, Kp=1) */
pub const PP_RWXX: u32 = 0;
pub const PP_RWRX: u32 = 1;
pub const PP_RWRW: u32 = 2;
pub const PP_RXRX: u32 = 3;

/* Values for Segment Registers */
pub const SR_NX: u32 = 0x10000000;
pub const SR_KP: u32 = 0x20000000;
pub const SR_KS: u32 = 0x40000000;

/*
 * Assembly-only macros uus_addi, uus_mtsr, uus_isync, and
 * update_user_segments_by_4 are preserved in the source header for the
 * assembler build; they have no direct Rust item equivalent.
 */

/*
 * This defines the mapping from contexts to VSIDs (virtual segment IDs).
 * We use a skew on both the context and the high 4 bits of the 32-bit
 * virtual address (the "effective segment ID") to spread out entries in
 * the MMU hash table.
 */
#[inline]
pub const fn CTX_TO_VSID(c: u64, id: u64) -> u64 {
    (((c.wrapping_mul(897 * 16)).wrapping_add(id.wrapping_mul(0x111))) & 0xffffff)
}

/* Hardware Page Table Entry.
 * The xpn and x fields are used only by processors supporting extended
 * addressing; otherwise those bits are reserved.
 */
#[repr(C)]
pub struct hash_pte {
    pub v: u64,
    pub vsid: u64,
    pub h: u64,
    pub api: u64,
    pub rpn: u64,
    pub xpn: u64,
    pub r: u64,
    pub c: u64,
    pub w: u64,
    pub i: u64,
    pub m: u64,
    pub g: u64,
    pub x: u64,
    pub pp: u64,
}

#[repr(C)]
pub struct mm_context_t {
    pub id: c_ulong,
    pub sr0: c_ulong,
    pub vdso: *mut c_void,
}

/* CONFIG_PPC_KUEP: INIT_MM_CONTEXT(mm) initializes context.sr0 to SR_NX. */

pub unsafe extern "C" {
    pub fn update_bats();
    pub static mut patch__hash_page_A0: i32;
    pub static mut patch__hash_page_A1: i32;
    pub static mut patch__hash_page_A2: i32;
    pub static mut patch__hash_page_B: i32;
    pub static mut patch__hash_page_C: i32;
    pub static mut patch__flush_hash_A0: i32;
    pub static mut patch__flush_hash_A1: i32;
    pub static mut patch__flush_hash_A2: i32;
    pub static mut patch__flush_hash_B: i32;
    pub fn find_free_bat() -> i32;
    pub fn bat_block_size(base: c_ulong, top: c_ulong) -> c_uint;
}

#[inline(always)]
pub unsafe fn cleanup_cpu_mmu_context() {}

/* External dependencies supplied by the surrounding translation. */
extern "C" {
    static TASK_SIZE: c_ulong;
    fn mtsr(val: u32, reg: u32);
    fn ALIGN(value: c_ulong, alignment: c_ulong) -> c_ulong;
}

#[inline(always)]
pub unsafe fn update_user_segment(n: u32, val: u32) {
    if (n << 28) < ALIGN(TASK_SIZE, 256 * 1024 * 1024) as u32 as c_ulong {
        mtsr(val.wrapping_add(n.wrapping_mul(0x111)), n << 28);
    }
}

#[inline(always)]
pub unsafe fn update_user_segments(mut val: u32) {
    val &= 0xf0ffffff;
    update_user_segment(0, val);
    update_user_segment(1, val);
    update_user_segment(2, val);
    update_user_segment(3, val);
    update_user_segment(4, val);
    update_user_segment(5, val);
    update_user_segment(6, val);
    update_user_segment(7, val);
    update_user_segment(8, val);
    update_user_segment(9, val);
    update_user_segment(10, val);
    update_user_segment(11, val);
    update_user_segment(12, val);
    update_user_segment(13, val);
    update_user_segment(14, val);
    update_user_segment(15, val);
}

pub const mmu_virtual_psize: u32 = MMU_PAGE_4K;
pub const mmu_linear_psize: u32 = MMU_PAGE_256M;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
