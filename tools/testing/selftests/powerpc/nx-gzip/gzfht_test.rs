// SPDX-License-Identifier: GPL-2.0-or-later

/* P9 gzip sample code for demonstrating the P9 NX hardware interface.
 * Not intended for productive uses or for performance or compression
 * ratio measurements.  For simplicity of demonstration, this sample
 * code compresses in to fixed Huffman blocks only (Deflate btype=1)
 * and has very simple memory management.  Dynamic Huffman blocks
 * (Deflate btype=2) are more involved as detailed in the user guide.
 * Note also that /dev/crypto/gzip, VAS and skiboot support are
 * required.
 *
 * Copyright 2020 IBM Corp.
 *
 * https://github.com/libnxz/power-gzip for zlib api and other utils
 *
 * Author: Bulent Abali <abali@us.ibm.com>
 *
 * Definitions of acronyms used here. See
 * P9 NX Gzip Accelerator User's Manual for details:
 * https://github.com/libnxz/power-gzip/blob/develop/doc/power_nx_gzip_um.pdf
 *
 * adler/crc: 32 bit checksums appended to stream tail
 * ce:       completion extension
 * cpb:      coprocessor parameter block (metadata)
 * crb:      coprocessor request block (command)
 * csb:      coprocessor status block (status)
 * dht:      dynamic huffman table
 * dde:      data descriptor element (address, length)
 * ddl:      list of ddes
 * dh/fh:    dynamic and fixed huffman types
 * fc:       coprocessor function code
 * histlen:  history/dictionary length
 * history:  sliding window of up to 32KB of data
 * lzcount:  Deflate LZ symbol counts
 * rembytecnt: remaining byte count
 * sfbt:     source final block type; last block's type during decomp
 * spbc:     source processed byte count
 * subc:     source unprocessed bit count
 * tebc:     target ending bit count; valid bits in the last byte
 * tpbc:     target processed byte count
 * vas:      virtual accelerator switch; the user mode interface
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type FILE = c_void;

static mut nx_dbg: c_int = 0;
static mut nx_gzip_log: *mut FILE = ptr::null_mut();

const FNAME_MAX: usize = 1024;
const FEXT: &[u8] = b".nx.gz\0";

const SYSFS_MAX_REQ_BUF_PATH: &[u8] =
    b"devices/vio/ibm,compression-v1/nx_gzip_caps/req_max_processed_len\0";

/* Header-supplied constants, types, and field encodings from utils.h, nxu.h,
 * and nx.h are external dependencies of this isolated translation.
 */
const GZIP_FC_COMPRESS_RESUME_FHT_COUNT: u32 = 0;
const GZIP_FC_COMPRESS_RESUME_FHT: u32 = 0;
const csb_address_mask: u64 = !0;
const NX_MAX_FAULTS: c_int = 0;
const ERR_NX_OK: c_int = 0;
const ERR_NX_TPBC_GT_SPBC: c_int = 0;
const ERR_NX_AT_FAULT: c_int = 0;
const SA_SIGINFO: c_int = 0;
const SIGSEGV: c_int = 11;
const NX_FUNC_COMP_GZIP: c_int = 0;

macro_rules! NX_MIN {
    ($x:expr, $y:expr) => {
        if ($x) < ($y) { $x } else { $y }
    };
}

macro_rules! NXPRT {
    ($e:expr) => {{
        $e
    }};
}

macro_rules! put32 {
    ($obj:expr, $field:ident, $val:expr) => {{
        put32_field((&mut $obj) as *mut _ as *mut c_void, stringify!($field).as_ptr() as *const c_char, $val as u32)
    }};
}

macro_rules! put64 {
    ($obj:expr, $field:ident, $val:expr) => {{
        put64_field((&mut $obj) as *mut _ as *mut c_void, stringify!($field).as_ptr() as *const c_char, $val as u64)
    }};
}

macro_rules! putnn {
    ($obj:expr, $field:ident, $val:expr) => {{
        putnn_field((&mut $obj) as *mut _ as *mut c_void, stringify!($field).as_ptr() as *const c_char, $val as u64)
    }};
}

macro_rules! get32 {
    ($obj:expr, $field:ident) => {{
        get32_field((&$obj) as *const _ as *const c_void, stringify!($field).as_ptr() as *const c_char)
    }};
}

macro_rules! getnn {
    ($obj:expr, $field:ident) => {{
        getnn_field((&$obj) as *const _ as *const c_void, stringify!($field).as_ptr() as *const c_char)
    }};
}

#[repr(C)]
struct csb_t {
    fsaddr: u64,
    tpbc: u32,
}

#[repr(C)]
struct dde_t {
    _opaque: [u8; 0],
}

#[repr(C)]
struct crb_t {
    csb: csb_t,
    source_dde: dde_t,
    target_dde: dde_t,
}

#[repr(C)]
struct cpb_t {
    _opaque: [u8; 0],
}

#[repr(C)]
struct nx_gzip_crb_cpb_t {
    crb: crb_t,
    cpb: cpb_t,
}

#[repr(C)]
struct sigset_t {
    _opaque: [c_ulong; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: usize,
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: usize,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn put32_field(obj: *mut c_void, field: *const c_char, val: u32);
    fn put64_field(obj: *mut c_void, field: *const c_char, val: u64);
    fn putnn_field(obj: *mut c_void, field: *const c_char, val: u64);
    fn get32_field(obj: *const c_void, field: *const c_char) -> u32;
    fn getnn_field(obj: *const c_void, field: *const c_char) -> u32;
    fn clear_dde(dde: dde_t);

    fn nxu_submit_job(cmdp: *mut nx_gzip_crb_cpb_t, handle: *mut c_void) -> c_int;
    fn nxu_touch_pages(buf: *mut c_void, len: usize, pagelen: u32, write: c_int);
    fn read_file_alloc(fname: *const c_char, buf: *mut *mut c_char, len: *mut usize) -> c_int;
    fn read_sysfs_file(path: *const c_char, buf: *mut c_char, len: usize) -> c_int;
    fn write_file(fname: *const c_char, buf: *mut c_char, len: usize) -> c_int;
    fn nxu_sigsegv_handler(sig: c_int, info: *mut c_void, ucontext: *mut c_void);
    fn nx_function_begin(func: c_int, pri: c_int) -> *mut c_void;
    fn nx_function_end(handle: *mut c_void);
}

fn be32toh(x: u32) -> u32 {
    u32::from_be(x)
}

/*
 * LZ counts returned in the user supplied nx_gzip_crb_cpb_t structure.
 */
unsafe fn compress_fht_sample(
    src: *mut c_char,
    srclen: u32,
    dst: *mut c_char,
    dstlen: u32,
    with_count: c_int,
    cmdp: *mut nx_gzip_crb_cpb_t,
    handle: *mut c_void,
) -> c_int {
    let fc: u32;

    assert!(!cmdp.is_null());

    put32!((*cmdp).crb, gzip_fc, 0); /* clear */
    fc = if with_count != 0 {
        GZIP_FC_COMPRESS_RESUME_FHT_COUNT
    } else {
        GZIP_FC_COMPRESS_RESUME_FHT
    };
    putnn!((*cmdp).crb, gzip_fc, fc);
    putnn!((*cmdp).cpb, in_histlen, 0); /* resuming with no history */
    memset(
        (&mut (*cmdp).crb.csb) as *mut csb_t as *mut c_void,
        0,
        size_of::<csb_t>(),
    );

    /* Section 6.6 programming notes; spbc may be in two different
     * places depending on FC.
     */
    if with_count == 0 {
        put32!((*cmdp).cpb, out_spbc_comp, 0);
    } else {
        put32!((*cmdp).cpb, out_spbc_comp_with_count, 0);
    }

    /* Figure 6-3 6-4; CSB location */
    put64!((*cmdp).crb, csb_address, 0);
    put64!(
        (*cmdp).crb,
        csb_address,
        ((&mut (*cmdp).crb.csb) as *mut csb_t as u64) & csb_address_mask
    );

    /* Source direct dde (scatter-gather list) */
    clear_dde((*cmdp).crb.source_dde);
    putnn!((*cmdp).crb.source_dde, dde_count, 0);
    put32!((*cmdp).crb.source_dde, ddebc, srclen);
    put64!((*cmdp).crb.source_dde, ddead, src as u64);

    /* Target direct dde (scatter-gather list) */
    clear_dde((*cmdp).crb.target_dde);
    putnn!((*cmdp).crb.target_dde, dde_count, 0);
    put32!((*cmdp).crb.target_dde, ddebc, dstlen);
    put64!((*cmdp).crb.target_dde, ddead, dst as u64);

    /* Submit the crb, the job descriptor, to the accelerator */
    nxu_submit_job(cmdp, handle)
}

/*
 * Prepares a blank no filename no timestamp gzip header and returns
 * the number of bytes written to buf.
 * Gzip specification at https://tools.ietf.org/html/rfc1952
 */
unsafe fn gzip_header_blank(buf: *mut c_char) -> c_int {
    let mut i: c_int = 0;

    *buf.add(i as usize) = 0x1f; /* ID1 */
    i += 1;
    *buf.add(i as usize) = 0x8b_u8 as c_char; /* ID2 */
    i += 1;
    *buf.add(i as usize) = 0x08; /* CM  */
    i += 1;
    *buf.add(i as usize) = 0x00; /* FLG */
    i += 1;
    *buf.add(i as usize) = 0x00; /* MTIME */
    i += 1;
    *buf.add(i as usize) = 0x00; /* MTIME */
    i += 1;
    *buf.add(i as usize) = 0x00; /* MTIME */
    i += 1;
    *buf.add(i as usize) = 0x00; /* MTIME */
    i += 1;
    *buf.add(i as usize) = 0x04; /* XFL 4=fastest */
    i += 1;
    *buf.add(i as usize) = 0x03; /* OS UNIX */
    i += 1;

    i
}

/*
 * Z_SYNC_FLUSH as described in zlib.h.
 * Returns number of appended bytes
 */
unsafe fn append_sync_flush(mut buf: *mut c_char, tebc: c_int, final_: c_int) -> c_int {
    let mut flush: u64;
    let mut shift: c_int = tebc & 0x7;

    if tebc > 0 {
        /* Last byte is partially full */
        buf = buf.offset(-1);
        *buf = ((*buf as u8) & (((1 << tebc) - 1) as u8)) as c_char;
    } else {
        *buf = 0;
    }
    flush = (((0x1_u64) & (final_ as u64)) << shift) | ((*buf as u8) as u64);
    shift = shift + 3; /* BFINAL and BTYPE written */
    shift = if shift <= 8 { 8 } else { 16 };
    flush |= (0xFFFF0000_u64) << shift; /* Zero length block */
    shift = shift + 32;
    while shift > 0 {
        *buf = (flush & 0xff_u64) as u8 as c_char;
        buf = buf.add(1);
        flush = flush >> 8;
        shift = shift - 8;
    }
    if (tebc > 5) || (tebc == 0) { 5 } else { 4 }
}

/*
 * Final deflate block bit.  This call assumes the block
 * beginning is byte aligned.
 */
unsafe fn set_bfinal(buf: *mut c_void, bfinal: c_int) {
    let b: *mut c_char = buf as *mut c_char;

    if bfinal != 0 {
        *b = ((*b as u8) | 0x01_u8) as c_char;
    } else {
        *b = ((*b as u8) & 0xfe_u8) as c_char;
    }
}

unsafe fn compress_file(argc: c_int, argv: *mut *mut c_char, handle: *mut c_void) -> c_int {
    let mut inbuf: *mut c_char = ptr::null_mut();
    let mut outbuf: *mut c_char;
    let mut srcbuf: *mut c_char;
    let mut dstbuf: *mut c_char;
    let mut outname: [c_char; FNAME_MAX] = [0; FNAME_MAX];
    let mut srclen: u32;
    let mut dstlen: u32;
    let mut flushlen: u32;
    let mut chunk: u32;
    let mut inlen: usize = 0;
    let mut outlen: usize;
    let mut dsttotlen: usize;
    let mut srctotlen: usize;
    let mut crc: u32 = 0;
    let mut spbc: u32;
    let mut tpbc: u32;
    let mut tebc: u32;
    let lzcounts: c_int = 0;
    let mut cc: c_int;
    let num_hdr_bytes: c_int;
    let cmdp: *mut nx_gzip_crb_cpb_t;
    let pagelen: u32 = 65536;
    let mut fault_tries: c_int = NX_MAX_FAULTS;
    let mut buf: [c_char; 32] = [0; 32];

    cmdp = aligned_alloc(
        size_of::<nx_gzip_crb_cpb_t>(),
        size_of::<nx_gzip_crb_cpb_t>(),
    ) as *mut nx_gzip_crb_cpb_t;

    if argc != 2 {
        fprintf(stderr, b"usage: %s <fname>\n\0".as_ptr() as *const c_char, *argv.add(0));
        exit(-1);
    }
    if read_file_alloc(*argv.add(1), &mut inbuf, &mut inlen) != 0 {
        exit(-1);
    }
    fprintf(
        stderr,
        b"file %s read, %ld bytes\n\0".as_ptr() as *const c_char,
        *argv.add(1),
        inlen as c_long,
    );

    /* Generous output buffer for header/trailer */
    outlen = 2 * inlen + 1024;

    outbuf = malloc(outlen) as *mut c_char;
    assert!(!outbuf.is_null());
    nxu_touch_pages(outbuf as *mut c_void, outlen, pagelen, 1);

    /*
     * On PowerVM, the hypervisor defines the maximum request buffer
     * size is defined and this value is available via sysfs.
     */
    if read_sysfs_file(
        SYSFS_MAX_REQ_BUF_PATH.as_ptr() as *const c_char,
        buf.as_mut_ptr(),
        buf.len(),
    ) == 0
    {
        chunk = atoi(buf.as_ptr()) as u32;
    } else {
        /* sysfs entry is not available on PowerNV */
        /* Compress piecemeal in smallish chunks */
        chunk = 1 << 22;
    }

    /* Write the gzip header to the stream */
    num_hdr_bytes = gzip_header_blank(outbuf);
    dstbuf = outbuf.add(num_hdr_bytes as usize);
    outlen = outlen - num_hdr_bytes as usize;
    dsttotlen = num_hdr_bytes as usize;

    srcbuf = inbuf;
    srctotlen = 0;

    /* Init the CRB, the coprocessor request block */
    memset(
        (&mut (*cmdp).crb) as *mut crb_t as *mut c_void,
        0,
        size_of::<crb_t>(),
    );

    /* Initial gzip crc32 */
    put32!((*cmdp).cpb, in_crc, 0);

    while inlen > 0 {
        /* Submit chunk size source data per job */
        srclen = NX_MIN!(chunk, inlen as u32);
        /* Supply large target in case data expands */
        dstlen = NX_MIN!(2 * srclen, outlen as u32);

        /* Page faults are handled by the user code */

        /* Fault-in pages; an improved code wouldn't touch so
         * many pages but would try to estimate the
         * compression ratio and adjust both the src and dst
         * touch amounts.
         */
        nxu_touch_pages(
            cmdp as *mut c_void,
            size_of::<nx_gzip_crb_cpb_t>(),
            pagelen,
            1,
        );
        nxu_touch_pages(srcbuf as *mut c_void, srclen as usize, pagelen, 0);
        nxu_touch_pages(dstbuf as *mut c_void, dstlen as usize, pagelen, 1);

        cc = compress_fht_sample(srcbuf, srclen, dstbuf, dstlen, lzcounts, cmdp, handle);

        if cc != ERR_NX_OK && cc != ERR_NX_TPBC_GT_SPBC && cc != ERR_NX_AT_FAULT {
            fprintf(stderr, b"nx error: cc= %d\n\0".as_ptr() as *const c_char, cc);
            exit(-1);
        }

        /* Page faults are handled by the user code */
        if cc == ERR_NX_AT_FAULT {
            NXPRT!(fprintf(stderr, b"page fault: cc= %d, \0".as_ptr() as *const c_char, cc));
            NXPRT!(fprintf(
                stderr,
                b"try= %d, fsa= %08llx\n\0".as_ptr() as *const c_char,
                fault_tries,
                (*cmdp).crb.csb.fsaddr as u64,
            ));
            fault_tries -= 1;
            if fault_tries > 0 {
                continue;
            } else {
                fprintf(stderr, b"error: cannot progress; \0".as_ptr() as *const c_char);
                fprintf(stderr, b"too many faults\n\0".as_ptr() as *const c_char);
                exit(-1);
            }
        }

        fault_tries = NX_MAX_FAULTS; /* Reset for the next chunk */

        inlen = inlen - srclen as usize;
        srcbuf = srcbuf.add(srclen as usize);
        srctotlen = srctotlen + srclen as usize;

        /* Two possible locations for spbc depending on the function
         * code.
         */
        spbc = if lzcounts == 0 {
            get32!((*cmdp).cpb, out_spbc_comp)
        } else {
            get32!((*cmdp).cpb, out_spbc_comp_with_count)
        };
        assert!(spbc == srclen);

        /* Target byte count */
        tpbc = get32!((*cmdp).crb.csb, tpbc);
        /* Target ending bit count */
        tebc = getnn!((*cmdp).cpb, out_tebc);
        NXPRT!(fprintf(
            stderr,
            b"compressed chunk %d \0".as_ptr() as *const c_char,
            spbc,
        ));
        NXPRT!(fprintf(
            stderr,
            b"to %d bytes, tebc= %d\n\0".as_ptr() as *const c_char,
            tpbc,
            tebc,
        ));

        if inlen > 0 {
            /* More chunks to go */
            set_bfinal(dstbuf as *mut c_void, 0);
            dstbuf = dstbuf.add(tpbc as usize);
            dsttotlen = dsttotlen + tpbc as usize;
            outlen = outlen - tpbc as usize;
            /* Round up to the next byte with a flush
             * block; do not set the BFINAqL bit.
             */
            flushlen = append_sync_flush(dstbuf, tebc as c_int, 0) as u32;
            dsttotlen = dsttotlen + flushlen as usize;
            outlen = outlen - flushlen as usize;
            dstbuf = dstbuf.add(flushlen as usize);
            NXPRT!(fprintf(
                stderr,
                b"added sync_flush %d bytes\n\0".as_ptr() as *const c_char,
                flushlen,
            ));
        } else {
            /* Done */
            /* Set the BFINAL bit of the last block per Deflate
             * specification.
             */
            set_bfinal(dstbuf as *mut c_void, 1);
            dstbuf = dstbuf.add(tpbc as usize);
            dsttotlen = dsttotlen + tpbc as usize;
            outlen = outlen - tpbc as usize;
        }

        /* Resuming crc32 for the next chunk */
        crc = get32!((*cmdp).cpb, out_crc);
        put32!((*cmdp).cpb, in_crc, crc);
        crc = be32toh(crc);
    }

    /* Append crc32 and ISIZE to the end */
    memcpy(dstbuf as *mut c_void, (&crc) as *const u32 as *const c_void, 4);
    memcpy(
        dstbuf.add(4) as *mut c_void,
        (&srctotlen) as *const usize as *const c_void,
        4,
    );
    dsttotlen = dsttotlen + 8;
    outlen = outlen - 8;

    assert!(FNAME_MAX > (strlen(*argv.add(1)) + strlen(FEXT.as_ptr() as *const c_char)));
    strcpy(outname.as_mut_ptr(), *argv.add(1));
    strcat(outname.as_mut_ptr(), FEXT.as_ptr() as *const c_char);
    if write_file(outname.as_mut_ptr(), outbuf, dsttotlen) != 0 {
        fprintf(
            stderr,
            b"write error: %s\n\0".as_ptr() as *const c_char,
            outname.as_mut_ptr(),
        );
        exit(-1);
    }

    fprintf(
        stderr,
        b"compressed %ld to %ld bytes total, \0".as_ptr() as *const c_char,
        srctotlen as c_long,
        dsttotlen as c_long,
    );
    fprintf(
        stderr,
        b"crc32 checksum = %08x\n\0".as_ptr() as *const c_char,
        crc,
    );

    if !inbuf.is_null() {
        free(inbuf as *mut c_void);
    }

    if !outbuf.is_null() {
        free(outbuf as *mut c_void);
    }

    0
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let rc: c_int;
    let mut act: sigaction = core::mem::zeroed();
    let handle: *mut c_void;

    nx_dbg = 0;
    nx_gzip_log = ptr::null_mut();
    act.sa_handler = 0;
    act.sa_sigaction = Some(nxu_sigsegv_handler);
    act.sa_flags = SA_SIGINFO;
    act.sa_restorer = 0;
    sigemptyset(&mut act.sa_mask);
    sigaction(SIGSEGV, &act, ptr::null_mut());

    handle = nx_function_begin(NX_FUNC_COMP_GZIP, 0);
    if handle.is_null() {
        fprintf(
            stderr,
            b"Unable to init NX, errno %d\n\0".as_ptr() as *const c_char,
            errno,
        );
        exit(-1);
    }

    rc = compress_file(argc, argv, handle);

    nx_function_end(handle);

    rc
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(ptr::null_mut());
    let rc = unsafe { main_0((args.len() - 1) as c_int, args.as_mut_ptr()) };
    for arg in args.into_iter().filter(|p| !p.is_null()) {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }
    std::process::exit(rc);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
