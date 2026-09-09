/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of arch/arm/include/asm/hardware/sa1111.h. */

/* Macro that calculates real address for registers in the SA-1111. */
pub const SA1111_SAC_DMA_MIN_XFER: u32 = 0x800;

pub const SA1111_SKCR: u32 = 0x0000;
pub const SA1111_SMCR: u32 = 0x0004;
pub const SA1111_SKID: u32 = 0x0008;

pub const SKCR_PLL_BYPASS: u32 = 1 << 0;
pub const SKCR_RCLKEN: u32 = 1 << 1;
pub const SKCR_SLEEP: u32 = 1 << 2;
pub const SKCR_DOZE: u32 = 1 << 3;
pub const SKCR_VCO_OFF: u32 = 1 << 4;
pub const SKCR_SCANTSTEN: u32 = 1 << 5;
pub const SKCR_CLKTSTEN: u32 = 1 << 6;
pub const SKCR_RDYEN: u32 = 1 << 7;
pub const SKCR_SELAC: u32 = 1 << 8;
pub const SKCR_OPPC: u32 = 1 << 9;
pub const SKCR_PLLTSTEN: u32 = 1 << 10;
pub const SKCR_USBIOTSTEN: u32 = 1 << 11;
/* Reality is bit 13, 1 to enable; see the original errata discussion. */
pub const SKCR_OE_EN: u32 = 1 << 13;

pub const SMCR_DTIM: u32 = 1 << 0;
pub const SMCR_MBGE: u32 = 1 << 1;
pub const SMCR_DRAC_0: u32 = 1 << 2;
pub const SMCR_DRAC_1: u32 = 1 << 3;
pub const SMCR_DRAC_2: u32 = 1 << 4;
/* Fld(3, 2) is supplied by the surrounding translated code. */
pub const SMCR_CLAT: u32 = 1 << 5;
pub const SKID_SIREV_MASK: u32 = 0x000000f0;
pub const SKID_MTREV_MASK: u32 = 0x0000000f;
pub const SKID_ID_MASK: u32 = 0xffffff00;
pub const SKID_SA1111_ID: u32 = 0x690cc200;

pub const SA1111_SKPCR: u32 = 0x0200;
pub const SA1111_SKCDR: u32 = 0x0204;
pub const SA1111_SKAUD: u32 = 0x0208;
pub const SA1111_SKPMC: u32 = 0x020c;
pub const SA1111_SKPTC: u32 = 0x0210;
pub const SA1111_SKPEN0: u32 = 0x0214;
pub const SA1111_SKPWM0: u32 = 0x0218;
pub const SA1111_SKPEN1: u32 = 0x021c;
pub const SA1111_SKPWM1: u32 = 0x0220;
pub const SKPCR_UCLKEN: u32 = 1 << 0;
pub const SKPCR_ACCLKEN: u32 = 1 << 1;
pub const SKPCR_I2SCLKEN: u32 = 1 << 2;
pub const SKPCR_L3CLKEN: u32 = 1 << 3;
pub const SKPCR_SCLKEN: u32 = 1 << 4;
pub const SKPCR_PMCLKEN: u32 = 1 << 5;
pub const SKPCR_PTCLKEN: u32 = 1 << 6;
pub const SKPCR_DCLKEN: u32 = 1 << 7;
pub const SKPCR_PWMCLKEN: u32 = 1 << 8;

pub const SA1111_USB: u32 = 0x0400;
pub const SA1111_SERAUDIO: u32 = 0x0600;
pub const SA1111_SACR0: u32 = 0x00;
pub const SA1111_SACR1: u32 = 0x04;
pub const SA1111_SACR2: u32 = 0x08;
pub const SA1111_SASR0: u32 = 0x0c;
pub const SA1111_SASR1: u32 = 0x10;
pub const SA1111_SASCR: u32 = 0x18;
pub const SA1111_L3_CAR: u32 = 0x1c;
pub const SA1111_L3_CDR: u32 = 0x20;
pub const SA1111_ACCAR: u32 = 0x24;
pub const SA1111_ACCDR: u32 = 0x28;
pub const SA1111_ACSAR: u32 = 0x2c;
pub const SA1111_ACSDR: u32 = 0x30;
pub const SA1111_SADTCS: u32 = 0x34;
pub const SA1111_SADTSA: u32 = 0x38;
pub const SA1111_SADTCA: u32 = 0x3c;
pub const SA1111_SADTSB: u32 = 0x40;
pub const SA1111_SADTCB: u32 = 0x44;
pub const SA1111_SADRCS: u32 = 0x48;
pub const SA1111_SADRSA: u32 = 0x4c;
pub const SA1111_SADRCA: u32 = 0x50;
pub const SA1111_SADRSB: u32 = 0x54;
pub const SA1111_SADRCB: u32 = 0x58;
pub const SA1111_SAITR: u32 = 0x5c;
pub const SA1111_SADR: u32 = 0x80;

/* The following serial-audio definitions are excluded for CONFIG_ARCH_PXA. */
#[cfg(not(feature = "CONFIG_ARCH_PXA"))]
pub mod serial_audio {
    pub const SACR0_ENB: u32 = 1 << 0; pub const SACR0_BCKD: u32 = 1 << 2; pub const SACR0_RST: u32 = 1 << 3;
    pub const SACR1_AMSL: u32 = 1 << 0; pub const SACR1_L3EN: u32 = 1 << 1; pub const SACR1_L3MB: u32 = 1 << 2; pub const SACR1_DREC: u32 = 1 << 3; pub const SACR1_DRPL: u32 = 1 << 4; pub const SACR1_ENLBF: u32 = 1 << 5;
    pub const SACR2_TS3V: u32 = 1 << 0; pub const SACR2_TS4V: u32 = 1 << 1; pub const SACR2_WKUP: u32 = 1 << 2; pub const SACR2_DREC: u32 = 1 << 3; pub const SACR2_DRPL: u32 = 1 << 4; pub const SACR2_ENLBF: u32 = 1 << 5; pub const SACR2_RESET: u32 = 1 << 6;
    pub const SASR0_TNF: u32 = 1 << 0; pub const SASR0_RNE: u32 = 1 << 1; pub const SASR0_BSY: u32 = 1 << 2; pub const SASR0_TFS: u32 = 1 << 3; pub const SASR0_RFS: u32 = 1 << 4; pub const SASR0_TUR: u32 = 1 << 5; pub const SASR0_ROR: u32 = 1 << 6; pub const SASR0_L3WD: u32 = 1 << 16; pub const SASR0_L3RD: u32 = 1 << 17;
    pub const SASR1_TNF: u32 = 1 << 0; pub const SASR1_RNE: u32 = 1 << 1; pub const SASR1_BSY: u32 = 1 << 2; pub const SASR1_TFS: u32 = 1 << 3; pub const SASR1_RFS: u32 = 1 << 4; pub const SASR1_TUR: u32 = 1 << 5; pub const SASR1_ROR: u32 = 1 << 6; pub const SASR1_CADT: u32 = 1 << 16; pub const SASR1_SADR: u32 = 1 << 17; pub const SASR1_RSTO: u32 = 1 << 18; pub const SASR1_CLPM: u32 = 1 << 19; pub const SASR1_CRDY: u32 = 1 << 20; pub const SASR1_RS3V: u32 = 1 << 21; pub const SASR1_RS4V: u32 = 1 << 22;
    pub const SASCR_TUR: u32 = 1 << 5; pub const SASCR_ROR: u32 = 1 << 6; pub const SASCR_DTS: u32 = 1 << 16; pub const SASCR_RDD: u32 = 1 << 17; pub const SASCR_STO: u32 = 1 << 18;
    pub const SADTCS_TDEN: u32 = 1 << 0; pub const SADTCS_TDIE: u32 = 1 << 1; pub const SADTCS_TDBDA: u32 = 1 << 3; pub const SADTCS_TDSTA: u32 = 1 << 4; pub const SADTCS_TDBDB: u32 = 1 << 5; pub const SADTCS_TDSTB: u32 = 1 << 6; pub const SADTCS_TBIU: u32 = 1 << 7;
    pub const SADRCS_RDEN: u32 = 1 << 0; pub const SADRCS_RDIE: u32 = 1 << 1; pub const SADRCS_RDBDA: u32 = 1 << 3; pub const SADRCS_RDSTA: u32 = 1 << 4; pub const SADRCS_RDBDB: u32 = 1 << 5; pub const SADRCS_RDSTB: u32 = 1 << 6; pub const SADRCS_RBIU: u32 = 1 << 7;
    pub const SAD_CS_DEN: u32 = 1 << 0; pub const SAD_CS_DIE: u32 = 1 << 1; pub const SAD_CS_DBDA: u32 = 1 << 3; pub const SAD_CS_DSTA: u32 = 1 << 4; pub const SAD_CS_DBDB: u32 = 1 << 5; pub const SAD_CS_DSTB: u32 = 1 << 6; pub const SAD_CS_BIU: u32 = 1 << 7;
    pub const SAITR_TFS: u32 = 1 << 0; pub const SAITR_RFS: u32 = 1 << 1; pub const SAITR_TUR: u32 = 1 << 2; pub const SAITR_ROR: u32 = 1 << 3; pub const SAITR_CADT: u32 = 1 << 4; pub const SAITR_SADR: u32 = 1 << 5; pub const SAITR_RSTO: u32 = 1 << 6; pub const SAITR_TDBDA: u32 = 1 << 8; pub const SAITR_TDBDB: u32 = 1 << 9; pub const SAITR_RDBDA: u32 = 1 << 10; pub const SAITR_RDBDB: u32 = 1 << 11;
}

pub const SA1111_GPIO: u32 = 0x1000;
pub const SA1111_GPIO_PADDR: u32 = 0x000; pub const SA1111_GPIO_PADRR: u32 = 0x004; pub const SA1111_GPIO_PADWR: u32 = 0x004; pub const SA1111_GPIO_PASDR: u32 = 0x008; pub const SA1111_GPIO_PASSR: u32 = 0x00c;
pub const SA1111_GPIO_PBDDR: u32 = 0x010; pub const SA1111_GPIO_PBDRR: u32 = 0x014; pub const SA1111_GPIO_PBDWR: u32 = 0x014; pub const SA1111_GPIO_PBSDR: u32 = 0x018; pub const SA1111_GPIO_PBSSR: u32 = 0x01c;
pub const SA1111_GPIO_PCDDR: u32 = 0x020; pub const SA1111_GPIO_PCDRR: u32 = 0x024; pub const SA1111_GPIO_PCDWR: u32 = 0x024; pub const SA1111_GPIO_PCSDR: u32 = 0x028; pub const SA1111_GPIO_PCSSR: u32 = 0x02c;
pub const GPIO_A0: u32 = 1 << 0; pub const GPIO_A1: u32 = 1 << 1; pub const GPIO_A2: u32 = 1 << 2; pub const GPIO_A3: u32 = 1 << 3;
pub const GPIO_B0: u32 = 1 << 8; pub const GPIO_B1: u32 = 1 << 9; pub const GPIO_B2: u32 = 1 << 10; pub const GPIO_B3: u32 = 1 << 11; pub const GPIO_B4: u32 = 1 << 12; pub const GPIO_B5: u32 = 1 << 13; pub const GPIO_B6: u32 = 1 << 14; pub const GPIO_B7: u32 = 1 << 15;
pub const GPIO_C0: u32 = 1 << 16; pub const GPIO_C1: u32 = 1 << 17; pub const GPIO_C2: u32 = 1 << 18; pub const GPIO_C3: u32 = 1 << 19; pub const GPIO_C4: u32 = 1 << 20; pub const GPIO_C5: u32 = 1 << 21; pub const GPIO_C6: u32 = 1 << 22; pub const GPIO_C7: u32 = 1 << 23;

pub const SA1111_INTC: u32 = 0x1600;
pub const SA1111_INTTEST0: u32 = 0x0000; pub const SA1111_INTTEST1: u32 = 0x0004; pub const SA1111_INTEN0: u32 = 0x0008; pub const SA1111_INTEN1: u32 = 0x000c; pub const SA1111_INTPOL0: u32 = 0x0010; pub const SA1111_INTPOL1: u32 = 0x0014; pub const SA1111_INTTSTSEL: u32 = 0x0018; pub const SA1111_INTSTATCLR0: u32 = 0x001c; pub const SA1111_INTSTATCLR1: u32 = 0x0020; pub const SA1111_INTSET0: u32 = 0x0024; pub const SA1111_INTSET1: u32 = 0x0028; pub const SA1111_WAKEEN0: u32 = 0x002c; pub const SA1111_WAKEEN1: u32 = 0x0030; pub const SA1111_WAKEPOL0: u32 = 0x0034; pub const SA1111_WAKEPOL1: u32 = 0x0038;
pub const SA1111_KBD: u32 = 0x0a00; pub const SA1111_MSE: u32 = 0x0c00; pub const SA1111_PCMCIA: u32 = 0x1600;

/* External kernel types and functions are supplied by other translated files. */
extern "C" {
    pub static sa1111_bus_type: bus_type;
    pub fn sa1111_enable_device(dev: *mut sa1111_dev) -> i32;
    pub fn sa1111_disable_device(dev: *mut sa1111_dev);
    pub fn sa1111_get_irq(dev: *mut sa1111_dev, num: u32) -> i32;
    pub fn sa1111_pll_clock(dev: *mut sa1111_dev) -> u32;
    pub fn sa1111_select_audio_mode(dev: *mut sa1111_dev, mode: i32);
    pub fn sa1111_set_audio_rate(dev: *mut sa1111_dev, rate: i32) -> i32;
    pub fn sa1111_get_audio_rate(dev: *mut sa1111_dev) -> i32;
    pub fn sa1111_check_dma_bug(addr: dma_addr_t) -> i32;
    pub fn sa1111_driver_register(driver: *mut sa1111_driver) -> i32;
    pub fn sa1111_driver_unregister(driver: *mut sa1111_driver);
}

#[repr(C)]
pub struct sa1111_dev {
    pub dev: device,
    pub devid: u32,
    pub res: resource,
    pub mapbase: *mut core::ffi::c_void,
    pub skpcr_mask: u32,
    pub hwirq: [u32; 6],
    pub dma_mask: u64,
}

#[repr(C)]
pub struct sa1111_driver {
    pub drv: device_driver,
    pub devid: u32,
    pub probe: Option<unsafe extern "C" fn(*mut sa1111_dev) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut sa1111_dev)>,
}

#[repr(C)]
pub struct sa1111_platform_data {
    pub irq_base: i32,
    pub disable_devs: u32,
    pub data: *mut core::ffi::c_void,
    pub enable: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32)>,
}

pub const SA1111_DEVID_SBI: u32 = 1 << 0; pub const SA1111_DEVID_SK: u32 = 1 << 1; pub const SA1111_DEVID_USB: u32 = 1 << 2; pub const SA1111_DEVID_SAC: u32 = 1 << 3; pub const SA1111_DEVID_SSP: u32 = 1 << 4; pub const SA1111_DEVID_PS2: u32 = 3 << 5; pub const SA1111_DEVID_PS2_KBD: u32 = 1 << 5; pub const SA1111_DEVID_PS2_MSE: u32 = 1 << 6; pub const SA1111_DEVID_GPIO: u32 = 1 << 7; pub const SA1111_DEVID_INT: u32 = 1 << 8; pub const SA1111_DEVID_PCMCIA: u32 = 1 << 9;
pub const SA1111_AUDIO_ACLINK: i32 = 0; pub const SA1111_AUDIO_I2S: i32 = 1;

/* C container_of/device-driver helper macros remain external-layout operations. */
#[macro_export] macro_rules! to_sa1111_device { ($x:expr) => { $x as *mut $crate::sa1111_dev }; }
#[macro_export] macro_rules! SA1111_DRV { ($x:expr) => { $x as *mut $crate::sa1111_driver }; }
pub type bus_type = core::ffi::c_void;
pub type device = core::ffi::c_void;
pub type resource = core::ffi::c_void;
pub type device_driver = core::ffi::c_void;
pub type dma_addr_t = usize;

#[macro_export]
macro_rules! sa1111_get_drvdata { ($d:expr) => { unsafe { core::ptr::read_volatile(($d).dev.driver as *const _ as *const core::ffi::c_void) } }; }
#[macro_export]
macro_rules! sa1111_set_drvdata { ($d:expr, $p:expr) => { (($d), ($p)) }; }
#[macro_export]
macro_rules! SA1111_DRIVER_NAME { ($sadev:expr) => { unsafe { (*(*$sadev).dev.driver).name } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
