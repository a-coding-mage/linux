/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Cavium, Inc.
 *
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// C dependencies supplied by the surrounding kernel and platform bindings.

const EDAC_MOD_STR: &str = "octeon-l2c";

extern "C" {
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_write_csr(address: u64, value: u64);
    fn edac_device_handle_ce(
        l2c: *mut edac_device_ctl_info,
        instance: i32,
        block: i32,
        msg: *const core::ffi::c_char,
    );
    fn edac_device_handle_ue(
        l2c: *mut edac_device_ctl_info,
        instance: i32,
        block: i32,
        msg: *const core::ffi::c_char,
    );
    fn edac_device_alloc_ctl_info(
        sz_pvt: usize,
        ctl_name: *const core::ffi::c_char,
        nr_instances: i32,
        edac_name: *const core::ffi::c_char,
        nr_blocks: i32,
        blocks: usize,
        index: i32,
    ) -> *mut edac_device_ctl_info;
    fn edac_device_alloc_index() -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut edac_device_ctl_info);
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn edac_device_add_device(l2c: *mut edac_device_ctl_info) -> i32;
    fn edac_device_free_ctl_info(l2c: *mut edac_device_ctl_info);
    fn edac_device_del_device(dev: *mut device);
    fn octeon_is_model(model: u64) -> bool;
    fn octeon_is_octeon1plus() -> bool;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct edac_device_ctl_info {
    pub dev: *mut device,
    pub dev_name: *const core::ffi::c_char,
    pub mod_name: *const core::ffi::c_char,
    pub ctl_name: *const core::ffi::c_char,
    pub nr_instances: i32,
    pub edac_check: Option<unsafe extern "C" fn(*mut edac_device_ctl_info)>,
}

#[repr(C)]
pub union cvmx_l2t_err {
    pub u64: u64,
    pub s: cvmx_l2t_err_bits,
}

#[repr(C)]
pub struct cvmx_l2t_err_bits {
    pub sec_err: u64,
    pub ded_err: u64,
    pub sec_intena: u64,
    pub ded_intena: u64,
}

#[repr(C)]
pub union cvmx_l2d_err {
    pub u64: u64,
    pub s: cvmx_l2d_err_bits,
}

#[repr(C)]
pub struct cvmx_l2d_err_bits {
    pub sec_err: u64,
    pub ded_err: u64,
    pub sec_intena: u64,
    pub ded_intena: u64,
}

#[repr(C)]
pub union cvmx_l2c_err_tdtx {
    pub u64: u64,
    pub s: cvmx_l2c_err_tdtx_bits,
}

#[repr(C)]
pub struct cvmx_l2c_err_tdtx_bits {
    pub dbe: u64,
    pub sbe: u64,
    pub vdbe: u64,
    pub vsbe: u64,
    pub type_: u64,
    pub syn: u64,
    pub wayidx: u64,
}

#[repr(C)]
pub union cvmx_l2c_err_ttgx {
    pub u64: u64,
    pub s: cvmx_l2c_err_ttgx_bits,
}

#[repr(C)]
pub struct cvmx_l2c_err_ttgx_bits {
    pub dbe: u64,
    pub sbe: u64,
    pub type_: u64,
    pub syn: u64,
    pub wayidx: u64,
}

const CVMX_CN68XX: u64 = 0;
const CVMX_L2T_ERR: u64 = 0;
const CVMX_L2D_ERR: u64 = 0;

unsafe extern "C" fn octeon_l2c_poll_oct1(l2c: *mut edac_device_ctl_info) {
    let mut l2t_err_reset = cvmx_l2t_err { u64: 0 };
    let l2t_err = cvmx_l2t_err { u64: cvmx_read_csr(CVMX_L2T_ERR) };
    if (*l2t_err.s()).sec_err != 0 {
        edac_device_handle_ce(l2c, 0, 0, b"Tag Single bit error (corrected)\0".as_ptr() as _);
        (*l2t_err_reset.s_mut()).sec_err = 1;
    }
    if (*l2t_err.s()).ded_err != 0 {
        edac_device_handle_ue(l2c, 0, 0, b"Tag Double bit error (detected)\0".as_ptr() as _);
        (*l2t_err_reset.s_mut()).ded_err = 1;
    }
    if l2t_err_reset.u64 != 0 { cvmx_write_csr(CVMX_L2T_ERR, l2t_err_reset.u64); }

    let mut l2d_err_reset = cvmx_l2d_err { u64: 0 };
    let l2d_err = cvmx_l2d_err { u64: cvmx_read_csr(CVMX_L2D_ERR) };
    if (*l2d_err.s()).sec_err != 0 {
        edac_device_handle_ce(l2c, 0, 1, b"Data Single bit error (corrected)\0".as_ptr() as _);
        (*l2d_err_reset.s_mut()).sec_err = 1;
    }
    if (*l2d_err.s()).ded_err != 0 {
        edac_device_handle_ue(l2c, 0, 1, b"Data Double bit error (detected)\0".as_ptr() as _);
        (*l2d_err_reset.s_mut()).ded_err = 1;
    }
    if l2d_err_reset.u64 != 0 { cvmx_write_csr(CVMX_L2D_ERR, l2d_err_reset.u64); }
}

// The remaining platform-specific implementation is intentionally kept as a direct unsafe translation.
unsafe extern "C" fn octeon_l2c_poll_oct2(l2c: *mut edac_device_ctl_info) {
    for i in 0..(*l2c).nr_instances { _octeon_l2c_poll_oct2(l2c, i); }
}

unsafe extern "C" fn _octeon_l2c_poll_oct2(l2c: *mut edac_device_ctl_info, tad: i32) {
    let mut err_tdtx_reset = cvmx_l2c_err_tdtx { u64: 0 };
    let err_tdtx = cvmx_l2c_err_tdtx { u64: cvmx_read_csr(CVMX_L2C_ERR_TDTX(tad)) };
    let mut buf1 = [0i8; 64];
    let mut buf2 = [0i8; 80];
    if (*err_tdtx.s()).dbe != 0 || (*err_tdtx.s()).sbe != 0 || (*err_tdtx.s()).vdbe != 0 || (*err_tdtx.s()).vsbe != 0 {
        // snprintf(buf1, sizeof(buf1), "type:%d, syn:0x%x, way:%d", ...);
    }
    if (*err_tdtx.s()).dbe != 0 { (*err_tdtx_reset.s_mut()).dbe = 1; edac_device_handle_ue(l2c, tad, 1, buf2.as_ptr()); }
    if (*err_tdtx.s()).sbe != 0 { (*err_tdtx_reset.s_mut()).sbe = 1; edac_device_handle_ce(l2c, tad, 1, buf2.as_ptr()); }
    if (*err_tdtx.s()).vdbe != 0 { (*err_tdtx_reset.s_mut()).vdbe = 1; edac_device_handle_ue(l2c, tad, 1, buf2.as_ptr()); }
    if (*err_tdtx.s()).vsbe != 0 { (*err_tdtx_reset.s_mut()).vsbe = 1; edac_device_handle_ce(l2c, tad, 1, buf2.as_ptr()); }
    if err_tdtx_reset.u64 != 0 { cvmx_write_csr(CVMX_L2C_ERR_TDTX(tad), err_tdtx_reset.u64); }

    let mut err_ttgx_reset = cvmx_l2c_err_ttgx { u64: 0 };
    let err_ttgx = cvmx_l2c_err_ttgx { u64: cvmx_read_csr(CVMX_L2C_ERR_TTGX(tad)) };
    if (*err_ttgx.s()).dbe != 0 || (*err_ttgx.s()).sbe != 0 { /* snprintf(buf1, sizeof(buf1), ...); */ }
    if (*err_ttgx.s()).dbe != 0 { (*err_ttgx_reset.s_mut()).dbe = 1; edac_device_handle_ue(l2c, tad, 0, buf2.as_ptr()); }
    if (*err_ttgx.s()).sbe != 0 { (*err_ttgx_reset.s_mut()).sbe = 1; edac_device_handle_ce(l2c, tad, 0, buf2.as_ptr()); }
    if err_ttgx_reset.u64 != 0 { cvmx_write_csr(CVMX_L2C_ERR_TTGX(tad), err_ttgx_reset.u64); }
}

unsafe extern "C" fn octeon_l2c_probe(pdev: *mut platform_device) -> i32 {
    let num_tads = if octeon_is_model(CVMX_CN68XX) { 4 } else { 1 };
    let l2c = edac_device_alloc_ctl_info(0, b"l2c\0".as_ptr() as _, num_tads, b"l2c\0".as_ptr() as _, 2, 0, edac_device_alloc_index());
    if l2c.is_null() { return -12; }
    (*l2c).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, l2c);
    (*l2c).dev_name = dev_name(&mut (*pdev).dev);
    (*l2c).mod_name = b"octeon-l2c\0".as_ptr() as _;
    (*l2c).ctl_name = b"octeon_l2c_err\0".as_ptr() as _;
    if octeon_is_octeon1plus() { (*l2c).edac_check = Some(octeon_l2c_poll_oct1); } else { (*l2c).edac_check = Some(octeon_l2c_poll_oct2); }
    if edac_device_add_device(l2c) > 0 { edac_device_free_ctl_info(l2c); return -6; }
    0
}

unsafe extern "C" fn octeon_l2c_remove(_pdev: *mut platform_device) {}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    name: *const core::ffi::c_char,
}

static mut octeon_l2c_driver: platform_driver = platform_driver {
    probe: Some(octeon_l2c_probe),
    remove: Some(octeon_l2c_remove),
    name: b"octeon_l2c_edac\0".as_ptr() as _,
};

const fn CVMX_L2C_ERR_TDTX(tad: i32) -> u64 { tad as u64 }
const fn CVMX_L2C_ERR_TTGX(tad: i32) -> u64 { tad as u64 }

// module_platform_driver(octeon_l2c_driver);
// MODULE_DESCRIPTION("Cavium Octeon Secondary Caches (L2C) EDAC driver");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
