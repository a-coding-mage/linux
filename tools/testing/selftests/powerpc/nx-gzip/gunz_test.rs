// SPDX-License-Identifier: GPL-2.0-or-later

/* P9 gunzip sample code for demonstrating the P9 NX hardware
 * interface.  Not intended for productive uses or for performance or
 * compression ratio measurements.  Note also that /dev/crypto/gzip,
 * VAS and skiboot support are required
 *
 * Copyright 2020 IBM Corp.
 *
 * Author: Bulent Abali <abali@us.ibm.com>
 *
 * https://github.com/libnxz/power-gzip for zlib api and other utils
 * Definitions of acronyms used here.  See
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

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type FILE = c_void;
type off_t = c_long;
type size_t = usize;
type uint32_t = u32;
type uint64_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_dde_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_csb_t {
    pub fsaddr: uint64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_crb_t {
    pub csb: nx_csb_t,
    pub source_dde: nx_dde_t,
    pub target_dde: nx_dde_t,
    pub gzip_fc: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_cpb_t {
    pub out_spbc_comp_wrap: uint32_t,
    pub out_spbc_comp_with_count: uint32_t,
    pub out_spbc_decomp: uint32_t,
    pub out_crc: uint32_t,
    pub out_adler: uint32_t,
    pub in_crc: uint32_t,
    pub in_adler: uint32_t,
    pub in_histlen: uint32_t,
    pub in_subc: uint32_t,
    pub in_sfbt: uint32_t,
    pub in_rembytecnt: uint32_t,
    pub in_dhtlen: uint32_t,
    pub out_dhtlen: uint32_t,
    pub out_rembytecnt: uint32_t,
    pub out_sfbt: uint32_t,
    pub out_subc: uint32_t,
    pub in_dht: [uint64_t; 286],
    pub out_dht: [uint64_t; 286],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_crb_cpb_t {
    pub crb: nx_gzip_crb_t,
    pub cpb: nx_gzip_cpb_t,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 128],
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: usize,
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: usize,
}

unsafe extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn fgetc(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn nxu_touch_pages(buf: *mut c_void, buf_len: c_long, page_sz: c_long, wr: c_int);
    fn nxu_submit_job(cmdp: *mut nx_gzip_crb_cpb_t, handle: *mut c_void) -> c_int;
    fn nxu_sigsegv_handler(sig: c_int, info: *mut c_void, context: *mut c_void);
    fn nx_function_begin(func: c_int, pri: c_int) -> *mut c_void;
    fn nx_function_end(handle: *mut c_void);

    fn clearp_dde(ddl: *mut nx_dde_t);
    fn clear_dde(dde: nx_dde_t);
    fn getpnn(ddl: *mut nx_dde_t, field: c_int) -> uint32_t;
    fn getp32(ddl: *mut nx_dde_t, field: c_int) -> uint32_t;
    fn getp64(ddl: *mut nx_dde_t, field: c_int) -> uint64_t;
    fn get32(dde: nx_dde_t, field: c_int) -> uint32_t;
    fn get64(dde: nx_dde_t, field: c_int) -> uint64_t;
    fn getnn(cpb: nx_gzip_cpb_t, field: c_int) -> c_int;
    fn putpnn(ddl: *mut nx_dde_t, field: c_int, val: uint32_t);
    fn putp32(ddl: *mut nx_dde_t, field: c_int, val: uint32_t);
    fn putp64(ddl: *mut nx_dde_t, field: c_int, val: uint64_t);
    fn put32<T>(obj: T, field: c_int, val: uint32_t);
    fn put64<T>(obj: T, field: c_int, val: uint64_t);
    fn putnn<T>(obj: T, field: c_int, val: c_int);
    fn get_csb_ce_ms3b(csb: nx_csb_t) -> c_int;
    fn csb_ce_termination(nx_ce: c_int) -> c_int;
    fn csb_ce_partial_completion(nx_ce: c_int) -> c_int;
}

unsafe extern "C" {
    static mut nx_dbg: c_int;
    static mut nx_gzip_log: *mut FILE;
}

const EOF: c_int = -1;
const SIGSEGV: c_int = 11;
const SA_SIGINFO: c_int = 4;

const FNAME_MAX: usize = 1024;
const fifo_in_len: c_int = 1 << 24;
const fifo_out_len: c_int = 1 << 24;
const page_sz: c_int = 1 << 16;
const line_sz: c_int = 1 << 7;
const window_max: c_int = 1 << 15;

const ERR_NX_OK: c_int = 0;
const ERR_NX_AT_FAULT: c_int = 1;
const ERR_NX_TARGET_SPACE: c_int = 2;
const ERR_NX_DATA_LENGTH: c_int = 3;
const ERR_NX_EXCESSIVE_DDE: c_int = -1;
const MAX_DDE_COUNT: uint32_t = 64;
const NX_MAX_FAULTS: c_int = 20;
const GZIP_FC_DECOMPRESS: c_int = 0;
const GZIP_FC_DECOMPRESS_RESUME: c_int = 1;
const INIT_CRC: uint32_t = 0;
const INIT_ADLER: uint32_t = 1;
const NX_FUNC_COMP_GZIP: c_int = 0;
const csb_address_mask: uint64_t = !0u64;

const dde_count: c_int = 0;
const ddebc: c_int = 1;
const ddead: c_int = 2;
const csb_address: c_int = 3;
const out_crc: c_int = 4;
const out_adler: c_int = 5;
const in_histlen: c_int = 6;
const gzip_fc: c_int = 7;
const in_crc: c_int = 8;
const in_adler: c_int = 9;
const out_sfbt: c_int = 10;
const out_subc: c_int = 11;
const out_spbc_decomp: c_int = 12;
const tpbc: c_int = 13;
const in_subc: c_int = 14;
const in_sfbt: c_int = 15;
const in_rembytecnt: c_int = 16;
const out_rembytecnt: c_int = 17;
const out_dhtlen: c_int = 18;
const in_dhtlen: c_int = 19;

unsafe fn NX_MIN<T: Ord>(x: T, y: T) -> T {
    if x < y { x } else { y }
}

unsafe fn NX_MAX<T: Ord>(x: T, y: T) -> T {
    if x > y { x } else { y }
}

unsafe fn GETINPC(x: *mut FILE) -> c_int {
    unsafe { fgetc(x) }
}

unsafe fn fifo_used_bytes(used: c_int) -> c_int {
    used
}

unsafe fn fifo_free_bytes(used: c_int, len: c_int) -> c_int {
    len - used
}

unsafe fn fifo_free_first_bytes(cur: c_int, used: c_int, len: c_int) -> c_int {
    if cur + used <= len { len - (cur + used) } else { 0 }
}

unsafe fn fifo_free_last_bytes(cur: c_int, used: c_int, len: c_int) -> c_int {
    if cur + used <= len { cur } else { len - used }
}

unsafe fn fifo_used_first_bytes(cur: c_int, used: c_int, len: c_int) -> c_int {
    if cur + used <= len { used } else { len - cur }
}

unsafe fn fifo_used_last_bytes(cur: c_int, used: c_int, len: c_int) -> c_int {
    if cur + used <= len { 0 } else { used + cur - len }
}

unsafe fn fifo_free_first_offset(cur: c_int, used: c_int) -> c_int {
    cur + used
}

unsafe fn fifo_free_last_offset(cur: c_int, used: c_int, len: c_int) -> c_int {
    unsafe { fifo_used_last_bytes(cur, used, len) }
}

unsafe fn fifo_used_first_offset(cur: c_int) -> c_int {
    cur
}

unsafe fn fifo_used_last_offset(_cur: c_int) -> c_int {
    0
}

/*
 * Adds an (address, len) pair to the list of ddes (ddl) and updates
 * the base dde.  ddl[0] is the only dde in a direct dde which
 * contains a single (addr,len) pair.  For more pairs, ddl[0] becomes
 * the indirect (base) dde that points to a list of direct ddes.
 * See Section 6.4 of the NX-gzip user manual for DDE description.
 * Addr=NULL, len=0 clears the ddl[0].  Returns the total number of
 * bytes in ddl.  Caller is responsible for allocting the array of
 * nx_dde_t *ddl.  If N addresses are required in the scatter-gather
 * list, the ddl array must have N+1 entries minimum.
 */
unsafe fn nx_append_dde(ddl: *mut nx_dde_t, addr: *mut c_void, len: uint32_t) -> uint32_t {
    let mut ddecnt: uint32_t;
    let mut bytes: uint32_t;

    if addr.is_null() && len == 0 {
        unsafe { clearp_dde(ddl) };
        return 0;
    }

    /* Number of ddes in the dde list ; == 0 when it is a direct dde */
    ddecnt = unsafe { getpnn(ddl, dde_count) };
    bytes = unsafe { getp32(ddl, ddebc) };

    if ddecnt == 0 && bytes == 0 {
        /* First dde is unused; make it a direct dde */
        bytes = len;
        unsafe {
            putp32(ddl, ddebc, bytes);
            putp64(ddl, ddead, addr as uint64_t);
        }
    } else if ddecnt == 0 {
        /* Converting direct to indirect dde
         * ddl[0] becomes head dde of ddl
         * copy direct to indirect first.
         */
        unsafe {
            *ddl.add(1) = *ddl.add(0);

            /* Add the new dde next */
            clear_dde(*ddl.add(2));
            put32(*ddl.add(2), ddebc, len);
            put64(*ddl.add(2), ddead, addr as uint64_t);

            /* Ddl head points to 2 direct ddes */
            ddecnt = 2;
            putpnn(ddl, dde_count, ddecnt);
            bytes = bytes.wrapping_add(len);
            putp32(ddl, ddebc, bytes);
            /* Pointer to the first direct dde */
            putp64(ddl, ddead, ddl.add(1) as uint64_t);
        }
    } else {
        /* Append a dde to an existing indirect ddl */
        ddecnt = ddecnt.wrapping_add(1);
        unsafe {
            clear_dde(*ddl.add(ddecnt as usize));
            put64(*ddl.add(ddecnt as usize), ddead, addr as uint64_t);
            put32(*ddl.add(ddecnt as usize), ddebc, len);

            putpnn(ddl, dde_count, ddecnt);
            bytes = bytes.wrapping_add(len);
            putp32(ddl, ddebc, bytes); /* byte sum of all dde */
        }
    }
    bytes
}

/*
 * Touch specified number of pages represented in number bytes
 * beginning from the first buffer in a dde list.
 * Do not touch the pages past buf_sz-th byte's page.
 *
 * Set buf_sz = 0 to touch all pages described by the ddep.
 */
unsafe fn nx_touch_pages_dde(ddep: *mut nx_dde_t, mut buf_sz: c_long, page_sz_: c_long, wr: c_int) -> c_int {
    let indirect_count: uint32_t;
    let mut buf_len: uint32_t;
    let mut total: c_long;
    let mut buf_addr: uint64_t;
    let dde_list: *mut nx_dde_t;
    let mut i: c_int;

    assert!(!ddep.is_null());

    indirect_count = unsafe { getpnn(ddep, dde_count) };

    if indirect_count == 0 {
        /* Direct dde */
        buf_len = unsafe { getp32(ddep, ddebc) };
        buf_addr = unsafe { getp64(ddep, ddead) };

        if buf_sz == 0 {
            unsafe { nxu_touch_pages(buf_addr as *mut c_void, buf_len as c_long, page_sz_, wr) };
        } else {
            unsafe { nxu_touch_pages(buf_addr as *mut c_void, NX_MIN(buf_len as c_long, buf_sz), page_sz_, wr) };
        }

        return ERR_NX_OK;
    }

    /* Indirect dde */
    if indirect_count > MAX_DDE_COUNT {
        return ERR_NX_EXCESSIVE_DDE;
    }

    /* First address of the list */
    dde_list = unsafe { getp64(ddep, ddead) as *mut nx_dde_t };

    if buf_sz == 0 {
        buf_sz = unsafe { getp32(ddep, ddebc) as c_long };
    }

    total = 0;
    i = 0;
    while i < indirect_count as c_int {
        unsafe {
            buf_len = get32(*dde_list.add(i as usize), ddebc);
            buf_addr = get64(*dde_list.add(i as usize), ddead);
        }
        total += buf_len as c_long;

        /* Touching fewer pages than encoded in the ddebc */
        if total > buf_sz {
            buf_len = unsafe { NX_MIN(buf_len as c_long, total - buf_sz) as uint32_t };
            unsafe { nxu_touch_pages(buf_addr as *mut c_void, buf_len as c_long, page_sz_, wr) };
            break;
        }
        unsafe { nxu_touch_pages(buf_addr as *mut c_void, buf_len as c_long, page_sz_, wr) };
        i += 1;
    }
    ERR_NX_OK
}

/*
 * Src and dst buffers are supplied in scatter gather lists.
 * NX function code and other parameters supplied in cmdp.
 */
unsafe fn nx_submit_job(
    src: *mut nx_dde_t,
    dst: *mut nx_dde_t,
    cmdp: *mut nx_gzip_crb_cpb_t,
    handle: *mut c_void,
) -> c_int {
    let csbaddr: uint64_t;

    unsafe {
        memset(&mut (*cmdp).crb.csb as *mut _ as *mut c_void, 0, size_of::<nx_csb_t>());

        (*cmdp).crb.source_dde = *src;
        (*cmdp).crb.target_dde = *dst;

        /* Status, output byte count in tpbc */
        csbaddr = (&mut (*cmdp).crb.csb as *mut _ as uint64_t) & csb_address_mask;
        put64((*cmdp).crb, csb_address, csbaddr);

        /* NX reports input bytes in spbc; cleared */
        (*cmdp).cpb.out_spbc_comp_wrap = 0;
        (*cmdp).cpb.out_spbc_comp_with_count = 0;
        (*cmdp).cpb.out_spbc_decomp = 0;

        /* Clear output */
        put32((*cmdp).cpb, out_crc, INIT_CRC);
        put32((*cmdp).cpb, out_adler, INIT_ADLER);

        /* Submit the crb, the job descriptor, to the accelerator. */
        nxu_submit_job(cmdp, handle)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Read,
    Write,
    Decomp,
    RestartNx,
    OkCc3,
    Offsets,
    Finish,
    Err1,
    Err2,
    Err3,
    Err4,
    Err5,
    Ok1,
}

pub unsafe extern "C" fn decompress_file(argc: c_int, argv: *mut *mut c_char, devhandle: *mut c_void) -> c_int {
    let mut inpf: *mut FILE = ptr::null_mut();
    let mut outf: *mut FILE = ptr::null_mut();

    let mut c: c_int = 0;
    let mut expect: c_int = 0;
    let mut i: c_int;
    let mut cc: c_int = 0;
    let mut rc: c_int = 0;
    let mut gzfname: [c_char; FNAME_MAX] = [0; FNAME_MAX];

    /* Queuing, file ops, byte counting */
    let mut fifo_in: *mut c_char;
    let mut fifo_out: *mut c_char;
    let mut used_in: c_int;
    let mut cur_in: c_int;
    let mut used_out: c_int;
    let mut cur_out: c_int;
    let mut read_sz: c_int;
    let mut n: c_int;
    let mut first_free: c_int;
    let mut last_free: c_int;
    let mut first_used: c_int;
    let mut last_used: c_int;
    let mut first_offset: c_int;
    let mut last_offset: c_int;
    let mut write_sz: c_int;
    let mut free_space: c_int;
    let mut source_sz: c_int = 0;
    let mut source_sz_estimate: c_int;
    let mut target_sz_estimate: c_int = 0;
    let mut last_comp_ratio: uint64_t = 0; /* 1000 max */
    let mut total_out: uint64_t = 0;
    let mut is_final: c_int;
    let mut is_eof: c_int;

    /* nx hardware */
    let mut sfbt: c_int = 0;
    let mut subc: c_int = 0;
    let mut spbc: c_int = 0;
    let mut tpbc_: c_int = 0;
    let mut nx_ce: c_int;
    let mut fc: c_int;
    let mut resuming: c_int = 0;
    let mut history_len: c_int = 0;
    let mut cmd: nx_gzip_crb_cpb_t = unsafe { core::mem::zeroed() };
    let mut cmdp: *mut nx_gzip_crb_cpb_t;
    let mut dde_in: [nx_dde_t; 6] = unsafe { core::mem::zeroed() };
    let mut ddl_in: *mut nx_dde_t;
    let mut dde_out: [nx_dde_t; 6] = unsafe { core::mem::zeroed() };
    let mut ddl_out: *mut nx_dde_t;
    let mut pgfault_retries: c_int = 0;

    /* when using mmap'ed files */
    let mut input_file_offset: off_t = 0;

    unsafe {
        if argc > 2 {
            fprintf(stderr, c"usage: %s <fname> or stdin\n".as_ptr(), *argv.add(0));
            fprintf(stderr, c"    writes to stdout or <fname>.nx.gunzip\n".as_ptr());
            return -1;
        }

        if argc == 1 {
            inpf = stdin;
            outf = stdout;
        } else if argc == 2 {
            let mut w: [c_char; 1024] = [0; 1024];
            let mut wp: *mut c_char;

            inpf = fopen(*argv.add(1), c"r".as_ptr());
            if inpf.is_null() {
                perror(*argv.add(1));
                return -1;
            }

            /* Make a new file name to write to.  Ignoring '.gz' */
            wp = strrchr(*argv.add(1), '/' as c_int);
            if !wp.is_null() {
                wp = wp.add(1);
            } else {
                wp = *argv.add(1);
            }
            strcpy(w.as_mut_ptr(), wp);
            strcat(w.as_mut_ptr(), c".nx.gunzip".as_ptr());

            outf = fopen(w.as_mut_ptr(), c"w".as_ptr());
            if outf.is_null() {
                perror(w.as_mut_ptr());
                return -1;
            }
        }

        /* Decode the gzip header */
        c = GETINPC(inpf);
        expect = 0x1f; /* ID1 */
        if c != expect {
            return err1(expect, c);
        }

        c = GETINPC(inpf);
        expect = 0x8b; /* ID2 */
        if c != expect {
            return err1(expect, c);
        }

        c = GETINPC(inpf);
        expect = 0x08; /* CM */
        if c != expect {
            return err1(expect, c);
        }

        let flg: c_int = GETINPC(inpf); /* FLG */

        if (flg & 0xE0) != 0 || (flg & 0x4) != 0 || flg == EOF {
            return err2();
        }

        fprintf(stderr, c"gzHeader FLG %x\n".as_ptr(), flg);

        /* Read 6 bytes; ignoring the MTIME, XFL, OS fields in this
         * sample code.
         */
        i = 0;
        while i < 6 {
            let mut tmp: [c_char; 10] = [0; 10];

            tmp[i as usize] = GETINPC(inpf) as c_char;
            if tmp[i as usize] as c_int == EOF {
                return err3();
            }
            fprintf(stderr, c"%02x ".as_ptr(), tmp[i as usize] as c_int);
            if i == 5 {
                fprintf(stderr, c"\n".as_ptr());
            }
            i += 1;
        }
        fprintf(stderr, c"gzHeader MTIME, XFL, OS ignored\n".as_ptr());

        /* FNAME */
        if (flg & 0x8) != 0 {
            let mut k: c_int = 0;

            loop {
                c = GETINPC(inpf);
                if c == EOF || k >= FNAME_MAX as c_int {
                    return err3();
                }
                gzfname[k as usize] = c as c_char;
                k += 1;
                if c == 0 {
                    break;
                }
            }
            fprintf(stderr, c"gzHeader FNAME: %s\n".as_ptr(), gzfname.as_ptr());
        }

        /* FHCRC */
        if (flg & 0x2) != 0 {
            c = GETINPC(inpf);
            if c == EOF {
                return err3();
            }
            c = GETINPC(inpf);
            if c == EOF {
                return err3();
            }
            fprintf(stderr, c"gzHeader FHCRC: ignored\n".as_ptr());
        }

        used_in = 0;
        cur_in = 0;
        used_out = 0;
        cur_out = 0;
        is_final = 0;
        is_eof = 0;

        /* Allocate one page larger to prevent page faults due to NX
         * overfetching.
         * Either do this (char*)(uintptr_t)aligned_alloc or use
         * -std=c11 flag to make the int-to-pointer warning go away.
         */
        fifo_in = aligned_alloc(line_sz as size_t, (fifo_in_len + page_sz) as size_t) as *mut c_char;
        assert!(!fifo_in.is_null());
        fifo_out = aligned_alloc(line_sz as size_t, (fifo_out_len + page_sz + line_sz) as size_t) as *mut c_char;
        assert!(!fifo_out.is_null());
        /* Leave unused space due to history rounding rules */
        fifo_out = fifo_out.add(line_sz as usize);
        nxu_touch_pages(fifo_out as *mut c_void, fifo_out_len as c_long, page_sz as c_long, 1);

        ddl_in = dde_in.as_mut_ptr();
        ddl_out = dde_out.as_mut_ptr();
        cmdp = &mut cmd;
        memset(&mut (*cmdp).crb as *mut _ as *mut c_void, 0, size_of::<nx_gzip_crb_t>());
    }

    let mut state = State::Read;
    loop {
        match state {
            State::Read => unsafe {
                /* Read from .gz file */
                if is_eof != 0 {
                    state = State::Write;
                    continue;
                }

                /* We read in to fifo_in in two steps: first: read in to from
                 * cur_in to the end of the buffer.  last: if free space wrapped
                 * around, read from fifo_in offset 0 to offset cur_in.
                 */

                /* Reset fifo head to reduce unnecessary wrap arounds */
                cur_in = if used_in == 0 { 0 } else { cur_in };

                /* Free space total is reduced by a gap */
                free_space = NX_MAX(0, fifo_free_bytes(used_in, fifo_in_len) - line_sz);

                /* Free space may wrap around as first and last */
                first_free = fifo_free_first_bytes(cur_in, used_in, fifo_in_len);
                last_free = fifo_free_last_bytes(cur_in, used_in, fifo_in_len);

                /* Start offsets of the free memory */
                first_offset = fifo_free_first_offset(cur_in, used_in);
                last_offset = fifo_free_last_offset(cur_in, used_in, fifo_in_len);

                /* Reduce read_sz because of the line_sz gap */
                read_sz = NX_MIN(free_space, first_free);
                n = 0;
                if read_sz > 0 {
                    /* Read in to offset cur_in + used_in */
                    n = fread(fifo_in.add(first_offset as usize) as *mut c_void, 1, read_sz as size_t, inpf) as c_int;
                    used_in += n;
                    free_space -= n;
                    assert!(n <= read_sz);
                    if n != read_sz {
                        /* Either EOF or error; exit the read loop */
                        is_eof = 1;
                        state = State::Write;
                        continue;
                    }
                }

                /* If free space wrapped around */
                if last_free > 0 {
                    /* Reduce read_sz because of the line_sz gap */
                    read_sz = NX_MIN(free_space, last_free);
                    n = 0;
                    if read_sz > 0 {
                        n = fread(fifo_in.add(last_offset as usize) as *mut c_void, 1, read_sz as size_t, inpf) as c_int;
                        used_in += n;       /* Increase used space */
                        free_space -= n; /* Decrease free space */
                        assert!(n <= read_sz);
                        if n != read_sz {
                            /* Either EOF or error; exit the read loop */
                            is_eof = 1;
                            state = State::Write;
                            continue;
                        }
                    }
                }

                /* At this point we have used_in bytes in fifo_in with the
                 * data head starting at cur_in and possibly wrapping around.
                 */
                state = State::Write;
            },
            State::Write => unsafe {
                /* Write decompressed data to output file */
                if used_out == 0 {
                    state = State::Decomp;
                    continue;
                }

                /* If fifo_out has data waiting, write it out to the file to
                 * make free target space for the accelerator used bytes in
                 * the first and last parts of fifo_out.
                 */

                first_used = fifo_used_first_bytes(cur_out, used_out, fifo_out_len);
                last_used = fifo_used_last_bytes(cur_out, used_out, fifo_out_len);

                write_sz = first_used;

                n = 0;
                if write_sz > 0 {
                    n = fwrite(fifo_out.add(cur_out as usize) as *const c_void, 1, write_sz as size_t, outf) as c_int;
                    used_out -= n;
                    /* Move head of the fifo */
                    cur_out = (cur_out + n) % fifo_out_len;
                    assert!(n <= write_sz);
                    if n != write_sz {
                        fprintf(stderr, c"error: write\n".as_ptr());
                        rc = -1;
                        state = State::Err5;
                        continue;
                    }
                }

                if last_used > 0 {
                    /* If more data available in the last part */
                    write_sz = last_used; /* Keep it here for later */
                    n = 0;
                    if write_sz > 0 {
                        n = fwrite(fifo_out as *const c_void, 1, write_sz as size_t, outf) as c_int;
                        used_out -= n;
                        cur_out = (cur_out + n) % fifo_out_len;
                        assert!(n <= write_sz);
                        if n != write_sz {
                            fprintf(stderr, c"error: write\n".as_ptr());
                            rc = -1;
                            state = State::Err5;
                            continue;
                        }
                    }
                }
                state = State::Decomp;
            },
            State::Decomp => unsafe {
                /* NX decompresses input data */
                if is_final != 0 {
                    state = State::Finish;
                    continue;
                }

                /* Address/len lists */
                clearp_dde(ddl_in);
                clearp_dde(ddl_out);

                /* FC, CRC, HistLen, Table 6-6 */
                if resuming != 0 {
                    /* Resuming a partially decompressed input.
                     * The key to resume is supplying the 32KB
                     * dictionary (history) to NX, which is basically
                     * the last 32KB of output produced.
                     */
                    fc = GZIP_FC_DECOMPRESS_RESUME;

                    (*cmdp).cpb.in_crc = (*cmdp).cpb.out_crc;
                    (*cmdp).cpb.in_adler = (*cmdp).cpb.out_adler;

                    /* Round up the history size to quadword.  Section 2.10 */
                    history_len = (history_len + 15) / 16;
                    putnn((*cmdp).cpb, in_histlen, history_len);
                    history_len *= 16; /* bytes */

                    if history_len > 0 {
                        /* Chain in the history buffer to the DDE list */
                        if cur_out >= history_len {
                            nx_append_dde(
                                ddl_in,
                                fifo_out.add((cur_out - history_len) as usize) as *mut c_void,
                                history_len as uint32_t,
                            );
                        } else {
                            nx_append_dde(
                                ddl_in,
                                fifo_out.add(((fifo_out_len + cur_out) - history_len) as usize) as *mut c_void,
                                (history_len - cur_out) as uint32_t,
                            );
                            /* Up to 32KB history wraps around fifo_out */
                            nx_append_dde(ddl_in, fifo_out as *mut c_void, cur_out as uint32_t);
                        }
                    }
                } else {
                    /* First decompress job */
                    fc = GZIP_FC_DECOMPRESS;

                    history_len = 0;
                    /* Writing 0 clears out subc as well */
                    (*cmdp).cpb.in_histlen = 0;
                    total_out = 0;

                    put32((*cmdp).cpb, in_crc, INIT_CRC);
                    put32((*cmdp).cpb, in_adler, INIT_ADLER);
                    put32((*cmdp).cpb, out_crc, INIT_CRC);
                    put32((*cmdp).cpb, out_adler, INIT_ADLER);

                    /* Assuming 10% compression ratio initially; use the
                     * most recently measured compression ratio as a
                     * heuristic to estimate the input and output
                     * sizes.  If we give too much input, the target buffer
                     * overflows and NX cycles are wasted, and then we
                     * must retry with smaller input size.  1000 is 100%.
                     */
                    last_comp_ratio = 100;
                }
                (*cmdp).crb.gzip_fc = 0;
                putnn((*cmdp).crb, gzip_fc, fc);

                /*
                 * NX source buffers
                 */
                first_used = fifo_used_first_bytes(cur_in, used_in, fifo_in_len);
                last_used = fifo_used_last_bytes(cur_in, used_in, fifo_in_len);

                if first_used > 0 {
                    nx_append_dde(ddl_in, fifo_in.add(cur_in as usize) as *mut c_void, first_used as uint32_t);
                }

                if last_used > 0 {
                    nx_append_dde(ddl_in, fifo_in as *mut c_void, last_used as uint32_t);
                }

                /*
                 * NX target buffers
                 */
                first_free = fifo_free_first_bytes(cur_out, used_out, fifo_out_len);
                last_free = fifo_free_last_bytes(cur_out, used_out, fifo_out_len);

                /* Reduce output free space amount not to overwrite the history */
                let target_max: c_int = NX_MAX(0, fifo_free_bytes(used_out, fifo_out_len) - (1 << 16));

                first_free = NX_MIN(target_max, first_free);
                if first_free > 0 {
                    first_offset = fifo_free_first_offset(cur_out, used_out);
                    nx_append_dde(ddl_out, fifo_out.add(first_offset as usize) as *mut c_void, first_free as uint32_t);
                }

                if last_free > 0 {
                    last_free = NX_MIN(target_max - first_free, last_free);
                    if last_free > 0 {
                        last_offset = fifo_free_last_offset(cur_out, used_out, fifo_out_len);
                        nx_append_dde(ddl_out, fifo_out.add(last_offset as usize) as *mut c_void, last_free as uint32_t);
                    }
                }

                /* Target buffer size is used to limit the source data size
                 * based on previous measurements of compression ratio.
                 */

                /* source_sz includes history */
                source_sz = getp32(ddl_in, ddebc) as c_int;
                assert!(source_sz > history_len);
                source_sz -= history_len;

                /* Estimating how much source is needed to 3/4 fill a
                 * target_max size target buffer.  If we overshoot, then NX
                 * must repeat the job with smaller input and we waste
                 * bandwidth.  If we undershoot then we use more NX calls than
                 * necessary.
                 */

                source_sz_estimate = (((target_max as uint64_t) * last_comp_ratio * 3) / 4000) as c_int;

                if source_sz_estimate < source_sz {
                    /* Target might be small, therefore limiting the
                     * source data.
                     */
                    source_sz = source_sz_estimate;
                    target_sz_estimate = target_max;
                } else {
                    /* Source file might be small, therefore limiting target
                     * touch pages to a smaller value to save processor cycles.
                     */
                    target_sz_estimate = (((source_sz as uint64_t) * 1000) / (last_comp_ratio + 1)) as c_int;
                    target_sz_estimate = NX_MIN(2 * target_sz_estimate, target_max);
                }

                source_sz += history_len;

                /* Some NX condition codes require submitting the NX job again.
                 * Kernel doesn't handle NX page faults. Expects user code to
                 * touch pages.
                 */
                pgfault_retries = NX_MAX_FAULTS;
                state = State::RestartNx;
            },
            State::RestartNx => unsafe {
                putp32(ddl_in, ddebc, source_sz as uint32_t);

                /* Fault in pages */
                nxu_touch_pages(cmdp as *mut c_void, size_of::<nx_gzip_crb_cpb_t>() as c_long, page_sz as c_long, 1);
                nx_touch_pages_dde(ddl_in, 0, page_sz as c_long, 0);
                nx_touch_pages_dde(ddl_out, target_sz_estimate as c_long, page_sz as c_long, 1);

                /* Send job to NX */
                cc = nx_submit_job(ddl_in, ddl_out, cmdp, devhandle);

                match cc {
                    ERR_NX_AT_FAULT => {
                        /* We touched the pages ahead of time.  In the most common case
                         * we shouldn't be here.  But may be some pages were paged out.
                         * Kernel should have placed the faulting address to fsaddr.
                         */

                        if pgfault_retries == NX_MAX_FAULTS {
                            /* Try once with exact number of pages */
                            pgfault_retries -= 1;
                            state = State::RestartNx;
                        } else if pgfault_retries > 0 {
                            /* If still faulting try fewer input pages
                             * assuming memory outage
                             */
                            if source_sz > page_sz {
                                source_sz = NX_MAX(source_sz / 2, page_sz);
                            }
                            pgfault_retries -= 1;
                            state = State::RestartNx;
                        } else {
                            fprintf(stderr, c"cannot make progress; too many ".as_ptr());
                            fprintf(stderr, c"page fault retries cc= %d\n".as_ptr(), cc);
                            rc = -1;
                            state = State::Err5;
                        }
                    }
                    ERR_NX_DATA_LENGTH => {
                        /* Not an error in the most common case; it just says
                         * there is trailing data that we must examine.
                         *
                         * CC=3 CE(1)=0 CE(0)=1 indicates partial completion
                         * Fig.6-7 and Table 6-8.
                         */
                        nx_ce = get_csb_ce_ms3b((*cmdp).crb.csb);

                        if csb_ce_termination(nx_ce) == 0 && csb_ce_partial_completion(nx_ce) != 0 {
                            /* Check CPB for more information
                             * spbc and tpbc are valid
                             */
                            sfbt = getnn((*cmdp).cpb, out_sfbt); /* Table 6-4 */
                            subc = getnn((*cmdp).cpb, out_subc); /* Table 6-4 */
                            spbc = get32_placeholder_cpb(&(*cmdp).cpb, out_spbc_decomp);
                            tpbc_ = get32_placeholder_csb(&(*cmdp).crb.csb, tpbc) as c_int;
                            assert!(target_sz_estimate >= tpbc_);

                            state = State::OkCc3; /* not an error */
                        } else {
                            /* History length error when CE(1)=1 CE(0)=0. */
                            rc = -1;
                            fprintf(stderr, c"history length error cc= %d\n".as_ptr(), cc);
                            state = State::Err5;
                        }
                    }
                    ERR_NX_TARGET_SPACE => {
                        /* Target buffer not large enough; retry smaller input
                         * data; give at least 1 byte.  SPBC/TPBC are not valid.
                         */
                        assert!(source_sz > history_len);
                        source_sz = ((source_sz - history_len + 2) / 2) + history_len;
                        state = State::RestartNx;
                    }
                    ERR_NX_OK => {
                        /* This should not happen for gzip formatted data;
                         * we need trailing crc and isize
                         */
                        fprintf(stderr, c"ERR_NX_OK\n".as_ptr());
                        spbc = get32_placeholder_cpb(&(*cmdp).cpb, out_spbc_decomp);
                        tpbc_ = get32_placeholder_csb(&(*cmdp).crb.csb, tpbc) as c_int;
                        assert!(target_sz_estimate >= tpbc_);
                        assert!(spbc >= history_len);
                        source_sz = spbc - history_len;
                        state = State::Offsets;
                    }
                    _ => {
                        fprintf(stderr, c"error: cc= %d\n".as_ptr(), cc);
                        rc = -1;
                        state = State::Err5;
                    }
                }
            },
            State::OkCc3 => unsafe {
                assert!(spbc > history_len);
                source_sz = spbc - history_len;

                /* Table 6-4: Source Final Block Type (SFBT) describes the
                 * last processed deflate block and clues the software how to
                 * resume the next job.  SUBC indicates how many input bits NX
                 * consumed but did not process.  SPBC indicates how many
                 * bytes of source were given to the accelerator including
                 * history bytes.
                 */

                match sfbt {
                    0x0 => {
                        /* Deflate final EOB received */

                        /* Calculating the checksum start position. */

                        source_sz -= subc / 8;
                        is_final = 1;
                    }
                    0x8 | 0x9 => {
                        /* Within a literal block; use rembytecount */
                        /* Within a literal block; use rembytecount; bfinal=1 */

                        /* Supply the partially processed source byte again */
                        source_sz -= (subc + 7) / 8;

                        /* SUBC LS 3bits: number of bits in the first source byte need
                         * to be processed.
                         * 000 means all 8 bits;  Table 6-3
                         * Clear subc, histlen, sfbt, rembytecnt, dhtlen
                         */
                        (*cmdp).cpb.in_subc = 0;
                        (*cmdp).cpb.in_sfbt = 0;
                        putnn((*cmdp).cpb, in_subc, subc % 8);
                        putnn((*cmdp).cpb, in_sfbt, sfbt);
                        let rembytecnt = getnn((*cmdp).cpb, out_rembytecnt);
                        putnn((*cmdp).cpb, in_rembytecnt, rembytecnt);
                    }
                    0xA | 0xB => {
                        /* Within a FH block; */
                        /* Within a FH block; bfinal=1 */

                        source_sz -= (subc + 7) / 8;

                        /* Clear subc, histlen, sfbt, rembytecnt, dhtlen */
                        (*cmdp).cpb.in_subc = 0;
                        (*cmdp).cpb.in_sfbt = 0;
                        putnn((*cmdp).cpb, in_subc, subc % 8);
                        putnn((*cmdp).cpb, in_sfbt, sfbt);
                    }
                    0xC | 0xD => {
                        /* Within a DH block; */
                        /* Within a DH block; bfinal=1 */

                        source_sz -= (subc + 7) / 8;

                        /* Clear subc, histlen, sfbt, rembytecnt, dhtlen */
                        (*cmdp).cpb.in_subc = 0;
                        (*cmdp).cpb.in_sfbt = 0;
                        putnn((*cmdp).cpb, in_subc, subc % 8);
                        putnn((*cmdp).cpb, in_sfbt, sfbt);

                        let mut dhtlen = getnn((*cmdp).cpb, out_dhtlen);
                        putnn((*cmdp).cpb, in_dhtlen, dhtlen);
                        assert!(dhtlen >= 42);

                        /* Round up to a qword */
                        dhtlen = (dhtlen + 127) / 128;

                        while dhtlen > 0 {
                            /* Copy dht from cpb.out to cpb.in */
                            dhtlen -= 1;
                            (*cmdp).cpb.in_dht[dhtlen as usize] = (*cmdp).cpb.out_dht[dhtlen as usize];
                        }
                    }
                    0xE | 0xF => {
                        /* Within a block header; bfinal=0; */
                        /* Also given if source data exactly ends (SUBC=0) with
                         * EOB code with BFINAL=0.  Means the next byte will
                         * contain a block header.
                         */
                        /* within a block header with BFINAL=1. */

                        source_sz -= (subc + 7) / 8;

                        /* Clear subc, histlen, sfbt, rembytecnt, dhtlen */
                        (*cmdp).cpb.in_subc = 0;
                        (*cmdp).cpb.in_sfbt = 0;
                        putnn((*cmdp).cpb, in_subc, subc % 8);
                        putnn((*cmdp).cpb, in_sfbt, sfbt);

                        /* Engine did not process any data */
                        if is_eof != 0 && source_sz == 0 {
                            is_final = 1;
                        }
                    }
                    _ => {}
                }
                state = State::Offsets;
            },
            State::Offsets => unsafe {
                /* Adjust the source and target buffer offsets and lengths  */

                /* Delete input data from fifo_in */
                used_in -= source_sz;
                cur_in = (cur_in + source_sz) % fifo_in_len;
                input_file_offset += source_sz as off_t;

                /* Add output data to fifo_out */
                used_out += tpbc_;

                assert!(used_out <= fifo_out_len);

                total_out += tpbc_ as uint64_t;

                /* Deflate history is 32KB max.  No need to supply more
                 * than 32KB on a resume.
                 */
                history_len = if total_out > window_max as uint64_t {
                    window_max
                } else {
                    total_out as c_int
                };

                /* To estimate expected expansion in the next NX job; 500 means 50%.
                 * Deflate best case is around 1 to 1000.
                 */
                last_comp_ratio = (1000 * ((source_sz as uint64_t) + 1)) / ((tpbc_ as uint64_t) + 1);
                last_comp_ratio = NX_MAX(NX_MIN(1000, last_comp_ratio), 1);

                resuming = 1;
                state = State::Finish;
            },
            State::Finish => unsafe {
                if is_final != 0 {
                    if used_out != 0 {
                        state = State::Write; /* More data to write out */
                    } else if used_in < 8 {
                        /* Need at least 8 more bytes containing gzip crc
                         * and isize.
                         */
                        rc = -1;
                        state = State::Err4;
                    } else {
                        /* Compare checksums and exit */
                        let mut i: c_int;
                        let mut tail: [u8; 8] = [0; 8];
                        let cksum: uint32_t;
                        let isize: uint32_t;

                        i = 0;
                        while i < 8 {
                            tail[i as usize] = *fifo_in.add(((cur_in + i) % fifo_in_len) as usize) as u8;
                            i += 1;
                        }
                        fprintf(
                            stderr,
                            c"computed checksum %08x isize %08x\n".as_ptr(),
                            (*cmdp).cpb.out_crc,
                            (total_out % (1u64 << 32)) as uint32_t,
                        );
                        cksum = (tail[0] as uint32_t)
                            | ((tail[1] as uint32_t) << 8)
                            | ((tail[2] as uint32_t) << 16)
                            | ((tail[3] as uint32_t) << 24);
                        isize = (tail[4] as uint32_t)
                            | ((tail[5] as uint32_t) << 8)
                            | ((tail[6] as uint32_t) << 16)
                            | ((tail[7] as uint32_t) << 24);
                        fprintf(stderr, c"stored   checksum %08x isize %08x\n".as_ptr(), cksum, isize);

                        if cksum == (*cmdp).cpb.out_crc && isize == (total_out % (1u64 << 32)) as uint32_t {
                            rc = 0;
                            state = State::Ok1;
                        } else {
                            rc = -1;
                            state = State::Err4;
                        }
                    }
                } else {
                    state = State::Read;
                }
            },
            State::Err1 => unsafe {
                return err1(expect, c);
            },
            State::Err2 => unsafe {
                return err2();
            },
            State::Err3 => unsafe {
                return err3();
            },
            State::Err4 => unsafe {
                fprintf(stderr, c"error: checksum missing or mismatch\n".as_ptr());
                state = State::Err5;
            },
            State::Err5 | State::Ok1 => unsafe {
                fprintf(stderr, c"decomp is complete: fclose\n".as_ptr());
                fclose(outf);

                return rc;
            },
        }
    }
}

unsafe fn err1(expect: c_int, c: c_int) -> c_int {
    unsafe {
        fprintf(
            stderr,
            c"error: not a gzip file, expect %x, read %x\n".as_ptr(),
            expect,
            c,
        );
    }
    -1
}

unsafe fn err2() -> c_int {
    unsafe {
        fprintf(stderr, c"error: the FLG byte is wrong or not being handled\n".as_ptr());
    }
    -1
}

unsafe fn err3() -> c_int {
    unsafe {
        fprintf(stderr, c"error: gzip header\n".as_ptr());
    }
    -1
}

unsafe fn get32_placeholder_cpb(cpb: *const nx_gzip_cpb_t, field: c_int) -> c_int {
    unsafe {
        match field {
            out_spbc_decomp => (*cpb).out_spbc_decomp as c_int,
            _ => 0,
        }
    }
}

unsafe fn get32_placeholder_csb(_csb: *const nx_csb_t, _field: c_int) -> uint32_t {
    0
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let rc: c_int;
    let mut act: sigaction = unsafe { core::mem::zeroed() };
    let handle: *mut c_void;

    unsafe {
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
            fprintf(stderr, c"Unable to init NX, errno %d\n".as_ptr(), errno);
            exit(-1);
        }

        rc = decompress_file(argc, argv, handle);

        nx_function_end(handle);
    }

    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
