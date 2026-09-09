/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright 2018-2020 Broadcom.
 */

// Translated from the Linux UAPI header bcm_vk.h.

pub const BCM_VK_MAX_FILENAME: usize = 64;

#[repr(C)]
pub struct vk_image {
    pub type_: u32, // Type of image
    // 1st stage (load to SRAM)
    // 2nd stage (load to DDR)
    pub filename: [u8; BCM_VK_MAX_FILENAME], // Filename of image
}

pub const VK_IMAGE_TYPE_BOOT1: u32 = 1;
pub const VK_IMAGE_TYPE_BOOT2: u32 = 2;

#[repr(C)]
pub struct vk_reset {
    pub arg1: u32,
    pub arg2: u32,
}

pub const VK_MAGIC: u32 = 0x5e;

// Linux _IOW(type, nr, data): _IOC(_IOC_WRITE, type, nr, sizeof(data)).
const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
const _IOC_WRITE: u32 = 1;

const fn _ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT)
}

// Load image to Valkyrie
pub const VK_IOCTL_LOAD_IMAGE: u32 = _ioc(_IOC_WRITE, VK_MAGIC, 0x2, 68);

// Send Reset to Valkyrie
pub const VK_IOCTL_RESET: u32 = _ioc(_IOC_WRITE, VK_MAGIC, 0x4, 8);

/*
 * Firmware Status accessed directly via BAR space
 */
pub const VK_BAR_FWSTS: u32 = 0x41c;
pub const VK_BAR_COP_FWSTS: u32 = 0x428;

// VK_FWSTS definitions
pub const VK_FWSTS_RELOCATION_ENTRY: u32 = 1u32 << 0;
pub const VK_FWSTS_RELOCATION_EXIT: u32 = 1u32 << 1;
pub const VK_FWSTS_INIT_START: u32 = 1u32 << 2;
pub const VK_FWSTS_ARCH_INIT_DONE: u32 = 1u32 << 3;
pub const VK_FWSTS_PRE_KNL1_INIT_DONE: u32 = 1u32 << 4;
pub const VK_FWSTS_PRE_KNL2_INIT_DONE: u32 = 1u32 << 5;
pub const VK_FWSTS_POST_KNL_INIT_DONE: u32 = 1u32 << 6;
pub const VK_FWSTS_INIT_DONE: u32 = 1u32 << 7;
pub const VK_FWSTS_APP_INIT_START: u32 = 1u32 << 8;
pub const VK_FWSTS_APP_INIT_DONE: u32 = 1u32 << 9;
pub const VK_FWSTS_MASK: u32 = 0xffffffff;
pub const VK_FWSTS_READY: u32 = VK_FWSTS_INIT_START
    | VK_FWSTS_ARCH_INIT_DONE
    | VK_FWSTS_PRE_KNL1_INIT_DONE
    | VK_FWSTS_PRE_KNL2_INIT_DONE
    | VK_FWSTS_POST_KNL_INIT_DONE
    | VK_FWSTS_INIT_DONE
    | VK_FWSTS_APP_INIT_START
    | VK_FWSTS_APP_INIT_DONE;

// Deinit
pub const VK_FWSTS_APP_DEINIT_START: u32 = 1u32 << 23;
pub const VK_FWSTS_APP_DEINIT_DONE: u32 = 1u32 << 24;
pub const VK_FWSTS_DRV_DEINIT_START: u32 = 1u32 << 25;
pub const VK_FWSTS_DRV_DEINIT_DONE: u32 = 1u32 << 26;
pub const VK_FWSTS_RESET_DONE: u32 = 1u32 << 27;
pub const VK_FWSTS_DEINIT_TRIGGERED: u32 = VK_FWSTS_APP_DEINIT_START
    | VK_FWSTS_APP_DEINIT_DONE
    | VK_FWSTS_DRV_DEINIT_START
    | VK_FWSTS_DRV_DEINIT_DONE;

// Last nibble for reboot reason
pub const VK_FWSTS_RESET_REASON_SHIFT: u32 = 28;
pub const VK_FWSTS_RESET_REASON_MASK: u32 = 0xf << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_SYS_PWRUP: u32 = 0x0 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_MBOX_DB: u32 = 0x1 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_M7_WDOG: u32 = 0x2 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_TEMP: u32 = 0x3 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_PCI_FLR: u32 = 0x4 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_PCI_HOT: u32 = 0x5 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_PCI_WARM: u32 = 0x6 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_PCI_COLD: u32 = 0x7 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_L1: u32 = 0x8 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_L0: u32 = 0x9 << VK_FWSTS_RESET_REASON_SHIFT;
pub const VK_FWSTS_RESET_UNKNOWN: u32 = 0xf << VK_FWSTS_RESET_REASON_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
