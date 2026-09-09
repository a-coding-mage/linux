/* SPDX-License-Identifier: GPL-2.0 */
/* Written 2000 by Andi Kleen */

/* Segment descriptor structure definitions, usable from both x86_64 and i386 archs. */

pub const _DESC_ACCESSED: u16 = 0x0001;
pub const _DESC_DATA_WRITABLE: u16 = 0x0002;
pub const _DESC_CODE_READABLE: u16 = 0x0002;
pub const _DESC_DATA_EXPAND_DOWN: u16 = 0x0004;
pub const _DESC_CODE_CONFORMING: u16 = 0x0004;
pub const _DESC_CODE_EXECUTABLE: u16 = 0x0008;
pub const _DESC_S: u16 = 0x0010;
pub const _DESC_PRESENT: u16 = 0x0080;
pub const _DESC_LONG_CODE: u16 = 0x2000;
pub const _DESC_DB: u16 = 0x4000;
pub const _DESC_GRANULARITY_4K: u16 = 0x8000;

#[inline]
pub const fn _DESC_DPL(dpl: u16) -> u16 { dpl << 5 }
#[inline]
pub const fn _DESC_SYSTEM(code: u16) -> u16 { code }

pub const _DESC_DATA: u16 = _DESC_S | _DESC_PRESENT | _DESC_ACCESSED | _DESC_DATA_WRITABLE;
pub const _DESC_CODE: u16 = _DESC_S | _DESC_PRESENT | _DESC_ACCESSED | _DESC_CODE_READABLE | _DESC_CODE_EXECUTABLE;
pub const DESC_DATA16: u16 = _DESC_DATA;
pub const DESC_CODE16: u16 = _DESC_CODE;
pub const DESC_DATA32: u16 = _DESC_DATA | _DESC_GRANULARITY_4K | _DESC_DB;
pub const DESC_DATA32_BIOS: u16 = _DESC_DATA | _DESC_DB;
pub const DESC_CODE32: u16 = _DESC_CODE | _DESC_GRANULARITY_4K | _DESC_DB;
pub const DESC_CODE32_BIOS: u16 = _DESC_CODE | _DESC_DB;
pub const DESC_TSS32: u16 = _DESC_SYSTEM(9) | _DESC_PRESENT;
pub const DESC_DATA64: u16 = _DESC_DATA | _DESC_GRANULARITY_4K | _DESC_DB;
pub const DESC_CODE64: u16 = _DESC_CODE | _DESC_GRANULARITY_4K | _DESC_LONG_CODE;
pub const DESC_USER: u16 = _DESC_DPL(3);

#[repr(C, packed)]
pub struct desc_struct {
    pub limit0: u16,
    pub base0: u16,
    pub base1_type_s_dpl_p: u16,
    pub limit1_avl_l_d_g_base2: u16,
}

#[inline]
pub const fn GDT_ENTRY_INIT(flags: u32, base: u32, limit: u32) -> desc_struct {
    desc_struct {
        limit0: ((limit >> 0) & 0xffff) as u16,
        limit1_avl_l_d_g_base2: (((limit >> 16) & 0x000f)
            | (((flags >> 12) & 0x0001) << 4)
            | (((flags >> 13) & 0x0001) << 5)
            | (((flags >> 14) & 0x0001) << 6)
            | (((flags >> 15) & 0x0001) << 7)
            | (((base >> 24) & 0x00ff) << 8)) as u16,
        base0: ((base >> 0) & 0xffff) as u16,
        base1_type_s_dpl_p: (((base >> 16) & 0x00ff)
            | (((flags >> 0) & 0x000f) << 8)
            | (((flags >> 4) & 0x0001) << 12)
            | (((flags >> 5) & 0x0003) << 13)
            | (((flags >> 7) & 0x0001) << 15)) as u16,
    }
}

pub const GATE_INTERRUPT: u32 = 0xE;
pub const GATE_TRAP: u32 = 0xF;
pub const GATE_CALL: u32 = 0xC;
pub const GATE_TASK: u32 = 0x5;
pub const DESC_TSS: u32 = 0x9;
pub const DESC_LDT: u32 = 0x2;
pub const DESCTYPE_S: u32 = 0x10; /* !system */

#[repr(C, packed)]
pub struct ldttss_desc {
    pub limit0: u16,
    pub base0: u16,
    pub base1_type_dpl_p: u16,
    pub limit1_zero0_g_base2: u16,
    #[cfg(target_arch = "x86_64")]
    pub base3: u32,
    #[cfg(target_arch = "x86_64")]
    pub zero1: u32,
}
pub type ldt_desc = ldttss_desc;
pub type tss_desc = ldttss_desc;

#[repr(C, packed)]
pub struct idt_bits { pub ist_zero_type_dpl_p: u16 }

#[repr(C)]
pub struct idt_data {
    pub vector: u32,
    pub segment: u32,
    pub bits: idt_bits,
    pub addr: *const core::ffi::c_void,
}

#[repr(C, packed)]
pub struct gate_struct {
    pub offset_low: u16,
    pub segment: u16,
    pub bits: idt_bits,
    pub offset_middle: u16,
    #[cfg(target_arch = "x86_64")]
    pub offset_high: u32,
    #[cfg(target_arch = "x86_64")]
    pub reserved: u32,
}
pub type gate_desc = gate_struct;

#[inline]
pub unsafe fn gate_offset(g: *const gate_desc) -> u64 {
    #[cfg(target_arch = "x86_64")]
    { (*g).offset_low as u64 | ((*g).offset_middle as u64) << 16 | ((*g).offset_high as u64) << 32 }
    #[cfg(not(target_arch = "x86_64"))]
    { (*g).offset_low as u64 | ((*g).offset_middle as u64) << 16 }
}

#[inline]
pub unsafe fn gate_segment(g: *const gate_desc) -> u64 { (*g).segment as u64 }

#[repr(C, packed)]
pub struct desc_ptr { pub size: u16, pub address: usize }

pub const BOOT_IDT_ENTRIES: u32 = 32;
pub const AR_TYPE_RODATA: u32 = 0 * (1 << 9);
pub const AR_TYPE_RWDATA: u32 = 1 * (1 << 9);
pub const AR_TYPE_RODATA_EXPDOWN: u32 = 2 * (1 << 9);
pub const AR_TYPE_RWDATA_EXPDOWN: u32 = 3 * (1 << 9);
pub const AR_TYPE_XOCODE: u32 = 4 * (1 << 9);
pub const AR_TYPE_XRCODE: u32 = 5 * (1 << 9);
pub const AR_TYPE_XOCODE_CONF: u32 = 6 * (1 << 9);
pub const AR_TYPE_XRCODE_CONF: u32 = 7 * (1 << 9);
pub const AR_TYPE_MASK: u32 = 7 * (1 << 9);
pub const AR_DPL0: u32 = 0 * (1 << 13);
pub const AR_DPL3: u32 = 3 * (1 << 13);
pub const AR_DPL_MASK: u32 = 3 * (1 << 13);
pub const AR_A: u32 = 1 << 8; /* "Accessed" */
pub const AR_S: u32 = 1 << 12; /* If clear, "System" segment */
pub const AR_P: u32 = 1 << 15; /* "Present" */
pub const AR_AVL: u32 = 1 << 20; /* "AVaiLable" (no HW effect) */
pub const AR_L: u32 = 1 << 21; /* "Long mode" for code segments */
pub const AR_DB: u32 = 1 << 22; /* D/B, effect depends on type */
pub const AR_G: u32 = 1 << 23; /* "Granularity" (limit in pages) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
