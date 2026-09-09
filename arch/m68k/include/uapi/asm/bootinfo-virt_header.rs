/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * asm/bootinfo-virt.h -- Virtual-m68k-specific boot information definitions
 */

// C header guard: _UAPI_ASM_M68K_BOOTINFO_VIRT_H

pub const BI_VIRT_QEMU_VERSION: u16 = 0x8000;
pub const BI_VIRT_GF_PIC_BASE: u16 = 0x8001;
pub const BI_VIRT_GF_RTC_BASE: u16 = 0x8002;
pub const BI_VIRT_GF_TTY_BASE: u16 = 0x8003;
pub const BI_VIRT_VIRTIO_BASE: u16 = 0x8004;
pub const BI_VIRT_CTRL_BASE: u16 = 0x8005;

/* No longer used -- replaced with BI_RNG_SEED -- but don't reuse this index:
 * #define BI_VIRT_RNG_SEED 0x8006 */

pub const VIRT_BOOTI_VERSION: u32 = mk_bi_version(2, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
