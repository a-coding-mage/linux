// SPDX-License-Identifier: GPL-2.0
/*
 * Image loader for kexec_file_load system call.
 *
 * Copyright IBM Corp. 2018
 *
 * Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Declarations supplied by the Linux kernel headers.
#[repr(C)]
pub struct kimage {
    pub kernel_buf: *mut c_char,
    pub kernel_buf_len: c_ulong,
    pub image_type: c_int,
}

#[repr(C)]
pub struct kexec_buf {
    pub image: *mut kimage,
    pub buffer: *mut c_char,
    pub bufsz: c_ulong,
    pub mem: u64,
    pub memsz: c_ulong,
}

#[repr(C)]
pub struct s390_load_data {
    pub report: *mut c_void,
    pub kernel_buf: *mut c_char,
    pub kernel_mem: u64,
    pub parm: *mut c_char,
    pub memsz: c_ulong,
}

#[repr(C)]
pub struct kexec_file_ops {
    pub probe: Option<unsafe extern "C" fn(*const c_char, c_ulong) -> c_int>,
    pub load: Option<unsafe extern "C" fn(
        *mut kimage,
        *mut c_char,
        c_ulong,
        *mut c_char,
        c_ulong,
        *mut c_char,
        c_ulong,
    ) -> *mut c_void>,
    #[cfg(CONFIG_KEXEC_SIG)]
    pub verify_sig: Option<unsafe extern "C" fn(*const c_char, c_ulong) -> c_int>,
}

extern "C" {
    fn kexec_file_add_components(
        image: *mut kimage,
        add_kernel_image: unsafe extern "C" fn(*mut kimage, *mut s390_load_data) -> c_int,
    ) -> *mut c_void;
    fn kexec_add_buffer(buf: *mut kexec_buf) -> c_int;
    fn ipl_report_add_component(
        report: *mut c_void,
        buf: *mut kexec_buf,
        flags: u32,
        cert: u32,
    );
    #[cfg(CONFIG_CRASH_DUMP)]
    static crashk_res: CrashResource;
    #[cfg(CONFIG_KEXEC_SIG)]
    fn s390_verify_sig(buf: *const c_char, len: c_ulong) -> c_int;
}

#[repr(C)]
struct CrashResource {
    start: u64,
}

extern "C" {
    static IPL_RB_COMPONENT_FLAG_SIGNED: u32;
    static IPL_RB_COMPONENT_FLAG_VERIFIED: u32;
    static IPL_RB_CERT_UNKNOWN: u32;
}

const KEXEC_TYPE_CRASH: c_int = 1;
const PARMAREA: usize = 0;

unsafe extern "C" fn kexec_file_add_kernel_image(
    image: *mut kimage,
    data: *mut s390_load_data,
) -> c_int {
    let mut buf: kexec_buf = core::mem::zeroed();

    (*(&mut buf)).image = image;

    buf.buffer = (*image).kernel_buf;
    buf.bufsz = (*image).kernel_buf_len;

    buf.mem = 0;
    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).image_type == KEXEC_TYPE_CRASH {
        buf.mem = crashk_res.start;
    }
    buf.memsz = buf.bufsz;

    (*data).kernel_buf = (*image).kernel_buf;
    (*data).kernel_mem = buf.mem;
    (*data).parm = (*image).kernel_buf.add(PARMAREA);
    (*data).memsz = (*data).memsz.wrapping_add(buf.memsz);

    ipl_report_add_component(
        (*data).report,
        &mut buf,
        IPL_RB_COMPONENT_FLAG_SIGNED | IPL_RB_COMPONENT_FLAG_VERIFIED,
        IPL_RB_CERT_UNKNOWN,
    );
    kexec_add_buffer(&mut buf)
}

unsafe extern "C" fn s390_image_load(
    image: *mut kimage,
    _kernel: *mut c_char,
    _kernel_len: c_ulong,
    _initrd: *mut c_char,
    _initrd_len: c_ulong,
    _cmdline: *mut c_char,
    _cmdline_len: c_ulong,
) -> *mut c_void {
    kexec_file_add_components(image, kexec_file_add_kernel_image)
}

unsafe extern "C" fn s390_image_probe(_buf: *const c_char, _len: c_ulong) -> c_int {
    /* Can't reliably tell if an image is valid.  Therefore give the
     * user whatever he wants.
     */
    0
}

#[no_mangle]
pub static s390_kexec_image_ops: kexec_file_ops = kexec_file_ops {
    probe: Some(s390_image_probe),
    load: Some(s390_image_load),
    #[cfg(CONFIG_KEXEC_SIG)]
    verify_sig: Some(s390_verify_sig),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
