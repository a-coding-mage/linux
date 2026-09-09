/* SPDX-License-Identifier: GPL-2.0 */
/* Intel OnChip System Fabric MailBox access support */

pub const MBI_MCR_OFFSET: u32 = 0xD0;
pub const MBI_MDR_OFFSET: u32 = 0xD4;
pub const MBI_MCRX_OFFSET: u32 = 0xD8;

pub const MBI_RD_MASK: u32 = 0xFEFFFFFF;
pub const MBI_WR_MASK: u32 = 0x01000000;

pub const MBI_MASK_HI: u32 = 0xFFFFFF00;
pub const MBI_MASK_LO: u32 = 0x000000FF;
pub const MBI_ENABLE: u32 = 0xF0;

/* IOSF SB read/write opcodes */
pub const MBI_MMIO_READ: u32 = 0x00;
pub const MBI_MMIO_WRITE: u32 = 0x01;
pub const MBI_CFG_READ: u32 = 0x04;
pub const MBI_CFG_WRITE: u32 = 0x05;
pub const MBI_CR_READ: u32 = 0x06;
pub const MBI_CR_WRITE: u32 = 0x07;
pub const MBI_REG_READ: u32 = 0x10;
pub const MBI_REG_WRITE: u32 = 0x11;
pub const MBI_ESRAM_READ: u32 = 0x12;
pub const MBI_ESRAM_WRITE: u32 = 0x13;

/* Baytrail available units */
pub const BT_MBI_UNIT_AUNIT: u32 = 0x00;
pub const BT_MBI_UNIT_SMC: u32 = 0x01;
pub const BT_MBI_UNIT_CPU: u32 = 0x02;
pub const BT_MBI_UNIT_BUNIT: u32 = 0x03;
pub const BT_MBI_UNIT_PMC: u32 = 0x04;
pub const BT_MBI_UNIT_GFX: u32 = 0x06;
pub const BT_MBI_UNIT_SMI: u32 = 0x0C;
pub const BT_MBI_UNIT_CCK: u32 = 0x14;
pub const BT_MBI_UNIT_USB: u32 = 0x43;
pub const BT_MBI_UNIT_SATA: u32 = 0xA3;
pub const BT_MBI_UNIT_PCIE: u32 = 0xA6;

/* Quark available units */
pub const QRK_MBI_UNIT_HBA: u32 = 0x00;
pub const QRK_MBI_UNIT_HB: u32 = 0x03;
pub const QRK_MBI_UNIT_RMU: u32 = 0x04;
pub const QRK_MBI_UNIT_MM: u32 = 0x05;
pub const QRK_MBI_UNIT_SOC: u32 = 0x31;

/* Action values for the pmic_bus_access_notifier functions */
pub const MBI_PMIC_BUS_ACCESS_BEGIN: i32 = 1;
pub const MBI_PMIC_BUS_ACCESS_END: i32 = 2;

/* CONFIG_IOSF_MBI is a build-time configuration condition. */
#[cfg(feature = "CONFIG_IOSF_MBI")]
extern "C" {
    pub fn iosf_mbi_available() -> bool;
    pub fn iosf_mbi_read(port: u8, opcode: u8, offset: u32, mdr: *mut u32) -> i32;
    pub fn iosf_mbi_write(port: u8, opcode: u8, offset: u32, mdr: u32) -> i32;
    pub fn iosf_mbi_modify(port: u8, opcode: u8, offset: u32, mdr: u32, mask: u32) -> i32;
    pub fn iosf_mbi_punit_acquire();
    pub fn iosf_mbi_punit_release();
    pub fn iosf_mbi_block_punit_i2c_access() -> i32;
    pub fn iosf_mbi_unblock_punit_i2c_access();
    pub fn iosf_mbi_register_pmic_bus_access_notifier(nb: *mut notifier_block) -> i32;
    pub fn iosf_mbi_unregister_pmic_bus_access_notifier_unlocked(nb: *mut notifier_block) -> i32;
    pub fn iosf_mbi_assert_punit_acquired();
}

extern "C" {
    pub type notifier_block;
}

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_available() -> bool { false }

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub unsafe fn iosf_mbi_read(_port: u8, _opcode: u8, _offset: u32, _mdr: *mut u32) -> i32 {
    -1
}

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_write(_port: u8, _opcode: u8, _offset: u32, _mdr: u32) -> i32 { -1 }

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_modify(_port: u8, _opcode: u8, _offset: u32, _mdr: u32, _mask: u32) -> i32 { -1 }

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_punit_acquire() {}
#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_punit_release() {}

#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub unsafe fn iosf_mbi_register_pmic_bus_access_notifier(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub unsafe fn iosf_mbi_unregister_pmic_bus_access_notifier(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub unsafe fn iosf_mbi_unregister_pmic_bus_access_notifier_unlocked(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub unsafe fn iosf_mbi_call_pmic_bus_access_notifier_chain(_val: usize, _v: *mut core::ffi::c_void) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_IOSF_MBI"))]
pub fn iosf_mbi_assert_punit_acquired() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
