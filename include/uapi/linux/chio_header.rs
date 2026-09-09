/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ioctl interface for the scsi media changer driver */

/* changer element types */
pub const CHET_MT: i32 = 0; /* media transport element (robot) */
pub const CHET_ST: i32 = 1; /* storage element (media slots) */
pub const CHET_IE: i32 = 2; /* import/export element */
pub const CHET_DT: i32 = 3; /* data transfer element (tape/cdrom/whatever) */
pub const CHET_V1: i32 = 4; /* vendor specific #1 */
pub const CHET_V2: i32 = 5; /* vendor specific #2 */
pub const CHET_V3: i32 = 6; /* vendor specific #3 */
pub const CHET_V4: i32 = 7; /* vendor specific #4 */

#[repr(C)]
pub struct changer_params {
    pub cp_curpicker: i32,
    pub cp_npickers: i32,
    pub cp_nslots: i32,
    pub cp_nportals: i32,
    pub cp_ndrives: i32,
}

#[repr(C)]
pub struct changer_vendor_params {
    pub cvp_n1: i32,
    pub cvp_label1: [core::ffi::c_char; 16],
    pub cvp_n2: i32,
    pub cvp_label2: [core::ffi::c_char; 16],
    pub cvp_n3: i32,
    pub cvp_label3: [core::ffi::c_char; 16],
    pub cvp_n4: i32,
    pub cvp_label4: [core::ffi::c_char; 16],
    pub reserved: [i32; 8],
}

#[repr(C)]
pub struct changer_move {
    pub cm_fromtype: i32,
    pub cm_fromunit: i32,
    pub cm_totype: i32,
    pub cm_tounit: i32,
    pub cm_flags: i32,
}
pub const CM_INVERT: i32 = 1;

#[repr(C)]
pub struct changer_exchange {
    pub ce_srctype: i32,
    pub ce_srcunit: i32,
    pub ce_fdsttype: i32,
    pub ce_fdstunit: i32,
    pub ce_sdsttype: i32,
    pub ce_sdstunit: i32,
    pub ce_flags: i32,
}
pub const CE_INVERT1: i32 = 1;
pub const CE_INVERT2: i32 = 2;

#[repr(C)]
pub struct changer_position {
    pub cp_type: i32,
    pub cp_unit: i32,
    pub cp_flags: i32,
}
pub const CP_INVERT: i32 = 1;

#[repr(C)]
pub struct changer_element_status {
    pub ces_type: i32,
    pub ces_data: *mut u8,
}
pub const CESTATUS_FULL: i32 = 0x01;
pub const CESTATUS_IMPEXP: i32 = 0x02;
pub const CESTATUS_EXCEPT: i32 = 0x04;
pub const CESTATUS_ACCESS: i32 = 0x08;
pub const CESTATUS_EXENAB: i32 = 0x10;
pub const CESTATUS_INENAB: i32 = 0x20;

#[repr(C)]
pub struct changer_get_element {
    pub cge_type: i32,
    pub cge_unit: i32,
    pub cge_status: i32,
    pub cge_errno: i32,
    pub cge_srctype: i32,
    pub cge_srcunit: i32,
    pub cge_id: i32,
    pub cge_lun: i32,
    pub cge_pvoltag: [core::ffi::c_char; 36],
    pub cge_avoltag: [core::ffi::c_char; 36],
    pub cge_flags: i32,
}
pub const CGE_ERRNO: i32 = 0x01;
pub const CGE_INVERT: i32 = 0x02;
pub const CGE_SRC: i32 = 0x04;
pub const CGE_IDLUN: i32 = 0x08;
pub const CGE_PVOLTAG: i32 = 0x10;
pub const CGE_AVOLTAG: i32 = 0x20;

#[repr(C)]
pub struct changer_set_voltag {
    pub csv_type: i32,
    pub csv_unit: i32,
    pub csv_voltag: [core::ffi::c_char; 36],
    pub csv_flags: i32,
}
pub const CSV_PVOLTAG: i32 = 0x01;
pub const CSV_AVOLTAG: i32 = 0x02;
pub const CSV_CLEARTAG: i32 = 0x04;

/* The ioctl encoding macros are supplied by the platform headers. */
pub const CHIOMOVE: _ = _IOW(b'c', 1, changer_move);
pub const CHIOEXCHANGE: _ = _IOW(b'c', 2, changer_exchange);
pub const CHIOPOSITION: _ = _IOW(b'c', 3, changer_position);
pub const CHIOGPICKER: _ = _IOR(b'c', 4, i32); /* not impl. */
pub const CHIOSPICKER: _ = _IOW(b'c', 5, i32); /* not impl. */
pub const CHIOGPARAMS: _ = _IOR(b'c', 6, changer_params);
pub const CHIOGSTATUS: _ = _IOW(b'c', 8, changer_element_status);
pub const CHIOGELEM: _ = _IOW(b'c', 16, changer_get_element);
pub const CHIOINITELEM: _ = _IO(b'c', 17);
pub const CHIOSVOLTAG: _ = _IOW(b'c', 18, changer_set_voltag);
pub const CHIOGVPARAMS: _ = _IOR(b'c', 19, changer_vendor_params);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
