/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes Linux type and layout definitions supplied by other
// translation units.

#[repr(C)]
pub struct tss_segment_32 {
    pub prev_task_link: u32,
    pub esp0: u32,
    pub ss0: u32,
    pub esp1: u32,
    pub ss1: u32,
    pub esp2: u32,
    pub ss2: u32,
    pub cr3: u32,
    pub eip: u32,
    pub eflags: u32,
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u32,
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub fs: u32,
    pub gs: u32,
    pub ldt_selector: u32,
    pub t: u16,
    pub io_map: u16,
}

#[repr(C)]
pub struct tss_segment_16 {
    pub prev_task_link: u16,
    pub sp0: u16,
    pub ss0: u16,
    pub sp1: u16,
    pub ss1: u16,
    pub sp2: u16,
    pub ss2: u16,
    pub ip: u16,
    pub flag: u16,
    pub ax: u16,
    pub cx: u16,
    pub dx: u16,
    pub bx: u16,
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,
    pub es: u16,
    pub cs: u16,
    pub ss: u16,
    pub ds: u16,
    pub ldt: u16,
}

pub const TSS_IOPB_BASE_OFFSET: usize = 0x66;
pub const TSS_BASE_SIZE: usize = 0x68;
pub const TSS_IOPB_SIZE: usize = 65536 / 8;
pub const TSS_REDIRECTION_SIZE: usize = 256 / 8;
pub const RMODE_TSS_SIZE: usize =
    TSS_BASE_SIZE + TSS_REDIRECTION_SIZE + TSS_IOPB_SIZE + 1;

const _: () = assert!(core::mem::offset_of!(tss_segment_32, io_map) == TSS_IOPB_BASE_OFFSET);
const _: () = assert!(core::mem::size_of::<tss_segment_32>() == TSS_BASE_SIZE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
