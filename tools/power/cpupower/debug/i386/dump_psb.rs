// SPDX-License-Identifier: GPL-2.0-only
// dump_psb. (c) 2004, Dave Jones, Red Hat Inc.

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

const LEN: usize = 0x100000 - 0xc0000;
const OFFSET: i64 = 0xc0000;

static mut RELEVANT: c_long = 0;

static FID_TO_MULT: [c_int; 32] = [
    110, 115, 120, 125, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 30, 190, 40, 200,
    130, 135, 140, 210, 150, 225, 160, 165, 170, 180, -1, -1,
];

static VID_TO_VOLTAGE: [c_int; 32] = [
    2000, 1950, 1900, 1850, 1800, 1750, 1700, 1650, 1600, 1550, 1500, 1450, 1400, 1350, 1300,
    0, 1275, 1250, 1225, 1200, 1175, 1150, 1125, 1100, 1075, 1050, 1024, 1000, 975, 950,
    925, 0,
];

#[repr(C, packed)]
struct psb_header {
    signature: [c_char; 10],
    version: u8,
    flags: u8,
    settlingtime: u16,
    res1: u8,
    numpst: u8,
}

#[repr(C, packed)]
struct pst_header {
    cpuid: u32,
    fsb: u8,
    maxfid: u8,
    startvid: u8,
    numpstates: u8,
}

static mut FSB: u32 = 0;
static mut SGTC: u32 = 0;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

const NO_ARGUMENT: c_int = 0;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn exit(status: c_int) -> !;
}

static INFO_OPTS: [option; 1] = [option {
    name: b"numpst\0".as_ptr() as *const c_char,
    has_arg: NO_ARGUMENT,
    flag: ptr::null_mut(),
    val: b'n' as c_int,
}];

unsafe fn decode_pst(mut p: *mut c_char, npstates: c_int) -> c_int {
    let mut i: c_int;
    let mut freq: c_int;
    let mut fid: c_int;
    let mut vid: c_int;

    i = 0;
    while i < npstates {
        fid = *p as c_int;
        p = p.add(1);
        vid = *p as c_int;
        p = p.add(1);
        freq = 100 * FID_TO_MULT[fid as usize] * FSB as c_int;

        printf(
            b"   %2d %8dkHz  FID %02x (%2d.%01d)  VID %02x (%4dmV)\n\0".as_ptr()
                as *const c_char,
            i,
            freq,
            fid,
            FID_TO_MULT[fid as usize] / 10,
            FID_TO_MULT[fid as usize] % 10,
            vid,
            VID_TO_VOLTAGE[vid as usize],
        );
        i += 1;
    }

    0
}

unsafe fn decode_psb(mut p: *mut c_char, mut numpst: c_int) {
    let mut i: c_int;
    let psb: *mut psb_header;
    let mut pst: *mut pst_header;

    psb = p as *mut psb_header;

    let version = ptr::addr_of!((*psb).version).read_unaligned();
    if version != 0x12 {
        return;
    }

    let flags = ptr::addr_of!((*psb).flags).read_unaligned();
    let settlingtime = ptr::addr_of!((*psb).settlingtime).read_unaligned();
    let res1 = ptr::addr_of!((*psb).res1).read_unaligned();
    let psb_numpst = ptr::addr_of!((*psb).numpst).read_unaligned();

    printf(
        b"PSB version: %hhx flags: %hhx settling time %hhuus res1 %hhx num pst %hhu\n\0"
            .as_ptr() as *const c_char,
        version as c_int,
        flags as c_int,
        settlingtime as c_int,
        res1 as c_int,
        psb_numpst as c_int,
    );
    SGTC = settlingtime as u32 * 100;

    if SGTC < 10000 {
        SGTC = 10000;
    }

    p = (psb as *mut c_char).add(std::mem::size_of::<psb_header>());

    if numpst < 0 {
        numpst = psb_numpst as c_int;
    } else {
        printf(
            b"Overriding number of pst :%d\n\0".as_ptr() as *const c_char,
            numpst,
        );
    }

    i = 0;
    while i < numpst {
        pst = p as *mut pst_header;

        let cpuid = ptr::addr_of!((*pst).cpuid).read_unaligned();
        let fsb = ptr::addr_of!((*pst).fsb).read_unaligned();
        let maxfid = ptr::addr_of!((*pst).maxfid).read_unaligned();
        let startvid = ptr::addr_of!((*pst).startvid).read_unaligned();
        let pst_numpstates = ptr::addr_of!((*pst).numpstates).read_unaligned();

        if RELEVANT != 0 {
            if RELEVANT != cpuid as c_long {
                p = p.add(std::mem::size_of::<pst_header>() + 2 * pst_numpstates as usize);
                i += 1;
                continue;
            }
        }

        printf(
            b"  PST %d  cpuid %.3x fsb %hhu mfid %hhx svid %hhx numberstates %hhu\n\0"
                .as_ptr() as *const c_char,
            i + 1,
            cpuid,
            fsb as c_int,
            maxfid as c_int,
            startvid as c_int,
            pst_numpstates as c_int,
        );

        FSB = fsb as u32;
        decode_pst(p.add(std::mem::size_of::<pst_header>()), pst_numpstates as c_int);

        p = p.add(std::mem::size_of::<pst_header>() + 2 * pst_numpstates as usize);
        i += 1;
    }
}

unsafe fn print_help() {
    printf(b"Usage: dump_psb [options]\n\0".as_ptr() as *const c_char);
    printf(b"Options:\n\0".as_ptr() as *const c_char);
    printf(b"  -n, --numpst     Set number of PST tables to scan\n\0".as_ptr() as *const c_char);
    printf(
        b"  -r, --relevant   Only display PSTs relevant to cpuid N\n\0".as_ptr() as *const c_char,
    );
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd: c_int;
    let mut numpst: c_int = -1;
    let mut ret: c_int = 0;
    let mut cont: c_int = 1;
    let mut mem: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char;

    while {
        ret = getopt_long(
            argc,
            argv,
            b"hr:n:\0".as_ptr() as *const c_char,
            INFO_OPTS.as_ptr(),
            ptr::null_mut(),
        );
        match ret {
            x if x == b'?' as c_int || x == b'h' as c_int => {
                print_help();
                cont = 0;
            }
            x if x == b'r' as c_int => {
                RELEVANT = strtol(optarg, ptr::null_mut(), 16);
            }
            x if x == b'n' as c_int => {
                numpst = strtol(optarg, ptr::null_mut(), 10) as c_int;
            }
            -1 => {
                cont = 0;
            }
            _ => {}
        }

        cont != 0
    } {}

    fd = open(b"/dev/mem\0".as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        printf(b"Couldn't open /dev/mem. Are you root?\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    mem = mmap(
        mem as *mut c_void,
        0x100000 - 0xc0000,
        PROT_READ,
        MAP_SHARED,
        fd,
        0xc0000,
    ) as *mut c_char;
    close(fd);

    p = mem;
    while p.offset_from(mem) < LEN as isize {
        if memcmp(
            p as *const c_void,
            b"AMDK7PNOW!\0".as_ptr() as *const c_void,
            10,
        ) == 0
        {
            decode_psb(p, numpst);
            break;
        }
        p = p.add(16);
    }

    munmap(mem as *mut c_void, LEN);
    0
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(ptr::null_mut());

    unsafe {
        std::process::exit(c_main((args.len() - 1) as c_int, args.as_mut_ptr()));
    }
}
