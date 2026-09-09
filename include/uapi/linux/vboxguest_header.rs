/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR CDDL-1.0) */
/* VBoxGuest - VirtualBox Guest Additions Driver Interface. */

/* C dependencies: asm/bitsperlong.h, linux/ioctl.h, linux/vbox_err.h,
 * linux/vbox_vmmdev_types.h. */

pub const VBG_IOCTL_HDR_VERSION: u32 = 0x10001;
pub const VBG_IOCTL_HDR_TYPE_DEFAULT: u32 = 0;

#[repr(C)]
pub struct vbg_ioctl_hdr {
    pub size_in: u32,
    pub version: u32,
    pub r#type: u32,
    pub rc: i32,
    pub size_out: u32,
    pub reserved: u32,
}

pub const VBG_IOC_VERSION: u32 = 0x00010000;

#[repr(C)]
pub struct vbg_ioctl_driver_version_info_in {
    pub req_version: u32,
    pub min_version: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
pub struct vbg_ioctl_driver_version_info_out {
    pub session_version: u32,
    pub driver_version: u32,
    pub driver_revision: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
pub union vbg_ioctl_driver_version_info_u {
    pub r#in: vbg_ioctl_driver_version_info_in,
    pub out: vbg_ioctl_driver_version_info_out,
}

#[repr(C)]
pub struct vbg_ioctl_driver_version_info {
    pub hdr: vbg_ioctl_hdr,
    pub u: vbg_ioctl_driver_version_info_u,
}

#[repr(C)]
pub struct vbg_ioctl_hgcm_connect_in {
    pub loc: vmmdev_hgcm_service_location,
}

#[repr(C)]
pub struct vbg_ioctl_hgcm_connect_out {
    pub client_id: u32,
}

#[repr(C)]
pub union vbg_ioctl_hgcm_connect_u {
    pub r#in: vbg_ioctl_hgcm_connect_in,
    pub out: vbg_ioctl_hgcm_connect_out,
}

#[repr(C)]
pub struct vbg_ioctl_hgcm_connect {
    pub hdr: vbg_ioctl_hdr,
    pub u: vbg_ioctl_hgcm_connect_u,
}

#[repr(C)]
pub struct vbg_ioctl_hgcm_disconnect_in { pub client_id: u32 }

#[repr(C)]
pub union vbg_ioctl_hgcm_disconnect_u { pub r#in: vbg_ioctl_hgcm_disconnect_in }

#[repr(C)]
pub struct vbg_ioctl_hgcm_disconnect {
    pub hdr: vbg_ioctl_hdr,
    pub u: vbg_ioctl_hgcm_disconnect_u,
}

#[repr(C)]
pub struct vbg_ioctl_hgcm_call {
    pub hdr: vbg_ioctl_hdr,
    pub client_id: u32,
    pub function: u32,
    pub timeout_ms: u32,
    pub interruptible: u8,
    pub reserved: u8,
    pub parm_count: u16,
}

#[repr(C)]
pub struct vbg_ioctl_log_in { pub msg: [core::ffi::c_char; 1] }
#[repr(C)]
pub union vbg_ioctl_log_u { pub r#in: vbg_ioctl_log_in }
#[repr(C)]
pub struct vbg_ioctl_log { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_log_u }

#[repr(C)]
pub struct vbg_ioctl_wait_for_events_in { pub timeout_ms: u32, pub events: u32 }
#[repr(C)]
pub struct vbg_ioctl_wait_for_events_out { pub events: u32 }
#[repr(C)]
pub union vbg_ioctl_wait_for_events_u {
    pub r#in: vbg_ioctl_wait_for_events_in,
    pub out: vbg_ioctl_wait_for_events_out,
}
#[repr(C)]
pub struct vbg_ioctl_wait_for_events { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_wait_for_events_u }

#[repr(C)]
pub struct vbg_ioctl_change_filter_in { pub or_mask: u32, pub not_mask: u32 }
#[repr(C)]
pub union vbg_ioctl_change_filter_u { pub r#in: vbg_ioctl_change_filter_in }
#[repr(C)]
pub struct vbg_ioctl_change_filter { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_change_filter_u }

#[repr(C)]
pub struct vbg_ioctl_acquire_guest_caps_in { pub flags: u32, pub or_mask: u32, pub not_mask: u32 }
#[repr(C)]
pub union vbg_ioctl_acquire_guest_caps_u { pub r#in: vbg_ioctl_acquire_guest_caps_in }
#[repr(C)]
pub struct vbg_ioctl_acquire_guest_caps { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_acquire_guest_caps_u }

pub const VBGL_IOC_AGC_FLAGS_CONFIG_ACQUIRE_MODE: u32 = 0x00000001;
pub const VBGL_IOC_AGC_FLAGS_VALID_MASK: u32 = 0x00000001;

#[repr(C)]
pub struct vbg_ioctl_set_guest_caps_in { pub or_mask: u32, pub not_mask: u32 }
#[repr(C)]
pub struct vbg_ioctl_set_guest_caps_out { pub session_caps: u32, pub global_caps: u32 }
#[repr(C)]
pub union vbg_ioctl_set_guest_caps_u {
    pub r#in: vbg_ioctl_set_guest_caps_in,
    pub out: vbg_ioctl_set_guest_caps_out,
}
#[repr(C)]
pub struct vbg_ioctl_set_guest_caps { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_set_guest_caps_u }

#[repr(C)]
pub struct vbg_ioctl_check_balloon_out {
    pub balloon_chunks: u32,
    pub handle_in_r3: u8,
    pub padding: [u8; 3],
}
#[repr(C)]
pub union vbg_ioctl_check_balloon_u { pub out: vbg_ioctl_check_balloon_out }
#[repr(C)]
pub struct vbg_ioctl_check_balloon { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_check_balloon_u }

#[repr(C)]
pub struct vbg_ioctl_write_coredump_in { pub flags: u32 }
#[repr(C)]
pub union vbg_ioctl_write_coredump_u { pub r#in: vbg_ioctl_write_coredump_in }
#[repr(C)]
pub struct vbg_ioctl_write_coredump { pub hdr: vbg_ioctl_hdr, pub u: vbg_ioctl_write_coredump_u }

/* ioctl request-number macros from linux/ioctl.h, retained as source-level declarations. */
// VBG_IOCTL_DRIVER_VERSION_INFO = _IOWR('V', 0, struct vbg_ioctl_driver_version_info)
// VBG_IOCTL_VMMDEV_REQUEST(s) = _IOC(_IOC_READ | _IOC_WRITE, 'V', 2, s)
// VBG_IOCTL_VMMDEV_REQUEST_BIG = _IO('V', 3)
// VBG_IOCTL_HGCM_CONNECT = _IOWR('V', 4, struct vbg_ioctl_hgcm_connect)
// VBG_IOCTL_HGCM_DISCONNECT = _IOWR('V', 5, struct vbg_ioctl_hgcm_disconnect)
// VBG_IOCTL_HGCM_CALL_32(s) = _IOC(_IOC_READ | _IOC_WRITE, 'V', 6, s)
// VBG_IOCTL_HGCM_CALL_64(s) = _IOC(_IOC_READ | _IOC_WRITE, 'V', 7, s)
// VBG_IOCTL_HGCM_CALL(s) selects the 64-bit form when __BITS_PER_LONG == 64,
// otherwise it selects the 32-bit form.
// VBG_IOCTL_LOG(s) = _IO('V', 9)
// VBG_IOCTL_WAIT_FOR_EVENTS = _IOWR('V', 10, struct vbg_ioctl_wait_for_events)
// VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS = _IOWR('V', 11, struct vbg_ioctl_hdr)
// VBG_IOCTL_CHANGE_FILTER_MASK = _IOWR('V', 12, struct vbg_ioctl_change_filter)
// VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES = _IOWR('V', 13, struct vbg_ioctl_acquire_guest_caps)
// VBG_IOCTL_CHANGE_GUEST_CAPABILITIES = _IOWR('V', 14, struct vbg_ioctl_set_guest_caps)
// VBG_IOCTL_CHECK_BALLOON = _IOWR('V', 17, struct vbg_ioctl_check_balloon)
// VBG_IOCTL_WRITE_CORE_DUMP = _IOWR('V', 19, struct vbg_ioctl_write_coredump)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
