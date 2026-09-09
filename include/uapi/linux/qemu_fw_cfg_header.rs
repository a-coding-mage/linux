/* SPDX-License-Identifier: BSD-3-Clause */

// Dependency: types such as __be16, __be32, __be64, __le16, __le32,
// __le64, and __u16 are supplied by the translated Linux type definitions.

pub const FW_CFG_ACPI_DEVICE_ID: &str = "QEMU0002";

/* selector key values for "well-known" fw_cfg entries */
pub const FW_CFG_SIGNATURE: u16 = 0x00;
pub const FW_CFG_ID: u16 = 0x01;
pub const FW_CFG_UUID: u16 = 0x02;
pub const FW_CFG_RAM_SIZE: u16 = 0x03;
pub const FW_CFG_NOGRAPHIC: u16 = 0x04;
pub const FW_CFG_NB_CPUS: u16 = 0x05;
pub const FW_CFG_MACHINE_ID: u16 = 0x06;
pub const FW_CFG_KERNEL_ADDR: u16 = 0x07;
pub const FW_CFG_KERNEL_SIZE: u16 = 0x08;
pub const FW_CFG_KERNEL_CMDLINE: u16 = 0x09;
pub const FW_CFG_INITRD_ADDR: u16 = 0x0a;
pub const FW_CFG_INITRD_SIZE: u16 = 0x0b;
pub const FW_CFG_BOOT_DEVICE: u16 = 0x0c;
pub const FW_CFG_NUMA: u16 = 0x0d;
pub const FW_CFG_BOOT_MENU: u16 = 0x0e;
pub const FW_CFG_MAX_CPUS: u16 = 0x0f;
pub const FW_CFG_KERNEL_ENTRY: u16 = 0x10;
pub const FW_CFG_KERNEL_DATA: u16 = 0x11;
pub const FW_CFG_INITRD_DATA: u16 = 0x12;
pub const FW_CFG_CMDLINE_ADDR: u16 = 0x13;
pub const FW_CFG_CMDLINE_SIZE: u16 = 0x14;
pub const FW_CFG_CMDLINE_DATA: u16 = 0x15;
pub const FW_CFG_SETUP_ADDR: u16 = 0x16;
pub const FW_CFG_SETUP_SIZE: u16 = 0x17;
pub const FW_CFG_SETUP_DATA: u16 = 0x18;
pub const FW_CFG_FILE_DIR: u16 = 0x19;

pub const FW_CFG_FILE_FIRST: u16 = 0x20;
pub const FW_CFG_FILE_SLOTS_MIN: u16 = 0x10;

pub const FW_CFG_WRITE_CHANNEL: u16 = 0x4000;
pub const FW_CFG_ARCH_LOCAL: u16 = 0x8000;
pub const FW_CFG_ENTRY_MASK: u16 = !(FW_CFG_WRITE_CHANNEL | FW_CFG_ARCH_LOCAL);

pub const FW_CFG_INVALID: u16 = 0xffff;

/* width in bytes of fw_cfg control register */
pub const FW_CFG_CTL_SIZE: u16 = 0x02;

/* fw_cfg "file name" is up to 56 characters (including terminating nul) */
pub const FW_CFG_MAX_FILE_PATH: usize = 56;

/* size in bytes of fw_cfg signature */
pub const FW_CFG_SIG_SIZE: usize = 4;

/* FW_CFG_ID bits */
pub const FW_CFG_VERSION: u16 = 0x01;
pub const FW_CFG_VERSION_DMA: u16 = 0x02;

#[repr(C)]
pub struct fw_cfg_file {
    pub size: __be32,
    pub select: __be16,
    pub reserved: __u16,
    pub name: [::core::ffi::c_char; FW_CFG_MAX_FILE_PATH],
}

/* FW_CFG_DMA_CONTROL bits */
pub const FW_CFG_DMA_CTL_ERROR: u32 = 0x01;
pub const FW_CFG_DMA_CTL_READ: u32 = 0x02;
pub const FW_CFG_DMA_CTL_SKIP: u32 = 0x04;
pub const FW_CFG_DMA_CTL_SELECT: u32 = 0x08;
pub const FW_CFG_DMA_CTL_WRITE: u32 = 0x10;

pub const FW_CFG_DMA_SIGNATURE: u64 = 0x51454d5520434647; /* "QEMU CFG" */

/* Control as first field allows for different structures selected by this
 * field, which might be useful in the future
 */
#[repr(C)]
pub struct fw_cfg_dma_access {
    pub control: __be32,
    pub length: __be32,
    pub address: __be64,
}

pub const FW_CFG_VMCOREINFO_FILENAME: &str = "etc/vmcoreinfo";

pub const FW_CFG_VMCOREINFO_FORMAT_NONE: u32 = 0x0;
pub const FW_CFG_VMCOREINFO_FORMAT_ELF: u32 = 0x1;

#[repr(C)]
pub struct fw_cfg_vmcoreinfo {
    pub host_format: __le16,
    pub guest_format: __le16,
    pub size: __le32,
    pub paddr: __le64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
