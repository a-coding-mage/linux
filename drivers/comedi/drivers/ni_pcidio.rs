// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of comedi/drivers/ni_pcidio.c. */

// External kernel/comedi symbols are intentionally left as dependencies supplied by the surrounding tree.

pub const WINDOW_ADDRESS: usize = 4;
pub const INTERRUPT_AND_WINDOW_STATUS: usize = 4;
pub const INT_STATUS_1: u8 = 1 << 0;
pub const INT_STATUS_2: u8 = 1 << 1;
pub const WINDOW_ADDRESS_STATUS_MASK: u8 = 0x7c;
pub const MASTER_DMA_AND_INTERRUPT_CONTROL: usize = 5;
pub const OPEN_INT: u8 = 1 << 2;
pub const GROUP_STATUS: usize = 5;
pub const DATA_LEFT: u8 = 1 << 0;
pub const REQ: u8 = 1 << 2;
pub const STOP_TRIG: u8 = 1 << 3;
pub const GROUP_1_FLAGS: usize = 6;
pub const GROUP_2_FLAGS: usize = 7;
pub const TRANSFER_READY: u8 = 1;
pub const COUNT_EXPIRED: u8 = 2;
pub const WAITED: u8 = 1 << 5;
pub const PRIMARY_TC: u8 = 1 << 6;
pub const SECONDARY_TC: u8 = 1 << 7;
pub const GROUP_1_FIRST_CLEAR: usize = 6;
pub const GROUP_2_FIRST_CLEAR: usize = 7;
pub const CLEAR_WAITED: u8 = 1 << 3;
pub const CLEAR_PRIMARY_TC: u8 = 1 << 4;
pub const CLEAR_SECONDARY_TC: u8 = 1 << 5;
pub const DMA_RESET: u8 = 1 << 6;
pub const FIFO_RESET: u8 = 1 << 7;
pub const CLEAR_ALL: u8 = 0xf8;
pub const GROUP_1_FIFO: usize = 8;
pub const GROUP_2_FIFO: usize = 12;
pub const TRANSFER_COUNT: usize = 20;
pub const CHIP_ID_D: usize = 24;
pub const CHIP_ID_I: usize = 25;
pub const CHIP_ID_O: usize = 26;
pub const CHIP_VERSION: usize = 27;
pub const DATA_PATH: usize = 64;
pub const FIFO_ENABLE_A: u8 = 1;
pub const FIFO_ENABLE_B: u8 = 2;
pub const FIFO_ENABLE_C: u8 = 4;
pub const FIFO_ENABLE_D: u8 = 8;
pub const GROUP_DIRECTION: u8 = 1 << 7;
pub const PROTOCOL_REGISTER_1: usize = 65;
pub const OP_MODE: usize = PROTOCOL_REGISTER_1;
pub const NUMBERED: u16 = 1 << 3;
pub const PROTOCOL_REGISTER_2: usize = 66;
pub const CLOCK_REG: usize = PROTOCOL_REGISTER_2;
pub const INVERT_STOP_TRIG: u8 = 1 << 7;
pub const PROTOCOL_REGISTER_3: usize = 67;
pub const SEQUENCE: usize = PROTOCOL_REGISTER_3;
pub const PROTOCOL_REGISTER_14: usize = 68;
pub const CLOCK_SPEED: usize = PROTOCOL_REGISTER_14;
pub const PROTOCOL_REGISTER_4: usize = 70;
pub const REQ_REG: usize = PROTOCOL_REGISTER_4;
pub const PROTOCOL_REGISTER_5: usize = 71;
pub const BLOCK_MODE: usize = PROTOCOL_REGISTER_5;
pub const FIFO_CONTROL: usize = 72;
pub const PROTOCOL_REGISTER_6: usize = 73;
pub const LINE_POLARITIES: usize = PROTOCOL_REGISTER_6;
pub const PROTOCOL_REGISTER_7: usize = 74;
pub const ACK_SER: usize = PROTOCOL_REGISTER_7;
pub const INTERRUPT_CONTROL: usize = 75;
pub const DMA_LINE_CONTROL_GROUP1: usize = 76;
pub const DMA_LINE_CONTROL_GROUP2: usize = 108;
pub const TRANSFER_SIZE_CONTROL: usize = 77;
pub const REQUIRE_R_LEVEL: u8 = 1 << 5;
pub const PROTOCOL_REGISTER_15: usize = 79;
pub const DAQ_OPTIONS: usize = PROTOCOL_REGISTER_15;
pub const INVERT_START: u8 = 1 << 2;
pub const REQ_START: u8 = 1 << 6;
pub const PRE_START: u8 = 1 << 7;
pub const PATTERN_DETECTION: usize = 81;
pub const PROTOCOL_REGISTER_8: usize = 88;
pub const START_DELAY: usize = PROTOCOL_REGISTER_8;
pub const FIRMWARE_CONTROL_REGISTER: usize = 0x100;
pub const FIRMWARE_STATUS_REGISTER: usize = 0x104;
pub const FIRMWARE_DATA_REGISTER: usize = 0x108;
pub const FIRMWARE_MASK_REGISTER: usize = 0x10c;
pub const FPGA_CONTROL1_REGISTER: usize = 0x200;
pub const FPGA_CONTROL2_REGISTER: usize = 0x204;
pub const FPGA_SCALS_COUNTER_REGISTER: usize = 0x280;
pub const FPGA_SCAMS_COUNTER_REGISTER: usize = 0x284;
pub const FPGA_SCBLS_COUNTER_REGISTER: usize = 0x288;
pub const FPGA_SCBMS_COUNTER_REGISTER: usize = 0x28c;
pub const TIMER_BASE: i32 = 50;
pub const FW_PCI_6534_MAIN: &str = "ni6534a.bin";
pub const FW_PCI_6534_SCARAB_DI: &str = "niscrb01.bin";
pub const FW_PCI_6534_SCARAB_DO: &str = "niscrb02.bin";

#[inline] pub const fn primary_DMAChannel_bits(channel: u32) -> u32 { channel & 3 }
#[inline] pub const fn secondary_DMAChannel_bits(channel: u32) -> u32 { (channel << 2) & 0xc }
#[inline] pub const fn port_io(x: usize) -> usize { 28 + x }
#[inline] pub const fn port_pin_directions(x: usize) -> usize { 32 + x }
#[inline] pub const fn port_pin_mask(x: usize) -> usize { 36 + x }
#[inline] pub const fn port_pin_polarities(x: usize) -> usize { 40 + x }
#[inline] pub const fn port_pattern(x: usize) -> usize { 48 + x }
#[inline] pub const fn transfer_width(x: u32) -> u32 { x & 3 }
#[inline] pub const fn transfer_length(x: u32) -> u32 { (x & 3) << 3 }
#[inline] pub const fn run_mode(x: u16) -> u16 { x & 7 }
#[inline] pub const fn data_latching(x: u16) -> u16 { (x & 3) << 5 }
#[inline] pub const fn clock_line(x: u16) -> u16 { (x & 3) << 5 }
#[inline] pub const fn ack_line(x: u16) -> u16 { (x & 3) << 2 }
#[inline] pub const fn req_conditioning(x: u16) -> u16 { (x & 7) << 3 }
#[inline] pub const fn ready_level(x: u16) -> u16 { x & 7 }
#[inline] pub const fn rtsi_clocking(x: u16) -> u16 { (x & 3) << 4 }

#[repr(C)]
pub struct nidio_board { pub name: *const core::ffi::c_char, pub uses_firmware: u32, pub dio_speed: u32 }
#[repr(C)]
pub struct nidio96_private {
    pub mite: *mut mite, pub boardtype: i32, pub dio: i32, pub OP_MODEBits: u16,
    pub di_mite_chan: *mut mite_channel, pub di_mite_ring: *mut mite_ring,
    pub mite_channel_lock: spinlock_t,
}
#[repr(C)] pub struct mite { pub pcidev: *mut pci_dev }
#[repr(C)] pub struct mite_channel { pub channel: u32, pub dir: i32 }
#[repr(C)] pub struct mite_ring;
#[repr(C)] pub struct pci_dev { pub irq: u32 }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct comedi_device;
#[repr(C)] pub struct comedi_subdevice;
#[repr(C)] pub struct comedi_insn { pub n: u32 }
#[repr(C)] pub struct comedi_cmd { pub start_src:u32, pub scan_begin_src:u32, pub convert_src:u32, pub scan_end_src:u32, pub stop_src:u32, pub start_arg:u32, pub scan_begin_arg:u32, pub convert_arg:u32, pub scan_end_arg:u32, pub stop_arg:u32, pub chanlist_len:u32, pub flags:u32 }

// The following driver entry points retain the original external ABI and are declared
// as unsafe stubs until the surrounding kernel/comedi bindings provide their types.
extern "C" {
    fn ni_pcidio_request_di_mite_channel(dev: *mut comedi_device) -> i32;
    fn ni_pcidio_release_di_mite_channel(dev: *mut comedi_device);
    fn setup_mite_dma(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32;
    fn ni_pcidio_poll(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32;
    fn nidio_interrupt(irq: i32, d: *mut core::ffi::c_void) -> i32;
    fn ni_pcidio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32;
    fn ni_pcidio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32;
    fn ni_pcidio_ns_to_timer(nanosec:*mut i32, flags:u32)->i32;
    fn ni_pcidio_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32;
    fn ni_pcidio_inttrig(dev:*mut comedi_device,s:*mut comedi_subdevice,trig_num:u32)->i32;
    fn ni_pcidio_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32;
    fn ni_pcidio_cancel(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32;
    fn ni_pcidio_change(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32;
    fn pci_6534_load_fpga(dev:*mut comedi_device,data:*const u8,data_len:usize,context:usize)->i32;
    fn pci_6534_reset_fpga(dev:*mut comedi_device,fpga_index:i32)->i32;
    fn pci_6534_reset_fpgas(dev:*mut comedi_device)->i32;
    fn pci_6534_init_main_fpga(dev:*mut comedi_device);
    fn pci_6534_upload_firmware(dev:*mut comedi_device)->i32;
    fn nidio_reset_board(dev:*mut comedi_device);
    fn nidio_auto_attach(dev:*mut comedi_device,context:usize)->i32;
    fn nidio_detach(dev:*mut comedi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
