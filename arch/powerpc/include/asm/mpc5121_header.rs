/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MPC5121 Prototypes and definitions
 */

/* MPC512x Reset module registers */
#[repr(C)]
pub struct mpc512x_reset_module {
    pub rcwlr: u32, /* Reset Configuration Word Low Register */
    pub rcwhr: u32, /* Reset Configuration Word High Register */
    pub reserved1: u32,
    pub reserved2: u32,
    pub rsr: u32, /* Reset Status Register */
    pub rmr: u32, /* Reset Mode Register */
    pub rpr: u32, /* Reset Protection Register */
    pub rcr: u32, /* Reset Control Register */
    pub rcer: u32, /* Reset Control Enable Register */
}

/*
 * Clock Control Module
 */
#[repr(C)]
pub struct mpc512x_ccm {
    pub spmr: u32, /* System PLL Mode Register */
    pub sccr1: u32, /* System Clock Control Register 1 */
    pub sccr2: u32, /* System Clock Control Register 2 */
    pub scfr1: u32, /* System Clock Frequency Register 1 */
    pub scfr2: u32, /* System Clock Frequency Register 2 */
    pub scfr2s: u32, /* System Clock Frequency Shadow Register 2 */
    pub bcr: u32, /* Bread Crumb Register */
    pub psc_ccr: [u32; 12], /* PSC Clock Control Registers */
    pub spccr: u32, /* SPDIF Clock Control Register */
    pub cccr: u32, /* CFM Clock Control Register */
    pub dccr: u32, /* DIU Clock Control Register */
    pub mscan_ccr: [u32; 4], /* MSCAN Clock Control Registers */
    pub out_ccr: [u32; 4], /* OUT CLK Configure Registers */
    pub rsv0: [u32; 2], /* Reserved */
    pub scfr3: u32, /* System Clock Frequency Register 3 */
    pub rsv1: [u32; 3], /* Reserved */
    pub spll_lock_cnt: u32, /* System PLL Lock Counter */
    pub res: [u8; 0x6c], /* Reserved */
}

unsafe extern "C" {
    pub fn mpc512x_cs_config(cs: core::ffi::c_uint, val: u32) -> core::ffi::c_int;
}

/*
 * LPC Module
 */
#[repr(C)]
pub struct mpc512x_lpc {
    pub cs_cfg: [u32; 8], /* CS config */
    pub cs_ctrl: u32, /* CS Control Register */
    pub cs_status: u32, /* CS Status Register */
    pub burst_ctrl: u32, /* CS Burst Control Register */
    pub deadcycle_ctrl: u32, /* CS Deadcycle Control Register */
    pub holdcycle_ctrl: u32, /* CS Holdcycle Control Register */
    pub alt: u32, /* Address Latch Timing Register */
}

/*
 * SCLPC Module (LPB FIFO)
 */
#[repr(C)]
pub struct mpc512x_lpbfifo {
    pub pkt_size: u32, /* SCLPC Packet Size Register */
    pub start_addr: u32, /* SCLPC Start Address Register */
    pub ctrl: u32, /* SCLPC Control Register */
    pub enable: u32, /* SCLPC Enable Register */
    pub reserved1: u32,
    pub status: u32, /* SCLPC Status Register */
    pub bytes_done: u32, /* SCLPC Bytes Done Register */
    pub emb_sc: u32, /* EMB Share Counter Register */
    pub emb_pc: u32, /* EMB Pause Control Register */
    pub reserved2: [u32; 7],
    pub data_word: u32, /* LPC RX/TX FIFO Data Word Register */
    pub fifo_status: u32, /* LPC RX/TX FIFO Status Register */
    pub fifo_ctrl: u32, /* LPC RX/TX FIFO Control Register */
    pub fifo_alarm: u32, /* LPC RX/TX FIFO Alarm Register */
}

pub const MPC512X_SCLPC_START: u32 = 1 << 31;
#[inline] pub const fn MPC512X_SCLPC_CS(x: u32) -> u32 { (x & 0x7) << 24 }
pub const MPC512X_SCLPC_FLUSH: u32 = 1 << 17;
pub const MPC512X_SCLPC_READ: u32 = 1 << 16;
pub const MPC512X_SCLPC_DAI: u32 = 1 << 8;
#[inline] pub const fn MPC512X_SCLPC_BPT(x: u32) -> u32 { x & 0x3f }
pub const MPC512X_SCLPC_RESET: u32 = 1 << 24;
pub const MPC512X_SCLPC_FIFO_RESET: u32 = 1 << 16;
pub const MPC512X_SCLPC_ABORT_INT_ENABLE: u32 = 1 << 9;
pub const MPC512X_SCLPC_NORM_INT_ENABLE: u32 = 1 << 8;
pub const MPC512X_SCLPC_ENABLE: u32 = 1 << 0;
pub const MPC512X_SCLPC_SUCCESS: u32 = 1 << 24;
#[inline] pub const fn MPC512X_SCLPC_FIFO_CTRL(x: u32) -> u32 { (x & 0x7) << 24 }
#[inline] pub const fn MPC512X_SCLPC_FIFO_ALARM(x: u32) -> u32 { x & 0x3ff }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lpb_dev_portsize {
    LPB_DEV_PORTSIZE_UNDEFINED = 0,
    LPB_DEV_PORTSIZE_1_BYTE = 1,
    LPB_DEV_PORTSIZE_2_BYTES = 2,
    LPB_DEV_PORTSIZE_4_BYTES = 4,
    LPB_DEV_PORTSIZE_8_BYTES = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mpc512x_lpbfifo_req_dir {
    MPC512X_LPBFIFO_REQ_DIR_READ,
    MPC512X_LPBFIFO_REQ_DIR_WRITE,
}

#[repr(C)]
pub struct mpc512x_lpbfifo_request {
    pub dev_phys_addr: phys_addr_t, /* physical address of some device on LPB */
    pub ram_virt_addr: *mut core::ffi::c_void, /* virtual address of some region in RAM */
    pub size: u32,
    pub portsize: lpb_dev_portsize,
    pub dir: mpc512x_lpbfifo_req_dir,
    pub callback: Option<unsafe extern "C" fn(*mut mpc512x_lpbfifo_request)>,
}

unsafe extern "C" {
    pub fn mpc512x_lpbfifo_submit(req: *mut mpc512x_lpbfifo_request) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
