/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *   copyright            : (C) 2004 by Frank Mori Hess                    *
 ***************************************************************************/

// Dependencies supplied by the surrounding kernel/GPIB bindings are intentionally
// left external to this translation unit.

#[repr(i32)]
pub enum UsbVendorIds {
    USB_VENDOR_ID_AGILENT = 0x0957,
}

#[repr(i32)]
pub enum UsbDeviceIds {
    USB_DEVICE_ID_AGILENT_82357A = 0x0107,
    // device id before firmware is loaded
    USB_DEVICE_ID_AGILENT_82357A_PREINIT = 0x0007,
    // device id before firmware is loaded
    USB_DEVICE_ID_AGILENT_82357B = 0x0718,
    // device id before firmware is loaded
    USB_DEVICE_ID_AGILENT_82357B_PREINIT = 0x0518,
}

#[repr(i32)]
pub enum EndpointAddresses {
    AGILENT_82357_CONTROL_ENDPOINT = 0x0,
    AGILENT_82357_BULK_IN_ENDPOINT = 0x2,
    AGILENT_82357A_BULK_OUT_ENDPOINT = 0x4,
    AGILENT_82357A_INTERRUPT_IN_ENDPOINT = 0x6,
    AGILENT_82357B_BULK_OUT_ENDPOINT = 0x6,
    AGILENT_82357B_INTERRUPT_IN_ENDPOINT = 0x8,
}

#[repr(i32)]
pub enum BulkCommands {
    DATA_PIPE_CMD_WRITE = 0x1,
    DATA_PIPE_CMD_READ = 0x3,
    DATA_PIPE_CMD_WR_REGS = 0x4,
    DATA_PIPE_CMD_RD_REGS = 0x5,
}

#[repr(i32)]
pub enum Agilent82357aReadFlags {
    ARF_END_ON_EOI = 0x1,
    ARF_NO_ADDRESS = 0x2,
    ARF_END_ON_EOS_CHAR = 0x4,
    ARF_SPOLL = 0x8,
}

#[repr(i32)]
pub enum Agilent82357aTrailingReadFlags {
    ATRF_EOI = 0x1,
    ATRF_ATN = 0x2,
    ATRF_IFC = 0x4,
    ATRF_EOS = 0x8,
    ATRF_ABORT = 0x10,
    ATRF_COUNT = 0x20,
    ATRF_DEAD_BUS = 0x40,
    ATRF_UNADDRESSED = 0x80,
}

#[repr(i32)]
pub enum Agilent82357aWriteFlags {
    AWF_SEND_EOI = 0x1,
    AWF_NO_FAST_TALKER_FIRST_BYTE = 0x2,
    AWF_NO_FAST_TALKER = 0x4,
    AWF_NO_ADDRESS = 0x8,
    AWF_ATN = 0x10,
    AWF_SEPARATE_HEADER = 0x80,
}

#[repr(i32)]
pub enum Agilent82357aInterruptFlagBitNumbers {
    AIF_SRQ_BN = 0,
    AIF_WRITE_COMPLETE_BN = 1,
    AIF_READ_COMPLETE_BN = 2,
}

#[repr(i32)]
pub enum Agilent82357ErrorCodes {
    UGP_SUCCESS = 0,
    UGP_ERR_INVALID_CMD = 1,
    UGP_ERR_INVALID_PARAM = 2,
    UGP_ERR_INVALID_REG = 3,
    UGP_ERR_GPIB_READ = 4,
    UGP_ERR_GPIB_WRITE = 5,
    UGP_ERR_FLUSHING = 6,
    UGP_ERR_FLUSHING_ALREADY = 7,
    UGP_ERR_UNSUPPORTED = 8,
    UGP_ERR_OTHER = 9,
}

#[repr(i32)]
pub enum Agilent82357ControlValues {
    XFER_ABORT = 0xa0,
    XFER_STATUS = 0xb0,
}

#[repr(i32)]
pub enum XferStatusBits {
    XS_COMPLETED = 0x1,
    XS_READ = 0x2,
}

#[repr(i32)]
pub enum XferStatusCompletionBits {
    XSC_EOI = 0x1,
    XSC_ATN = 0x2,
    XSC_IFC = 0x4,
    XSC_EOS = 0x8,
    XSC_ABORT = 0x10,
    XSC_COUNT = 0x20,
    XSC_DEAD_BUS = 0x40,
    XSC_BUS_NOT_ADDRESSED = 0x80,
}

#[repr(i32)]
pub enum XferAbortType {
    XA_FLUSH = 0x1,
}

pub const STATUS_DATA_LEN: usize = 8;
pub const INTERRUPT_BUF_LEN: usize = 8;

#[repr(C)]
pub struct Agilent82357aUrbCtx {
    pub complete: Completion,
    pub timed_out: u32, // C bitfield: unsigned timed_out : 1;
}

// struct which defines local data for each 82357 device
#[repr(C)]
pub struct Agilent82357aPriv {
    pub bus_interface: *mut UsbInterface,
    pub eos_char: u16,
    pub eos_mode: u16,
    pub hw_control_bits: u16,
    pub interrupt_flags: c_ulong,
    pub bulk_urb: *mut Urb,
    pub interrupt_urb: *mut Urb,
    pub interrupt_buffer: *mut u8,
    pub bulk_transfer_lock: Mutex, // bulk transfer lock
    pub bulk_alloc_lock: Mutex, // bulk transfer allocation lock
    pub interrupt_alloc_lock: Mutex, // interrupt allocation lock
    pub control_alloc_lock: Mutex, // control message allocation lock
    pub bulk_timer: TimerList,
    pub context: Agilent82357aUrbCtx,
    pub bulk_out_endpoint: c_uint,
    pub interrupt_in_endpoint: c_uint,
    pub is_cic: c_uint, // C bitfield: unsigned is_cic : 1;
    pub ren_state: c_uint, // C bitfield: unsigned ren_state : 1;
}

#[repr(C)]
pub struct Agilent82357aRegisterPairlet {
    pub address: i16,
    pub value: u16,
}

#[repr(i32)]
pub enum FirmwareRegisters {
    HW_CONTROL = 0xa,
    LED_CONTROL = 0xb,
    RESET_TO_POWERUP = 0xc,
    PROTOCOL_CONTROL = 0xd,
    FAST_TALKER_T1 = 0xe,
}

#[repr(i32)]
pub enum HardwareControlBits {
    NOT_TI_RESET = 0x1,
    SYSTEM_CONTROLLER = 0x2,
    NOT_PARALLEL_POLL = 0x4,
    OSCILLATOR_5V_ON = 0x8,
    OUTPUT_5V_ON = 0x20,
    CPLD_3V_ON = 0x80,
}

#[repr(i32)]
pub enum LedControlBits {
    FIRMWARE_LED_CONTROL = 0x1,
    FAIL_LED_ON = 0x20,
    READY_LED_ON = 0x40,
    ACCESS_LED_ON = 0x80,
}

#[repr(i32)]
pub enum ResetToPowerupBits {
    // wait 2 millisec after sending
    RESET_SPACEBALL = 0x1,
}

#[repr(i32)]
pub enum ProtocolControlBits {
    WRITE_COMPLETE_INTERRUPT_EN = 0x1,
}

pub const agilent_82357a_control_request: i32 = 0x4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
