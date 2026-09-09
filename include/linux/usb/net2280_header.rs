// SPDX-License-Identifier: GPL-2.0+
/*
 * NetChip 2280 high/full speed USB device controller.
 * Unlike many such controllers, this one talks PCI.
 *
 * Copyright (C) 2002 NetChip Technology, Inc. (http://www.netchip.com)
 * Copyright (C) 2003 David Brownell
 */

/* NET2280 MEMORY MAPPED REGISTERS */

#[repr(C, packed)]
pub struct net2280_regs {
    pub devinit: u32,
    pub eectl: u32,
    pub eeclkfreq: u32,
    pub _unused0: u32,
    pub pciirqenb0: u32,
    pub pciirqenb1: u32,
    pub cpu_irqenb0: u32,
    pub cpu_irqenb1: u32,
    pub _unused1: u32,
    pub usbirqenb1: u32,
    pub irqstat0: u32,
    pub irqstat1: u32,
    pub idxaddr: u32,
    pub idxdata: u32,
    pub fifoctl: u32,
    pub _unused2: u32,
    pub memaddr: u32,
    pub memdata0: u32,
    pub memdata1: u32,
    pub _unused3: u32,
    pub gpioctl: u32,
    pub gpiostat: u32,
}

pub const LOCAL_CLOCK_FREQUENCY: u32 = 8;
pub const FORCE_PCI_RESET: u32 = 7;
pub const PCI_ID: u32 = 6;
pub const PCI_ENABLE: u32 = 5;
pub const FIFO_SOFT_RESET: u32 = 4;
pub const CFG_SOFT_RESET: u32 = 3;
pub const PCI_SOFT_RESET: u32 = 2;
pub const USB_SOFT_RESET: u32 = 1;
pub const M8051_RESET: u32 = 0;
pub const EEPROM_ADDRESS_WIDTH: u32 = 23;
pub const EEPROM_CHIP_SELECT_ACTIVE: u32 = 22;
pub const EEPROM_PRESENT: u32 = 21;
pub const EEPROM_VALID: u32 = 20;
pub const EEPROM_BUSY: u32 = 19;
pub const EEPROM_CHIP_SELECT_ENABLE: u32 = 18;
pub const EEPROM_BYTE_READ_START: u32 = 17;
pub const EEPROM_BYTE_WRITE_START: u32 = 16;
pub const EEPROM_READ_DATA: u32 = 8;
pub const EEPROM_WRITE_DATA: u32 = 0;
pub const SETUP_PACKET_INTERRUPT_ENABLE: u32 = 7;
pub const ENDPOINT_F_INTERRUPT_ENABLE: u32 = 6;
pub const ENDPOINT_E_INTERRUPT_ENABLE: u32 = 5;
pub const ENDPOINT_D_INTERRUPT_ENABLE: u32 = 4;
pub const ENDPOINT_C_INTERRUPT_ENABLE: u32 = 3;
pub const ENDPOINT_B_INTERRUPT_ENABLE: u32 = 2;
pub const ENDPOINT_A_INTERRUPT_ENABLE: u32 = 1;
pub const ENDPOINT_0_INTERRUPT_ENABLE: u32 = 0;
pub const PCI_INTERRUPT_ENABLE: u32 = 31;
pub const USB_INTERRUPT_ENABLE: u32 = 31;
pub const CPU_INTERRUPT_ENABLE: u32 = 31;
pub const POWER_STATE_CHANGE_INTERRUPT_ENABLE: u32 = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT_ENABLE: u32 = 26;
pub const PCI_PARITY_ERROR_INTERRUPT_ENABLE: u32 = 25;
pub const PCI_INTA_INTERRUPT_ENABLE: u32 = 24;
pub const PCI_PME_INTERRUPT_ENABLE: u32 = 23;
pub const PCI_SERR_INTERRUPT_ENABLE: u32 = 22;
pub const PCI_PERR_INTERRUPT_ENABLE: u32 = 21;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT_ENABLE: u32 = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT_ENABLE: u32 = 19;
pub const PCI_TARGET_ABORT_ASSERTED_INTERRUPT_ENABLE: u32 = 18;
pub const PCI_RETRY_ABORT_INTERRUPT_ENABLE: u32 = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT_ENABLE: u32 = 16;
pub const GPIO_INTERRUPT_ENABLE: u32 = 13;
pub const DMA_D_INTERRUPT_ENABLE: u32 = 12;
pub const DMA_C_INTERRUPT_ENABLE: u32 = 11;
pub const DMA_B_INTERRUPT_ENABLE: u32 = 10;
pub const DMA_A_INTERRUPT_ENABLE: u32 = 9;
pub const EEPROM_DONE_INTERRUPT_ENABLE: u32 = 8;
pub const VBUS_INTERRUPT_ENABLE: u32 = 7;
pub const CONTROL_STATUS_INTERRUPT_ENABLE: u32 = 6;
pub const ROOT_PORT_RESET_INTERRUPT_ENABLE: u32 = 4;
pub const SUSPEND_REQUEST_INTERRUPT_ENABLE: u32 = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT_ENABLE: u32 = 2;
pub const RESUME_INTERRUPT_ENABLE: u32 = 1;
pub const SOF_INTERRUPT_ENABLE: u32 = 0;
pub const USB3380_IRQSTAT0_EP_INTR_MASK_IN: u32 = 0xF << 17;
pub const USB3380_IRQSTAT0_EP_INTR_MASK_OUT: u32 = 0xF << 1;
pub const INTA_ASSERTED: u32 = 12;
pub const SETUP_PACKET_INTERRUPT: u32 = 7;
pub const ENDPOINT_F_INTERRUPT: u32 = 6;
pub const ENDPOINT_E_INTERRUPT: u32 = 5;
pub const ENDPOINT_D_INTERRUPT: u32 = 4;
pub const ENDPOINT_C_INTERRUPT: u32 = 3;
pub const ENDPOINT_B_INTERRUPT: u32 = 2;
pub const ENDPOINT_A_INTERRUPT: u32 = 1;
pub const ENDPOINT_0_INTERRUPT: u32 = 0;
pub const SOF_DOWN_INTERRUPT: u32 = 14;
pub const POWER_STATE_CHANGE_INTERRUPT: u32 = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT: u32 = 26;
pub const PCI_PARITY_ERROR_INTERRUPT: u32 = 25;
pub const PCI_INTA_INTERRUPT: u32 = 24;
pub const PCI_PME_INTERRUPT: u32 = 23;
pub const PCI_SERR_INTERRUPT: u32 = 22;
pub const PCI_PERR_INTERRUPT: u32 = 21;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT: u32 = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT: u32 = 19;
pub const PCI_RETRY_ABORT_INTERRUPT: u32 = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT: u32 = 16;
pub const SOF_DOWN_INTERRUPT_1: u32 = 14;
pub const GPIO_INTERRUPT: u32 = 13;
pub const DMA_D_INTERRUPT: u32 = 12;
pub const DMA_C_INTERRUPT: u32 = 11;
pub const DMA_B_INTERRUPT: u32 = 10;
pub const DMA_A_INTERRUPT: u32 = 9;
pub const EEPROM_DONE_INTERRUPT: u32 = 8;
pub const VBUS_INTERRUPT: u32 = 7;
pub const CONTROL_STATUS_INTERRUPT: u32 = 6;
pub const ROOT_PORT_RESET_INTERRUPT: u32 = 4;
pub const SUSPEND_REQUEST_INTERRUPT: u32 = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT: u32 = 2;
pub const RESUME_INTERRUPT: u32 = 1;
pub const SOF_INTERRUPT: u32 = 0;
pub const PCI_BASE2_RANGE: u32 = 16;
pub const IGNORE_FIFO_AVAILABILITY: u32 = 3;
pub const PCI_BASE2_SELECT: u32 = 2;
pub const FIFO_CONFIGURATION_SELECT: u32 = 0;
pub const START: u32 = 28;
pub const DIRECTION: u32 = 27;
pub const FIFO_DIAGNOSTIC_SELECT: u32 = 24;
pub const MEMORY_ADDRESS: u32 = 0;
pub const GPIO3_LED_SELECT: u32 = 12;
pub const GPIO3_INTERRUPT_ENABLE: u32 = 11;
pub const GPIO2_INTERRUPT_ENABLE: u32 = 10;
pub const GPIO1_INTERRUPT_ENABLE: u32 = 9;
pub const GPIO0_INTERRUPT_ENABLE: u32 = 8;
pub const GPIO3_OUTPUT_ENABLE: u32 = 7;
pub const GPIO2_OUTPUT_ENABLE: u32 = 6;
pub const GPIO1_OUTPUT_ENABLE: u32 = 5;
pub const GPIO0_OUTPUT_ENABLE: u32 = 4;
pub const GPIO3_DATA: u32 = 3;
pub const GPIO2_DATA: u32 = 2;
pub const GPIO1_DATA: u32 = 1;
pub const GPIO0_DATA: u32 = 0;
pub const GPIO3_INTERRUPT: u32 = 3;
pub const GPIO2_INTERRUPT: u32 = 2;
pub const GPIO1_INTERRUPT: u32 = 1;
pub const GPIO0_INTERRUPT: u32 = 0;

#[repr(C, packed)]
pub struct net2280_usb_regs {
    pub stdrsp: u32,
    pub prodvendid: u32,
    pub relnum: u32,
    pub usbctl: u32,
    pub usbstat: u32,
    pub xcvrdiag: u32,
    pub setup0123: u32,
    pub setup4567: u32,
    pub _unused0: u32,
    pub ouraddr: u32,
    pub ourconfig: u32,
}

pub const STALL_UNSUPPORTED_REQUESTS: u32 = 31;
pub const SET_TEST_MODE: u32 = 16;
pub const GET_OTHER_SPEED_CONFIGURATION: u32 = 15;
pub const GET_DEVICE_QUALIFIER: u32 = 14;
pub const SET_ADDRESS: u32 = 13;
pub const ENDPOINT_SET_CLEAR_HALT: u32 = 12;
pub const DEVICE_SET_CLEAR_DEVICE_REMOTE_WAKEUP: u32 = 11;
pub const GET_STRING_DESCRIPTOR_2: u32 = 10;
pub const GET_STRING_DESCRIPTOR_1: u32 = 9;
pub const GET_STRING_DESCRIPTOR_0: u32 = 8;
pub const GET_SET_INTERFACE: u32 = 6;
pub const GET_SET_CONFIGURATION: u32 = 5;
pub const GET_CONFIGURATION_DESCRIPTOR: u32 = 4;
pub const GET_DEVICE_DESCRIPTOR: u32 = 3;
pub const GET_ENDPOINT_STATUS: u32 = 2;
pub const GET_INTERFACE_STATUS: u32 = 1;
pub const GET_DEVICE_STATUS: u32 = 0;
pub const PRODUCT_ID: u32 = 16;
pub const VENDOR_ID: u32 = 0;
pub const SERIAL_NUMBER_INDEX: u32 = 16;
pub const PRODUCT_ID_STRING_ENABLE: u32 = 13;
pub const VENDOR_ID_STRING_ENABLE: u32 = 12;
pub const USB_ROOT_PORT_WAKEUP_ENABLE: u32 = 11;
pub const VBUS_PIN: u32 = 10;
pub const TIMED_DISCONNECT: u32 = 9;
pub const SUSPEND_IMMEDIATELY: u32 = 7;
pub const SELF_POWERED_USB_DEVICE: u32 = 6;
pub const REMOTE_WAKEUP_SUPPORT: u32 = 5;
pub const PME_POLARITY: u32 = 4;
pub const USB_DETECT_ENABLE: u32 = 3;
pub const PME_WAKEUP_ENABLE: u32 = 2;
pub const DEVICE_REMOTE_WAKEUP_ENABLE: u32 = 1;
pub const SELF_POWERED_STATUS: u32 = 0;
pub const HIGH_SPEED: u32 = 7;
pub const FULL_SPEED: u32 = 6;
pub const GENERATE_RESUME: u32 = 5;
pub const GENERATE_DEVICE_REMOTE_WAKEUP: u32 = 4;
pub const FORCE_HIGH_SPEED_MODE: u32 = 31;
pub const FORCE_FULL_SPEED_MODE: u32 = 30;
pub const USB_TEST_MODE: u32 = 24;
pub const LINE_STATE: u32 = 16;
pub const TRANSCEIVER_OPERATION_MODE: u32 = 2;
pub const TRANSCEIVER_SELECT: u32 = 1;
pub const TERMINATION_SELECT: u32 = 0;
pub const FORCE_IMMEDIATE: u32 = 7;
pub const OUR_USB_ADDRESS: u32 = 0;

#[repr(C, packed)]
pub struct net2280_pci_regs {
    pub pcimstctl: u32,
    pub pcimstaddr: u32,
    pub pcimstdata: u32,
    pub pcimststat: u32,
}

pub const PCI_ARBITER_PARK_SELECT: u32 = 13;
pub const PCI_MULTI_LEVEL_ARBITER: u32 = 12;
pub const PCI_RETRY_ABORT_ENABLE: u32 = 11;
pub const DMA_MEMORY_WRITE_AND_INVALIDATE_ENABLE: u32 = 10;
pub const DMA_READ_MULTIPLE_ENABLE: u32 = 9;
pub const DMA_READ_LINE_ENABLE: u32 = 8;
pub const PCI_MASTER_COMMAND_SELECT: u32 = 6;
pub const MEM_READ_OR_WRITE: u32 = 0;
pub const IO_READ_OR_WRITE: u32 = 1;
pub const CFG_READ_OR_WRITE: u32 = 2;
pub const PCI_MASTER_START: u32 = 5;
pub const PCI_MASTER_READ_WRITE: u32 = 4;
pub const PCI_MASTER_WRITE: u32 = 0;
pub const PCI_MASTER_READ: u32 = 1;
pub const PCI_MASTER_BYTE_WRITE_ENABLES: u32 = 0;
pub const PCI_ARBITER_CLEAR: u32 = 2;
pub const PCI_EXTERNAL_ARBITER: u32 = 1;
pub const PCI_HOST_MODE: u32 = 0;

#[repr(C, packed)]
pub struct net2280_dma_regs {
    pub dmactl: u32,
    pub dmastat: u32,
    pub _unused0: [u32; 2],
    pub dmacount: u32,
    pub dmaaddr: u32,
    pub dmadesc: u32,
    pub _unused1: u32,
}

pub const DMA_SCATTER_GATHER_DONE_INTERRUPT_ENABLE: u32 = 25;
pub const DMA_CLEAR_COUNT_ENABLE: u32 = 21;
pub const DESCRIPTOR_POLLING_RATE: u32 = 19;
pub const POLL_CONTINUOUS: u32 = 0;
pub const POLL_1_USEC: u32 = 1;
pub const POLL_100_USEC: u32 = 2;
pub const POLL_1_MSEC: u32 = 3;
pub const DMA_VALID_BIT_POLLING_ENABLE: u32 = 18;
pub const DMA_VALID_BIT_ENABLE: u32 = 17;
pub const DMA_SCATTER_GATHER_ENABLE: u32 = 16;
pub const DMA_OUT_AUTO_START_ENABLE: u32 = 4;
pub const DMA_PREEMPT_ENABLE: u32 = 3;
pub const DMA_FIFO_VALIDATE: u32 = 2;
pub const DMA_ENABLE: u32 = 1;
pub const DMA_ADDRESS_HOLD: u32 = 0;
pub const DMA_ABORT_DONE_INTERRUPT: u32 = 27;
pub const DMA_SCATTER_GATHER_DONE_INTERRUPT: u32 = 25;
pub const DMA_TRANSACTION_DONE_INTERRUPT: u32 = 24;
pub const DMA_ABORT: u32 = 1;
pub const DMA_START: u32 = 0;
pub const VALID_BIT: u32 = 31;
pub const DMA_DIRECTION: u32 = 30;
pub const DMA_DONE_INTERRUPT_ENABLE: u32 = 29;
pub const END_OF_CHAIN: u32 = 28;
pub const DMA_BYTE_COUNT_MASK: u32 = (1 << 24) - 1;
pub const DMA_BYTE_COUNT: u32 = 0;

#[repr(C, packed)]
pub struct net2280_dep_regs {
    pub dep_cfg: u32,
    pub dep_rsp: u32,
    pub _unused: [u32; 2],
}

#[repr(C, packed)]
pub struct net2280_ep_regs {
    pub ep_cfg: u32,
    pub ep_rsp: u32,
    pub ep_irqenb: u32,
    pub ep_stat: u32,
    pub ep_avail: u32,
    pub ep_data: u32,
    pub _unused0: [u32; 2],
}

pub const ENDPOINT_BYTE_COUNT: u32 = 16;
pub const ENDPOINT_ENABLE: u32 = 10;
pub const ENDPOINT_TYPE: u32 = 8;
pub const ENDPOINT_DIRECTION: u32 = 7;
pub const ENDPOINT_NUMBER: u32 = 0;
pub const SET_NAK_OUT_PACKETS: u32 = 15;
pub const SET_EP_HIDE_STATUS_PHASE: u32 = 14;
pub const SET_EP_FORCE_CRC_ERROR: u32 = 13;
pub const SET_INTERRUPT_MODE: u32 = 12;
pub const SET_CONTROL_STATUS_PHASE_HANDSHAKE: u32 = 11;
pub const SET_NAK_OUT_PACKETS_MODE: u32 = 10;
pub const SET_ENDPOINT_TOGGLE: u32 = 9;
pub const SET_ENDPOINT_HALT: u32 = 8;
pub const CLEAR_NAK_OUT_PACKETS: u32 = 7;
pub const CLEAR_EP_HIDE_STATUS_PHASE: u32 = 6;
pub const CLEAR_EP_FORCE_CRC_ERROR: u32 = 5;
pub const CLEAR_INTERRUPT_MODE: u32 = 4;
pub const CLEAR_CONTROL_STATUS_PHASE_HANDSHAKE: u32 = 3;
pub const CLEAR_NAK_OUT_PACKETS_MODE: u32 = 2;
pub const CLEAR_ENDPOINT_TOGGLE: u32 = 1;
pub const CLEAR_ENDPOINT_HALT: u32 = 0;
pub const SHORT_PACKET_OUT_DONE_INTERRUPT_ENABLE: u32 = 6;
pub const SHORT_PACKET_TRANSFERRED_INTERRUPT_ENABLE: u32 = 5;
pub const DATA_PACKET_RECEIVED_INTERRUPT_ENABLE: u32 = 3;
pub const DATA_PACKET_TRANSMITTED_INTERRUPT_ENABLE: u32 = 2;
pub const DATA_OUT_PING_TOKEN_INTERRUPT_ENABLE: u32 = 1;
pub const DATA_IN_TOKEN_INTERRUPT_ENABLE: u32 = 0;
pub const FIFO_VALID_COUNT: u32 = 24;
pub const HIGH_BANDWIDTH_OUT_TRANSACTION_PID: u32 = 22;
pub const TIMEOUT: u32 = 21;
pub const USB_STALL_SENT: u32 = 20;
pub const USB_IN_NAK_SENT: u32 = 19;
pub const USB_IN_ACK_RCVD: u32 = 18;
pub const USB_OUT_PING_NAK_SENT: u32 = 17;
pub const USB_OUT_ACK_SENT: u32 = 16;
pub const FIFO_OVERFLOW: u32 = 13;
pub const FIFO_UNDERFLOW: u32 = 12;
pub const FIFO_FULL: u32 = 11;
pub const FIFO_EMPTY: u32 = 10;
pub const FIFO_FLUSH: u32 = 9;
pub const SHORT_PACKET_OUT_DONE_INTERRUPT: u32 = 6;
pub const SHORT_PACKET_TRANSFERRED_INTERRUPT: u32 = 5;
pub const NAK_OUT_PACKETS: u32 = 4;
pub const DATA_PACKET_RECEIVED_INTERRUPT: u32 = 3;
pub const DATA_PACKET_TRANSMITTED_INTERRUPT: u32 = 2;
pub const DATA_OUT_PING_TOKEN_INTERRUPT: u32 = 1;
pub const DATA_IN_TOKEN_INTERRUPT: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
