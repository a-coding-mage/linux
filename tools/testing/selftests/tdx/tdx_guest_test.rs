// SPDX-License-Identifier: GPL-2.0
/*
 * Test TDX guest features
 *
 * Copyright (C) 2022 Intel Corporation.
 *
 * Author: Kuppuswamy Sathyanarayanan <sathyanarayanan.kuppuswamy@linux.intel.com>
 */

/* C dependencies:
 *   <sys/ioctl.h>
 *   <errno.h>
 *   <fcntl.h>
 *   <linux/tdx-guest.h>
 *   "kselftest_harness.h"
 */

type __u8 = u8;
type __u64 = u64;

const TDX_GUEST_DEVNAME: &[u8] = b"/dev/tdx_guest\0";
const HEX_DUMP_SIZE: i32 = 8;
const DEBUG: i32 = 0;

/* External constants/types/functions supplied by the C includes above. */
extern "C" {
    static TDX_REPORTDATA_LEN: i32;
    static TDX_CMD_GET_REPORT0: u64;
}

#[repr(C)]
pub struct tdx_report_req {
    pub reportdata: [__u8; 64],
    pub tdreport: [__u8; 1024],
}

extern "C" {
    fn open(pathname: *const i8, flags: i32, ...) -> i32;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
    fn close(fd: i32) -> i32;
    fn printf(format: *const i8, ...) -> i32;
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
}

const O_RDWR: i32 = 0o2;
const O_SYNC: i32 = 0o4010000;

/**
 * struct tdreport_type - Type header of TDREPORT_STRUCT.
 * @type: Type of the TDREPORT (0 - SGX, 81 - TDX, rest are reserved)
 * @sub_type: Subtype of the TDREPORT (Default value is 0).
 * @version: TDREPORT version (Default value is 0).
 * @reserved: Added for future extension.
 *
 * More details can be found in TDX v1.0 module specification, sec
 * titled "REPORTTYPE".
 */
#[repr(C)]
struct tdreport_type {
    type_: __u8,
    sub_type: __u8,
    version: __u8,
    reserved: __u8,
}

/**
 * struct reportmac - TDX guest report data, MAC and TEE hashes.
 * @type: TDREPORT type header.
 * @reserved1: Reserved for future extension.
 * @cpu_svn: CPU security version.
 * @tee_tcb_info_hash: SHA384 hash of TEE TCB INFO.
 * @tee_td_info_hash: SHA384 hash of TDINFO_STRUCT.
 * @reportdata: User defined unique data passed in TDG.MR.REPORT request.
 * @reserved2: Reserved for future extension.
 * @mac: CPU MAC ID.
 *
 * It is MAC-protected and contains hashes of the remainder of the
 * report structure along with user provided report data. More details can
 * be found in TDX v1.0 Module specification, sec titled "REPORTMACSTRUCT"
 */
#[repr(C)]
struct reportmac {
    type_: tdreport_type,
    reserved1: [__u8; 12],
    cpu_svn: [__u8; 16],
    tee_tcb_info_hash: [__u8; 48],
    tee_td_info_hash: [__u8; 48],
    reportdata: [__u8; 64],
    reserved2: [__u8; 32],
    mac: [__u8; 32],
}

/**
 * struct td_info - TDX guest measurements and configuration.
 * @attr: TDX Guest attributes (like debug, spet_disable, etc).
 * @xfam: Extended features allowed mask.
 * @mrtd: Build time measurement register.
 * @mrconfigid: Software-defined ID for non-owner-defined configuration
 *              of the guest - e.g., run-time or OS configuration.
 * @mrowner: Software-defined ID for the guest owner.
 * @mrownerconfig: Software-defined ID for owner-defined configuration of
 *                 the guest - e.g., specific to the workload.
 * @rtmr: Run time measurement registers.
 * @reserved: Added for future extension.
 *
 * It contains the measurements and initial configuration of the TDX guest
 * that was locked at initialization and a set of measurement registers
 * that are run-time extendable. More details can be found in TDX v1.0
 * Module specification, sec titled "TDINFO_STRUCT".
 */
#[repr(C)]
struct td_info {
    attr: [__u8; 8],
    xfam: __u64,
    mrtd: [__u64; 6],
    mrconfigid: [__u64; 6],
    mrowner: [__u64; 6],
    mrownerconfig: [__u64; 6],
    rtmr: [__u64; 24],
    reserved: [__u64; 14],
}

/*
 * struct tdreport - Output of TDCALL[TDG.MR.REPORT].
 * @reportmac: Mac protected header of size 256 bytes.
 * @tee_tcb_info: Additional attestable elements in the TCB are not
 *                reflected in the reportmac.
 * @reserved: Added for future extension.
 * @tdinfo: Measurements and configuration data of size 512 bytes.
 *
 * More details can be found in TDX v1.0 Module specification, sec
 * titled "TDREPORT_STRUCT".
 */
#[repr(C)]
struct tdreport {
    reportmac: reportmac,
    tee_tcb_info: [__u8; 239],
    reserved: [__u8; 17],
    tdinfo: td_info,
}

unsafe fn print_array_hex(
    title: *const i8,
    prefix_str: *const i8,
    buf: *const core::ffi::c_void,
    len: i32,
) {
    let mut i: i32;
    let mut j: i32;
    let mut line_len: i32;
    let rowsize: i32 = HEX_DUMP_SIZE;
    let ptr = buf as *const __u8;

    printf(b"\t\t%s\0".as_ptr() as *const i8, title);

    j = 0;
    while j < len {
        line_len = if rowsize < len - j { rowsize } else { len - j };
        printf(b"%s%.8x:\0".as_ptr() as *const i8, prefix_str, j);
        i = 0;
        while i < line_len {
            printf(
                b" %.2x\0".as_ptr() as *const i8,
                *ptr.offset((j + i) as isize) as i32,
            );
            i += 1;
        }
        printf(b"\n\0".as_ptr() as *const i8);
        j += rowsize;
    }

    printf(b"\n\0".as_ptr() as *const i8);
}

/* TEST(verify_report) */
unsafe fn verify_report() {
    let mut req: tdx_report_req = core::mem::zeroed();
    let tdreport: *mut tdreport;
    let devfd: i32;
    let mut i: i32;

    devfd = open(TDX_GUEST_DEVNAME.as_ptr() as *const i8, O_RDWR | O_SYNC);
    assert!(0 < devfd);

    /* Generate sample report data */
    i = 0;
    while i < TDX_REPORTDATA_LEN {
        req.reportdata[i as usize] = i as __u8;
        i += 1;
    }

    /* Get TDREPORT */
    assert_eq!(0, ioctl(devfd, TDX_CMD_GET_REPORT0, &mut req as *mut tdx_report_req));

    if DEBUG != 0 {
        print_array_hex(
            b"\n\t\tTDX report data\n\0".as_ptr() as *const i8,
            b"\0".as_ptr() as *const i8,
            req.reportdata.as_ptr() as *const core::ffi::c_void,
            core::mem::size_of_val(&req.reportdata) as i32,
        );

        print_array_hex(
            b"\n\t\tTDX tdreport data\n\0".as_ptr() as *const i8,
            b"\0".as_ptr() as *const i8,
            req.tdreport.as_ptr() as *const core::ffi::c_void,
            core::mem::size_of_val(&req.tdreport) as i32,
        );
    }

    /* Make sure TDREPORT data includes the REPORTDATA passed */
    tdreport = req.tdreport.as_mut_ptr() as *mut tdreport;
    assert_eq!(
        0,
        memcmp(
            (*tdreport).reportmac.reportdata.as_ptr() as *const core::ffi::c_void,
            req.reportdata.as_ptr() as *const core::ffi::c_void,
            core::mem::size_of_val(&req.reportdata),
        )
    );

    assert_eq!(0, close(devfd));
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
