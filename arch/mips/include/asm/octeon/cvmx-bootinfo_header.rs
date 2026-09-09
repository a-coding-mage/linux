/*
 * Header file containing the ABI with the bootloader.
 *
 * Translated from cvmx-bootinfo.h.
 */

// Dependency supplied by the surrounding translation unit:
// use cvmx_coremask::cvmx_coremask;

pub const CVMX_BOOTINFO_MAJ_VER: u32 = 1;
pub const CVMX_BOOTINFO_MIN_VER: u32 = 4;
pub const CVMX_BOOTINFO_OCTEON_SERIAL_LEN: usize = 20;

pub const CVMX_BOOTINFO_CFG_FLAG_PCI_HOST: u64 = 1u64 << 0;
pub const CVMX_BOOTINFO_CFG_FLAG_PCI_TARGET: u64 = 1u64 << 1;
pub const CVMX_BOOTINFO_CFG_FLAG_DEBUG: u64 = 1u64 << 2;
pub const CVMX_BOOTINFO_CFG_FLAG_NO_MAGIC: u64 = 1u64 << 3;
/* This flag is set if the TLB mappings are outside the boot bus region. */
pub const CVMX_BOOTINFO_CFG_FLAG_OVERSIZE_TLB_MAPPING: u64 = 1u64 << 4;
pub const CVMX_BOOTINFO_CFG_FLAG_BREAK: u64 = 1u64 << 5;

/* The cvmx_coremask type is supplied by cvmx-coremask.h. */
#[cfg(target_endian = "big")]
#[repr(C)]
pub struct cvmx_bootinfo {
    pub major_version: u32,
    pub minor_version: u32,
    pub stack_top: u64,
    pub heap_base: u64,
    pub heap_end: u64,
    pub desc_vaddr: u64,
    pub exception_base_addr: u32,
    pub stack_size: u32,
    pub flags: u32,
    pub core_mask: u32,
    pub dram_size: u32,
    pub phy_mem_desc_addr: u32,
    pub debugger_flags_base_addr: u32,
    pub eclock_hz: u32,
    pub dclock_hz: u32,
    pub reserved0: u32,
    pub board_type: u16,
    pub board_rev_major: u8,
    pub board_rev_minor: u8,
    pub reserved1: u16,
    pub reserved2: u8,
    pub reserved3: u8,
    pub board_serial_number: [i8; CVMX_BOOTINFO_OCTEON_SERIAL_LEN],
    pub mac_addr_base: [u8; 6],
    pub mac_addr_count: u8,
    pub compact_flash_common_base_addr: u64,
    pub compact_flash_attribute_base_addr: u64,
    pub led_display_base_addr: u64,
    pub dfa_ref_clock_hz: u32,
    pub config_flags: u32,
    pub fdt_addr: u64,
    pub ext_core_mask: cvmx_coremask,
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct cvmx_bootinfo {
    pub minor_version: u32,
    pub major_version: u32,
    pub stack_top: u64,
    pub heap_base: u64,
    pub heap_end: u64,
    pub desc_vaddr: u64,
    pub stack_size: u32,
    pub exception_base_addr: u32,
    pub core_mask: u32,
    pub flags: u32,
    pub phy_mem_desc_addr: u32,
    pub dram_size: u32,
    pub eclock_hz: u32,
    pub debugger_flags_base_addr: u32,
    pub reserved0: u32,
    pub dclock_hz: u32,
    pub reserved3: u8,
    pub reserved2: u8,
    pub reserved1: u16,
    pub board_rev_minor: u8,
    pub board_rev_major: u8,
    pub board_type: u16,
    pub board_serial_number: [i8; CVMX_BOOTINFO_OCTEON_SERIAL_LEN],
    pub mac_addr_base: [u8; 6],
    pub mac_addr_count: u8,
    pub pad: [u8; 5],
    pub compact_flash_common_base_addr: u64,
    pub compact_flash_attribute_base_addr: u64,
    pub led_display_base_addr: u64,
    pub config_flags: u32,
    pub dfa_ref_clock_hz: u32,
    pub fdt_addr: u64,
    pub ext_core_mask: cvmx_coremask,
}

#[repr(i32)]
pub enum cvmx_board_types_enum {
    CVMX_BOARD_TYPE_NULL = 0, CVMX_BOARD_TYPE_SIM, CVMX_BOARD_TYPE_EBT3000,
    CVMX_BOARD_TYPE_KODAMA, CVMX_BOARD_TYPE_NIAGARA, CVMX_BOARD_TYPE_NAC38,
    CVMX_BOARD_TYPE_THUNDER, CVMX_BOARD_TYPE_TRANTOR, CVMX_BOARD_TYPE_EBH3000,
    CVMX_BOARD_TYPE_EBH3100, CVMX_BOARD_TYPE_HIKARI, CVMX_BOARD_TYPE_CN3010_EVB_HS5,
    CVMX_BOARD_TYPE_CN3005_EVB_HS5, CVMX_BOARD_TYPE_KBP, CVMX_BOARD_TYPE_CN3020_EVB_HS5,
    CVMX_BOARD_TYPE_EBT5800, CVMX_BOARD_TYPE_NICPRO2, CVMX_BOARD_TYPE_EBH5600,
    CVMX_BOARD_TYPE_EBH5601, CVMX_BOARD_TYPE_EBH5200, CVMX_BOARD_TYPE_BBGW_REF,
    CVMX_BOARD_TYPE_NIC_XLE_4G, CVMX_BOARD_TYPE_EBT5600, CVMX_BOARD_TYPE_EBH5201,
    CVMX_BOARD_TYPE_EBT5200, CVMX_BOARD_TYPE_CB5600, CVMX_BOARD_TYPE_CB5601,
    CVMX_BOARD_TYPE_CB5200, CVMX_BOARD_TYPE_GENERIC, CVMX_BOARD_TYPE_EBH5610,
    CVMX_BOARD_TYPE_LANAI2_A, CVMX_BOARD_TYPE_LANAI2_U, CVMX_BOARD_TYPE_EBB5600,
    CVMX_BOARD_TYPE_EBB6300, CVMX_BOARD_TYPE_NIC_XLE_10G, CVMX_BOARD_TYPE_LANAI2_G,
    CVMX_BOARD_TYPE_EBT5810, CVMX_BOARD_TYPE_NIC10E, CVMX_BOARD_TYPE_EP6300C,
    CVMX_BOARD_TYPE_EBB6800, CVMX_BOARD_TYPE_NIC4E, CVMX_BOARD_TYPE_NIC2E,
    CVMX_BOARD_TYPE_EBB6600, CVMX_BOARD_TYPE_REDWING, CVMX_BOARD_TYPE_NIC68_4,
    CVMX_BOARD_TYPE_NIC10E_66, CVMX_BOARD_TYPE_SNIC10E = 50,
    CVMX_BOARD_TYPE_MAX,
    CVMX_BOARD_TYPE_CUST_DEFINED_MIN = 10000, CVMX_BOARD_TYPE_CUST_WSX16,
    CVMX_BOARD_TYPE_CUST_NS0216, CVMX_BOARD_TYPE_CUST_NB5, CVMX_BOARD_TYPE_CUST_WMR500,
    CVMX_BOARD_TYPE_CUST_ITB101, CVMX_BOARD_TYPE_CUST_NTE102, CVMX_BOARD_TYPE_CUST_AGS103,
    CVMX_BOARD_TYPE_CUST_GST104, CVMX_BOARD_TYPE_CUST_GCT105, CVMX_BOARD_TYPE_CUST_AGS106,
    CVMX_BOARD_TYPE_CUST_SGM107, CVMX_BOARD_TYPE_CUST_GCT108, CVMX_BOARD_TYPE_CUST_AGS109,
    CVMX_BOARD_TYPE_CUST_GCT110, CVMX_BOARD_TYPE_CUST_L2_AIR_SENDER,
    CVMX_BOARD_TYPE_CUST_L2_AIR_RECEIVER, CVMX_BOARD_TYPE_CUST_L2_ACCTON2_TX,
    CVMX_BOARD_TYPE_CUST_L2_ACCTON2_RX, CVMX_BOARD_TYPE_CUST_L2_WSTRNSNIC_TX,
    CVMX_BOARD_TYPE_CUST_L2_WSTRNSNIC_RX, CVMX_BOARD_TYPE_CUST_L2_ZINWELL,
    CVMX_BOARD_TYPE_CUST_DEFINED_MAX = 20000, CVMX_BOARD_TYPE_CUST_PRIVATE_MIN,
    CVMX_BOARD_TYPE_UBNT_E100, CVMX_BOARD_TYPE_UBNT_E200,
    CVMX_BOARD_TYPE_UBNT_E220 = 20005, CVMX_BOARD_TYPE_CUST_DSR1000N,
    CVMX_BOARD_TYPE_UBNT_E300 = 20300, CVMX_BOARD_TYPE_KONTRON_S1901 = 21901,
    CVMX_BOARD_TYPE_CUST_PRIVATE_MAX = 30000,
}

pub const CVMX_BOARD_TYPE_NAO38: cvmx_board_types_enum = cvmx_board_types_enum::CVMX_BOARD_TYPE_NAC38;

#[repr(i32)]
pub enum cvmx_chip_types_enum {
    CVMX_CHIP_TYPE_NULL = 0,
    CVMX_CHIP_SIM_TYPE_DEPRECATED,
    CVMX_CHIP_TYPE_OCTEON_SAMPLE,
    CVMX_CHIP_TYPE_MAX,
}

pub unsafe fn cvmx_board_type_to_string(type_: cvmx_board_types_enum) -> *const i8 {
    let s = match type_ {
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NULL => "NULL\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_SIM => "SIM\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBT3000 => "EBT3000\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_KODAMA => "KODAMA\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIAGARA => "NIAGARA\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NAC38 => "NAC38\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_THUNDER => "THUNDER\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_TRANTOR => "TRANTOR\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH3000 => "EBH3000\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH3100 => "EBH3100\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_HIKARI => "HIKARI\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CN3010_EVB_HS5 => "CN3010_EVB_HS5\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CN3005_EVB_HS5 => "CN3005_EVB_HS5\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_KBP => "KBP\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CN3020_EVB_HS5 => "CN3020_EVB_HS5\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBT5800 => "EBT5800\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NICPRO2 => "NICPRO2\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH5600 => "EBH5600\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH5601 => "EBH5601\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH5200 => "EBH5200\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_BBGW_REF => "BBGW_REF\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC_XLE_4G => "NIC_XLE_4G\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBT5600 => "EBT5600\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH5201 => "EBH5201\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBT5200 => "EBT5200\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CB5600 => "CB5600\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CB5601 => "CB5601\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CB5200 => "CB5200\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_GENERIC => "GENERIC\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBH5610 => "EBH5610\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_LANAI2_A => "LANAI2_A\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_LANAI2_U => "LANAI2_U\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBB5600 => "EBB5600\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBB6300 => "EBB6300\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC_XLE_10G => "NIC_XLE_10G\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_LANAI2_G => "LANAI2_G\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBT5810 => "EBT5810\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC10E => "NIC10E\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EP6300C => "EP6300C\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBB6800 => "EBB6800\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC4E => "NIC4E\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC2E => "NIC2E\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_EBB6600 => "EBB6600\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_REDWING => "REDWING\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC68_4 => "NIC68_4\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_NIC10E_66 => "NIC10E_66\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_SNIC10E => "SNIC10E\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_MAX => "MAX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_DEFINED_MIN => "CUST_DEFINED_MIN\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_WSX16 => "CUST_WSX16\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_NS0216 => "CUST_NS0216\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_NB5 => "CUST_NB5\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_WMR500 => "CUST_WMR500\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_ITB101 => "CUST_ITB101\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_NTE102 => "CUST_NTE102\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_AGS103 => "CUST_AGS103\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_GST104 => "CUST_GST104\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_GCT105 => "CUST_GCT105\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_AGS106 => "CUST_AGS106\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_SGM107 => "CUST_SGM107\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_GCT108 => "CUST_GCT108\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_AGS109 => "CUST_AGS109\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_GCT110 => "CUST_GCT110\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_AIR_SENDER => "CUST_L2_AIR_SENDER\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_AIR_RECEIVER => "CUST_L2_AIR_RECEIVER\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_ACCTON2_TX => "CUST_L2_ACCTON2_TX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_ACCTON2_RX => "CUST_L2_ACCTON2_RX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_WSTRNSNIC_TX => "CUST_L2_WSTRNSNIC_TX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_WSTRNSNIC_RX => "CUST_L2_WSTRNSNIC_RX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_L2_ZINWELL => "CUST_L2_ZINWELL\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_DEFINED_MAX => "CUST_DEFINED_MAX\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_PRIVATE_MIN => "CUST_PRIVATE_MIN\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_UBNT_E100 => "UBNT_E100\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_UBNT_E200 => "UBNT_E200\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_UBNT_E220 => "UBNT_E220\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_DSR1000N => "CUST_DSR1000N\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_UBNT_E300 => "UBNT_E300\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_KONTRON_S1901 => "KONTRON_S1901\0",
        cvmx_board_types_enum::CVMX_BOARD_TYPE_CUST_PRIVATE_MAX => "CUST_PRIVATE_MAX\0",
        _ => return core::ptr::null(),
    };
    s.as_ptr() as *const i8
}

pub unsafe fn cvmx_chip_type_to_string(type_: cvmx_chip_types_enum) -> *const i8 {
    match type_ {
        cvmx_chip_types_enum::CVMX_CHIP_TYPE_NULL => "TYPE_NULL\0".as_ptr() as *const i8,
        cvmx_chip_types_enum::CVMX_CHIP_SIM_TYPE_DEPRECATED => "SIM_TYPE_DEPRECATED\0".as_ptr() as *const i8,
        cvmx_chip_types_enum::CVMX_CHIP_TYPE_OCTEON_SAMPLE => "OCTEON_SAMPLE\0".as_ptr() as *const i8,
        cvmx_chip_types_enum::CVMX_CHIP_TYPE_MAX => "MAX\0".as_ptr() as *const i8,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
