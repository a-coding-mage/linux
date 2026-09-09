/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of the Linux UAPI IPMI header. */

// Dependency intent: linux/ipmi_msgdefs.h and linux/compiler.h are supplied by
// the surrounding translation and are not reproduced here.

pub const IPMI_MAX_ADDR_SIZE: usize = 32;

#[repr(C)]
pub struct ipmi_addr {
    pub addr_type: core::ffi::c_int,
    pub channel: core::ffi::c_short,
    pub data: [core::ffi::c_char; IPMI_MAX_ADDR_SIZE],
}

pub const IPMI_SYSTEM_INTERFACE_ADDR_TYPE: core::ffi::c_int = 0x0c;
#[repr(C)]
pub struct ipmi_system_interface_addr {
    pub addr_type: core::ffi::c_int,
    pub channel: core::ffi::c_short,
    pub lun: u8,
}

pub const IPMI_IPMB_ADDR_TYPE: core::ffi::c_int = 0x01;
pub const IPMI_IPMB_BROADCAST_ADDR_TYPE: core::ffi::c_int = 0x41;
#[repr(C)]
pub struct ipmi_ipmb_addr {
    pub addr_type: core::ffi::c_int,
    pub channel: core::ffi::c_short,
    pub slave_addr: u8,
    pub lun: u8,
}

pub const IPMI_IPMB_DIRECT_ADDR_TYPE: core::ffi::c_int = 0x81;
#[repr(C)]
pub struct ipmi_ipmb_direct_addr {
    pub addr_type: core::ffi::c_int,
    pub channel: core::ffi::c_short,
    pub slave_addr: u8,
    pub rs_lun: u8,
    pub rq_lun: u8,
}

pub const IPMI_LAN_ADDR_TYPE: core::ffi::c_int = 0x04;
#[repr(C)]
pub struct ipmi_lan_addr {
    pub addr_type: core::ffi::c_int,
    pub channel: core::ffi::c_short,
    pub privilege: u8,
    pub session_handle: u8,
    pub remote_SWID: u8,
    pub local_SWID: u8,
    pub lun: u8,
}

pub const IPMI_BMC_CHANNEL: u32 = 0xf;
pub const IPMI_NUM_CHANNELS: u32 = 0x10;
pub const IPMI_CHAN_ALL: u32 = !0u32;

#[repr(C)]
pub struct ipmi_msg {
    pub netfn: u8,
    pub cmd: u8,
    pub data_len: u16,
    pub data: *mut u8, // __user pointer
}

#[repr(C)]
pub struct kernel_ipmi_msg {
    pub netfn: u8,
    pub cmd: u8,
    pub data_len: u16,
    pub data: *mut u8,
}

pub const IPMI_INVALID_CMD_COMPLETION_CODE: u8 = 0xC1;
pub const IPMI_TIMEOUT_COMPLETION_CODE: u8 = 0xC3;
pub const IPMI_UNKNOWN_ERR_COMPLETION_CODE: u8 = 0xff;

pub const IPMI_RESPONSE_RECV_TYPE: core::ffi::c_int = 1;
pub const IPMI_ASYNC_EVENT_RECV_TYPE: core::ffi::c_int = 2;
pub const IPMI_CMD_RECV_TYPE: core::ffi::c_int = 3;
pub const IPMI_RESPONSE_RESPONSE_TYPE: core::ffi::c_int = 4;
pub const IPMI_OEM_RECV_TYPE: core::ffi::c_int = 5;

pub const IPMI_MAINTENANCE_MODE_AUTO: core::ffi::c_int = 0;
pub const IPMI_MAINTENANCE_MODE_OFF: core::ffi::c_int = 1;
pub const IPMI_MAINTENANCE_MODE_ON: core::ffi::c_int = 2;

pub const IPMI_IOC_MAGIC: u8 = b'i';

#[repr(C)]
pub struct ipmi_req {
    pub addr: *mut u8, // __user pointer
    pub addr_len: u32,
    pub msgid: core::ffi::c_long,
    pub msg: ipmi_msg,
}

#[repr(C)]
pub struct ipmi_req_settime {
    pub req: ipmi_req,
    pub retries: core::ffi::c_int,
    pub retry_time_ms: u32,
}

#[repr(C)]
pub struct ipmi_recv {
    pub recv_type: core::ffi::c_int,
    pub addr: *mut u8, // __user pointer
    pub addr_len: u32,
    pub msgid: core::ffi::c_long,
    pub msg: ipmi_msg,
}

// The following ioctl values retain their source definitions; _IOR/_IOWR/_IOW
// are provided by the dependent Linux ioctl translation.
// #define IPMICTL_SEND_COMMAND          _IOR(IPMI_IOC_MAGIC, 13, struct ipmi_req)
// #define IPMICTL_SEND_COMMAND_SETTIME  _IOR(IPMI_IOC_MAGIC, 21, struct ipmi_req_settime)
// #define IPMICTL_RECEIVE_MSG            _IOWR(IPMI_IOC_MAGIC, 12, struct ipmi_recv)
// #define IPMICTL_RECEIVE_MSG_TRUNC      _IOWR(IPMI_IOC_MAGIC, 11, struct ipmi_recv)

#[repr(C)]
pub struct ipmi_cmdspec {
    pub netfn: u8,
    pub cmd: u8,
}

// #define IPMICTL_REGISTER_FOR_CMD       _IOR(IPMI_IOC_MAGIC, 14, struct ipmi_cmdspec)
// #define IPMICTL_UNREGISTER_FOR_CMD     _IOR(IPMI_IOC_MAGIC, 15, struct ipmi_cmdspec)

#[repr(C)]
pub struct ipmi_cmdspec_chans {
    pub netfn: u32,
    pub cmd: u32,
    pub chans: u32,
}

// #define IPMICTL_REGISTER_FOR_CMD_CHANS _IOR(IPMI_IOC_MAGIC, 28, struct ipmi_cmdspec_chans)
// #define IPMICTL_UNREGISTER_FOR_CMD_CHANS _IOR(IPMI_IOC_MAGIC, 29, struct ipmi_cmdspec_chans)
// #define IPMICTL_SET_GETS_EVENTS_CMD    _IOR(IPMI_IOC_MAGIC, 16, int)

#[repr(C)]
pub struct ipmi_channel_lun_address_set {
    pub channel: u16,
    pub value: u8,
}

// #define IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD _IOR(IPMI_IOC_MAGIC, 24, struct ipmi_channel_lun_address_set)
// #define IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD _IOR(IPMI_IOC_MAGIC, 25, struct ipmi_channel_lun_address_set)
// #define IPMICTL_SET_MY_CHANNEL_LUN_CMD     _IOR(IPMI_IOC_MAGIC, 26, struct ipmi_channel_lun_address_set)
// #define IPMICTL_GET_MY_CHANNEL_LUN_CMD     _IOR(IPMI_IOC_MAGIC, 27, struct ipmi_channel_lun_address_set)
// #define IPMICTL_SET_MY_ADDRESS_CMD         _IOR(IPMI_IOC_MAGIC, 17, unsigned int)
// #define IPMICTL_GET_MY_ADDRESS_CMD         _IOR(IPMI_IOC_MAGIC, 18, unsigned int)
// #define IPMICTL_SET_MY_LUN_CMD             _IOR(IPMI_IOC_MAGIC, 19, unsigned int)
// #define IPMICTL_GET_MY_LUN_CMD             _IOR(IPMI_IOC_MAGIC, 20, unsigned int)

#[repr(C)]
pub struct ipmi_timing_parms {
    pub retries: core::ffi::c_int,
    pub retry_time_ms: u32,
}

// #define IPMICTL_SET_TIMING_PARMS_CMD      _IOR(IPMI_IOC_MAGIC, 22, struct ipmi_timing_parms)
// #define IPMICTL_GET_TIMING_PARMS_CMD      _IOR(IPMI_IOC_MAGIC, 23, struct ipmi_timing_parms)
// #define IPMICTL_GET_MAINTENANCE_MODE_CMD  _IOR(IPMI_IOC_MAGIC, 30, int)
// #define IPMICTL_SET_MAINTENANCE_MODE_CMD  _IOW(IPMI_IOC_MAGIC, 31, int)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
