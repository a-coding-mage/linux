/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: the C header includes <linux/types.h> for these integer types.

#[repr(C)]
pub struct v86_regs {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub eax: u32,
    pub eip: u32,
    pub eflags: u32,
    pub esp: u32,
    pub cs: u16,
    pub ss: u16,
    pub es: u16,
    pub ds: u16,
    pub fs: u16,
    pub gs: u16,
}

/* Task flags */
pub const TF_VBEIB: u8 = 0x01;
pub const TF_BUF_ESDI: u8 = 0x02;
pub const TF_BUF_ESBX: u8 = 0x04;
pub const TF_BUF_RET: u8 = 0x08;
pub const TF_EXIT: u8 = 0x10;

#[repr(C)]
pub struct uvesafb_task {
    pub flags: u8,
    pub buf_len: i32,
    pub regs: v86_regs,
}

/* Constants for the capabilities field
 * in vbe_ib */
pub const VBE_CAP_CAN_SWITCH_DAC: u32 = 0x01;
pub const VBE_CAP_VGACOMPAT: u32 = 0x02;

/* The VBE Info Block */
#[repr(C, packed)]
pub struct vbe_ib {
    pub vbe_signature: [u8; 4],
    pub vbe_version: u16,
    pub oem_string_ptr: u32,
    pub capabilities: u32,
    pub mode_list_ptr: u32,
    pub total_memory: u16,
    pub oem_software_rev: u16,
    pub oem_vendor_name_ptr: u32,
    pub oem_product_name_ptr: u32,
    pub oem_product_rev_ptr: u32,
    pub reserved: [u8; 222],
    pub oem_data: [u8; 256],
    pub misc_data: [u8; 512],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
