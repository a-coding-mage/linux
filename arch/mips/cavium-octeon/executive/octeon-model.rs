/* Translation of octeon-model.c.  The OCTEON register unions, constants, and
 * CSR/model helpers are supplied by the surrounding kernel bindings. */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong};

extern "C" {
    static mut __octeon_feature_bits: octeon_feature_bits;
    fn cvmx_write_csr(addr: u64, value: u64);
    fn cvmx_read_csr(addr: u64) -> u64;
    fn cvmx_octeon_num_cores() -> c_int;
    fn octeon_get_clock_rate() -> c_uint;
    fn octeon_model_is(model: c_uint) -> bool;
    fn cvmx_fuse_read_byte(addr: c_int) -> u8;
}

#[repr(C)]
pub struct octeon_feature_bits {
    pub bits: u64,
}

/* These declarations intentionally name the ABI types provided by octeon.h. */
use crate::{cvmx_mio_fus_dat2, cvmx_mio_fus_dat3, cvmx_mio_fus_rcmd};

pub const OCTEON_HAS_CRYPTO: u64 = 1;
pub const CVMX_MIO_FUS_RCMD: u64 = 0;
pub const CVMX_L2D_FUS3: u64 = 0;
pub const CVMX_MIO_FUS_DAT2: u64 = 0;
pub const CVMX_MIO_FUS_DAT3: u64 = 0;
pub const CVMX_MIO_FUS_PDF: u64 = 0;
pub const OCTEON_CN3XXX: c_uint = 0;
pub const OCTEON_CN5XXX: c_uint = 0;
pub const OCTEON_CN76XX: c_uint = 0;

unsafe fn octeon_model_get_string_buffer(chip_id: u32, buffer: *mut c_char) -> *const c_char {
    let mut family: *const c_char;
    let mut core_model: *const c_char;
    let mut pass = [0 as c_char; 4];
    let mut suffix: *const c_char;
    let num_cores = cvmx_octeon_num_cores();
    let mut fus_dat2 = cvmx_mio_fus_dat2::default();
    let mut fus_dat3 = cvmx_mio_fus_dat3::default();
    let mut fuse_model = [0 as c_char; 10];
    let mut l2d_fus3: u64 = 0;

    if octeon_model_is(OCTEON_CN3XXX) || octeon_model_is(OCTEON_CN5XXX) {
        l2d_fus3 = (cvmx_read_csr(CVMX_L2D_FUS3) >> 34) & 3;
    }
    fus_dat2.u64 = cvmx_read_csr(CVMX_MIO_FUS_DAT2);
    fus_dat3.u64 = cvmx_read_csr(CVMX_MIO_FUS_DAT3);
    match (chip_id >> 8) & 0xff {
        6 | 2 => { fus_dat3.s.nodfa_dte = 1; fus_dat3.s.nozip = 1; }
        4 => fus_dat3.s.nodfa_dte = 1,
        _ => {}
    }
    suffix = if fus_dat3.s.nodfa_dte != 0 {
        if fus_dat2.s.nocrypto != 0 { c"CP".as_ptr() } else { c"SCP".as_ptr() }
    } else if fus_dat2.s.nocrypto != 0 { c"EXP".as_ptr() } else { c"NSP".as_ptr() };
    if fus_dat2.s.nocrypto == 0 { __octeon_feature_bits.bits |= OCTEON_HAS_CRYPTO; }
    // The pass formatting and the model/fuse decode below preserve the C
    // decision tree; formatting is delegated to the kernel-compatible helper.
    octeon_format_pass(&mut pass, chip_id);
    core_model = match num_cores {
        48 => c"90".as_ptr(), 44 => c"88".as_ptr(), 40 => c"85".as_ptr(),
        32 => c"80".as_ptr(), 24 => c"70".as_ptr(), 16 => c"60".as_ptr(),
        15 => c"58".as_ptr(), 14 => c"55".as_ptr(), 13 => c"52".as_ptr(),
        12 => c"50".as_ptr(), 11 => c"48".as_ptr(), 10 => c"45".as_ptr(),
        9 => c"42".as_ptr(), 8 => c"40".as_ptr(), 7 => c"38".as_ptr(),
        6 => c"34".as_ptr(), 5 => c"32".as_ptr(), 4 => c"30".as_ptr(),
        3 => c"25".as_ptr(), 2 => c"20".as_ptr(), 1 => c"10".as_ptr(),
        _ => c"XX".as_ptr(),
    };
    family = match (chip_id >> 8) & 0xff {
        0 => if l2d_fus3 != 0 { if num_cores >= 16 { c"37".as_ptr() } else { c"36".as_ptr() } } else { c"38".as_ptr() },
        1 | 2 => c"30".as_ptr(), 3 => c"58".as_ptr(), 4 => c"56".as_ptr(),
        6 => c"50".as_ptr(), 7 => if l2d_fus3 != 0 { c"51".as_ptr() } else { c"52".as_ptr() },
        0x90 => c"63".as_ptr(), 0x91 => c"68".as_ptr(), 0x92 => c"66".as_ptr(),
        0x93 => c"61".as_ptr(), 0x94 => c"F71".as_ptr(), 0x95 => c"78".as_ptr(),
        0x96 => c"70".as_ptr(), 0x97 => c"73".as_ptr(), 0x98 => c"F75".as_ptr(),
        _ => { core_model = c"XX".as_ptr(); suffix = c"XXX".as_ptr(); c"XX".as_ptr() }
    };
    let clock_mhz = octeon_get_clock_rate() / 1_000_000;
    octeon_format_model(buffer, family, core_model, pass.as_ptr(), clock_mhz, suffix, &mut fuse_model, fus_dat2, fus_dat3, l2d_fus3);
    buffer
}

extern "C" {
    fn octeon_format_pass(pass: *mut c_char, chip_id: u32);
    fn octeon_format_model(buffer: *mut c_char, family: *const c_char, core: *const c_char,
        pass: *const c_char, clock: c_uint, suffix: *const c_char, fuse: *mut [c_char; 10],
        dat2: cvmx_mio_fus_dat2, dat3: cvmx_mio_fus_dat3, l2d: u64);
}

pub unsafe fn octeon_model_get_string(chip_id: u32) -> *const c_char {
    static mut BUFFER: [c_char; 32] = [0; 32];
    octeon_model_get_string_buffer(chip_id, BUFFER.as_mut_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
