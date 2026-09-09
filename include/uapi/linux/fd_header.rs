/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* ioctl encoding is supplied by the corresponding Linux ioctl definitions. */

#[repr(C)]
pub struct floppy_struct {
    pub size: ::std::os::raw::c_uint,
    pub sect: ::std::os::raw::c_uint,
    pub head: ::std::os::raw::c_uint,
    pub track: ::std::os::raw::c_uint,
    pub stretch: ::std::os::raw::c_uint,
    pub gap: ::std::os::raw::c_uchar,
    pub rate: ::std::os::raw::c_uchar,
    pub spec1: ::std::os::raw::c_uchar,
    pub fmt_gap: ::std::os::raw::c_uchar,
    pub name: *const ::std::os::raw::c_char,
}
pub const FD_STRETCH: u32 = 1;
pub const FD_SWAPSIDES: u32 = 2;
pub const FD_ZEROBASED: u32 = 4;
pub const FD_SECTBASEMASK: u32 = 0x3fc;
#[inline] pub const fn FD_MKSECTBASE(s: u32) -> u32 { (s ^ 1) << 2 }
#[inline] pub unsafe fn FD_SECTBASE(floppy: *const floppy_struct) -> u32 { ((((*floppy).stretch & FD_SECTBASEMASK) >> 2) ^ 1) }
pub const FD_2M: u8 = 0x4;
pub const FD_SIZECODEMASK: u8 = 0x38;
#[inline] pub unsafe fn FD_SIZECODE(floppy: *const floppy_struct) -> u8 { (((((*floppy).rate & FD_SIZECODEMASK) >> 3) + 2) % 8) }
#[inline] pub unsafe fn FD_SECTSIZE(floppy: *const floppy_struct) -> u32 { if (*floppy).rate & FD_2M != 0 { 512 } else { 128 << FD_SIZECODE(floppy) } }
pub const FD_PERP: u8 = 0x40;

#[repr(C)] pub struct format_descr { pub device: u32, pub head: u32, pub track: u32 }
#[repr(C)] pub struct floppy_max_errors { pub abort: u32, pub read_track: u32, pub reset: u32, pub recal: u32, pub reporting: u32 }
pub type floppy_drive_name = [::std::os::raw::c_char; 16];

#[repr(C)]
pub struct floppy_drive_params {
    pub cmos: i8, pub max_dtr: ::std::os::raw::c_ulong, pub hlt: ::std::os::raw::c_ulong,
    pub hut: ::std::os::raw::c_ulong, pub srt: ::std::os::raw::c_ulong,
    pub spinup: ::std::os::raw::c_ulong, pub spindown: ::std::os::raw::c_ulong,
    pub spindown_offset: u8, pub select_delay: u8, pub rps: u8, pub tracks: u8,
    pub timeout: ::std::os::raw::c_ulong, pub interleave_sect: u8,
    pub max_errors: floppy_max_errors, pub flags: ::std::os::raw::c_char,
    pub read_track: ::std::os::raw::c_char, pub autodetect: [i16; 8],
    pub checkfreq: i32, pub native_format: i32,
}
pub const FTD_MSG: i8 = 0x10; pub const FD_BROKEN_DCL: i8 = 0x20; pub const FD_DEBUG: i8 = 0x02;
pub const FD_SILENT_DCL_CLEAR: i8 = 0x4; pub const FD_INVERTED_DCL: i8 = 0x80; pub const FD_AUTODETECT_SIZE: usize = 8;
pub const FD_NEED_TWADDLE_BIT: u32 = 0; pub const FD_VERIFY_BIT: u32 = 1; pub const FD_DISK_NEWCHANGE_BIT: u32 = 2;
pub const FD_UNUSED_BIT: u32 = 3; pub const FD_DISK_CHANGED_BIT: u32 = 4; pub const FD_DISK_WRITABLE_BIT: u32 = 5; pub const FD_OPEN_SHOULD_FAIL_BIT: u32 = 6;

#[repr(C)] pub struct floppy_drive_struct {
    pub flags: ::std::os::raw::c_ulong, pub spinup_date: ::std::os::raw::c_ulong, pub select_date: ::std::os::raw::c_ulong,
    pub first_read_date: ::std::os::raw::c_ulong, pub probed_format: i16, pub track: i16, pub maxblock: i16, pub maxtrack: i16,
    pub generation: i32, pub keep_data: i32, pub fd_ref: i32, pub fd_device: i32, pub last_checked: ::std::os::raw::c_ulong,
    pub dmabuf: *mut ::std::os::raw::c_char, pub bufblocks: i32,
}
pub const FD_NEED_TWADDLE: u64 = 1 << FD_NEED_TWADDLE_BIT; pub const FD_VERIFY: u64 = 1 << FD_VERIFY_BIT;
pub const FD_DISK_NEWCHANGE: u64 = 1 << FD_DISK_NEWCHANGE_BIT; pub const FD_DISK_CHANGED: u64 = 1 << FD_DISK_CHANGED_BIT; pub const FD_DISK_WRITABLE: u64 = 1 << FD_DISK_WRITABLE_BIT;

#[repr(C)] pub enum reset_mode { FD_RESET_IF_NEEDED, FD_RESET_IF_RAWCMD, FD_RESET_ALWAYS }
#[repr(C)] pub struct floppy_fdc_state {
    pub spec1: i32, pub spec2: i32, pub dtr: i32, pub version: u8, pub dor: u8, pub address: ::std::os::raw::c_ulong,
    pub rawcmd: u32, pub reset: u32, pub need_configure: u32, pub perp_mode: u32, pub has_fifo: u32,
    pub driver_version: u32, pub track: [u8; 4],
}
pub const FD_DRIVER_VERSION: u32 = 0x100;
#[repr(C)] pub struct floppy_write_errors { pub write_errors: u32, pub first_error_sector: ::std::os::raw::c_ulong, pub first_error_generation: i32, pub last_error_sector: ::std::os::raw::c_ulong, pub last_error_generation: i32, pub badness: u32 }

#[repr(C)] pub struct floppy_raw_cmd {
    pub flags: u32, pub data: *mut ::std::os::raw::c_void, pub kernel_data: *mut ::std::os::raw::c_char,
    pub next: *mut floppy_raw_cmd, pub length: isize, pub phys_length: isize, pub buffer_length: i32, pub rate: u8,
    pub cmd_count: u8, pub command: floppy_raw_cmd_union, pub track: i32, pub resultcode: i32, pub reserved1: i32, pub reserved2: i32,
}
#[repr(C)] pub union floppy_raw_cmd_union { pub cmd_reply: floppy_raw_cmd_reply, pub fullcmd: [u8; 33] }
#[repr(C)] pub struct floppy_raw_cmd_reply { pub cmd: [u8; 16], pub reply_count: u8, pub reply: [u8; 16] }
pub const FD_RAW_CMD_SIZE: usize = 16; pub const FD_RAW_REPLY_SIZE: usize = 16; pub const FD_RAW_CMD_FULLSIZE: usize = 33;
pub const FDHAVEBATCHEDRAWCMD: bool = true;
pub const FD_RAW_READ: u32 = 1; pub const FD_RAW_WRITE: u32 = 2; pub const FD_RAW_NO_MOTOR: u32 = 4;
pub const FD_RAW_DISK_CHANGE: u32 = 4; pub const FD_RAW_INTR: u32 = 8; pub const FD_RAW_SPIN: u32 = 0x10;
pub const FD_RAW_NO_MOTOR_AFTER: u32 = 0x20; pub const FD_RAW_NEED_DISK: u32 = 0x40; pub const FD_RAW_NEED_SEEK: u32 = 0x80;
pub const FD_RAW_MORE: u32 = 0x100; pub const FD_RAW_STOP_IF_FAILURE: u32 = 0x200; pub const FD_RAW_STOP_IF_SUCCESS: u32 = 0x400; pub const FD_RAW_SOFTFAILURE: u32 = 0x800;
pub const FD_RAW_FAILURE: u32 = 0x10000; pub const FD_RAW_HARDFAILURE: u32 = 0x20000;
pub const FD_FILL_BYTE: u8 = 0xf6;
/* The following ioctl values retain their source macro expressions; _IO/_IOR/_IOW
 * are supplied by the Linux ioctl dependency. */
pub const FDCLRPRM: _ = _IO(2, 0x41); pub const FDSETPRM: _ = _IOW(2, 0x42, floppy_struct);
pub const FDSETMEDIAPRM: _ = FDSETPRM; pub const FDDEFPRM: _ = _IOW(2, 0x43, floppy_struct); pub const FDGETPRM: _ = _IOR(2, 0x04, floppy_struct);
pub const FDDEFMEDIAPRM: _ = FDDEFPRM; pub const FDGETMEDIAPRM: _ = FDGETPRM; pub const FDMSGON: _ = _IO(2, 0x45); pub const FDMSGOFF: _ = _IO(2, 0x46);
pub const FDFMTBEG: _ = _IO(2, 0x47); pub const FDFMTTRK: _ = _IOW(2, 0x48, format_descr); pub const FDFMTEND: _ = _IO(2, 0x49);
pub const FDSETEMSGTRESH: _ = _IO(2, 0x4a); pub const FDFLUSH: _ = _IO(2, 0x4b); pub const FDSETMAXERRS: _ = _IOW(2, 0x4c, floppy_max_errors); pub const FDGETMAXERRS: _ = _IOR(2, 0x0e, floppy_max_errors);
pub const FDGETDRVTYP: _ = _IOR(2, 0x0f, floppy_drive_name); pub const FDSETDRVPRM: _ = _IOW(2, 0x90, floppy_drive_params); pub const FDGETDRVPRM: _ = _IOR(2, 0x11, floppy_drive_params);
pub const FDGETDRVSTAT: _ = _IOR(2, 0x12, floppy_drive_struct); pub const FDPOLLDRVSTAT: _ = _IOR(2, 0x13, floppy_drive_struct); pub const FDRESET: _ = _IO(2, 0x54);
pub const FDGETFDCSTAT: _ = _IOR(2, 0x15, floppy_fdc_state); pub const FDWERRORCLR: _ = _IO(2, 0x56); pub const FDWERRORGET: _ = _IOR(2, 0x17, floppy_write_errors);
pub const FDRAWCMD: _ = _IO(2, 0x58); pub const FDTWADDLE: _ = _IO(2, 0x59); pub const FDEJECT: _ = _IO(2, 0x5a);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
