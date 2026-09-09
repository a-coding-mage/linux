/*
 * Char device interface. Rust translation of linux/firewire-cdev.h.
 *
 * The Linux ioctl encoding macros and types are supplied by the surrounding
 * kernel bindings; ioctl declarations are retained below as comments because
 * their numeric values are architecture-dependent.
 */

pub const FW_CDEV_EVENT_BUS_RESET: u32 = 0x00;
pub const FW_CDEV_EVENT_RESPONSE: u32 = 0x01;
pub const FW_CDEV_EVENT_REQUEST: u32 = 0x02;
pub const FW_CDEV_EVENT_ISO_INTERRUPT: u32 = 0x03;
pub const FW_CDEV_EVENT_ISO_RESOURCE_ALLOCATED: u32 = 0x04;
pub const FW_CDEV_EVENT_ISO_RESOURCE_DEALLOCATED: u32 = 0x05;
pub const FW_CDEV_EVENT_REQUEST2: u32 = 0x06;
pub const FW_CDEV_EVENT_PHY_PACKET_SENT: u32 = 0x07;
pub const FW_CDEV_EVENT_PHY_PACKET_RECEIVED: u32 = 0x08;
pub const FW_CDEV_EVENT_ISO_INTERRUPT_MULTICHANNEL: u32 = 0x09;
pub const FW_CDEV_EVENT_REQUEST3: u32 = 0x0a;
pub const FW_CDEV_EVENT_RESPONSE2: u32 = 0x0b;
pub const FW_CDEV_EVENT_PHY_PACKET_SENT2: u32 = 0x0c;
pub const FW_CDEV_EVENT_PHY_PACKET_RECEIVED2: u32 = 0x0d;

#[repr(C)] pub struct fw_cdev_event_common { pub closure: u64, pub type_: u32 }
#[repr(C)] pub struct fw_cdev_event_bus_reset { pub closure: u64, pub type_: u32, pub node_id: u32, pub local_node_id: u32, pub bm_node_id: u32, pub irm_node_id: u32, pub root_node_id: u32, pub generation: u32 }
#[repr(C)] pub struct fw_cdev_event_response { pub closure: u64, pub type_: u32, pub rcode: u32, pub length: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_response2 { pub closure: u64, pub type_: u32, pub rcode: u32, pub length: u32, pub request_tstamp: u32, pub response_tstamp: u32, pub padding: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_request { pub closure: u64, pub type_: u32, pub tcode: u32, pub offset: u64, pub handle: u32, pub length: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_request2 { pub closure: u64, pub type_: u32, pub tcode: u32, pub offset: u64, pub source_node_id: u32, pub destination_node_id: u32, pub card: u32, pub generation: u32, pub handle: u32, pub length: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_request3 { pub closure: u64, pub type_: u32, pub tcode: u32, pub offset: u64, pub source_node_id: u32, pub destination_node_id: u32, pub card: u32, pub generation: u32, pub handle: u32, pub length: u32, pub tstamp: u32, pub padding: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_iso_interrupt { pub closure: u64, pub type_: u32, pub cycle: u32, pub header_length: u32, pub header: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_iso_interrupt_mc { pub closure: u64, pub type_: u32, pub completed: u32 }
#[repr(C)] pub struct fw_cdev_event_iso_resource { pub closure: u64, pub type_: u32, pub handle: u32, pub channel: i32, pub bandwidth: i32 }
#[repr(C)] pub struct fw_cdev_event_phy_packet { pub closure: u64, pub type_: u32, pub rcode: u32, pub length: u32, pub data: [u32; 0] }
#[repr(C)] pub struct fw_cdev_event_phy_packet2 { pub closure: u64, pub type_: u32, pub rcode: u32, pub length: u32, pub tstamp: u32, pub data: [u32; 0] }

#[repr(C)] pub union fw_cdev_event {
    pub common: fw_cdev_event_common, pub bus_reset: fw_cdev_event_bus_reset,
    pub response: fw_cdev_event_response, pub request: fw_cdev_event_request,
    pub request2: fw_cdev_event_request2, pub iso_interrupt: fw_cdev_event_iso_interrupt,
    pub iso_interrupt_mc: fw_cdev_event_iso_interrupt_mc, pub iso_resource: fw_cdev_event_iso_resource,
    pub phy_packet: fw_cdev_event_phy_packet, pub request3: fw_cdev_event_request3,
    pub response2: fw_cdev_event_response2, pub phy_packet2: fw_cdev_event_phy_packet2,
}

#[repr(C)] pub struct fw_cdev_get_info { pub version: u32, pub rom_length: u32, pub rom: u64, pub bus_reset: u64, pub bus_reset_closure: u64, pub card: u32 }
#[repr(C)] pub struct fw_cdev_send_request { pub tcode: u32, pub length: u32, pub offset: u64, pub closure: u64, pub data: u64, pub generation: u32 }
#[repr(C)] pub struct fw_cdev_send_response { pub rcode: u32, pub length: u32, pub data: u64, pub handle: u32 }
#[repr(C)] pub struct fw_cdev_allocate { pub offset: u64, pub closure: u64, pub length: u32, pub handle: u32, pub region_end: u64 }
#[repr(C)] pub struct fw_cdev_deallocate { pub handle: u32 }
pub const FW_CDEV_LONG_RESET: u32 = 0; pub const FW_CDEV_SHORT_RESET: u32 = 1;
#[repr(C)] pub struct fw_cdev_initiate_bus_reset { pub type_: u32 }
#[repr(C)] pub struct fw_cdev_add_descriptor { pub immediate: u32, pub key: u32, pub data: u64, pub length: u32, pub handle: u32 }
#[repr(C)] pub struct fw_cdev_remove_descriptor { pub handle: u32 }

pub const FW_CDEV_ISO_CONTEXT_TRANSMIT: u32 = 0;
pub const FW_CDEV_ISO_CONTEXT_RECEIVE: u32 = 1;
pub const FW_CDEV_ISO_CONTEXT_RECEIVE_MULTICHANNEL: u32 = 2;
#[repr(C)] pub struct fw_cdev_create_iso_context { pub type_: u32, pub header_size: u32, pub channel: u32, pub speed: u32, pub closure: u64, pub handle: u32 }
#[repr(C)] pub struct fw_cdev_set_iso_channels { pub channels: u64, pub handle: u32 }
pub const FW_CDEV_ISO_INTERRUPT: u32 = 1 << 16;
pub const FW_CDEV_ISO_SKIP: u32 = 1 << 17;
pub const FW_CDEV_ISO_SYNC: u32 = 1 << 17;
#[inline] pub const fn FW_CDEV_ISO_PAYLOAD_LENGTH(v: u32) -> u32 { v }
#[inline] pub const fn FW_CDEV_ISO_TAG(v: u32) -> u32 { v << 18 }
#[inline] pub const fn FW_CDEV_ISO_SY(v: u32) -> u32 { v << 20 }
#[inline] pub const fn FW_CDEV_ISO_HEADER_LENGTH(v: u32) -> u32 { v << 24 }
#[repr(C)] pub struct fw_cdev_iso_packet { pub control: u32, pub header: [u32; 0] }
#[repr(C)] pub struct fw_cdev_queue_iso { pub packets: u64, pub data: u64, pub size: u32, pub handle: u32 }
pub const FW_CDEV_ISO_CONTEXT_MATCH_TAG0: u32 = 1; pub const FW_CDEV_ISO_CONTEXT_MATCH_TAG1: u32 = 2; pub const FW_CDEV_ISO_CONTEXT_MATCH_TAG2: u32 = 4; pub const FW_CDEV_ISO_CONTEXT_MATCH_TAG3: u32 = 8; pub const FW_CDEV_ISO_CONTEXT_MATCH_ALL_TAGS: u32 = 15;
#[repr(C)] pub struct fw_cdev_start_iso { pub cycle: i32, pub sync: u32, pub tags: u32, pub handle: u32 }
#[repr(C)] pub struct fw_cdev_stop_iso { pub handle: u32 }
#[repr(C)] pub struct fw_cdev_flush_iso { pub handle: u32 }
#[repr(C)] pub struct fw_cdev_get_cycle_timer { pub local_time: u64, pub cycle_timer: u32 }
#[repr(C)] pub struct fw_cdev_get_cycle_timer2 { pub tv_sec: i64, pub tv_nsec: i32, pub clk_id: i32, pub cycle_timer: u32 }
#[repr(C)] pub struct fw_cdev_allocate_iso_resource { pub closure: u64, pub channels: u64, pub bandwidth: u32, pub handle: u32 }
#[repr(C)] pub struct fw_cdev_send_stream_packet { pub length: u32, pub tag: u32, pub channel: u32, pub sy: u32, pub closure: u64, pub data: u64, pub generation: u32, pub speed: u32 }
#[repr(C)] pub struct fw_cdev_send_phy_packet { pub closure: u64, pub data: [u32; 2], pub generation: u32 }
#[repr(C)] pub struct fw_cdev_receive_phy_packets { pub closure: u64 }

/* _IO, _IOR, _IOW, and _IOWR ioctl declarations from the C header retain
 * their original request numbers and depend on the target Linux ABI. */
pub const FW_CDEV_VERSION: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
