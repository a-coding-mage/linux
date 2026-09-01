// SPDX-License-Identifier: GPL-2.0-only
/*
 * Minimal BPF JIT image disassembler
 *
 * Disassembles BPF JIT compiler emitted opcodes back to asm insn's for
 * debugging or verification purposes.
 *
 * To get the disassembly of the JIT code, do the following:
 *
 *  1) `echo 2 > /proc/sys/net/core/bpf_jit_enable`
 *  2) Load a BPF filter (e.g. `tcpdump -p -n -s 0 -i eth1 host 192.168.20.0/24`)
 *  3) Run e.g. `bpf_jit_disasm -o` to read out the last JIT code
 *
 * Copyright 2013 Daniel Borkmann <borkmann@redhat.com>
 */

use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

const CMD_ACTION_SIZE_BUFFER: c_int = 10;
const CMD_ACTION_READ_ALL: c_int = 3;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const DEFFILEMODE: c_uint = 0o666;
const REG_EXTENDED: c_int = 1;
const BFD_OBJECT: c_int = 1;

type SizeT = usize;
type SSizeT = isize;
type OffT = isize;
type ModeT = c_uint;
type BfdBoolean = c_int;
type DisassemblerFtype = Option<unsafe extern "C" fn(c_ulong, *mut DisassembleInfo) -> c_int>;
type FprintfFtype = unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int;

#[repr(C)]
struct Bfd {
    _private: [u8; 0],
}

#[repr(C)]
struct RegexT {
    _opaque: [usize; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RegmatchT {
    rm_so: isize,
    rm_eo: isize,
}

#[repr(C)]
struct Stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: ModeT,
    _pad0: c_int,
    st_uid: c_uint,
    st_gid: c_uint,
    _pad1: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    _rest: [u8; 80],
}

#[repr(C)]
struct DisassembleInfo {
    arch: c_int,
    mach: c_ulong,
    buffer: *mut u8,
    buffer_length: SizeT,
    _opaque: [usize; 128],
}

extern "C" {
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;

    fn snprintf(s: *mut c_char, maxlen: SizeT, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn malloc(size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: SizeT) -> SSizeT;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SSizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SSizeT;
    fn fstat(fd: c_int, statbuf: *mut Stat) -> c_int;
    fn klogctl(type_: c_int, bufp: *mut c_char, len: c_int) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn regcomp(preg: *mut RegexT, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const RegexT,
        string: *const c_char,
        nmatch: SizeT,
        pmatch: *mut RegmatchT,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut RegexT);

    fn bfd_init();
    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *mut Bfd;
    fn bfd_check_format(abfd: *mut Bfd, format: c_int) -> BfdBoolean;
    fn bfd_get_arch(abfd: *mut Bfd) -> c_int;
    fn bfd_get_mach(abfd: *mut Bfd) -> c_ulong;
    fn bfd_big_endian(abfd: *mut Bfd) -> BfdBoolean;
    fn bfd_close(abfd: *mut Bfd) -> BfdBoolean;

    fn init_disassemble_info_compat(
        info: *mut DisassembleInfo,
        stream: *mut c_void,
        fprintf_func: FprintfFtype,
        fprintf_styled_func: *const c_void,
    );
    static fprintf_styled: *const c_void;
    fn disassemble_init_for_target(info: *mut DisassembleInfo);

    /*
     * The C source uses either disassembler(info.arch, bfd_big_endian(bfdf),
     * info.mach, bfdf) when DISASM_FOUR_ARGS_SIGNATURE is defined, or
     * disassembler(bfdf) otherwise. This translation keeps the currently
     * selected external form as a declaration; adjust this declaration to match
     * the external binutils headers used by the build.
     */
    fn disassembler(abfd: *mut Bfd) -> DisassemblerFtype;
}

fn s_isreg(mode: ModeT) -> bool {
    (mode & 0o170000) == 0o100000
}

unsafe fn c_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn get_exec_path(tpath: *mut c_char, size: SizeT) {
    let path: *mut c_char;
    let mut len: SSizeT;

    snprintf(
        tpath,
        size,
        c_lit(b"/proc/%d/exe\0"),
        getpid() as c_int,
    );
    *tpath.add(size - 1) = 0;

    path = strdup(tpath);
    assert!(!path.is_null());

    len = readlink(path, tpath, size);
    if len < 0 {
        len = 0;
    }
    *tpath.offset(len) = 0;

    free(path as *mut c_void);
}

unsafe fn get_asm_insns(image: *mut u8, len: SizeT, opcodes: c_int) {
    let mut count: c_int;
    let mut i: c_int;
    let mut pc: c_int = 0;
    let mut tpath: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut info: DisassembleInfo = mem::zeroed();
    let disassemble: DisassemblerFtype;
    let bfdf: *mut Bfd;

    memset(
        tpath.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&tpath),
    );
    get_exec_path(tpath.as_mut_ptr(), mem::size_of_val(&tpath));

    bfdf = bfd_openr(tpath.as_ptr(), ptr::null());
    assert!(!bfdf.is_null());
    assert!(bfd_check_format(bfdf, BFD_OBJECT) != 0);

    init_disassemble_info_compat(&mut info, stdout, fprintf, fprintf_styled);
    info.arch = bfd_get_arch(bfdf);
    info.mach = bfd_get_mach(bfdf);
    info.buffer = image;
    info.buffer_length = len;

    disassemble_init_for_target(&mut info);

    /*
     * If DISASM_FOUR_ARGS_SIGNATURE is selected by the external headers, the C
     * call is:
     * disassembler(info.arch, bfd_big_endian(bfdf), info.mach, bfdf)
     */
    disassemble = disassembler(bfdf);
    assert!(disassemble.is_some());

    loop {
        printf(c_lit(b"%4x:\t\0"), pc);

        count = disassemble.unwrap()(pc as c_ulong, &mut info);

        if opcodes != 0 {
            printf(c_lit(b"\n\t\0"));
            i = 0;
            while i < count {
                printf(
                    c_lit(b"%02x \0"),
                    *image.offset((pc + i) as isize) as u8 as c_int,
                );
                i += 1;
            }
        }
        printf(c_lit(b"\n\0"));

        pc += count;
        if !(count > 0 && (pc as SizeT) < len) {
            break;
        }
    }

    bfd_close(bfdf);
}

unsafe fn get_klog_buff(klen: *mut c_uint) -> *mut c_char {
    let ret: c_int;
    let len: c_int;
    let buff: *mut c_char;

    len = klogctl(CMD_ACTION_SIZE_BUFFER, ptr::null_mut(), 0);
    if len < 0 {
        return ptr::null_mut();
    }

    buff = malloc(len as SizeT) as *mut c_char;
    if buff.is_null() {
        return ptr::null_mut();
    }

    ret = klogctl(CMD_ACTION_READ_ALL, buff, len);
    if ret < 0 {
        free(buff as *mut c_void);
        return ptr::null_mut();
    }

    *klen = ret as c_uint;
    buff
}

unsafe fn get_flog_buff(file: *const c_char, klen: *mut c_uint) -> *mut c_char {
    let fd: c_int;
    let mut ret: c_int;
    let len: c_int;
    let mut fi: Stat = mem::zeroed();
    let buff: *mut c_char;

    fd = open(file, O_RDONLY);
    if fd < 0 {
        return ptr::null_mut();
    }

    ret = fstat(fd, &mut fi);
    if ret < 0 || !s_isreg(fi.st_mode) {
        close(fd);
        return ptr::null_mut();
    }

    len = (fi.st_size + 1) as c_int;
    buff = malloc(len as SizeT) as *mut c_char;
    if buff.is_null() {
        close(fd);
        return ptr::null_mut();
    }

    memset(buff as *mut c_void, 0, len as SizeT);
    ret = read(fd, buff as *mut c_void, (len - 1) as SizeT) as c_int;
    if ret <= 0 {
        free(buff as *mut c_void);
        close(fd);
        return ptr::null_mut();
    }

    close(fd);
    *klen = ret as c_uint;
    buff
}

unsafe fn get_log_buff(file: *const c_char, klen: *mut c_uint) -> *mut c_char {
    if !file.is_null() {
        get_flog_buff(file, klen)
    } else {
        get_klog_buff(klen)
    }
}

unsafe fn put_log_buff(buff: *mut c_char) {
    free(buff as *mut c_void);
}

unsafe fn get_last_jit_image(
    haystack: *mut c_char,
    hlen: SizeT,
    ilen: *mut c_uint,
) -> *mut u8 {
    let mut ptr_: *mut c_char;
    let mut pptr: *mut c_char;
    let mut tmp: *mut c_char;
    let mut off: OffT = 0;
    let mut proglen: c_uint = 0;
    let mut ret: c_int;
    let mut flen: c_int = 0;
    let mut pass: c_int = 0;
    let mut ulen: c_int = 0;
    let mut pmatch: [RegmatchT; 1] = [RegmatchT { rm_so: 0, rm_eo: 0 }; 1];
    let mut base: c_ulong = 0;
    let mut regex: RegexT = mem::zeroed();
    let image: *mut u8;

    if hlen == 0 {
        return ptr::null_mut();
    }

    ret = regcomp(
        &mut regex,
        c_lit(
            b"flen=[[:alnum:]]+ proglen=[[:digit:]]+ pass=[[:digit:]]+ image=[[:xdigit:]]+\0",
        ),
        REG_EXTENDED,
    );
    assert!(ret == 0);

    ptr_ = haystack;
    memset(
        pmatch.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&pmatch),
    );

    loop {
        ret = regexec(&regex, ptr_, 1, pmatch.as_mut_ptr(), 0);
        if ret == 0 {
            ptr_ = ptr_.offset(pmatch[0].rm_eo);
            off += pmatch[0].rm_eo;
            assert!((off as SizeT) < hlen);
        } else {
            break;
        }
    }

    ptr_ = haystack.offset(off - (pmatch[0].rm_eo - pmatch[0].rm_so));
    ret = sscanf(
        ptr_,
        c_lit(b"flen=%d proglen=%u pass=%d image=%lx\0"),
        &mut flen,
        &mut proglen,
        &mut pass,
        &mut base,
    );
    if ret != 4 {
        regfree(&mut regex);
        return ptr::null_mut();
    }
    if proglen > 1000000 {
        printf(
            c_lit(b"proglen of %u too big, stopping\n\0"),
            proglen,
        );
        return ptr::null_mut();
    }

    image = malloc(proglen as SizeT) as *mut u8;
    if image.is_null() {
        printf(c_lit(b"Out of memory\n\0"));
        return ptr::null_mut();
    }
    memset(image as *mut c_void, 0, proglen as SizeT);

    tmp = haystack.offset(off);
    loop {
        ptr_ = strtok(tmp, c_lit(b"\n\0"));
        if ptr_.is_null() || ulen >= proglen as c_int {
            break;
        }
        tmp = ptr::null_mut();
        if strstr(ptr_, c_lit(b"JIT code\0")).is_null() {
            continue;
        }
        pptr = ptr_;
        loop {
            ptr_ = strstr(pptr, c_lit(b":\0"));
            if ptr_.is_null() {
                break;
            }
            pptr = ptr_.offset(1);
        }
        ptr_ = pptr;
        loop {
            *image.offset(ulen as isize) = strtoul(pptr, &mut pptr, 16) as u8;
            ulen += 1;
            if ptr_ == pptr {
                ulen -= 1;
                break;
            }
            if ulen >= proglen as c_int {
                break;
            }
            ptr_ = pptr;
        }
    }

    assert!(ulen == proglen as c_int);
    printf(
        c_lit(b"%u bytes emitted from JIT compiler (pass:%d, flen:%d)\n\0"),
        proglen,
        pass,
        flen,
    );
    printf(c_lit(b"%lx + <x>:\n\0"), base);

    regfree(&mut regex);
    *ilen = ulen as c_uint;
    image
}

unsafe fn usage() {
    printf(c_lit(b"Usage: bpf_jit_disasm [...]\n\0"));
    printf(c_lit(b"       -o          Also display related opcodes (default: off).\n\0"));
    printf(c_lit(
        b"       -O <file>   Write binary image of code to file, don't disassemble to stdout.\n\0",
    ));
    printf(c_lit(
        b"       -f <file>   Read last image dump from file or stdin (default: klog).\n\0",
    ));
    printf(c_lit(b"       -h          Display this help.\n\0"));
}

pub unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut len: c_uint = 0;
    let mut klen: c_uint = 0;
    let mut opt: c_uint;
    let mut opcodes: c_uint = 0;
    let mut kbuff: *mut c_char;
    let mut file: *mut c_char = ptr::null_mut();
    let mut ofile: *mut c_char = ptr::null_mut();
    let ofd: c_int;
    let mut nr: SSizeT;
    let mut pos: *mut u8;
    let mut image: *mut u8 = ptr::null_mut();

    loop {
        let getopt_ret = getopt(argc, argv, c_lit(b"of:O:\0"));
        opt = getopt_ret as c_uint;
        if getopt_ret == -1 {
            break;
        }
        match opt as c_int {
            111 => {
                opcodes = 1;
            }
            79 => {
                ofile = optarg;
            }
            102 => {
                file = optarg;
            }
            _ => {
                usage();
                return -1;
            }
        }
    }

    bfd_init();

    kbuff = get_log_buff(file, &mut klen);
    if kbuff.is_null() {
        fprintf(stderr, c_lit(b"Could not retrieve log buffer!\n\0"));
        return -1;
    }

    image = get_last_jit_image(kbuff, klen as SizeT, &mut len);
    if image.is_null() {
        fprintf(stderr, c_lit(b"No JIT image found!\n\0"));
        put_log_buff(kbuff);
        free(image as *mut c_void);
        return 0;
    }
    if ofile.is_null() {
        get_asm_insns(image, len as SizeT, opcodes as c_int);
        put_log_buff(kbuff);
        free(image as *mut c_void);
        return 0;
    }

    ofd = open(ofile, O_WRONLY | O_CREAT | O_TRUNC, DEFFILEMODE);
    if ofd < 0 {
        fprintf(
            stderr,
            c_lit(b"Could not open file %s for writing: \0"),
            ofile,
        );
        perror(ptr::null());
        put_log_buff(kbuff);
        free(image as *mut c_void);
        return 0;
    }
    pos = image;
    loop {
        nr = write(ofd, pos as *const c_void, len as SizeT);
        if nr < 0 {
            fprintf(
                stderr,
                c_lit(b"Could not write data to %s: \0"),
                ofile,
            );
            perror(ptr::null());
            put_log_buff(kbuff);
            free(image as *mut c_void);
            return 0;
        }
        len = len.wrapping_sub(nr as c_uint);
        pos = pos.offset(nr);
        if len == 0 {
            break;
        }
    }
    close(ofd);

    put_log_buff(kbuff);
    free(image as *mut c_void);
    0
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .chain(std::iter::once(ptr::null_mut()))
        .collect();

    unsafe {
        std::process::exit(main_0(args.len() as c_int, argv.as_mut_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
