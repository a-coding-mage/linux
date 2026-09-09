/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2013 Imagination Technologies Ltd.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...);
    fn read_c0_segctl0() -> c_uint;
    fn read_c0_segctl1() -> c_uint;
    fn read_c0_segctl2() -> c_uint;
}

// Supplied by the MIPS architecture headers.
unsafe extern "C" {
    static cpu_has_segments: bool;
    static mut mips_debugfs_dir: *mut c_void;
    static segments_fops: c_void;
}

// MIPS_SEGCFG_* constants and the init/debugfs registration macros are supplied
// by the corresponding kernel headers and build environment.

unsafe fn build_segment_config(mut str_: *mut c_char, cfg: c_uint) {
    let mut am: c_uint;
    static AM_STR: [*const c_char; 8] = [
        b"UK\0".as_ptr() as *const c_char,
        b"MK\0".as_ptr() as *const c_char,
        b"MSK\0".as_ptr() as *const c_char,
        b"MUSK\0".as_ptr() as *const c_char,
        b"MUSUK\0".as_ptr() as *const c_char,
        b"USK\0".as_ptr() as *const c_char,
        b"RSRVD\0".as_ptr() as *const c_char,
        b"UUSK\0".as_ptr() as *const c_char,
    ];

    /* Segment access mode. */
    am = (cfg & MIPS_SEGCFG_AM) >> MIPS_SEGCFG_AM_SHIFT;
    str_ = str_.add(sprintf(str_, b"%-5s\0".as_ptr() as *const c_char, AM_STR[am as usize]) as usize);

    /*
     * Access modes MK, MSK and MUSK are mapped segments. Therefore
     * there is no direct physical address mapping unless it becomes
     * unmapped uncached at error level due to EU.
     */
    if am == 0 || am > 3 || (cfg & MIPS_SEGCFG_EU) != 0 {
        str_ = str_.add(sprintf(
            str_,
            b"         %03lx\0".as_ptr() as *const c_char,
            ((cfg & MIPS_SEGCFG_PA) >> MIPS_SEGCFG_PA_SHIFT) as c_ulong,
        ) as usize);
    } else {
        str_ = str_.add(sprintf(str_, b"         UND\0".as_ptr() as *const c_char) as usize);
    }

    if am == 0 || am > 3 {
        str_ = str_.add(sprintf(
            str_,
            b"         %01ld\0".as_ptr() as *const c_char,
            ((cfg & MIPS_SEGCFG_C) >> MIPS_SEGCFG_C_SHIFT) as c_ulong,
        ) as usize);
    } else {
        str_ = str_.add(sprintf(str_, b"         U\0".as_ptr() as *const c_char) as usize);
    }

    /* Exception configuration. */
    let _ = sprintf(
        str_,
        b"       %01ld\n\0".as_ptr() as *const c_char,
        ((cfg & MIPS_SEGCFG_EU) >> MIPS_SEGCFG_EU_SHIFT) as c_ulong,
    );
}

unsafe fn segments_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut segcfg: c_uint;
    let mut str_ = [0 as c_char; 42];

    seq_puts(m, b"Segment   Virtual    Size   Access Mode   Physical   Caching   EU\n\0".as_ptr() as *const c_char);
    seq_puts(m, b"-------   -------    ----   -----------   --------   -------   --\n\0".as_ptr() as *const c_char);

    segcfg = read_c0_segctl0();
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   0      e0000000   512M      %s\0".as_ptr() as *const c_char, str_.as_ptr());

    segcfg >>= 16;
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   1      c0000000   512M      %s\0".as_ptr() as *const c_char, str_.as_ptr());

    segcfg = read_c0_segctl1();
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   2      a0000000   512M      %s\0".as_ptr() as *const c_char, str_.as_ptr());

    segcfg >>= 16;
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   3      80000000   512M      %s\0".as_ptr() as *const c_char, str_.as_ptr());

    segcfg = read_c0_segctl2();
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   4      40000000    1G       %s\0".as_ptr() as *const c_char, str_.as_ptr());

    segcfg >>= 16;
    build_segment_config(str_.as_mut_ptr(), segcfg);
    seq_printf(m, b"   5      00000000    1G       %s\n\0".as_ptr() as *const c_char, str_.as_ptr());

    0
}

// DEFINE_SHOW_ATTRIBUTE(segments);

unsafe fn segments_info() -> c_int {
    if cpu_has_segments {
        // debugfs_create_file("segments", S_IRUGO, mips_debugfs_dir, NULL, &segments_fops);
    }
    0
}

// device_initcall(segments_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
