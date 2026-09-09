/* Translated from bestcomm_priv.h. Dependencies are supplied externally. */

pub const BCOM_MAX_TASKS: usize = 16;
pub const BCOM_MAX_VAR: usize = 24;
pub const BCOM_MAX_INC: usize = 8;
pub const BCOM_MAX_FDT: usize = 64;
pub const BCOM_MAX_CTX: usize = 20;
pub const BCOM_CTX_SIZE: usize = BCOM_MAX_CTX * core::mem::size_of::<u32>();
pub const BCOM_CTX_ALIGN: usize = 0x100;
pub const BCOM_VAR_SIZE: usize = BCOM_MAX_VAR * core::mem::size_of::<u32>();
pub const BCOM_INC_SIZE: usize = BCOM_MAX_INC * core::mem::size_of::<u32>();
pub const BCOM_VAR_ALIGN: usize = 0x80;
pub const BCOM_FDT_SIZE: usize = BCOM_MAX_FDT * core::mem::size_of::<u32>();
pub const BCOM_FDT_ALIGN: usize = 0x100;

#[repr(C)]
pub struct bcom_tdt {
    pub start: u32, pub stop: u32, pub var: u32, pub fdt: u32,
    pub exec_status: u32, pub mvtp: u32, pub context: u32, pub litbase: u32,
}

#[repr(C)]
pub struct bcom_engine {
    pub ofnode: *mut device_node,
    pub regs: *mut mpc52xx_sdma,
    pub regs_base: phys_addr_t,
    pub tdt: *mut bcom_tdt,
    pub ctx: *mut u32,
    pub var: *mut u32,
    pub fdt: *mut u32,
    pub lock: spinlock_t,
}

extern "C" { pub static mut bcom_eng: *mut bcom_engine; }

pub const BCOM_TASK_MAGIC: u32 = 0x4243544b;
#[repr(C)]
pub struct bcom_task_header {
    pub magic: u32, pub desc_size: u8, pub var_size: u8, pub inc_size: u8,
    pub first_var: u8, pub reserved: [u8; 8],
}

pub const BCOM_DESC_NOP: u32 = 0x000001f8;
pub const BCOM_LCD_MASK: u32 = 0x80000000;
pub const BCOM_DRD_EXTENDED: u32 = 0x40000000;
pub const BCOM_DRD_INITIATOR_SHIFT: u32 = 21;
pub const BCOM_PRAGMA_BIT_RSV: u32 = 7;
pub const BCOM_PRAGMA_BIT_PRECISE_INC: u32 = 6;
pub const BCOM_PRAGMA_BIT_RST_ERROR_NO: u32 = 5;
pub const BCOM_PRAGMA_BIT_PACK: u32 = 4;
pub const BCOM_PRAGMA_BIT_INTEGER: u32 = 3;
pub const BCOM_PRAGMA_BIT_SPECREAD: u32 = 2;
pub const BCOM_PRAGMA_BIT_CW: u32 = 1;
pub const BCOM_PRAGMA_BIT_RL: u32 = 0;
/* XLB speculative reads can generate errors at the end of physical memory. */
pub const BCOM_STD_PRAGMA: u32 = (1 << BCOM_PRAGMA_BIT_CW) | (1 << BCOM_PRAGMA_BIT_RL);
pub const BCOM_PCI_PRAGMA: u32 = (1 << BCOM_PRAGMA_BIT_INTEGER) | (1 << BCOM_PRAGMA_BIT_CW) | (1 << BCOM_PRAGMA_BIT_RL);
+pub const BCOM_ATA_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_CRC16_DP_0_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_CRC16_DP_1_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_FEC_RX_BD_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_FEC_TX_BD_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_0_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_1_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_2_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_3_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_BD_0_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_DP_BD_1_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_RX_BD_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_TX_BD_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_GEN_LPC_PRAGMA: u32 = BCOM_STD_PRAGMA;
pub const BCOM_PCI_RX_PRAGMA: u32 = BCOM_PCI_PRAGMA;
pub const BCOM_PCI_TX_PRAGMA: u32 = BCOM_PCI_PRAGMA;
pub const BCOM_INITIATOR_ALWAYS: i32 = 0;
pub const BCOM_INITIATOR_SCTMR_0: i32 = 1;
pub const BCOM_INITIATOR_SCTMR_1: i32 = 2;
pub const BCOM_INITIATOR_FEC_RX: i32 = 3;
pub const BCOM_INITIATOR_FEC_TX: i32 = 4;
pub const BCOM_INITIATOR_ATA_RX: i32 = 5;
pub const BCOM_INITIATOR_ATA_TX: i32 = 6;
pub const BCOM_INITIATOR_SCPCI_RX: i32 = 7;
pub const BCOM_INITIATOR_SCPCI_TX: i32 = 8;
pub const BCOM_INITIATOR_PSC3_RX: i32 = 9;
pub const BCOM_INITIATOR_PSC3_TX: i32 = 10;
pub const BCOM_INITIATOR_PSC2_RX: i32 = 11;
pub const BCOM_INITIATOR_PSC2_TX: i32 = 12;
pub const BCOM_INITIATOR_PSC1_RX: i32 = 13;
pub const BCOM_INITIATOR_PSC1_TX: i32 = 14;
pub const BCOM_INITIATOR_SCTMR_2: i32 = 15;
pub const BCOM_INITIATOR_SCLPC: i32 = 16;
pub const BCOM_INITIATOR_PSC5_RX: i32 = 17;
pub const BCOM_INITIATOR_PSC5_TX: i32 = 18;
pub const BCOM_INITIATOR_PSC4_RX: i32 = 19;
pub const BCOM_INITIATOR_PSC4_TX: i32 = 20;
pub const BCOM_INITIATOR_I2C2_RX: i32 = 21;
pub const BCOM_INITIATOR_I2C2_TX: i32 = 22;
pub const BCOM_INITIATOR_I2C1_RX: i32 = 23;
pub const BCOM_INITIATOR_I2C1_TX: i32 = 24;
pub const BCOM_INITIATOR_PSC6_RX: i32 = 25;
pub const BCOM_INITIATOR_PSC6_TX: i32 = 26;
pub const BCOM_INITIATOR_IRDA_RX: i32 = 25;
pub const BCOM_INITIATOR_IRDA_TX: i32 = 26;
pub const BCOM_INITIATOR_SCTMR_3: i32 = 27;
pub const BCOM_INITIATOR_SCTMR_4: i32 = 28;
pub const BCOM_INITIATOR_SCTMR_5: i32 = 29;
pub const BCOM_INITIATOR_SCTMR_6: i32 = 30;
pub const BCOM_INITIATOR_SCTMR_7: i32 = 31;

pub const BCOM_IPR_ALWAYS: i32 = 7;
pub const BCOM_IPR_FEC_RX: i32 = 6;
pub const BCOM_IPR_FEC_TX: i32 = 5;
pub const BCOM_IPR_ATA_RX: i32 = 7;
pub const BCOM_IPR_ATA_TX: i32 = 7;
+pub const BCOM_IPR_SCTMR_0: i32 = 2;
pub const BCOM_IPR_SCTMR_1: i32 = 2;
pub const BCOM_IPR_SCPCI_RX: i32 = 2;
pub const BCOM_IPR_SCPCI_TX: i32 = 2;
pub const BCOM_IPR_PSC3_RX: i32 = 2;
pub const BCOM_IPR_PSC3_TX: i32 = 2;
pub const BCOM_IPR_PSC2_RX: i32 = 2;
pub const BCOM_IPR_PSC2_TX: i32 = 2;
pub const BCOM_IPR_PSC1_RX: i32 = 2;
pub const BCOM_IPR_PSC1_TX: i32 = 2;
pub const BCOM_IPR_SCTMR_2: i32 = 2;
pub const BCOM_IPR_SCLPC: i32 = 2;
pub const BCOM_IPR_PSC5_RX: i32 = 2;
pub const BCOM_IPR_PSC5_TX: i32 = 2;
pub const BCOM_IPR_PSC4_RX: i32 = 2;
pub const BCOM_IPR_PSC4_TX: i32 = 2;
pub const BCOM_IPR_I2C2_RX: i32 = 2;
pub const BCOM_IPR_I2C2_TX: i32 = 2;
pub const BCOM_IPR_I2C1_RX: i32 = 2;
pub const BCOM_IPR_I2C1_TX: i32 = 2;
pub const BCOM_IPR_PSC6_RX: i32 = 2;
pub const BCOM_IPR_PSC6_TX: i32 = 2;
pub const BCOM_IPR_IRDA_RX: i32 = 2;
pub const BCOM_IPR_IRDA_TX: i32 = 2;
pub const BCOM_IPR_SCTMR_3: i32 = 2;
pub const BCOM_IPR_SCTMR_4: i32 = 2;
pub const BCOM_IPR_SCTMR_5: i32 = 2;
pub const BCOM_IPR_SCTMR_6: i32 = 2;
pub const BCOM_IPR_SCTMR_7: i32 = 2;

extern "C" {
    pub fn bcom_task_alloc(bd_count: core::ffi::c_int, bd_size: core::ffi::c_int, priv_size: core::ffi::c_int) -> *mut bcom_task;
    pub fn bcom_task_free(tsk: *mut bcom_task);
    pub fn bcom_load_image(task: core::ffi::c_int, task_image: *mut u32) -> core::ffi::c_int;
    pub fn bcom_set_initiator(task: core::ffi::c_int, initiator: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
