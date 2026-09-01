/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2012  AudioScience Inc. <support@audioscience.com>


HPI internal definitions

(C) Copyright AudioScience Inc. 1996-2009
******************************************************************************/

use core::ffi::{c_char, c_void};

pub type hpi_handler_func =
    Option<unsafe extern "C" fn(*mut hpi_message, *mut hpi_response)>;

/** maximum number of memory regions mapped to an adapter */
pub const HPI_MAX_ADAPTER_MEM_SPACES: usize = 2;

/* Each OS needs its own hpios.h */
/* physical memory allocation */

unsafe extern "C" {
    /** Allocate and map an area of locked memory for bus master DMA operations.

    On success, *pLockedMemeHandle is a valid handle, and 0 is returned
    On error *pLockedMemHandle marked invalid, non-zero returned.

    If this function succeeds, then HpiOs_LockedMem_GetVirtAddr() and
    HpiOs_LockedMem_GetPyhsAddr() will always succed on the returned handle.
    */
    pub fn hpios_locked_mem_alloc(
        p_locked_mem_handle: *mut consistent_dma_area,
        size: u32,
        p_os_reference: *mut pci_dev,
    ) -> u16;

    /** Free mapping and memory represented by LockedMemHandle

    Frees any resources, then invalidates the handle.
    Returns 0 on success, 1 if handle is invalid.

    */
    pub fn hpios_locked_mem_free(locked_mem_handle: *mut consistent_dma_area) -> u16;

    /** Get the physical PCI address of memory represented by LockedMemHandle.

    If handle is invalid *pPhysicalAddr is set to zero and return 1
    */
    pub fn hpios_locked_mem_get_phys_addr(
        locked_mem_handle: *mut consistent_dma_area,
        p_physical_addr: *mut u32,
    ) -> u16;

    /** Get the CPU address of memory represented by LockedMemHandle.

    If handle is NULL *ppvVirtualAddr is set to NULL and return 1
    */
    pub fn hpios_locked_mem_get_virt_addr(
        locked_mem_handle: *mut consistent_dma_area,
        ppv_virtual_addr: *mut *mut c_void,
    ) -> u16;

    /** Check that handle is valid
    i.e it represents a valid memory area
    */
    pub fn hpios_locked_mem_valid(locked_mem_handle: *mut consistent_dma_area) -> u16;

    /* timing/delay */
    pub fn hpios_delay_micro_seconds(num_micro_sec: u32);
}

/* If the assert fails, compiler complains
   something like size of array `msg' is negative.
   Unlike linux BUILD_BUG_ON, this works outside function scope.
*/
#[macro_export]
macro_rules! compile_time_assert {
    ($cond:expr, $msg:ident) => {
        const _: [(); 1] = [(); ($cond) as usize];
    };
}

/******************************************* bus types */
pub const HPI_BUS_ISAPNP: u32 = 1;
pub const HPI_BUS_PCI: u32 = 2;
pub const HPI_BUS_USB: u32 = 3;
pub const HPI_BUS_NET: u32 = 4;

pub const HPI_SUBSYS_OPT_NET_ENABLE: u32 = 257;
pub const HPI_SUBSYS_OPT_NET_BROADCAST: u32 = 258;
pub const HPI_SUBSYS_OPT_NET_UNICAST: u32 = 259;
pub const HPI_SUBSYS_OPT_NET_ADDR: u32 = 260;
pub const HPI_SUBSYS_OPT_NET_MASK: u32 = 261;
pub const HPI_SUBSYS_OPT_NET_ADAPTER_ADDRESS_ADD: u32 = 262;

/** Volume flags
*/
/** Set if the volume control is muted */
pub const HPI_VOLUME_FLAG_MUTED: u32 = 1 << 0;
/** Set if the volume control has a mute function */
pub const HPI_VOLUME_FLAG_HAS_MUTE: u32 = 1 << 1;
/** Set if volume control can do autofading */
pub const HPI_VOLUME_FLAG_HAS_AUTOFADE: u32 = 1 << 2;
/* Note Flags >= (1<<8) are for DSP internal use only */

/******************************************* CONTROL ATTRIBUTES ****/
/* (in order of control type ID */

/* This allows for 255 control types, 256 unique attributes each */
pub const fn HPI_CTL_ATTR(ctl: u32, ai: u32) -> u32 {
    (ctl << 8) + ai
}

/* Get the sub-index of the attribute for a control type */
pub const fn HPI_CTL_ATTR_INDEX(i: u32) -> u32 {
    i & 0xff
}

/* Extract the control from the control attribute */
pub const fn HPI_CTL_ATTR_CONTROL(i: u32) -> u32 {
    i >> 8
}

/** Enable event generation for a control.
0=disable, 1=enable
\note generic to all controls that can generate events
*/

/** Unique identifiers for every control attribute
*/
pub const HPI_GENERIC_ENABLE: u32 = HPI_CTL_ATTR(HPI_CONTROL_GENERIC, 1);
pub const HPI_GENERIC_EVENT_ENABLE: u32 = HPI_CTL_ATTR(HPI_CONTROL_GENERIC, 2);
pub const HPI_VOLUME_GAIN: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 1);
pub const HPI_VOLUME_AUTOFADE: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 2);
pub const HPI_VOLUME_MUTE: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 3);
pub const HPI_VOLUME_GAIN_AND_FLAGS: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 4);
pub const HPI_VOLUME_NUM_CHANNELS: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 6);
pub const HPI_VOLUME_RANGE: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOLUME, 10);
pub const HPI_METER_RMS: u32 = HPI_CTL_ATTR(HPI_CONTROL_METER, 1);
pub const HPI_METER_PEAK: u32 = HPI_CTL_ATTR(HPI_CONTROL_METER, 2);
pub const HPI_METER_RMS_BALLISTICS: u32 = HPI_CTL_ATTR(HPI_CONTROL_METER, 3);
pub const HPI_METER_PEAK_BALLISTICS: u32 = HPI_CTL_ATTR(HPI_CONTROL_METER, 4);
pub const HPI_METER_NUM_CHANNELS: u32 = HPI_CTL_ATTR(HPI_CONTROL_METER, 5);
pub const HPI_MULTIPLEXER_SOURCE: u32 = HPI_CTL_ATTR(HPI_CONTROL_MULTIPLEXER, 1);
pub const HPI_MULTIPLEXER_QUERYSOURCE: u32 = HPI_CTL_ATTR(HPI_CONTROL_MULTIPLEXER, 2);
pub const HPI_AESEBUTX_FORMAT: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBUTX, 1);
pub const HPI_AESEBUTX_SAMPLERATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBUTX, 3);
pub const HPI_AESEBUTX_CHANNELSTATUS: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBUTX, 4);
pub const HPI_AESEBUTX_USERDATA: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBUTX, 5);
pub const HPI_AESEBURX_FORMAT: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBURX, 1);
pub const HPI_AESEBURX_ERRORSTATUS: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBURX, 2);
pub const HPI_AESEBURX_SAMPLERATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBURX, 3);
pub const HPI_AESEBURX_CHANNELSTATUS: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBURX, 4);
pub const HPI_AESEBURX_USERDATA: u32 = HPI_CTL_ATTR(HPI_CONTROL_AESEBURX, 5);
pub const HPI_LEVEL_GAIN: u32 = HPI_CTL_ATTR(HPI_CONTROL_LEVEL, 1);
pub const HPI_LEVEL_RANGE: u32 = HPI_CTL_ATTR(HPI_CONTROL_LEVEL, 10);
pub const HPI_TUNER_BAND: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 1);
pub const HPI_TUNER_FREQ: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 2);
pub const HPI_TUNER_LEVEL_AVG: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 3);
pub const HPI_TUNER_LEVEL_RAW: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 4);
pub const HPI_TUNER_SNR: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 5);
pub const HPI_TUNER_GAIN: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 6);
pub const HPI_TUNER_STATUS: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 7);
pub const HPI_TUNER_MODE: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 8);
pub const HPI_TUNER_RDS: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 9);
pub const HPI_TUNER_DEEMPHASIS: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 10);
pub const HPI_TUNER_PROGRAM: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 11);
pub const HPI_TUNER_HDRADIO_SIGNAL_QUALITY: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 12);
pub const HPI_TUNER_HDRADIO_SDK_VERSION: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 13);
pub const HPI_TUNER_HDRADIO_DSP_VERSION: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 14);
pub const HPI_TUNER_HDRADIO_BLEND: u32 = HPI_CTL_ATTR(HPI_CONTROL_TUNER, 15);
pub const HPI_VOX_THRESHOLD: u32 = HPI_CTL_ATTR(HPI_CONTROL_VOX, 1);
pub const HPI_CHANNEL_MODE_MODE: u32 = HPI_CTL_ATTR(HPI_CONTROL_CHANNEL_MODE, 1);
pub const HPI_BITSTREAM_DATA_POLARITY: u32 = HPI_CTL_ATTR(HPI_CONTROL_BITSTREAM, 1);
pub const HPI_BITSTREAM_CLOCK_EDGE: u32 = HPI_CTL_ATTR(HPI_CONTROL_BITSTREAM, 2);
pub const HPI_BITSTREAM_CLOCK_SOURCE: u32 = HPI_CTL_ATTR(HPI_CONTROL_BITSTREAM, 3);
pub const HPI_BITSTREAM_ACTIVITY: u32 = HPI_CTL_ATTR(HPI_CONTROL_BITSTREAM, 4);
pub const HPI_SAMPLECLOCK_SOURCE: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 1);
pub const HPI_SAMPLECLOCK_SAMPLERATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 2);
pub const HPI_SAMPLECLOCK_SOURCE_INDEX: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 3);
pub const HPI_SAMPLECLOCK_LOCAL_SAMPLERATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 4);
pub const HPI_SAMPLECLOCK_AUTO: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 5);
pub const HPI_SAMPLECLOCK_LOCAL_LOCK: u32 = HPI_CTL_ATTR(HPI_CONTROL_SAMPLECLOCK, 6);
pub const HPI_MICROPHONE_PHANTOM_POWER: u32 = HPI_CTL_ATTR(HPI_CONTROL_MICROPHONE, 1);
pub const HPI_EQUALIZER_NUM_FILTERS: u32 = HPI_CTL_ATTR(HPI_CONTROL_EQUALIZER, 1);
pub const HPI_EQUALIZER_FILTER: u32 = HPI_CTL_ATTR(HPI_CONTROL_EQUALIZER, 2);
pub const HPI_EQUALIZER_COEFFICIENTS: u32 = HPI_CTL_ATTR(HPI_CONTROL_EQUALIZER, 3);
pub const HPI_COMPANDER_PARAMS: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 1);
pub const HPI_COMPANDER_MAKEUPGAIN: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 2);
pub const HPI_COMPANDER_THRESHOLD: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 3);
pub const HPI_COMPANDER_RATIO: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 4);
pub const HPI_COMPANDER_ATTACK: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 5);
pub const HPI_COMPANDER_DECAY: u32 = HPI_CTL_ATTR(HPI_CONTROL_COMPANDER, 6);
pub const HPI_COBRANET_SET: u32 = HPI_CTL_ATTR(HPI_CONTROL_COBRANET, 1);
pub const HPI_COBRANET_GET: u32 = HPI_CTL_ATTR(HPI_CONTROL_COBRANET, 2);
pub const HPI_COBRANET_GET_STATUS: u32 = HPI_CTL_ATTR(HPI_CONTROL_COBRANET, 5);
pub const HPI_COBRANET_SEND_PACKET: u32 = HPI_CTL_ATTR(HPI_CONTROL_COBRANET, 6);
pub const HPI_COBRANET_GET_PACKET: u32 = HPI_CTL_ATTR(HPI_CONTROL_COBRANET, 7);
pub const HPI_TONEDETECTOR_THRESHOLD: u32 = HPI_CTL_ATTR(HPI_CONTROL_TONEDETECTOR, 1);
pub const HPI_TONEDETECTOR_STATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_TONEDETECTOR, 2);
pub const HPI_TONEDETECTOR_FREQUENCY: u32 = HPI_CTL_ATTR(HPI_CONTROL_TONEDETECTOR, 3);
pub const HPI_SILENCEDETECTOR_THRESHOLD: u32 = HPI_CTL_ATTR(HPI_CONTROL_SILENCEDETECTOR, 1);
pub const HPI_SILENCEDETECTOR_STATE: u32 = HPI_CTL_ATTR(HPI_CONTROL_SILENCEDETECTOR, 2);
pub const HPI_SILENCEDETECTOR_DELAY: u32 = HPI_CTL_ATTR(HPI_CONTROL_SILENCEDETECTOR, 3);
pub const HPI_PAD_CHANNEL_NAME: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 1);
pub const HPI_PAD_ARTIST: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 2);
pub const HPI_PAD_TITLE: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 3);
pub const HPI_PAD_COMMENT: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 4);
pub const HPI_PAD_PROGRAM_TYPE: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 5);
pub const HPI_PAD_PROGRAM_ID: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 6);
pub const HPI_PAD_TA_SUPPORT: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 7);
pub const HPI_PAD_TA_ACTIVE: u32 = HPI_CTL_ATTR(HPI_CONTROL_PAD, 8);
pub const HPI_UNIVERSAL_ENTITY: u32 = HPI_CTL_ATTR(HPI_CONTROL_UNIVERSAL, 1);

pub const HPI_POLARITY_POSITIVE: u32 = 0;
pub const HPI_POLARITY_NEGATIVE: u32 = 1;

/*------------------------------------------------------------
 Cobranet Chip Bridge - copied from HMI.H
------------------------------------------------------------*/
pub const HPI_COBRANET_HMI_cobra_bridge: u32 = 0x20000;
pub const HPI_COBRANET_HMI_cobra_bridge_tx_pkt_buf: u32 =
    HPI_COBRANET_HMI_cobra_bridge + 0x1000;
pub const HPI_COBRANET_HMI_cobra_bridge_rx_pkt_buf: u32 =
    HPI_COBRANET_HMI_cobra_bridge + 0x2000;
pub const HPI_COBRANET_HMI_cobra_if_table1: u32 = 0x110000;
pub const HPI_COBRANET_HMI_cobra_if_phy_address: u32 =
    HPI_COBRANET_HMI_cobra_if_table1 + 0xd;
pub const HPI_COBRANET_HMI_cobra_protocolIP: u32 = 0x72000;
pub const HPI_COBRANET_HMI_cobra_ip_mon_currentIP: u32 =
    HPI_COBRANET_HMI_cobra_protocolIP;
pub const HPI_COBRANET_HMI_cobra_ip_mon_staticIP: u32 =
    HPI_COBRANET_HMI_cobra_protocolIP + 0x2;
pub const HPI_COBRANET_HMI_cobra_sys: u32 = 0x100000;
pub const HPI_COBRANET_HMI_cobra_sys_desc: u32 = HPI_COBRANET_HMI_cobra_sys;
pub const HPI_COBRANET_HMI_cobra_sys_objectID: u32 = HPI_COBRANET_HMI_cobra_sys + 0x100;
pub const HPI_COBRANET_HMI_cobra_sys_contact: u32 = HPI_COBRANET_HMI_cobra_sys + 0x200;
pub const HPI_COBRANET_HMI_cobra_sys_name: u32 = HPI_COBRANET_HMI_cobra_sys + 0x300;
pub const HPI_COBRANET_HMI_cobra_sys_location: u32 = HPI_COBRANET_HMI_cobra_sys + 0x400;

/*------------------------------------------------------------
 Cobranet Chip Status bits
------------------------------------------------------------*/
pub const HPI_COBRANET_HMI_STATUS_RXPACKET: u32 = 2;
pub const HPI_COBRANET_HMI_STATUS_TXPACKET: u32 = 3;

/*------------------------------------------------------------
 Ethernet header size
------------------------------------------------------------*/
pub const HPI_ETHERNET_HEADER_SIZE: usize = 16;

/** ID supplied by Cirrus for ASI packets. */
pub const HPI_ETHERNET_PACKET_ID: u32 = 0x85;
/** Simple packet - no special routing required */
pub const HPI_ETHERNET_PACKET_V1: u32 = 0x01;
/** This packet must make its way to the host across the HPI interface */
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HMI: u32 = 0x20;
/** This packet must make its way to the host across the HPI interface */
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HMI_V1: u32 = 0x21;
/** This packet must make its way to the host across the HPI interface */
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HPI: u32 = 0x40;
/** This packet must make its way to the host across the HPI interface */
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HPI_V1: u32 = 0x41;

pub const HPI_ETHERNET_UDP_PORT: u32 = 44600;
/** Default network timeout in milli-seconds. */
pub const HPI_ETHERNET_TIMEOUT_MS: u32 = 500;

/** Locked memory buffer alloc/free phases */
pub const HPI_BUFFER_CMD_EXTERNAL: u32 = 0;
pub const HPI_BUFFER_CMD_INTERNAL_ALLOC: u32 = 1;
pub const HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER: u32 = 2;
pub const HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER: u32 = 3;
pub const HPI_BUFFER_CMD_INTERNAL_FREE: u32 = 4;

/*****************************************************************************/
/*****************************************************************************/
/********               HPI LOW LEVEL MESSAGES                  *******/
/*****************************************************************************/
/*****************************************************************************/
/** Pnp ids */
/** "ASI"  - actual is "ASX" - need to change */
pub const HPI_ID_ISAPNP_AUDIOSCIENCE: u32 = 0x0669;
/** PCI vendor ID that AudioScience uses */
pub const HPI_PCI_VENDOR_ID_AUDIOSCIENCE: u32 = 0x175C;
/** PCI vendor ID that the DSP56301 has */
pub const HPI_PCI_VENDOR_ID_MOTOROLA: u32 = 0x1057;
/** PCI vendor ID that TI uses */
pub const HPI_PCI_VENDOR_ID_TI: u32 = 0x104C;
pub const HPI_PCI_DEV_ID_PCI2040: u32 = 0xAC60;
/** TI's C6205 PCI interface has this ID */
pub const HPI_PCI_DEV_ID_DSP6205: u32 = 0xA106;
pub const HPI_USB_VENDOR_ID_AUDIOSCIENCE: u32 = 0x1257;
pub const HPI_USB_W2K_TAG: u32 = 0x57495341; /* "ASIW"       */
pub const HPI_USB_LINUX_TAG: u32 = 0x4C495341; /* "ASIL"       */

/** Invalid Adapter index
Used in HPI messages that are not addressed to a specific adapter
Used in DLL to indicate device not present
*/
pub const HPI_ADAPTER_INDEX_INVALID: u32 = 0xFFFF;

/** First 2 hex digits define the adapter family */
pub const HPI_ADAPTER_FAMILY_MASK: u32 = 0xff00;
pub const HPI_MODULE_FAMILY_MASK: u32 = 0xfff0;

pub const fn HPI_ADAPTER_FAMILY_ASI(f: u32) -> u32 {
    f & HPI_ADAPTER_FAMILY_MASK
}
pub const fn HPI_MODULE_FAMILY_ASI(f: u32) -> u32 {
    f & HPI_MODULE_FAMILY_MASK
}
pub const fn HPI_ADAPTER_ASI(f: u32) -> u32 {
    f
}

pub const HPI_TYPE_REQUEST: u32 = 1;
pub const HPI_TYPE_RESPONSE: u32 = 2;
pub const HPI_TYPE_DATA: u32 = 3;
pub const HPI_TYPE_SSX2BYPASS_MESSAGE: u32 = 4;
pub const HPI_TYPE_COMMAND: u32 = 5;
pub const HPI_TYPE_NOTIFICATION: u32 = 6;

pub const HPI_OBJ_SUBSYSTEM: u32 = 1;
pub const HPI_OBJ_ADAPTER: u32 = 2;
pub const HPI_OBJ_OSTREAM: u32 = 3;
pub const HPI_OBJ_ISTREAM: u32 = 4;
pub const HPI_OBJ_MIXER: u32 = 5;
pub const HPI_OBJ_NODE: u32 = 6;
pub const HPI_OBJ_CONTROL: u32 = 7;
pub const HPI_OBJ_NVMEMORY: u32 = 8;
pub const HPI_OBJ_GPIO: u32 = 9;
pub const HPI_OBJ_WATCHDOG: u32 = 10;
pub const HPI_OBJ_CLOCK: u32 = 11;
pub const HPI_OBJ_PROFILE: u32 = 12;
/* HPI_ OBJ_ CONTROLEX  = 13, */
pub const HPI_OBJ_ASYNCEVENT: u32 = 14;
pub const HPI_OBJ_MAXINDEX: u32 = 14;

pub const HPI_OBJ_FUNCTION_SPACING: u32 = 0x100;
pub const fn HPI_FUNC_ID(obj: u32, i: u32) -> u32 {
    obj * HPI_OBJ_FUNCTION_SPACING + i
}

pub const fn HPI_EXTRACT_INDEX(fn_: u32) -> u32 {
    fn_ & 0xff
}

pub const HPI_SUBSYS_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 1);
pub const HPI_SUBSYS_GET_VERSION: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 2);
pub const HPI_SUBSYS_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 3);
pub const HPI_SUBSYS_CREATE_ADAPTER: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 5);
pub const HPI_SUBSYS_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 6);
pub const HPI_SUBSYS_DRIVER_LOAD: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 8);
pub const HPI_SUBSYS_DRIVER_UNLOAD: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 9);
pub const HPI_SUBSYS_GET_NUM_ADAPTERS: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 12);
pub const HPI_SUBSYS_GET_ADAPTER: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 13);
pub const HPI_SUBSYS_SET_NETWORK_INTERFACE: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 14);
pub const HPI_SUBSYS_OPTION_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 15);
pub const HPI_SUBSYS_OPTION_GET: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 16);
pub const HPI_SUBSYS_OPTION_SET: u32 = HPI_FUNC_ID(HPI_OBJ_SUBSYSTEM, 17);
pub const HPI_SUBSYS_FUNCTION_COUNT: u32 = 17;

pub const HPI_ADAPTER_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 1);
pub const HPI_ADAPTER_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 2);
pub const HPI_ADAPTER_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 3);
pub const HPI_ADAPTER_GET_ASSERT: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 4);
pub const HPI_ADAPTER_TEST_ASSERT: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 5);
pub const HPI_ADAPTER_SET_MODE: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 6);
pub const HPI_ADAPTER_GET_MODE: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 7);
pub const HPI_ADAPTER_ENABLE_CAPABILITY: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 8);
pub const HPI_ADAPTER_SELFTEST: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 9);
pub const HPI_ADAPTER_FIND_OBJECT: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 10);
pub const HPI_ADAPTER_QUERY_FLASH: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 11);
pub const HPI_ADAPTER_START_FLASH: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 12);
pub const HPI_ADAPTER_PROGRAM_FLASH: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 13);
pub const HPI_ADAPTER_SET_PROPERTY: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 14);
pub const HPI_ADAPTER_GET_PROPERTY: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 15);
pub const HPI_ADAPTER_ENUM_PROPERTY: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 16);
pub const HPI_ADAPTER_MODULE_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 17);
pub const HPI_ADAPTER_DEBUG_READ: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 18);
pub const HPI_ADAPTER_IRQ_QUERY_AND_CLEAR: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 19);
pub const HPI_ADAPTER_IRQ_CALLBACK: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 20);
pub const HPI_ADAPTER_DELETE: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 21);
pub const HPI_ADAPTER_READ_FLASH: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 22);
pub const HPI_ADAPTER_END_FLASH: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 23);
pub const HPI_ADAPTER_FILESTORE_DELETE_ALL: u32 = HPI_FUNC_ID(HPI_OBJ_ADAPTER, 24);
pub const HPI_ADAPTER_FUNCTION_COUNT: u32 = 24;

pub const HPI_OSTREAM_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 1);
pub const HPI_OSTREAM_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 2);
pub const HPI_OSTREAM_WRITE: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 3);
pub const HPI_OSTREAM_START: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 4);
pub const HPI_OSTREAM_STOP: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 5);
pub const HPI_OSTREAM_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 6);
pub const HPI_OSTREAM_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 7);
pub const HPI_OSTREAM_QUERY_FORMAT: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 8);
pub const HPI_OSTREAM_DATA: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 9);
pub const HPI_OSTREAM_SET_VELOCITY: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 10);
pub const HPI_OSTREAM_SET_PUNCHINOUT: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 11);
pub const HPI_OSTREAM_SINEGEN: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 12);
pub const HPI_OSTREAM_ANC_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 13);
pub const HPI_OSTREAM_ANC_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 14);
pub const HPI_OSTREAM_ANC_READ: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 15);
pub const HPI_OSTREAM_SET_TIMESCALE: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 16);
pub const HPI_OSTREAM_SET_FORMAT: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 17);
pub const HPI_OSTREAM_HOSTBUFFER_ALLOC: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 18);
pub const HPI_OSTREAM_HOSTBUFFER_FREE: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 19);
pub const HPI_OSTREAM_GROUP_ADD: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 20);
pub const HPI_OSTREAM_GROUP_GETMAP: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 21);
pub const HPI_OSTREAM_GROUP_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 22);
pub const HPI_OSTREAM_HOSTBUFFER_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 23);
pub const HPI_OSTREAM_WAIT_START: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 24);
pub const HPI_OSTREAM_WAIT: u32 = HPI_FUNC_ID(HPI_OBJ_OSTREAM, 25);
pub const HPI_OSTREAM_FUNCTION_COUNT: u32 = 25;

pub const HPI_ISTREAM_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 1);
pub const HPI_ISTREAM_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 2);
pub const HPI_ISTREAM_SET_FORMAT: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 3);
pub const HPI_ISTREAM_READ: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 4);
pub const HPI_ISTREAM_START: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 5);
pub const HPI_ISTREAM_STOP: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 6);
pub const HPI_ISTREAM_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 7);
pub const HPI_ISTREAM_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 8);
pub const HPI_ISTREAM_QUERY_FORMAT: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 9);
pub const HPI_ISTREAM_ANC_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 10);
pub const HPI_ISTREAM_ANC_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 11);
pub const HPI_ISTREAM_ANC_WRITE: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 12);
pub const HPI_ISTREAM_HOSTBUFFER_ALLOC: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 13);
pub const HPI_ISTREAM_HOSTBUFFER_FREE: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 14);
pub const HPI_ISTREAM_GROUP_ADD: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 15);
pub const HPI_ISTREAM_GROUP_GETMAP: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 16);
pub const HPI_ISTREAM_GROUP_RESET: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 17);
pub const HPI_ISTREAM_HOSTBUFFER_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 18);
pub const HPI_ISTREAM_WAIT_START: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 19);
pub const HPI_ISTREAM_WAIT: u32 = HPI_FUNC_ID(HPI_OBJ_ISTREAM, 20);
pub const HPI_ISTREAM_FUNCTION_COUNT: u32 = 20;

/* NOTE:
   GET_NODE_INFO, SET_CONNECTION, GET_CONNECTIONS are not currently used */
pub const HPI_MIXER_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 1);
pub const HPI_MIXER_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 2);
pub const HPI_MIXER_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 3);
pub const HPI_MIXER_GET_NODE_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 4);
pub const HPI_MIXER_GET_CONTROL: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 5);
pub const HPI_MIXER_SET_CONNECTION: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 6);
pub const HPI_MIXER_GET_CONNECTIONS: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 7);
pub const HPI_MIXER_GET_CONTROL_BY_INDEX: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 8);
pub const HPI_MIXER_GET_CONTROL_ARRAY_BY_INDEX: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 9);
pub const HPI_MIXER_GET_CONTROL_MULTIPLE_VALUES: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 10);
pub const HPI_MIXER_STORE: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 11);
pub const HPI_MIXER_GET_CACHE_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 12);
pub const HPI_MIXER_GET_BLOCK_HANDLE: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 13);
pub const HPI_MIXER_GET_PARAMETER_HANDLE: u32 = HPI_FUNC_ID(HPI_OBJ_MIXER, 14);
pub const HPI_MIXER_FUNCTION_COUNT: u32 = 14;

pub const HPI_CONTROL_GET_INFO: u32 = HPI_FUNC_ID(HPI_OBJ_CONTROL, 1);
pub const HPI_CONTROL_GET_STATE: u32 = HPI_FUNC_ID(HPI_OBJ_CONTROL, 2);
pub const HPI_CONTROL_SET_STATE: u32 = HPI_FUNC_ID(HPI_OBJ_CONTROL, 3);
pub const HPI_CONTROL_FUNCTION_COUNT: u32 = 3;

pub const HPI_NVMEMORY_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_NVMEMORY, 1);
pub const HPI_NVMEMORY_READ_BYTE: u32 = HPI_FUNC_ID(HPI_OBJ_NVMEMORY, 2);
pub const HPI_NVMEMORY_WRITE_BYTE: u32 = HPI_FUNC_ID(HPI_OBJ_NVMEMORY, 3);
pub const HPI_NVMEMORY_FUNCTION_COUNT: u32 = 3;

pub const HPI_GPIO_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_GPIO, 1);
pub const HPI_GPIO_READ_BIT: u32 = HPI_FUNC_ID(HPI_OBJ_GPIO, 2);
pub const HPI_GPIO_WRITE_BIT: u32 = HPI_FUNC_ID(HPI_OBJ_GPIO, 3);
pub const HPI_GPIO_READ_ALL: u32 = HPI_FUNC_ID(HPI_OBJ_GPIO, 4);
pub const HPI_GPIO_WRITE_STATUS: u32 = HPI_FUNC_ID(HPI_OBJ_GPIO, 5);
pub const HPI_GPIO_FUNCTION_COUNT: u32 = 5;

pub const HPI_ASYNCEVENT_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 1);
pub const HPI_ASYNCEVENT_CLOSE: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 2);
pub const HPI_ASYNCEVENT_WAIT: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 3);
pub const HPI_ASYNCEVENT_GETCOUNT: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 4);
pub const HPI_ASYNCEVENT_GET: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 5);
pub const HPI_ASYNCEVENT_SENDEVENTS: u32 = HPI_FUNC_ID(HPI_OBJ_ASYNCEVENT, 6);
pub const HPI_ASYNCEVENT_FUNCTION_COUNT: u32 = 6;

pub const HPI_WATCHDOG_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_WATCHDOG, 1);
pub const HPI_WATCHDOG_SET_TIME: u32 = HPI_FUNC_ID(HPI_OBJ_WATCHDOG, 2);
pub const HPI_WATCHDOG_PING: u32 = HPI_FUNC_ID(HPI_OBJ_WATCHDOG, 3);
pub const HPI_CLOCK_OPEN: u32 = HPI_FUNC_ID(HPI_OBJ_CLOCK, 1);
pub const HPI_CLOCK_SET_TIME: u32 = HPI_FUNC_ID(HPI_OBJ_CLOCK, 2);
pub const HPI_CLOCK_GET_TIME: u32 = HPI_FUNC_ID(HPI_OBJ_CLOCK, 3);
pub const HPI_PROFILE_OPEN_ALL: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 1);
pub const HPI_PROFILE_START_ALL: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 2);
pub const HPI_PROFILE_STOP_ALL: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 3);
pub const HPI_PROFILE_GET: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 4);
pub const HPI_PROFILE_GET_IDLECOUNT: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 5);
pub const HPI_PROFILE_GET_NAME: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 6);
pub const HPI_PROFILE_GET_UTILIZATION: u32 = HPI_FUNC_ID(HPI_OBJ_PROFILE, 7);
pub const HPI_PROFILE_FUNCTION_COUNT: u32 = 7;

/* ////////////////////////////////////////////////////////////////////// */
/* STRUCTURES */
/* C used #pragma pack(push, 1) unless DISABLE_PRAGMA_PACK1 was defined. */

/** PCI bus resource */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_pci {
    pub ap_mem_base: [*mut u32; HPI_MAX_ADAPTER_MEM_SPACES],
    pub pci_dev: *mut pci_dev,
}

/** Adapter specification resource */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_specification {
    pub type_: u32,
    pub modules: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_resource_r {
    pub pci: *const hpi_pci,
    pub net_if: *const c_char,
    pub adapter_spec: hpi_adapter_specification,
    pub sw_if: *const c_void,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_resource {
    pub r: hpi_resource_r,
    pub bus_type: u16,
    pub padding: u16,
}

/** Format info used inside struct hpi_message
    Not the same as public API struct hpi_format */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_format {
    pub sample_rate: u32,
    pub bit_rate: u32,
    pub attributes: u32,
    pub channels: u16,
    pub format: u16,
}

/**  Buffer+format structure.
     Must be kept 7 * 32 bits to match public struct hpi_datastruct */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_data {
    pub format: hpi_msg_format,
    pub pb_data: *mut u8,
    /* #ifndef CONFIG_64BIT */
    pub padding: u32,
    /* #endif */
    pub data_size: u32,
}

/** struct hpi_datastructure used up to 3.04 driver */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_data_legacy32 {
    pub format: hpi_format,
    pub pb_data: u32,
    pub data_size: u32,
}

/* #ifdef CONFIG_64BIT */
/* Compatibility version of struct hpi_data*/
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_data_compat32 {
    pub format: hpi_msg_format,
    pub pb_data: u32,
    pub padding: u32,
    pub data_size: u32,
}
/* #endif */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_buffer {
    /** placeholder for backward compatibility (see dwBufferSize) */
    pub reserved: hpi_msg_format,
    pub command: u32,
    pub pci_address: u32,
    pub buffer_size: u32,
}

/*/////////////////////////////////////////////////////////////////////////// */
/* This is used for background buffer bus mastering stream buffers.           */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_hostbuffer_status {
    pub samples_processed: u32,
    pub auxiliary_data_available: u32,
    pub stream_state: u32,
    /* DSP index in to the host bus master buffer. */
    pub dsp_index: u32,
    /* Host index in to the host bus master buffer. */
    pub host_index: u32,
    pub size_in_bytes: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_streamid {
    pub object_type: u16,
    pub stream_index: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_punchinout {
    pub punch_in_sample: u32,
    pub punch_out_sample: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_msg {
    pub resource: hpi_resource,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_res {
    pub version: u32,
    pub data: u32,
    pub num_adapters: u16,
    pub adapter_index: u16,
    pub adapter_type: u16,
    pub pad16: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_debug_read {
    pub dsp_address: u32,
    pub count_bytes: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_mode {
    pub adapter_mode: u32,
    pub query_or_set: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_module_info {
    pub index: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_property_enum {
    pub index: u16,
    pub what: u16,
    pub property_index: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_property_set {
    pub property: u16,
    pub parameter1: u16,
    pub parameter2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_restart {
    pub pad32: u32,
    pub key1: u16,
    pub key2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_test_assert {
    pub pad32: u32,
    pub value: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_msg_irq {
    pub message: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_adapterx_msg {
    pub debug_read: hpi_adapterx_msg_debug_read,
    pub mode: hpi_adapterx_msg_mode,
    pub module_info: hpi_adapterx_msg_module_info,
    pub property_enum: hpi_adapterx_msg_property_enum,
    pub property_set: hpi_adapterx_msg_property_set,
    pub restart: hpi_adapterx_msg_restart,
    pub test_assert: hpi_adapterx_msg_test_assert,
    pub irq: hpi_adapterx_msg_irq,
    pub pad: [u32; 3],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_res {
    pub serial_number: u32,
    pub adapter_type: u16,
    pub adapter_index: u16,
    pub num_instreams: u16,
    pub num_outstreams: u16,
    pub num_mixers: u16,
    pub version: u16,
    pub sz_adapter_assert: [u8; HPI_STRING_LEN],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_res_assert {
    pub p1: u32,
    pub count: u16,
    pub dsp_index: u16,
    pub p2: u32,
    pub dsp_msg_addr: u32,
    pub sz_message: [c_char; HPI_STRING_LEN],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_res_mode {
    pub adapter_mode: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_res_property_get {
    pub parameter1: u16,
    pub parameter2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_adapterx_res_irq_query {
    pub yes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_adapterx_res {
    pub info: hpi_adapter_res,
    pub assert: hpi_adapterx_res_assert,
    pub mode: hpi_adapterx_res_mode,
    pub property_get: hpi_adapterx_res_property_get,
    pub irq_query: hpi_adapterx_res_irq_query,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_stream_msg_u {
    pub data: hpi_msg_data,
    pub data32: hpi_data_legacy32,
    pub velocity: u16,
    pub pio: hpi_punchinout,
    pub time_scale: u32,
    pub buffer: hpi_buffer,
    pub stream: hpi_streamid,
    pub threshold_bytes: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_msg {
    pub u: hpi_stream_msg_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res_stream_info {
    pub buffer_size: u32,
    pub data_available: u32,
    pub samples_transferred: u32,
    pub auxiliary_data_available: u32,
    pub state: u16,
    pub padding: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res_legacy_stream_info {
    pub buffer_size: u32,
    pub data_available: u32,
    pub samples_transfered: u32,
    pub state: u16,
    pub outstream_index: u16,
    pub instream_index: u16,
    pub padding: u16,
    pub auxiliary_data_available: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res_group_info {
    pub outstream_group_map: u32,
    pub instream_group_map: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res_hostbuffer_info {
    pub p_buffer: *mut u8,
    pub p_status: *mut hpi_hostbuffer_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_stream_res_u {
    pub stream_info: hpi_stream_res_stream_info,
    pub legacy_stream_info: hpi_stream_res_legacy_stream_info,
    pub group_info: hpi_stream_res_group_info,
    pub hostbuffer_info: hpi_stream_res_hostbuffer_info,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res {
    pub u: hpi_stream_res_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixer_msg {
    pub control_index: u16,
    pub control_type: u16,
    pub padding1: u16,
    pub node_type1: u16,
    pub node_index1: u16,
    pub node_type2: u16,
    pub node_index2: u16,
    pub padding2: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixer_res {
    pub src_node_type: u16,
    pub src_node_index: u16,
    pub dst_node_type: u16,
    pub dst_node_index: u16,
    pub control_index: u16,
    pub dsp_index: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixerx_msg_gcabi {
    pub starting_index: u16,
    pub flags: u16,
    pub length_in_bytes: u32,
    pub p_data: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixerx_msg_store {
    pub command: u16,
    pub index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_mixerx_msg {
    pub gcabi: hpi_mixerx_msg_gcabi,
    pub store: hpi_mixerx_msg_store,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixerx_res_gcabi {
    pub bytes_returned: u32,
    pub p_data: u32,
    pub more_to_do: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_mixerx_res_cache_info {
    pub total_controls: u32,
    pub cache_controls: u32,
    pub cache_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_mixerx_res {
    pub gcabi: hpi_mixerx_res_gcabi,
    pub cache_info: hpi_mixerx_res_cache_info,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_msg {
    pub attribute: u16,
    pub saved_index: u16,
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [i16; HPI_MAX_CHANNELS],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_msg_old {
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [i16; HPI_MAX_CHANNELS],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_msg_tuner_mode {
    pub mode: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_msg_tuner {
    pub frequency: u32,
    pub gain: u32,
    pub band: u32,
    pub deemphasis: u32,
    pub program: u32,
    pub mode: hpi_control_union_msg_tuner_mode,
    pub blend: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_msg_u {
    pub old: hpi_control_union_msg_old,
    pub tuner: hpi_control_union_msg_tuner,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_msg {
    pub attribute: u16,
    pub saved_index: u16,
    pub u: hpi_control_union_msg_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_res {
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [i16; HPI_MAX_CHANNELS],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_res_old {
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [i16; HPI_MAX_CHANNELS],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_res_tuner_rds {
    pub data: [u32; 2],
    pub bLER: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_res_tuner_status {
    pub value: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_res_tuner {
    pub band: u32,
    pub frequency: u32,
    pub gain: u32,
    pub deemphasis: u32,
    pub rds: hpi_control_union_res_tuner_rds,
    pub s_level: i16,
    pub status: hpi_control_union_res_tuner_status,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_res_chars8 {
    pub sz_data: [c_char; 8],
    pub remaining_chars: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_res_cobranet_status {
    pub status: u32,
    pub readable_size: u32,
    pub writeable_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_res_cobranet {
    pub status: hpi_control_union_res_cobranet_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_res {
    pub old: hpi_control_union_res_old,
    pub tuner: hpi_control_union_res_tuner,
    pub chars8: hpi_control_union_res_chars8,
    pub c_data12: [c_char; 12],
    pub cobranet: hpi_control_union_res_cobranet,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_nvmemory_msg {
    pub address: u16,
    pub data: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_nvmemory_res {
    pub size_in_bytes: u16,
    pub data: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_gpio_msg {
    pub bit_index: u16,
    pub bit_data: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_gpio_res {
    pub number_input_bits: u16,
    pub number_output_bits: u16,
    pub bit_data: [u16; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_async_msg {
    pub events: u32,
    pub maximum_events: u16,
    pub padding: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_async_res_count {
    pub count: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_async_res_get {
    pub events: u32,
    pub number_returned: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_async_res_u {
    pub count: hpi_async_res_count,
    pub get: hpi_async_res_get,
    pub event: hpi_async_event,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_async_res {
    pub u: hpi_async_res_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_watchdog_msg {
    pub time_ms: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_watchdog_res {
    pub time_ms: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_clock_msg {
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub milli_seconds: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_clock_res {
    pub size_in_bytes: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub milli_seconds: u16,
    pub padding: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_profile_msg {
    pub bin_index: u16,
    pub padding: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_open {
    pub max_profiles: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_time {
    pub total_tick_count: u32,
    pub call_count: u32,
    pub max_tick_count: u32,
    pub ticks_per_millisecond: u32,
    pub profile_interval: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_name {
    pub sz_name: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_profile_res_u {
    pub o: hpi_profile_res_open,
    pub t: hpi_profile_res_time,
    pub n: hpi_profile_res_name,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res {
    pub u: hpi_profile_res_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_message_header {
    pub size: u16,
    pub type_: u8,
    pub version: u8,
    pub object: u16,
    pub function: u16,
    pub adapter_index: u16,
    pub obj_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_message_u {
    pub s: hpi_subsys_msg,
    pub ax: hpi_adapterx_msg,
    pub d: hpi_stream_msg,
    pub m: hpi_mixer_msg,
    pub mx: hpi_mixerx_msg,
    pub c: hpi_control_msg,
    pub cu: hpi_control_union_msg,
    pub n: hpi_nvmemory_msg,
    pub l: hpi_gpio_msg,
    pub w: hpi_watchdog_msg,
    pub t: hpi_clock_msg,
    pub p: hpi_profile_msg,
    pub as_: hpi_async_msg,
    pub fixed_size: [c_char; 32],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_message {
    pub size: u16,
    pub type_: u8,
    pub version: u8,
    pub object: u16,
    pub function: u16,
    pub adapter_index: u16,
    pub obj_index: u16,
    pub u: hpi_message_u,
}

pub const HPI_MESSAGE_SIZE_BY_OBJECT: [usize; 15] = [
    core::mem::size_of::<hpi_message_header>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_subsys_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_adapterx_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_stream_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_stream_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_mixer_msg>(),
    core::mem::size_of::<hpi_message_header>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_control_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_nvmemory_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_gpio_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_watchdog_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_clock_msg>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_profile_msg>(),
    core::mem::size_of::<hpi_message_header>(),
    core::mem::size_of::<hpi_message_header>() + core::mem::size_of::<hpi_async_msg>(),
];

/*
Note that the wSpecificError error field should be inspected and potentially
reported whenever HPI_ERROR_DSP_COMMUNICATION or HPI_ERROR_DSP_BOOTLOAD is
returned in wError.
*/
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_response_header {
    pub size: u16,
    pub type_: u8,
    pub version: u8,
    pub object: u16,
    pub function: u16,
    pub error: u16,
    pub specific_error: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_response_u {
    pub s: hpi_subsys_res,
    pub ax: hpi_adapterx_res,
    pub d: hpi_stream_res,
    pub m: hpi_mixer_res,
    pub mx: hpi_mixerx_res,
    pub c: hpi_control_res,
    pub cu: hpi_control_union_res,
    pub n: hpi_nvmemory_res,
    pub l: hpi_gpio_res,
    pub w: hpi_watchdog_res,
    pub t: hpi_clock_res,
    pub p: hpi_profile_res,
    pub as_: hpi_async_res,
    pub bytes: [u8; 52],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_response {
    pub size: u16,
    pub type_: u8,
    pub version: u8,
    pub object: u16,
    pub function: u16,
    pub error: u16,
    pub specific_error: u16,
    pub u: hpi_response_u,
}

pub const HPI_RESPONSE_SIZE_BY_OBJECT: [usize; 15] = [
    core::mem::size_of::<hpi_response_header>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_subsys_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_adapterx_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_stream_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_stream_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_mixer_res>(),
    core::mem::size_of::<hpi_response_header>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_control_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_nvmemory_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_gpio_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_watchdog_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_clock_res>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_profile_res>(),
    core::mem::size_of::<hpi_response_header>(),
    core::mem::size_of::<hpi_response_header>() + core::mem::size_of::<hpi_async_res>(),
];

/*********************** version 1 message/response **************************/
pub const HPINET_ETHERNET_DATA_SIZE: usize = 1500;
pub const HPINET_IP_HDR_SIZE: usize = 20;
pub const HPINET_IP_DATA_SIZE: usize = HPINET_ETHERNET_DATA_SIZE - HPINET_IP_HDR_SIZE;
pub const HPINET_UDP_HDR_SIZE: usize = 8;
pub const HPINET_UDP_DATA_SIZE: usize = HPINET_IP_DATA_SIZE - HPINET_UDP_HDR_SIZE;
pub const HPINET_ASI_HDR_SIZE: usize = 2;
pub const HPINET_ASI_DATA_SIZE: usize = HPINET_UDP_DATA_SIZE - HPINET_ASI_HDR_SIZE;
pub const HPI_MAX_PAYLOAD_SIZE: usize = HPINET_ASI_DATA_SIZE - 2;

/* New style message/response, but still V0 compatible */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_adapter_get_info {
    pub h: hpi_message_header,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_res_adapter_get_info {
    pub h: hpi_response_header,
    pub p: hpi_adapter_res,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_res_adapter_debug_read {
    pub h: hpi_response_header,
    pub bytes: [u8; 1024],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmi {
    pub attribute: u16,
    pub padding: u16,
    pub hmi_address: u32,
    pub byte_count: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmiwrite {
    pub h: hpi_message_header,
    pub p: hpi_msg_cobranet_hmi,
    pub bytes: [u8; 256],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmiread {
    pub h: hpi_message_header,
    pub p: hpi_msg_cobranet_hmi,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_res_cobranet_hmiread {
    pub h: hpi_response_header,
    pub byte_count: u32,
    pub bytes: [u8; 256],
}

/* #if 1 */
pub type hpi_message_header_v1 = hpi_message_header;
pub type hpi_response_header_v1 = hpi_response_header;
/* #else V1 headers in Addition to v0 headers omitted by active C preprocessor branch. */

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_msg_payload_v0_u {
    pub s: hpi_subsys_msg,
    pub ax: hpi_adapterx_msg,
    pub d: hpi_stream_msg,
    pub m: hpi_mixer_msg,
    pub mx: hpi_mixerx_msg,
    pub c: hpi_control_msg,
    pub cu: hpi_control_union_msg,
    pub n: hpi_nvmemory_msg,
    pub l: hpi_gpio_msg,
    pub w: hpi_watchdog_msg,
    pub t: hpi_clock_msg,
    pub p: hpi_profile_msg,
    pub as_: hpi_async_msg,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_msg_payload_v0 {
    pub h: hpi_message_header,
    pub u: hpi_msg_payload_v0_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_res_payload_v0_u {
    pub s: hpi_subsys_res,
    pub ax: hpi_adapterx_res,
    pub d: hpi_stream_res,
    pub m: hpi_mixer_res,
    pub mx: hpi_mixerx_res,
    pub c: hpi_control_res,
    pub cu: hpi_control_union_res,
    pub n: hpi_nvmemory_res,
    pub l: hpi_gpio_res,
    pub w: hpi_watchdog_res,
    pub t: hpi_clock_res,
    pub p: hpi_profile_res,
    pub as_: hpi_async_res,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_res_payload_v0 {
    pub h: hpi_response_header,
    pub u: hpi_res_payload_v0_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_message_buffer_v1 {
    pub m0: hpi_message,
    pub h: hpi_message_header_v1,
    pub buf: [u8; HPI_MAX_PAYLOAD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_response_buffer_v1 {
    pub r0: hpi_response,
    pub h: hpi_response_header_v1,
    pub buf: [u8; HPI_MAX_PAYLOAD_SIZE],
}

const _: [(); 1] = [(); (core::mem::size_of::<hpi_message_buffer_v1>() <= HPI_MAX_PAYLOAD_SIZE) as usize];
const _: [(); 1] = [(); (core::mem::size_of::<hpi_response_buffer_v1>() <= HPI_MAX_PAYLOAD_SIZE) as usize];

/*////////////////////////////////////////////////////////////////////////// */
/* declarations for compact control calls  */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_defn {
    pub type_: u8,
    pub channels: u8,
    pub src_node_type: u8,
    pub src_node_index: u8,
    pub dest_node_type: u8,
    pub dest_node_index: u8,
}

/*////////////////////////////////////////////////////////////////////////// */
/* declarations for control caching (internal to HPI<->DSP interaction)      */

/** indicates a cached u16 value is invalid. */
pub const HPI_CACHE_INVALID_UINT16: u32 = 0xFFFF;
/** indicates a cached short value is invalid. */
pub const HPI_CACHE_INVALID_SHORT: i16 = -32768;

/** A compact representation of (part of) a controls state.
Used for efficient transfer of the control state
between DSP and host or across a network
*/
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_info {
    pub control_type: u8,
    pub size_in32bit_words: u8,
    pub control_index: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_vol {
    pub i: hpi_control_cache_info,
    pub an_log: [i16; 2],
    pub flags: u16,
    pub padding: [c_char; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_meter {
    pub i: hpi_control_cache_info,
    pub an_log_peak: [i16; 2],
    pub an_logRMS: [i16; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_channelmode {
    pub i: hpi_control_cache_info,
    pub mode: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_mux {
    pub i: hpi_control_cache_info,
    pub source_node_type: u16,
    pub source_node_index: u16,
    pub temp_padding: [c_char; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_level {
    pub i: hpi_control_cache_info,
    pub an_log: [i16; 2],
    pub temp_padding: [c_char; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_tuner {
    pub i: hpi_control_cache_info,
    pub freq_ink_hz: u32,
    pub band: u16,
    pub s_level_avg: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_aes3rx {
    pub i: hpi_control_cache_info,
    pub error_status: u32,
    pub format: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_aes3tx {
    pub i: hpi_control_cache_info,
    pub format: u32,
    pub temp_padding: [c_char; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_tonedetector {
    pub i: hpi_control_cache_info,
    pub state: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_silencedetector {
    pub i: hpi_control_cache_info,
    pub state: u32,
    pub temp_padding: [c_char; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_sampleclock {
    pub i: hpi_control_cache_info,
    pub source: u16,
    pub source_index: u16,
    pub sample_rate: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_microphone {
    pub i: hpi_control_cache_info,
    pub phantom_state: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_cache_single_u {
    pub i: hpi_control_cache_info,
    pub vol: hpi_control_cache_vol,
    pub meter: hpi_control_cache_meter,
    pub mode: hpi_control_cache_channelmode,
    pub mux: hpi_control_cache_mux,
    pub level: hpi_control_cache_level,
    pub tuner: hpi_control_cache_tuner,
    pub aes3rx: hpi_control_cache_aes3rx,
    pub aes3tx: hpi_control_cache_aes3tx,
    pub tone: hpi_control_cache_tonedetector,
    pub silence: hpi_control_cache_silencedetector,
    pub clk: hpi_control_cache_sampleclock,
    pub microphone: hpi_control_cache_microphone,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_single {
    pub u: hpi_control_cache_single_u,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_pad {
    pub i: hpi_control_cache_info,
    pub field_valid_flags: u32,
    pub c_channel: [u8; 40],
    pub c_artist: [u8; 100],
    pub c_title: [u8; 100],
    pub c_comment: [u8; 200],
    pub pTY: u32,
    pub pI: u32,
    pub traffic_supported: u32,
    pub traffic_anouncement: u32,
}

/* 2^N sized FIFO buffer (internal to HPI<->DSP interaction) */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hpi_fifo_buffer {
    pub size: u32,
    pub dsp_index: u32,
    pub host_index: u32,
}

/* skip host side function declarations for DSP
   compile and documentation extraction */

unsafe extern "C" {
    pub fn hpi_handle_object(handle: u32) -> c_char;

    pub fn hpi_handle_to_indexes(
        handle: u32,
        pw_adapter_index: *mut u16,
        pw_object_index: *mut u16,
    );

    pub fn hpi_indexes_to_handle(c_object: c_char, adapter_index: u16, object_index: u16) -> u32;

    /*////////////////////////////////////////////////////////////////////////// */

    /* main HPI entry point */
    pub fn hpi_send_recv(phm: *mut hpi_message, phr: *mut hpi_response);

    /* used in PnP OS/driver */
    pub fn hpi_outstream_host_buffer_get_info(
        h_outstream: u32,
        pp_buffer: *mut *mut u8,
        pp_status: *mut *mut hpi_hostbuffer_status,
    ) -> u16;

    pub fn hpi_instream_host_buffer_get_info(
        h_instream: u32,
        pp_buffer: *mut *mut u8,
        pp_status: *mut *mut hpi_hostbuffer_status,
    ) -> u16;

    /*
    The following 3 functions were last declared in header files for
    driver 3.10. HPI_ControlQuery() used to be the recommended way
    of getting a volume range. Declared here for binary asihpi32.dll
    compatibility.
    */

    pub fn hpi_format_to_msg(pMF: *mut hpi_msg_format, pF: *const hpi_format);
    pub fn hpi_stream_response_to_legacy(pSR: *mut hpi_stream_res);

    /*////////////////////////////////////////////////////////////////////////// */
    /* declarations for individual HPI entry points */
    pub fn HPI_6000(arg1: *mut hpi_message, arg2: *mut hpi_response);
    pub fn HPI_6205(arg1: *mut hpi_message, arg2: *mut hpi_response);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
