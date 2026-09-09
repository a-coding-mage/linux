/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for the wakeup data structure at the head of the
 * wakeup code.
 */

/* C header guard: ARCH_X86_KERNEL_ACPI_RM_WAKEUP_H */
/* The declarations below correspond to the non-assembler C interface. */

/* This must match data at wakeup.S */
#[repr(C, packed)]
pub struct wakeup_header {
    pub video_mode: u16,          /* Video mode number */
    pub pmode_entry: u32,         /* Protected mode resume point, 32-bit only */
    pub pmode_cs: u16,
    pub pmode_cr0: u32,           /* Protected mode cr0 */
    pub pmode_cr3: u32,           /* Protected mode cr3 */
    pub pmode_cr4: u32,           /* Protected mode cr4 */
    pub pmode_efer_low: u32,      /* Protected mode EFER */
    pub pmode_efer_high: u32,
    pub pmode_gdt: u64,
    pub pmode_misc_en_low: u32,   /* Protected mode MISC_ENABLE */
    pub pmode_misc_en_high: u32,
    pub pmode_behavior: u32,      /* Wakeup routine behavior flags */
    pub realmode_flags: u32,
    pub real_magic: u32,
    pub signature: u32,           /* To check we have correct structure */
}

extern "C" {
    pub static mut wakeup_header: wakeup_header;
}

pub const WAKEUP_HEADER_OFFSET: u32 = 8;
pub const WAKEUP_HEADER_SIGNATURE: u32 = 0x51ee1111;

/* Wakeup behavior bits */
pub const WAKEUP_BEHAVIOR_RESTORE_MISC_ENABLE: u32 = 0;
pub const WAKEUP_BEHAVIOR_RESTORE_CR4: u32 = 1;
pub const WAKEUP_BEHAVIOR_RESTORE_EFER: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
