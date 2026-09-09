/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR CDDL-1.0) */
/*
 * Virtual Device for Guest <-> VMM/Host communication, type definitions
 * which are also used for the vboxguest ioctl interface / by vboxsf
 *
 * Copyright (C) 2006-2016 Oracle Corporation
 */

/* C header dependencies: asm/bitsperlong.h and linux/types.h. */

/* The C size assertion macro is represented by the explicit layouts below. */

#[repr(i32)]
pub enum vmmdev_request_type {
    VMMDEVREQ_INVALID_REQUEST = 0,
    VMMDEVREQ_GET_MOUSE_STATUS = 1,
    VMMDEVREQ_SET_MOUSE_STATUS = 2,
    VMMDEVREQ_SET_POINTER_SHAPE = 3,
    VMMDEVREQ_GET_HOST_VERSION = 4,
    VMMDEVREQ_IDLE = 5,
    VMMDEVREQ_GET_HOST_TIME = 10,
    VMMDEVREQ_GET_HYPERVISOR_INFO = 20,
    VMMDEVREQ_SET_HYPERVISOR_INFO = 21,
    VMMDEVREQ_REGISTER_PATCH_MEMORY = 22, /* since version 3.0.6 */
    VMMDEVREQ_DEREGISTER_PATCH_MEMORY = 23, /* since version 3.0.6 */
    VMMDEVREQ_SET_POWER_STATUS = 30,
    VMMDEVREQ_ACKNOWLEDGE_EVENTS = 41,
    VMMDEVREQ_CTL_GUEST_FILTER_MASK = 42,
    VMMDEVREQ_REPORT_GUEST_INFO = 50,
    VMMDEVREQ_REPORT_GUEST_INFO2 = 58, /* since version 3.2.0 */
    VMMDEVREQ_REPORT_GUEST_STATUS = 59, /* since version 3.2.8 */
    VMMDEVREQ_REPORT_GUEST_USER_STATE = 74, /* since version 4.3 */
    /* Retrieve a display resize request sent by the host, deprecated. */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ = 51,
    VMMDEVREQ_VIDEMODE_SUPPORTED = 52,
    VMMDEVREQ_GET_HEIGHT_REDUCTION = 53,
    /** Retrieve a display resize request sent by the host. */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ2 = 54,
    VMMDEVREQ_REPORT_GUEST_CAPABILITIES = 55,
    VMMDEVREQ_SET_GUEST_CAPABILITIES = 56,
    VMMDEVREQ_VIDEMODE_SUPPORTED2 = 57, /* since version 3.2.0 */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQEX = 80, /* since version 4.2.4 */
    VMMDEVREQ_GET_DISPLAY_CHANGE_REQ_MULTI = 81,
    VMMDEVREQ_HGCM_CONNECT = 60,
    VMMDEVREQ_HGCM_DISCONNECT = 61,
    VMMDEVREQ_HGCM_CALL32 = 62,
    VMMDEVREQ_HGCM_CALL64 = 63,
    VMMDEVREQ_HGCM_CANCEL = 64,
    VMMDEVREQ_HGCM_CANCEL2 = 65,
    VMMDEVREQ_VIDEO_ACCEL_ENABLE = 70,
    VMMDEVREQ_VIDEO_ACCEL_FLUSH = 71,
    VMMDEVREQ_VIDEO_SET_VISIBLE_REGION = 72,
    VMMDEVREQ_GET_SEAMLESS_CHANGE_REQ = 73,
    VMMDEVREQ_QUERY_CREDENTIALS = 100,
    VMMDEVREQ_REPORT_CREDENTIALS_JUDGEMENT = 101,
    VMMDEVREQ_REPORT_GUEST_STATS = 110,
    VMMDEVREQ_GET_MEMBALLOON_CHANGE_REQ = 111,
    VMMDEVREQ_GET_STATISTICS_CHANGE_REQ = 112,
    VMMDEVREQ_CHANGE_MEMBALLOON = 113,
    VMMDEVREQ_GET_VRDPCHANGE_REQ = 150,
    VMMDEVREQ_LOG_STRING = 200,
    VMMDEVREQ_GET_CPU_HOTPLUG_REQ = 210,
    VMMDEVREQ_SET_CPU_HOTPLUG_STATUS = 211,
    VMMDEVREQ_REGISTER_SHARED_MODULE = 212,
    VMMDEVREQ_UNREGISTER_SHARED_MODULE = 213,
    VMMDEVREQ_CHECK_SHARED_MODULES = 214,
    VMMDEVREQ_GET_PAGE_SHARING_STATUS = 215,
    VMMDEVREQ_DEBUG_IS_PAGE_SHARED = 216,
    VMMDEVREQ_GET_SESSION_ID = 217, /* since version 3.2.8 */
    VMMDEVREQ_WRITE_COREDUMP = 218,
    VMMDEVREQ_GUEST_HEARTBEAT = 219,
    VMMDEVREQ_HEARTBEAT_CONFIGURE = 220,
    VMMDEVREQ_NT_BUG_CHECK = 221,
    VMMDEVREQ_VIDEO_UPDATE_MONITOR_POSITIONS = 222,
    VMMDEVREQ_SIZEHACK = 0x7fffffff,
}

#[cfg(target_pointer_width = "64")]
pub const VMMDEVREQ_HGCM_CALL: vmmdev_request_type = vmmdev_request_type::VMMDEVREQ_HGCM_CALL64;
#[cfg(not(target_pointer_width = "64"))]
pub const VMMDEVREQ_HGCM_CALL: vmmdev_request_type = vmmdev_request_type::VMMDEVREQ_HGCM_CALL32;

pub const VMMDEV_REQUESTOR_USR_NOT_GIVEN: u32 = 0x00000000;
pub const VMMDEV_REQUESTOR_USR_DRV: u32 = 0x00000001;
pub const VMMDEV_REQUESTOR_USR_DRV_OTHER: u32 = 0x00000002;
pub const VMMDEV_REQUESTOR_USR_ROOT: u32 = 0x00000003;
pub const VMMDEV_REQUESTOR_USR_USER: u32 = 0x00000006;
pub const VMMDEV_REQUESTOR_USR_MASK: u32 = 0x00000007;
pub const VMMDEV_REQUESTOR_KERNEL: u32 = 0x00000000;
pub const VMMDEV_REQUESTOR_USERMODE: u32 = 0x00000008;
pub const VMMDEV_REQUESTOR_MODE_MASK: u32 = 0x00000008;
pub const VMMDEV_REQUESTOR_CON_DONT_KNOW: u32 = 0x00000000;
pub const VMMDEV_REQUESTOR_CON_NO: u32 = 0x00000010;
pub const VMMDEV_REQUESTOR_CON_YES: u32 = 0x00000020;
pub const VMMDEV_REQUESTOR_CON_MASK: u32 = 0x00000030;
pub const VMMDEV_REQUESTOR_GRP_VBOX: u32 = 0x00000080;
pub const VMMDEV_REQUESTOR_TRUST_NOT_GIVEN: u32 = 0x00000000;
pub const VMMDEV_REQUESTOR_TRUST_UNTRUSTED: u32 = 0x00001000;
pub const VMMDEV_REQUESTOR_TRUST_LOW: u32 = 0x00002000;
pub const VMMDEV_REQUESTOR_TRUST_MEDIUM: u32 = 0x00003000;
pub const VMMDEV_REQUESTOR_TRUST_MEDIUM_PLUS: u32 = 0x00004000;
pub const VMMDEV_REQUESTOR_TRUST_HIGH: u32 = 0x00005000;
pub const VMMDEV_REQUESTOR_TRUST_SYSTEM: u32 = 0x00006000;
pub const VMMDEV_REQUESTOR_TRUST_PROTECTED: u32 = 0x00007000;
pub const VMMDEV_REQUESTOR_TRUST_MASK: u32 = 0x00007000;
pub const VMMDEV_REQUESTOR_USER_DEVICE: u32 = 0x00008000;

#[repr(i32)]
pub enum vmmdev_hgcm_service_location_type { VMMDEV_HGCM_LOC_INVALID = 0, VMMDEV_HGCM_LOC_LOCALHOST = 1, VMMDEV_HGCM_LOC_LOCALHOST_EXISTING = 2, VMMDEV_HGCM_LOC_SIZEHACK = 0x7fffffff }

#[repr(C)]
pub struct vmmdev_hgcm_service_location_localhost { pub service_name: [i8; 128] }
#[repr(C)]
pub union vmmdev_hgcm_service_location_u { pub localhost: vmmdev_hgcm_service_location_localhost }
#[repr(C)]
pub struct vmmdev_hgcm_service_location { pub type_: vmmdev_hgcm_service_location_type, pub u: vmmdev_hgcm_service_location_u }

#[repr(i32)]
pub enum vmmdev_hgcm_function_parameter_type { VMMDEV_HGCM_PARM_TYPE_INVALID = 0, VMMDEV_HGCM_PARM_TYPE_32BIT = 1, VMMDEV_HGCM_PARM_TYPE_64BIT = 2, VMMDEV_HGCM_PARM_TYPE_PHYSADDR = 3, VMMDEV_HGCM_PARM_TYPE_LINADDR = 4, VMMDEV_HGCM_PARM_TYPE_LINADDR_IN = 5, VMMDEV_HGCM_PARM_TYPE_LINADDR_OUT = 6, VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL = 7, VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL_IN = 8, VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL_OUT = 9, VMMDEV_HGCM_PARM_TYPE_PAGELIST = 10, VMMDEV_HGCM_PARM_TYPE_SIZEHACK = 0x7fffffff }

#[repr(C, packed)]
pub union vmmdev_hgcm_function_parameter32_u { pub value32: u32, pub value64: u64, pub pointer: vmmdev_hgcm_function_parameter32_pointer, pub page_list: vmmdev_hgcm_function_parameter32_page_list }
#[repr(C, packed)]
pub struct vmmdev_hgcm_function_parameter32_pointer { pub size: u32, pub u: vmmdev_hgcm_function_parameter32_pointer_u }
#[repr(C)]
pub union vmmdev_hgcm_function_parameter32_pointer_u { pub phys_addr: u32, pub linear_addr: u32 }
#[repr(C)]
pub struct vmmdev_hgcm_function_parameter32_page_list { pub size: u32, pub offset: u32 }
#[repr(C, packed)]
pub struct vmmdev_hgcm_function_parameter32 { pub type_: vmmdev_hgcm_function_parameter_type, pub u: vmmdev_hgcm_function_parameter32_u }

#[repr(C, packed)]
pub union vmmdev_hgcm_function_parameter64_u { pub value32: u32, pub value64: u64, pub pointer: vmmdev_hgcm_function_parameter64_pointer, pub page_list: vmmdev_hgcm_function_parameter64_page_list }
#[repr(C, packed)]
pub struct vmmdev_hgcm_function_parameter64_pointer { pub size: u32, pub u: vmmdev_hgcm_function_parameter64_pointer_u }
#[repr(C, packed)]
pub union vmmdev_hgcm_function_parameter64_pointer_u { pub phys_addr: u64, pub linear_addr: u64 }
#[repr(C)]
pub struct vmmdev_hgcm_function_parameter64_page_list { pub size: u32, pub offset: u32 }
#[repr(C, packed)]
pub struct vmmdev_hgcm_function_parameter64 { pub type_: vmmdev_hgcm_function_parameter_type, pub u: vmmdev_hgcm_function_parameter64_u }

#[cfg(target_pointer_width = "64")]
pub type vmmdev_hgcm_function_parameter = vmmdev_hgcm_function_parameter64;
#[cfg(not(target_pointer_width = "64"))]
pub type vmmdev_hgcm_function_parameter = vmmdev_hgcm_function_parameter32;

pub const VMMDEV_HGCM_F_PARM_DIRECTION_NONE: u32 = 0x00000000;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_TO_HOST: u32 = 0x00000001;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_FROM_HOST: u32 = 0x00000002;
pub const VMMDEV_HGCM_F_PARM_DIRECTION_BOTH: u32 = 0x00000003;

#[repr(C)]
pub union vmmdev_hgcm_pagelist_u { pub unused: u64, pub pages: [u64; 0] }
#[repr(C)]
pub struct vmmdev_hgcm_pagelist { pub flags: u32, pub offset_first_page: u16, pub page_count: u16, pub u: vmmdev_hgcm_pagelist_u }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
