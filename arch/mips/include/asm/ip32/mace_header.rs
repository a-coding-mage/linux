/*
 * Definitions for the SGI MACE (Multimedia, Audio and Communications Engine)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 */

pub const MACE_BASE: u32 = 0x1f000000;

#[inline(always)]
pub const fn bit(x: u32) -> u64 { 1u64 << x }

#[repr(C)]
pub struct mace_pci {
    pub error_addr: u32,
    pub error: u32,
    pub control: u32,
    pub rev: u32,
    pub _pad: [u32; 0xcf8 / 4 - 4],
    pub config_addr: u32,
    pub config_data: mace_pci_config_data,
}

pub const MACEPCI_ERROR_MASTER_ABORT: u64 = bit(31);
pub const MACEPCI_ERROR_TARGET_ABORT: u64 = bit(30);
pub const MACEPCI_ERROR_DATA_PARITY_ERR: u64 = bit(29);
pub const MACEPCI_ERROR_RETRY_ERR: u64 = bit(28);
pub const MACEPCI_ERROR_ILLEGAL_CMD: u64 = bit(27);
pub const MACEPCI_ERROR_SYSTEM_ERR: u64 = bit(26);
pub const MACEPCI_ERROR_INTERRUPT_TEST: u64 = bit(25);
pub const MACEPCI_ERROR_PARITY_ERR: u64 = bit(24);
pub const MACEPCI_ERROR_OVERRUN: u64 = bit(23);
pub const MACEPCI_ERROR_RSVD: u64 = bit(22);
pub const MACEPCI_ERROR_MEMORY_ADDR: u64 = bit(21);
pub const MACEPCI_ERROR_CONFIG_ADDR: u64 = bit(20);
pub const MACEPCI_ERROR_MASTER_ABORT_ADDR_VALID: u64 = bit(19);
pub const MACEPCI_ERROR_TARGET_ABORT_ADDR_VALID: u64 = bit(18);
pub const MACEPCI_ERROR_DATA_PARITY_ADDR_VALID: u64 = bit(17);
pub const MACEPCI_ERROR_RETRY_ADDR_VALID: u64 = bit(16);
pub const MACEPCI_ERROR_SIG_TABORT: u64 = bit(4);
pub const MACEPCI_ERROR_DEVSEL_MASK: u32 = 0xc0;
pub const MACEPCI_ERROR_DEVSEL_FAST: u32 = 0;
pub const MACEPCI_ERROR_DEVSEL_MED: u32 = 0x40;
pub const MACEPCI_ERROR_DEVSEL_SLOW: u32 = 0x80;
pub const MACEPCI_ERROR_FBB: u64 = bit(1);
pub const MACEPCI_ERROR_66MHZ: u64 = bit(0);
pub const MACEPCI_CONTROL_INT_MASK: u32 = 0xff;
#[inline(always)] pub const fn macepci_control_int(x: u32) -> u64 { bit(x) }
pub const MACEPCI_CONTROL_SERR_ENA: u64 = bit(8);
pub const MACEPCI_CONTROL_ARB_N6: u64 = bit(9);
pub const MACEPCI_CONTROL_PARITY_ERR: u64 = bit(10);
pub const MACEPCI_CONTROL_MRMRA_ENA: u64 = bit(11);
pub const MACEPCI_CONTROL_ARB_N3: u64 = bit(12);
pub const MACEPCI_CONTROL_ARB_N4: u64 = bit(13);
pub const MACEPCI_CONTROL_ARB_N5: u64 = bit(14);
pub const MACEPCI_CONTROL_PARK_LIU: u64 = bit(15);
#[inline(always)] pub const fn macepci_control_inv_int(x: u32) -> u64 { bit(16 + x) }
pub const MACEPCI_CONTROL_INV_INT_MASK: u32 = 0x00ff0000;
pub const MACEPCI_CONTROL_OVERRUN_INT: u64 = bit(24);
pub const MACEPCI_CONTROL_PARITY_INT: u64 = bit(25);
pub const MACEPCI_CONTROL_SERR_INT: u64 = bit(26);
pub const MACEPCI_CONTROL_IT_INT: u64 = bit(27);
pub const MACEPCI_CONTROL_RE_INT: u64 = bit(28);
pub const MACEPCI_CONTROL_DPED_INT: u64 = bit(29);
pub const MACEPCI_CONTROL_TAR_INT: u64 = bit(30);
pub const MACEPCI_CONTROL_MAR_INT: u64 = bit(31);
pub const MACEPCI_LOW_MEMORY: u64 = 0x1a000000;
pub const MACEPCI_LOW_IO: u64 = 0x18000000;
pub const MACEPCI_SWAPPED_VIEW: u64 = 0;
pub const MACEPCI_NATIVE_VIEW: u64 = 0x40000000;
pub const MACEPCI_IO: u64 = 0x80000000;
pub const MACEPCI_HI_MEMORY: u64 = 0x280000000;
pub const MACEPCI_HI_IO: u64 = 0x100000000;

#[repr(C)]
pub union mace_pci_config_data { pub b: [u8; 4], pub w: [u16; 2], pub l: u32 }

#[repr(C)] pub struct mace_video { pub xxx: u64 }

#[repr(C)]
pub struct mace_ethernet {
    pub mac_ctrl: u64, pub int_stat: u64, pub dma_ctrl: u64, pub timer: u64,
    pub tx_int_al: u64, pub rx_int_al: u64, pub tx_info: u64, pub tx_info_al: u64,
    pub rx_buff: u64, pub rx_buff_al1: u64, pub rx_buff_al2: u64, pub diag: u64,
    pub phy_data: u64, pub phy_regs: u64, pub phy_trans_go: u64, pub backoff_seed: u64,
    pub imq_reserved: [u64; 4], pub mac_addr: u64, pub mac_addr2: u64,
    pub mcast_filter: u64, pub tx_ring_base: u64, pub tx_pkt1_hdr: u64,
    pub tx_pkt1_ptr: [u64; 3], pub tx_pkt2_hdr: u64, pub tx_pkt2_ptr: [u64; 3],
    pub rx_fifo: u64,
}

#[repr(C)] pub struct mace_audio {
    pub control: u64, pub codec_control: u64, pub codec_mask: u64, pub codec_read: u64,
    pub chan: [mace_audio_chan; 3],
}
#[repr(C)] pub struct mace_audio_chan { pub control: u64, pub read_ptr: u64, pub write_ptr: u64, pub depth: u64 }

pub const MACEPAR_CONTEXT_LASTFLAG: u64 = bit(63);
pub const MACEPAR_CONTEXT_DATA_BOUND: u64 = 0x1000;
pub const MACEPAR_CONTEXT_DATALEN_MASK: u64 = 0x00000fff00000000;
pub const MACEPAR_CONTEXT_DATALEN_SHIFT: u32 = 32;
pub const MACEPAR_CONTEXT_BASEADDR_MASK: u64 = 0x00000000ffffffff;
#[repr(C)] pub struct mace_parport { pub context_a: u64, pub context_b: u64, pub cntlstat: u64, pub diagnostic: u64 }
pub const MACEPAR_CTLSTAT_DIRECTION: u64 = bit(0);
pub const MACEPAR_CTLSTAT_ENABLE: u64 = bit(1);
pub const MACEPAR_CTLSTAT_RESET: u64 = bit(2);
pub const MACEPAR_CTLSTAT_CTXB_VALID: u64 = bit(3);
pub const MACEPAR_CTLSTAT_CTXA_VALID: u64 = bit(4);
pub const MACEPAR_DIAG_CTXINUSE: u64 = bit(0);
pub const MACEPAR_DIAG_DMACTIVE: u64 = bit(1);
pub const MACEPAR_DIAG_CTRMASK: u64 = 0x3ffc;
pub const MACEPAR_DIAG_CTRSHIFT: u32 = 2;

#[repr(C)] pub struct mace_isactrl {
    pub ringbase: u64, pub misc: u64, pub istat: u64, pub imask: u64,
    pub _pad: [u64; 0x2000 / 8 - 4], pub dp_ram: [u64; 0x400], pub parport: mace_parport,
}
pub const MACEISA_RINGBUFFERS_SIZE: usize = 8 * 4096;
pub const MACEISA_FLASH_WE: u64 = bit(0); pub const MACEISA_PWD_CLEAR: u64 = bit(1);
pub const MACEISA_NIC_DEASSERT: u64 = bit(2); pub const MACEISA_NIC_DATA: u64 = bit(3);
pub const MACEISA_LED_RED: u64 = bit(4); pub const MACEISA_LED_GREEN: u64 = bit(5); pub const MACEISA_DP_RAM_ENABLE: u64 = bit(6);
pub const MACEISA_AUDIO_SW_INT: u64 = bit(0); pub const MACEISA_AUDIO_SC_INT: u64 = bit(1);
pub const MACEISA_AUDIO1_DMAT_INT: u64 = bit(2); pub const MACEISA_AUDIO1_OF_INT: u64 = bit(3);
pub const MACEISA_AUDIO2_DMAT_INT: u64 = bit(4); pub const MACEISA_AUDIO2_MERR_INT: u64 = bit(5);
pub const MACEISA_AUDIO3_DMAT_INT: u64 = bit(6); pub const MACEISA_AUDIO3_MERR_INT: u64 = bit(7);
pub const MACEISA_RTC_INT: u64 = bit(8); pub const MACEISA_KEYB_INT: u64 = bit(9); pub const MACEISA_KEYB_POLL_INT: u64 = bit(10);
pub const MACEISA_MOUSE_INT: u64 = bit(11); pub const MACEISA_MOUSE_POLL_INT: u64 = bit(12);
pub const MACEISA_TIMER0_INT: u64 = bit(13); pub const MACEISA_TIMER1_INT: u64 = bit(14); pub const MACEISA_TIMER2_INT: u64 = bit(15);
pub const MACEISA_PARALLEL_INT: u64 = bit(16); pub const MACEISA_PAR_CTXA_INT: u64 = bit(17); pub const MACEISA_PAR_CTXB_INT: u64 = bit(18); pub const MACEISA_PAR_MERR_INT: u64 = bit(19);
pub const MACEISA_SERIAL1_INT: u64 = bit(20); pub const MACEISA_SERIAL1_TDMAT_INT: u64 = bit(21); pub const MACEISA_SERIAL1_TDMAPR_INT: u64 = bit(22); pub const MACEISA_SERIAL1_TDMAME_INT: u64 = bit(23); pub const MACEISA_SERIAL1_RDMAT_INT: u64 = bit(24); pub const MACEISA_SERIAL1_RDMAOR_INT: u64 = bit(25);
pub const MACEISA_SERIAL2_INT: u64 = bit(26); pub const MACEISA_SERIAL2_TDMAT_INT: u64 = bit(27); pub const MACEISA_SERIAL2_TDMAPR_INT: u64 = bit(28); pub const MACEISA_SERIAL2_TDMAME_INT: u64 = bit(29); pub const MACEISA_SERIAL2_RDMAT_INT: u64 = bit(30); pub const MACEISA_SERIAL2_RDMAOR_INT: u64 = bit(31);

#[repr(C)] pub struct mace_ps2port { pub tx: u64, pub rx: u64, pub control: u64, pub status: u64 }
#[repr(C)] pub struct mace_ps2 { pub keyb: mace_ps2port, pub mouse: mace_ps2port }
#[repr(C)] pub struct mace_i2c { pub config: u64, pub control: u64, pub data: u64 }
pub const MACEI2C_RESET: u64 = bit(0); pub const MACEI2C_FAST: u64 = bit(1); pub const MACEI2C_DATA_OVERRIDE: u64 = bit(2); pub const MACEI2C_CLOCK_OVERRIDE: u64 = bit(3); pub const MACEI2C_DATA_STATUS: u64 = bit(4); pub const MACEI2C_CLOCK_STATUS: u64 = bit(5);

#[repr(C)] pub union timer_reg { pub ust_msc: u64, pub reg: timer_reg_pair }
#[repr(C)] pub struct timer_reg_pair { pub ust: u32, pub msc: u32 }
#[repr(C)] pub struct mace_timers { pub ust: u64, pub compare1: u64, pub compare2: u64, pub compare3: u64, pub audio_in: timer_reg, pub audio_out1: timer_reg, pub audio_out2: timer_reg, pub video_in1: timer_reg, pub video_in2: timer_reg, pub video_out: timer_reg }
pub const MACE_UST_PERIOD_NS: u64 = 960;

#[repr(C)] pub struct mace_perif { pub audio: mace_audio, pub _pad0: [u8; 0x10000 - core::mem::size_of::<mace_audio>()], pub ctrl: mace_isactrl, pub _pad1: [u8; 0x10000 - core::mem::size_of::<mace_isactrl>()], pub ps2: mace_ps2, pub _pad2: [u8; 0x10000 - core::mem::size_of::<mace_ps2>()], pub i2c: mace_i2c, pub _pad3: [u8; 0x10000 - core::mem::size_of::<mace_i2c>()], pub timers: mace_timers, pub _pad4: [u8; 0x10000 - core::mem::size_of::<mace_timers>()] }

#[repr(C)] pub struct mace_parallel {}
#[repr(C)] pub struct mace_ecp1284 {}
#[repr(C)] pub struct mace_serial { pub xxx: u64 }
#[repr(C)] pub struct mace_isa { pub parallel: mace_parallel, pub _pad1: [u8; 0x8000 - core::mem::size_of::<mace_parallel>()], pub ecp1284: mace_ecp1284, pub _pad2: [u8; 0x8000 - core::mem::size_of::<mace_ecp1284>()], pub serial1: mace_serial, pub _pad3: [u8; 0x8000 - core::mem::size_of::<mace_serial>()], pub serial2: mace_serial, pub _pad4: [u8; 0x8000 - core::mem::size_of::<mace_serial>()], pub rtc: [u8; 0x10000] }
#[repr(C)] pub struct sgi_mace { pub _reserved: [u8; 0x80000], pub pci: mace_pci, pub _pad0: [u8; 0x80000 - core::mem::size_of::<mace_pci>()], pub video_in1: mace_video, pub _pad1: [u8; 0x80000 - core::mem::size_of::<mace_video>()], pub video_in2: mace_video, pub _pad2: [u8; 0x80000 - core::mem::size_of::<mace_video>()], pub video_out: mace_video, pub _pad3: [u8; 0x80000 - core::mem::size_of::<mace_video>()], pub eth: mace_ethernet, pub _pad4: [u8; 0x80000 - core::mem::size_of::<mace_ethernet>()], pub perif: mace_perif, pub _pad5: [u8; 0x80000 - core::mem::size_of::<mace_perif>()], pub isa: mace_isa, pub _pad6: [u8; 0x80000 - core::mem::size_of::<mace_isa>()] }

// External MMIO instance declared by the surrounding platform.
extern "C" { pub static mut mace: *mut sgi_mace; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
