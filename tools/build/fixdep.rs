// SPDX-License-Identifier: GPL-2.0
/*
 * "Optimize" a list of dependencies as spit out by gcc -MD
 * for the build framework.
 *
 * Original author:
 *   Copyright    2002 by Kai Germaschewski  <kai.germaschewski@gmx.de>
 *
 * This code has been borrowed from kbuild's fixdep (scripts/basic/fixdep.c),
 * Please check it for detailed explanation. This fixdep borow only the
 * base transformation of dependecies without the CONFIG mangle.
 */

use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::os::unix::ffi::OsStrExt;
use std::process;

const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const PATH_MAX: usize = 4096;

type SizeT = usize;
type OffT = i64;
type ModeT = u32;
type NlinkT = u64;
type UidT = u32;
type GidT = u32;
type DevT = u64;
type InoT = u64;
type BlkcntT = i64;
type BlksizeT = i64;
type TimeT = i64;

#[repr(C)]
struct timespec {
    tv_sec: TimeT,
    tv_nsec: c_long,
}

#[repr(C)]
struct stat {
    st_dev: DevT,
    st_ino: InoT,
    st_nlink: NlinkT,
    st_mode: ModeT,
    st_uid: UidT,
    st_gid: GidT,
    __pad0: c_int,
    st_rdev: DevT,
    st_size: OffT,
    st_blksize: BlksizeT,
    st_blocks: BlkcntT,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [c_long; 3],
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: SizeT,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: OffT,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: SizeT) -> c_int;
    fn perror(s: *const c_char);
}

static mut TARGET: *mut c_char = std::ptr::null_mut();
static mut DEPFILE: *mut c_char = std::ptr::null_mut();
static mut CMDLINE: *mut c_char = std::ptr::null_mut();

unsafe fn cstr_bytes(ptr: *const c_char) -> &'static [u8] {
    CStr::from_ptr(ptr).to_bytes()
}

unsafe fn write_cstr(out: &mut dyn Write, ptr: *const c_char) {
    let _ = out.write_all(cstr_bytes(ptr));
}

fn usage() -> ! {
    let _ = io::stderr().write_all(b"Usage: fixdep <depfile> <target> <cmdline>\n");
    process::exit(1);
}

/*
 * Print out the commandline prefixed with cmd_<target filename> :=
 */
unsafe fn print_cmdline() {
    let mut out = io::stdout();
    let _ = out.write_all(b"cmd_");
    write_cstr(&mut out, TARGET);
    let _ = out.write_all(b" := ");
    write_cstr(&mut out, CMDLINE);
    let _ = out.write_all(b"\n\n");
}

/*
 * Important: The below generated source_foo.o and deps_foo.o variable
 * assignments are parsed not only by make, but also by the rather simple
 * parser in scripts/mod/sumversion.c.
 */
unsafe fn parse_dep_file(map: *mut c_void, len: SizeT) {
    let mut m = map as *mut u8;
    let end = m.add(len);
    let mut p: *mut u8;
    let mut s = [0u8; PATH_MAX];
    let mut is_target: c_int;
    let mut has_target: c_int = 0;
    let mut saw_any_target: c_int = 0;
    let mut is_first_dep: c_int = 0;
    let mut out = io::stdout();

    while m < end {
        /* Skip any "white space" */
        while m < end && (*m == b' ' || *m == b'\\' || *m == b'\n') {
            m = m.add(1);
        }
        /* Find next "white space" */
        p = m;
        while p < end && *p != b' ' && *p != b'\\' && *p != b'\n' {
            p = p.add(1);
        }
        /* Is the token we found a target name? */
        is_target = (*p.offset(-1) == b':') as c_int;
        /* Don't write any target names into the dependency file */
        if is_target != 0 {
            /* The /next/ file is the first dependency */
            is_first_dep = 1;
            has_target = 1;
        } else if has_target != 0 {
            /* Save this token/filename */
            let n = p.offset_from(m) as usize;
            std::ptr::copy_nonoverlapping(m, s.as_mut_ptr(), n);
            s[n] = 0;

            /*
             * Do not list the source file as dependency,
             * so that kbuild is not confused if a .c file
             * is rewritten into .S or vice versa. Storing
             * it in source_* is needed for modpost to
             * compute srcversions.
             */
            if is_first_dep != 0 {
                /*
                 * If processing the concatenation of
                 * multiple dependency files, only
                 * process the first target name, which
                 * will be the original source name,
                 * and ignore any other target names,
                 * which will be intermediate temporary
                 * files.
                 */
                if saw_any_target == 0 {
                    saw_any_target = 1;
                    let _ = out.write_all(b"source_");
                    write_cstr(&mut out, TARGET);
                    let _ = out.write_all(b" := ");
                    let _ = out.write_all(CStr::from_ptr(s.as_ptr() as *const c_char).to_bytes());
                    let _ = out.write_all(b"\n\n");
                    let _ = out.write_all(b"deps_");
                    write_cstr(&mut out, TARGET);
                    let _ = out.write_all(b" := \\\n");
                }
                is_first_dep = 0;
            } else {
                let _ = out.write_all(b"  ");
                let _ = out.write_all(CStr::from_ptr(s.as_ptr() as *const c_char).to_bytes());
                let _ = out.write_all(b" \\\n");
            }
        }
        /*
         * Start searching for next token immediately after the first
         * "whitespace" character that follows this token.
         */
        m = p.add(1);
    }

    if saw_any_target == 0 {
        let _ = io::stderr().write_all(b"fixdep: parse error; no targets found\n");
        process::exit(1);
    }

    let _ = out.write_all(b"\n");
    write_cstr(&mut out, TARGET);
    let _ = out.write_all(b": $(deps_");
    write_cstr(&mut out, TARGET);
    let _ = out.write_all(b")\n\n$(deps_");
    write_cstr(&mut out, TARGET);
    let _ = out.write_all(b"):\n");
}

unsafe fn print_deps() {
    let mut st = std::mem::MaybeUninit::<stat>::uninit();
    let fd: c_int;
    let map: *mut c_void;

    fd = open(DEPFILE, O_RDONLY);
    if fd < 0 {
        let _ = io::stderr().write_all(b"fixdep: error opening depfile: ");
        perror(DEPFILE);
        process::exit(2);
    }
    if fstat(fd, st.as_mut_ptr()) < 0 {
        let _ = io::stderr().write_all(b"fixdep: error fstat'ing depfile: ");
        perror(DEPFILE);
        process::exit(2);
    }
    let st = st.assume_init();
    if st.st_size == 0 {
        let mut err = io::stderr();
        let _ = err.write_all(b"fixdep: ");
        write_cstr(&mut err, DEPFILE);
        let _ = err.write_all(b" is empty\n");
        close(fd);
        return;
    }
    map = mmap(
        std::ptr::null_mut(),
        st.st_size as SizeT,
        PROT_READ,
        MAP_PRIVATE,
        fd,
        0,
    );
    if map as c_long == -1 {
        perror(c"fixdep: mmap".as_ptr());
        close(fd);
        return;
    }

    parse_dep_file(map, st.st_size as SizeT);

    munmap(map, st.st_size as SizeT);

    close(fd);
}

fn main() {
    let args: Vec<CString> = std::env::args_os()
        .map(|arg| CString::new(arg.as_os_str().as_bytes()).unwrap())
        .collect();

    if args.len() != 4 {
        usage();
    }

    unsafe {
        DEPFILE = args[1].as_ptr() as *mut c_char;
        TARGET = args[2].as_ptr() as *mut c_char;
        CMDLINE = args[3].as_ptr() as *mut c_char;

        print_cmdline();
        print_deps();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
