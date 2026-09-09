/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from tms9914.h. C includes and header guards are omitted. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tms9914_holdoff_mode {
    TMS9914_HOLDOFF_NONE,
    TMS9914_HOLDOFF_EOI,
    TMS9914_HOLDOFF_ALL,
}

#[repr(C)]
pub struct tms9914_priv {
    /* CONFIG_HAS_IOPORT controls whether this field is present in C. */
    #[cfg(feature = "CONFIG_HAS_IOPORT")]
    pub iobase: u32,
    pub mmiobase: *mut core::ffi::c_void,
    pub offset: core::ffi::c_uint,
    pub dma_channel: core::ffi::c_uint,
    pub imr0_bits: u8,
    pub imr1_bits: u8,
    pub admr_bits: u8,
    pub auxa_bits: u8,
    pub state: core::ffi::c_ulong,
    pub eos: u8,
    pub eos_flags: i16,
    pub spoll_status: u8,
    pub holdoff_mode: tms9914_holdoff_mode,
    pub ppoll_line: core::ffi::c_uint,
    pub talker_state: crate::talker_function_state,
    pub listener_state: crate::listener_function_state,
    /* C bitfields: ppoll_sense through holdoff_active, one-bit each. */
    pub flags: u32,
    pub read_byte: Option<unsafe extern "C" fn(*mut tms9914_priv, core::ffi::c_uint) -> u8>,
    pub write_byte: Option<unsafe extern "C" fn(*mut tms9914_priv, u8, core::ffi::c_uint)>,
}

#[inline]
pub unsafe fn read_byte(priv_: *mut tms9914_priv, register_number: core::ffi::c_uint) -> u8 {
    ((*priv_).read_byte.unwrap())(priv_, register_number)
}

#[inline]
pub unsafe fn write_byte(priv_: *mut tms9914_priv, byte: u8, register_number: core::ffi::c_uint) {
    ((*priv_).write_byte.unwrap())(priv_, byte, register_number);
}

pub const PIO_IN_PROGRESS_BN: u32 = 0;
pub const DMA_READ_IN_PROGRESS_BN: u32 = 1;
pub const DMA_WRITE_IN_PROGRESS_BN: u32 = 2;
pub const READ_READY_BN: u32 = 3;
pub const WRITE_READY_BN: u32 = 4;
pub const COMMAND_READY_BN: u32 = 5;
pub const RECEIVED_END_BN: u32 = 6;
pub const BUS_ERROR_BN: u32 = 7;
pub const DEV_CLEAR_BN: u32 = 8;

extern "C" {
    pub fn tms9914_read(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8, length: usize, end: *mut i32, bytes_read: *mut usize) -> i32;
    pub fn tms9914_write(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8, length: usize, send_eoi: i32, bytes_written: *mut usize) -> i32;
    pub fn tms9914_command(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8, length: usize, bytes_written: *mut usize) -> i32;
    pub fn tms9914_take_control(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, syncronous: i32) -> i32;
    pub fn tms9914_take_control_workaround(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, syncronous: i32) -> i32;
    pub fn tms9914_go_to_standby(board: *mut crate::gpib_board, priv_: *mut tms9914_priv) -> i32;
    pub fn tms9914_request_system_control(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, request_control: i32) -> i32;
    pub fn tms9914_interface_clear(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, assert_: i32);
    pub fn tms9914_remote_enable(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, enable: i32);
    pub fn tms9914_enable_eos(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, eos_bytes: u8, compare_8_bits: i32) -> i32;
    pub fn tms9914_disable_eos(board: *mut crate::gpib_board, priv_: *mut tms9914_priv);
    pub fn tms9914_update_status(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, clear_mask: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn tms9914_primary_address(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, address: core::ffi::c_uint) -> i32;
    pub fn tms9914_secondary_address(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, address: core::ffi::c_uint, enable: i32) -> i32;
    pub fn tms9914_parallel_poll(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, result: *mut u8) -> i32;
    pub fn tms9914_parallel_poll_configure(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, config: u8);
    pub fn tms9914_parallel_poll_response(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, ist: i32);
    pub fn tms9914_serial_poll_response(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, status: u8);
    pub fn tms9914_serial_poll_status(board: *mut crate::gpib_board, priv_: *mut tms9914_priv) -> u8;
    pub fn tms9914_line_status(board: *const crate::gpib_board, priv_: *mut tms9914_priv) -> i32;
    pub fn tms9914_t1_delay(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, nano_sec: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn tms9914_return_to_local(board: *const crate::gpib_board, priv_: *mut tms9914_priv);
    pub fn tms9914_board_reset(priv_: *mut tms9914_priv);
    pub fn tms9914_online(board: *mut crate::gpib_board, priv_: *mut tms9914_priv);
    pub fn tms9914_release_holdoff(priv_: *mut tms9914_priv);
    pub fn tms9914_set_holdoff_mode(priv_: *mut tms9914_priv, mode: tms9914_holdoff_mode);
    pub fn tms9914_ioport_read_byte(priv_: *mut tms9914_priv, register_num: core::ffi::c_uint) -> u8;
    pub fn tms9914_ioport_write_byte(priv_: *mut tms9914_priv, data: u8, register_num: core::ffi::c_uint);
    pub fn tms9914_iomem_read_byte(priv_: *mut tms9914_priv, register_num: core::ffi::c_uint) -> u8;
    pub fn tms9914_iomem_write_byte(priv_: *mut tms9914_priv, data: u8, register_num: core::ffi::c_uint);
    pub fn tms9914_interrupt(board: *mut crate::gpib_board, priv_: *mut tms9914_priv) -> crate::irqreturn_t;
    pub fn tms9914_interrupt_have_status(board: *mut crate::gpib_board, priv_: *mut tms9914_priv, status1: i32, status2: i32) -> crate::irqreturn_t;
}

pub const ms9914_num_registers: i32 = 8;
pub const IMR0: i32 = 0; pub const IMR1: i32 = 1; pub const AUXCR: i32 = 3; pub const ADR: i32 = 4; pub const SPMR: i32 = 5; pub const PPR: i32 = 6; pub const CDOR: i32 = 7;
pub const ISR0: i32 = 0; pub const ISR1: i32 = 1; pub const ADSR: i32 = 2; pub const BSR: i32 = 3; pub const CPTR: i32 = 6; pub const DIR: i32 = 7;

pub const HR_MAC: u8 = 1 << 0; pub const HR_RLC: u8 = 1 << 1; pub const HR_SPAS: u8 = 1 << 2; pub const HR_END: u8 = 1 << 3; pub const HR_BO: u8 = 1 << 4; pub const HR_BI: u8 = 1 << 5;
pub const HR_MACIE: u8 = 1 << 0; pub const HR_RLCIE: u8 = 1 << 1; pub const HR_SPASIE: u8 = 1 << 2; pub const HR_ENDIE: u8 = 1 << 3; pub const HR_BOIE: u8 = 1 << 4; pub const HR_BIIE: u8 = 1 << 5;
pub const HR_IFC: u8 = 1 << 0; pub const HR_SRQ: u8 = 1 << 1; pub const HR_MA: u8 = 1 << 2; pub const HR_DCAS: u8 = 1 << 3; pub const HR_APT: u8 = 1 << 4; pub const HR_UNC: u8 = 1 << 5; pub const HR_ERR: u8 = 1 << 6; pub const HR_GET: u8 = 1 << 7;
pub const HR_IFCIE: u8 = 1 << 0; pub const HR_SRQIE: u8 = 1 << 1; pub const HR_MAIE: u8 = 1 << 2; pub const HR_DCASIE: u8 = 1 << 3; pub const HR_APTIE: u8 = 1 << 4; pub const HR_UNCIE: u8 = 1 << 5; pub const HR_ERRIE: u8 = 1 << 6; pub const HR_GETIE: u8 = 1 << 7;
pub const HR_ULPA: u8 = 1 << 0; pub const HR_TA: u8 = 1 << 1; pub const HR_LA: u8 = 1 << 2; pub const HR_TPAS: u8 = 1 << 3; pub const HR_LPAS: u8 = 1 << 4; pub const HR_ATN: u8 = 1 << 5; pub const HR_LLO: u8 = 1 << 6; pub const HR_REM: u8 = 1 << 7;
pub const ADDRESS_MASK: u8 = 0x1f; pub const HR_DAT: u8 = 1 << 5; pub const HR_DAL: u8 = 1 << 6; pub const HR_EDPA: u8 = 1 << 7;
pub const BSR_REN_BIT: u8 = 0x1; pub const BSR_IFC_BIT: u8 = 0x2; pub const BSR_SRQ_BIT: u8 = 0x4; pub const BSR_EOI_BIT: u8 = 0x8; pub const BSR_NRFD_BIT: u8 = 0x10; pub const BSR_NDAC_BIT: u8 = 0x20; pub const BSR_DAV_BIT: u8 = 0x40; pub const BSR_ATN_BIT: u8 = 0x80;

pub const AUX_CS: u8 = 0x80; pub const AUX_CHIP_RESET: u8 = 0x0; pub const AUX_INVAL: u8 = 0x1; pub const AUX_VAL: u8 = AUX_INVAL | AUX_CS; pub const AUX_RHDF: u8 = 0x2; pub const AUX_HLDA: u8 = 0x3; pub const AUX_HLDE: u8 = 0x4; pub const AUX_NBAF: u8 = 0x5; pub const AUX_FGET: u8 = 0x6; pub const AUX_RTL: u8 = 0x7; pub const AUX_SEOI: u8 = 0x8; pub const AUX_LON: u8 = 0x9; pub const AUX_TON: u8 = 0xa; pub const AUX_GTS: u8 = 0xb; pub const AUX_TCA: u8 = 0xc; pub const AUX_TCS: u8 = 0xd; pub const AUX_RPP: u8 = 0xe; pub const AUX_SIC: u8 = 0xf; pub const AUX_SRE: u8 = 0x10; pub const AUX_RQC: u8 = 0x11; pub const AUX_RLC: u8 = 0x12; pub const AUX_DAI: u8 = 0x13; pub const AUX_PTS: u8 = 0x14; pub const AUX_STDL: u8 = 0x15; pub const AUX_SHDW: u8 = 0x16; pub const AUX_VSTDL: u8 = 0x17; pub const AUX_RSV2: u8 = 0x18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
