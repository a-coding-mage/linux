/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* setup_data/setup_indirect types */
pub const SETUP_NONE: u32 = 0;
pub const SETUP_E820_EXT: u32 = 1;
pub const SETUP_DTB: u32 = 2;
pub const SETUP_PCI: u32 = 3;
pub const SETUP_EFI: u32 = 4;
pub const SETUP_APPLE_PROPERTIES: u32 = 5;
pub const SETUP_JAILHOUSE: u32 = 6;
pub const SETUP_CC_BLOB: u32 = 7;
pub const SETUP_IMA: u32 = 8;
pub const SETUP_RNG_SEED: u32 = 9;
pub const SETUP_KEXEC_KHO: u32 = 10;
pub const SETUP_ENUM_MAX: u32 = SETUP_KEXEC_KHO;

pub const SETUP_INDIRECT: u32 = 1u32 << 31;
pub const SETUP_TYPE_MAX: u32 = SETUP_ENUM_MAX | SETUP_INDIRECT;

/* extensible setup data list node */
#[repr(C)]
pub struct setup_data {
    pub next: u64,
    pub type_: u32,
    pub len: u32,
    pub data: [u8; 0],
}

/* extensible setup indirect data node */
#[repr(C)]
pub struct setup_indirect {
    pub type_: u32,
    pub reserved: u32, /* Reserved, must be set to zero. */
    pub len: u64,
    pub addr: u64,
}

/*
 * The E820 memory region entry of the boot protocol ABI:
 */
#[repr(C, packed)]
pub struct boot_e820_entry {
    pub addr: u64,
    pub size: u64,
    pub type_: u32,
}

/*
 * The boot loader is passing platform information via this Jailhouse-specific
 * setup data structure.
 */
#[repr(C, packed)]
pub struct jailhouse_setup_data {
    pub hdr: jailhouse_setup_data_hdr,
    pub v1: jailhouse_setup_data_v1,
    pub v2: jailhouse_setup_data_v2,
}

#[repr(C, packed)]
pub struct jailhouse_setup_data_hdr {
    pub version: u16,
    pub compatible_version: u16,
}

#[repr(C, packed)]
pub struct jailhouse_setup_data_v1 {
    pub pm_timer_address: u16,
    pub num_cpus: u16,
    pub pci_mmconfig_base: u64,
    pub tsc_khz: u32,
    pub apic_khz: u32,
    pub standard_ioapic: u8,
    pub cpu_ids: [u8; 255],
}

#[repr(C, packed)]
pub struct jailhouse_setup_data_v2 {
    pub flags: u32,
}

/*
 * IMA buffer setup data information from the previous kernel during kexec
 */
#[repr(C, packed)]
pub struct ima_setup_data {
    pub addr: u64,
    pub size: u64,
}

/*
 * Locations of kexec handover metadata
 */
#[repr(C, packed)]
pub struct kho_data {
    pub fdt_addr: u64,
    pub fdt_size: u64,
    pub scratch_addr: u64,
    pub scratch_size: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
