/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file describes the structure passed from the BootX application
 * (for MacOS) when it is used to boot Linux.
 *
 * Written by Benjamin Herrenschmidt.
 */

/* The original header includes Linux types and, under `macintosh`, MacOS
 * definitions and PowerPC alignment directives. */

/* On kernel entry:
 *
 * r3 = 0x426f6f58    ('BooX')
 * r4 = pointer to boot_infos
 * r5 = NULL
 *
 * Data and instruction translation disabled, interrupts
 * disabled, kernel loaded at physical 0x00000000 on PCI
 * machines (will be different on NuBus).
 */

pub const BOOT_INFO_VERSION: u32 = 5;
pub const BOOT_INFO_COMPATIBLE_VERSION: u32 = 1;

/* Bit in the architecture flag mask. More to be defined in
   future versions. Note that either BOOT_ARCH_PCI or
   BOOT_ARCH_NUBUS is set. The other BOOT_ARCH_NUBUS_xxx are
   set additionally when BOOT_ARCH_NUBUS is set.
 */
pub const BOOT_ARCH_PCI: usize = 0x00000001;
pub const BOOT_ARCH_NUBUS: usize = 0x00000002;
pub const BOOT_ARCH_NUBUS_PDM: usize = 0x00000010;
pub const BOOT_ARCH_NUBUS_PERFORMA: usize = 0x00000020;
pub const BOOT_ARCH_NUBUS_POWERBOOK: usize = 0x00000040;

/* Maximum number of ranges in phys memory map */
pub const MAX_MEM_MAP_SIZE: usize = 26;

/* This is the format of an element in the physical memory map. Note that
   the map is optional and current BootX will only build it for pre-PCI
   machines */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_info_map_entry_t {
    pub physAddr: u32, /* Physical starting address */
    pub size: u32,     /* Size in bytes */
}

/* Here are the boot informations that are passed to the bootstrap
 * Note that the kernel arguments and the device tree are appended
 * at the end of this structure. */
#[repr(C)]
pub struct boot_infos_t {
    /* Version of this structure */
    pub version: u32,
    /* backward compatible down to version: */
    pub compatible_version: u32,

    /* NEW (vers. 2) this holds the current _logical_ base addr of
       the frame buffer (for use by early boot message) */
    pub logicalDisplayBase: *mut u8,

    /* NEW (vers. 4) Apple's machine identification */
    pub machineID: u32,

    /* NEW (vers. 4) Detected hw architecture */
    pub architecture: u32,

    /* The device tree (internal addresses relative to the beginning of the tree,
     * device tree offset relative to the beginning of this structure).
     * On pre-PCI macintosh (BOOT_ARCH_PCI bit set to 0 in architecture), this
     * field is 0.
     */
    pub deviceTreeOffset: u32, /* Device tree offset */
    pub deviceTreeSize: u32,   /* Size of the device tree */

    /* Some infos about the current MacOS display */
    pub dispDeviceRect: [u32; 4], /* left,top,right,bottom */
    pub dispDeviceDepth: u32,     /* (8, 16 or 32) */
    pub dispDeviceBase: *mut u8,  /* base address (physical) */
    pub dispDeviceRowBytes: u32,  /* rowbytes (in bytes) */
    pub dispDeviceColorsOffset: u32, /* Colormap (8 bits only) or 0 (*) */
    /* Optional offset in the registry to the current
     * MacOS display. (Can be 0 when not detected) */
    pub dispDeviceRegEntryOffset: u32,

    /* Optional pointer to boot ramdisk (offset from this structure) */
    pub ramDisk: u32,
    pub ramDiskSize: u32, /* size of ramdisk image */

    /* Kernel command line arguments (offset from this structure) */
    pub kernelParamsOffset: u32,

    /* ALL BELOW NEW (vers. 4) */

    /* This defines the physical memory. Valid with BOOT_ARCH_NUBUS flag
       (non-PCI) only. On PCI, memory is contiguous and its size is in the
       device-tree. */
    pub physMemoryMap: [boot_info_map_entry_t; MAX_MEM_MAP_SIZE], /* Where the phys memory is */
    pub physMemoryMapSize: u32, /* How many entries in map */

    /* The framebuffer size (optional, currently 0) */
    pub frameBufferSize: u32, /* Represents a max size, can be 0. */

    /* NEW (vers. 5) */

    /* Total params size (args + colormap + device tree + ramdisk) */
    pub totalParamsSize: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
