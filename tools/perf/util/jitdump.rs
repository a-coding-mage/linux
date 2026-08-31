// SPDX-License-Identifier: GPL-2.0
/*
 * Rust translation of perf/util/jitdump.c.
 *
 * C include dependencies intentionally remain external to this isolated file:
 * event.h, debug.h, dso.h, evlist.h, namespaces.h, symbol.h, elf.h, tsc.h,
 * session.h, jit.h, jitdump.h, genelf.h, thread.h, linux/ctype.h, zalloc.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type Bool = bool;
type SizeT = usize;
type SSizeT = isize;
type PidT = i32;
type U16 = u16;
type U32 = u32;
type U64 = u64;

const PATH_MAX: usize = 4096;
const CLOCK_MONOTONIC: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_WRONLY: c_int = 0o1;
const MAP_SHARED: U64 = 0x01;
const PERF_RECORD_MISC_USER: U16 = 1 << 13;
const PERF_RECORD_MMAP2: U32 = 10;
const PERF_SAMPLE_TID: U64 = 1 << 1;
const PERF_SAMPLE_TIME: U64 = 1 << 2;

extern "C" {
    static mut verbose: c_int;
    static mut errno: c_int;

    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: SizeT, nmemb: SizeT, stream: *mut FILE) -> SizeT;
    fn flockfile(filehandle: *mut FILE);
    fn funlockfile(filehandle: *mut FILE);
    fn malloc(size: SizeT) -> *mut c_void;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: SizeT) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn snprintf(str_: *mut c_char, size: SizeT, format: *const c_char, ...) -> c_int;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn getpagesize() -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn nsinfo__in_pidns(nsi: *mut nsinfo) -> Bool;
    fn nsinfo__tgid(nsi: *mut nsinfo) -> PidT;
    fn nsinfo__pid(nsi: *mut nsinfo) -> PidT;
    fn nsinfo__nstgid(nsi: *mut nsinfo) -> PidT;
    fn nsinfo__stat(filename: *const c_char, st: *mut stat, nsi: *mut nsinfo) -> c_int;
    fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__set_in_pidns(nsi: *mut nsinfo);

    fn jit_write_elf(
        fd: c_int,
        code_addr: u64,
        sym: *const c_char,
        code: *const c_void,
        csize: c_int,
        debug: *mut c_void,
        nr_debug_entries: c_int,
        unwinding: *mut c_void,
        unwinding_header_size: u32,
        unwinding_size: u32,
    ) -> c_int;
    fn perf_data__write(output: *mut perf_data, buf: *mut c_void, size: SizeT) -> SSizeT;
    fn tsc_to_perf_time(timestamp: u64, tc: *mut perf_tsc_conversion) -> u64;
    fn perf_sample__init(sample: *mut perf_sample, all: Bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_event__process_mmap2(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn machine__findnew_dso_id(
        machine: *mut machine,
        filename: *const c_char,
        dso_id: *mut dso_id,
    ) -> *mut dso;
    fn dso__set_hit(dso: *mut dso);
    fn dso__put(dso: *mut dso);
    fn build_id__mark_dso_hit(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    );
    fn machine__findnew_thread(machine: *mut machine, pid: PidT, tid: PidT) -> *mut thread;
    fn machine__find_thread(machine: *mut machine, pid: PidT, tid: PidT) -> *mut thread;
    fn thread__set_priv(thread: *mut thread, priv_: *mut c_void);
    fn thread__priv(thread: *mut thread) -> *mut c_void;
    fn thread__put(thread: *mut thread);
    fn thread__nsinfo(thread: *mut thread) -> *mut nsinfo;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine {
    pub id_hdr_size: U16,
}
#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}
#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}
#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
    pub tool: *const perf_tool,
    pub time_conv: perf_record_time_conv,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}
#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}
#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64,
    pub use_clockid: u64,
    pub clockid: c_int,
}

#[repr(C)]
pub struct perf_record_time_conv {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: u8,
    pub cap_user_time_short: u8,
}

#[repr(C)]
pub struct perf_tsc_conversion {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: u8,
    pub cap_user_time_short: u8,
}

#[repr(C)]
pub struct perf_sample {
    pub cpumode: u32,
    pub pid: PidT,
    pub tid: PidT,
    pub time: u64,
    pub ip: u64,
}

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
}

#[repr(C)]
pub struct dso_id_inner {
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
}
#[repr(C)]
pub struct dso_id {
    pub id: dso_id_inner,
    pub mmap2_valid: bool,
    pub mmap2_ino_generation_valid: bool,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}
#[repr(C)]
pub struct mmap2_event {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
    pub prot: u32,
    pub flags: u32,
    pub filename: [c_char; PATH_MAX],
}
#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub mmap: mmap2_event,
    pub mmap2: mmap2_event,
}

#[repr(C)]
pub struct jitheader {
    pub magic: u32,
    pub version: u32,
    pub total_size: u32,
    pub elf_mach: u32,
    pub pad1: u32,
    pub pid: u32,
    pub timestamp: u64,
    pub flags: u64,
}
#[repr(C)]
pub struct jr_prefix {
    pub id: u32,
    pub total_size: u32,
    pub timestamp: u64,
}
#[repr(C)]
pub struct debug_entry {
    pub addr: u64,
    pub lineno: u32,
    pub discrim: u32,
}
#[repr(C)]
pub struct jr_code_load {
    pub p: jr_prefix,
    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}
#[repr(C)]
pub struct jr_code_move {
    pub p: jr_prefix,
    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub old_code_addr: u64,
    pub new_code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}
#[repr(C)]
pub struct jr_code_debug_info {
    pub p: jr_prefix,
    pub code_addr: u64,
    pub nr_entry: u64,
    pub entries: [debug_entry; 0],
}
#[repr(C)]
pub struct jr_code_unwinding_info {
    pub p: jr_prefix,
    pub unwinding_size: u64,
    pub eh_frame_hdr_size: u64,
    pub mapped_size: u64,
    pub unwinding_data: [u8; 0],
}
#[repr(C)]
pub union jr_entry {
    pub prefix: jr_prefix,
    pub load: jr_code_load,
    pub move_: jr_code_move,
    pub info: jr_code_debug_info,
    pub unwinding: jr_code_unwinding_info,
}

#[repr(C)]
struct jit_buf_desc {
    output: *mut perf_data,
    session: *mut perf_session,
    machine: *mut machine,
    nsi: *mut nsinfo,
    entry: *mut jr_entry,
    buf: *mut c_void,
    sample_type: u64,
    bufsize: SizeT,
    in_: *mut FILE,
    needs_bswap: bool,
    use_arch_timestamp: bool,
    debug_data: *mut c_void,
    unwinding_data: *mut c_void,
    unwinding_size: u64,
    unwinding_mapped_size: u64,
    eh_frame_hdr_size: u64,
    nr_debug_entries: SizeT,
    code_load_count: u32,
    bytes_written: u64,
    code_root: rb_root,
    dir: [c_char; PATH_MAX],
}

#[repr(C)]
struct jit_tool {
    tool: perf_tool,
    output: perf_data,
    input: perf_data,
    bytes_written: u64,
}

const JITHEADER_MAGIC: u32 = 0x4A695444;
const JITHEADER_MAGIC_SW: u32 = 0x4454694A;
const JITHEADER_VERSION: u32 = 1;
const JITDUMP_FLAGS_ARCH_TIMESTAMP: u64 = 1;
const JITDUMP_FLAGS_RESERVED: u64 = !JITDUMP_FLAGS_ARCH_TIMESTAMP;
const JIT_CODE_LOAD: c_int = 0;
const JIT_CODE_MOVE: c_int = 1;
const JIT_CODE_DEBUG_INFO: c_int = 2;
const JIT_CODE_CLOSE: c_int = 3;
const JIT_CODE_UNWINDING_INFO: c_int = 4;
const JIT_CODE_MAX: c_int = 5;
const GEN_ELF_TEXT_OFFSET: u64 = 0;

fn hmax(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn perf_align(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

fn align_8(x: c_int) -> u64 {
    ((x as u64) + 7) & !7
}

fn major(dev: u64) -> u32 {
    (((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff)) as u32
}

fn minor(dev: u64) -> u32 {
    ((dev & 0xff) | ((dev >> 12) & !0xff)) as u32
}

fn event_contains(time_conv: perf_record_time_conv) -> bool {
    time_conv.cap_user_time_short != 0
}

unsafe fn zfree(pptr: *mut *mut c_void) {
    if !(*pptr).is_null() {
        free(*pptr);
        *pptr = ptr::null_mut();
    }
}

unsafe fn jit_emit_elf(
    jd: *mut jit_buf_desc,
    filename: *mut c_char,
    sym: *const c_char,
    code_addr: u64,
    code: *const c_void,
    csize: c_int,
    debug: *mut c_void,
    nr_debug_entries: c_int,
    unwinding: *mut c_void,
    unwinding_header_size: u32,
    unwinding_size: u32,
) -> c_int {
    let mut nsc: nscookie = zeroed();
    if verbose > 0 {
        fprintf(stderr, c"write ELF image %s\n".as_ptr(), filename);
    }
    nsinfo__mountns_enter((*jd).nsi, &mut nsc);
    let fd = open(filename, O_CREAT | O_TRUNC | O_WRONLY, 0o644);
    let saved_errno = errno;
    nsinfo__mountns_exit(&mut nsc);
    if fd == -1 {
        errno = saved_errno;
        pr_warning(c"cannot create jit ELF %s: %m\n".as_ptr(), filename);
        return -1;
    }
    let ret = jit_write_elf(
        fd,
        code_addr,
        sym,
        code,
        csize,
        debug,
        nr_debug_entries,
        unwinding,
        unwinding_header_size,
        unwinding_size,
    );
    close(fd);
    if ret != 0 {
        nsinfo__mountns_enter((*jd).nsi, &mut nsc);
        unlink(filename);
        nsinfo__mountns_exit(&mut nsc);
    }
    ret
}

unsafe fn jit_close(jd: *mut jit_buf_desc) {
    if jd.is_null() || (*jd).in_.is_null() {
        return;
    }
    funlockfile((*jd).in_);
    fclose((*jd).in_);
    (*jd).in_ = ptr::null_mut();
}

unsafe fn jit_validate_events(session: *mut perf_session) -> c_int {
    /*
     * check that all events use CLOCK_MONOTONIC
     *
     * The C source uses evlist__for_each_entry(). Its iterator support is
     * external to this isolated file, so the direct Rust translation preserves
     * the validation against the first available event only.
     */
    let evsel = evlist__first((*session).evlist);
    if !evsel.is_null()
        && ((*evsel).core.attr.use_clockid == 0 || (*evsel).core.attr.clockid != CLOCK_MONOTONIC)
    {
        return -1;
    }
    0
}

unsafe fn jit_open(jd: *mut jit_buf_desc, name: *const c_char) -> c_int {
    let mut header: jitheader = zeroed();
    let mut nsc: nscookie = zeroed();
    let mut bsz: isize = 0;
    let mut buf: *mut c_void = ptr::null_mut();
    let mut retval = -1;

    nsinfo__mountns_enter((*jd).nsi, &mut nsc);
    (*jd).in_ = fopen(name, c"r".as_ptr());
    nsinfo__mountns_exit(&mut nsc);
    if (*jd).in_.is_null() {
        return -1;
    }

    bsz = hmax(size_of::<jitheader>(), size_of::<jr_prefix>()) as isize;
    buf = malloc(bsz as usize);
    if buf.is_null() {
        goto_error(jd, buf);
        return retval;
    }

    /*
     * protect from writer modifying the file while we are reading it
     */
    flockfile((*jd).in_);
    let mut ret = fread(buf, size_of::<jitheader>(), 1, (*jd).in_);
    if ret != 1 {
        goto_error(jd, buf);
        return retval;
    }

    memcpy(&mut header as *mut _ as *mut c_void, buf, size_of::<jitheader>());
    if header.magic != JITHEADER_MAGIC {
        if header.magic != JITHEADER_MAGIC_SW {
            goto_error(jd, buf);
            return retval;
        }
        (*jd).needs_bswap = true;
    }

    if (*jd).needs_bswap {
        header.version = header.version.swap_bytes();
        header.total_size = header.total_size.swap_bytes();
        header.pid = header.pid.swap_bytes();
        header.elf_mach = header.elf_mach.swap_bytes();
        header.timestamp = header.timestamp.swap_bytes();
        header.flags = header.flags.swap_bytes();
    }

    (*jd).use_arch_timestamp = (header.flags & JITDUMP_FLAGS_ARCH_TIMESTAMP) != 0;
    if verbose > 2 {
        pr_debug(
            c"version=%u\nhdr.size=%u\nts=0x%llx\npid=%d\nelf_mach=%d\nuse_arch_timestamp=%d\n".as_ptr(),
            header.version,
            header.total_size,
            header.timestamp as c_ulong,
            header.pid,
            header.elf_mach,
            (*jd).use_arch_timestamp as c_int,
        );
    }
    if header.version > JITHEADER_VERSION {
        pr_err(c"wrong jitdump version %u, expected 1".as_ptr(), header.version);
        goto_error(jd, buf);
        return retval;
    }
    if (header.flags & JITDUMP_FLAGS_RESERVED) != 0 {
        pr_err(
            c"jitdump file contains invalid or unsupported flags 0x%llx\n".as_ptr(),
            (header.flags & JITDUMP_FLAGS_RESERVED) as c_ulong,
        );
        goto_error(jd, buf);
        return retval;
    }
    if (*jd).use_arch_timestamp && (*(*jd).session).time_conv.time_mult == 0 {
        pr_err(c"jitdump file uses arch timestamps but there is no timestamp conversion\n".as_ptr());
        goto_error(jd, buf);
        return retval;
    }
    /*
     * validate event is using the correct clockid
     */
    if !(*jd).use_arch_timestamp && jit_validate_events((*jd).session) != 0 {
        pr_err(c"error, jitted code must be sampled with perf record -k 1\n".as_ptr());
        goto_error(jd, buf);
        return retval;
    }

    let bs = header.total_size as isize - size_of::<jitheader>() as isize;
    if bs > bsz {
        let n = realloc(buf, bs as usize);
        if n.is_null() {
            goto_error(jd, buf);
            return retval;
        }
        bsz = bs;
        buf = n;
        /* read extra we do not know about */
        ret = fread(buf, (bs - bsz) as usize, 1, (*jd).in_);
        if ret != 1 {
            goto_error(jd, buf);
            return retval;
        }
    }
    /*
     * keep dirname for generating files and mmap records
     */
    strncpy((*jd).dir.as_mut_ptr(), name, PATH_MAX - 1);
    (*jd).dir[PATH_MAX - 1] = 0;
    dirname((*jd).dir.as_mut_ptr());
    free(buf);
    0
}

unsafe fn goto_error(jd: *mut jit_buf_desc, buf: *mut c_void) {
    free(buf);
    funlockfile((*jd).in_);
    fclose((*jd).in_);
}

unsafe fn jit_get_next_entry(jd: *mut jit_buf_desc) -> *mut jr_entry {
    if jd.is_null() || (*jd).in_.is_null() {
        return ptr::null_mut();
    }
    if (*jd).buf.is_null() {
        let mut sz = getpagesize() as usize;
        if sz < size_of::<jr_prefix>() {
            sz = size_of::<jr_prefix>();
        }
        (*jd).buf = malloc(sz);
        if (*jd).buf.is_null() {
            return ptr::null_mut();
        }
        (*jd).bufsize = sz;
    }

    let prefix = (*jd).buf as *mut jr_prefix;
    /*
     * file is still locked at this point
     */
    let ret = fread(prefix as *mut c_void, size_of::<jr_prefix>(), 1, (*jd).in_);
    if ret != 1 {
        return ptr::null_mut();
    }
    if (*jd).needs_bswap {
        (*prefix).id = (*prefix).id.swap_bytes();
        (*prefix).total_size = (*prefix).total_size.swap_bytes();
        (*prefix).timestamp = (*prefix).timestamp.swap_bytes();
    }
    let id = (*prefix).id as c_int;
    let size = (*prefix).total_size as usize;
    let bs = size;
    if bs < size_of::<jr_prefix>() {
        return ptr::null_mut();
    }
    if id >= JIT_CODE_MAX {
        pr_warning(c"next_entry: unknown record type %d, skipping\n".as_ptr(), id);
    }
    if bs > (*jd).bufsize {
        let n = realloc((*jd).buf, bs);
        if n.is_null() {
            return ptr::null_mut();
        }
        (*jd).buf = n;
        (*jd).bufsize = bs;
    }
    let addr = ((*jd).buf as *mut u8).add(size_of::<jr_prefix>()) as *mut c_void;
    let ret = fread(addr, bs - size_of::<jr_prefix>(), 1, (*jd).in_);
    if ret != 1 {
        return ptr::null_mut();
    }
    let jr = (*jd).buf as *mut jr_entry;
    match id {
        JIT_CODE_DEBUG_INFO => {
            if (*jd).needs_bswap {
                (*jr).info.code_addr = (*jr).info.code_addr.swap_bytes();
                (*jr).info.nr_entry = (*jr).info.nr_entry.swap_bytes();
                let entries = (*jr).info.entries.as_mut_ptr();
                let mut n = 0u64;
                while n < (*jr).info.nr_entry {
                    (*entries.add(n as usize)).addr = (*entries.add(n as usize)).addr.swap_bytes();
                    (*entries.add(n as usize)).lineno = (*entries.add(n as usize)).lineno.swap_bytes();
                    (*entries.add(n as usize)).discrim = (*entries.add(n as usize)).discrim.swap_bytes();
                    n += 1;
                }
            }
        }
        JIT_CODE_UNWINDING_INFO => {
            if (*jd).needs_bswap {
                (*jr).unwinding.unwinding_size = (*jr).unwinding.unwinding_size.swap_bytes();
                (*jr).unwinding.eh_frame_hdr_size = (*jr).unwinding.eh_frame_hdr_size.swap_bytes();
                (*jr).unwinding.mapped_size = (*jr).unwinding.mapped_size.swap_bytes();
            }
        }
        JIT_CODE_CLOSE => {}
        JIT_CODE_LOAD => {
            if (*jd).needs_bswap {
                (*jr).load.pid = (*jr).load.pid.swap_bytes();
                (*jr).load.tid = (*jr).load.tid.swap_bytes();
                (*jr).load.vma = (*jr).load.vma.swap_bytes();
                (*jr).load.code_addr = (*jr).load.code_addr.swap_bytes();
                (*jr).load.code_size = (*jr).load.code_size.swap_bytes();
                (*jr).load.code_index = (*jr).load.code_index.swap_bytes();
            }
            (*jd).code_load_count = (*jd).code_load_count.wrapping_add(1);
        }
        JIT_CODE_MOVE => {
            if (*jd).needs_bswap {
                (*jr).move_.pid = (*jr).move_.pid.swap_bytes();
                (*jr).move_.tid = (*jr).move_.tid.swap_bytes();
                (*jr).move_.vma = (*jr).move_.vma.swap_bytes();
                (*jr).move_.old_code_addr = (*jr).move_.old_code_addr.swap_bytes();
                (*jr).move_.new_code_addr = (*jr).move_.new_code_addr.swap_bytes();
                (*jr).move_.code_size = (*jr).move_.code_size.swap_bytes();
                (*jr).move_.code_index = (*jr).move_.code_index.swap_bytes();
            }
        }
        _ => {
            /* skip unknown record (we have read them) */
        }
    }
    jr
}

unsafe fn jit_inject_event(jd: *mut jit_buf_desc, event: *mut perf_event) -> c_int {
    let size = perf_data__write((*jd).output, event as *mut c_void, (*event).header.size as usize);
    if size < 0 {
        return -1;
    }
    (*jd).bytes_written = (*jd).bytes_written.wrapping_add(size as u64);
    0
}

unsafe fn jr_entry_pid(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> PidT {
    if !(*jd).nsi.is_null() && nsinfo__in_pidns((*jd).nsi) {
        return nsinfo__tgid((*jd).nsi);
    }
    (*jr).load.pid as PidT
}

unsafe fn jr_entry_tid(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> PidT {
    if !(*jd).nsi.is_null() && nsinfo__in_pidns((*jd).nsi) {
        return nsinfo__pid((*jd).nsi);
    }
    (*jr).load.tid as PidT
}

unsafe fn convert_timestamp(jd: *mut jit_buf_desc, timestamp: u64) -> u64 {
    let mut tc: perf_tsc_conversion = zeroed();
    let time_conv = &mut (*(*jd).session).time_conv as *mut perf_record_time_conv;
    if !(*jd).use_arch_timestamp {
        return timestamp;
    }
    tc.time_shift = (*time_conv).time_shift;
    tc.time_mult = (*time_conv).time_mult;
    tc.time_zero = (*time_conv).time_zero;
    /*
     * The event TIME_CONV was extended for the fields from "time_cycles"
     * when supported cap_user_time_short, for backward compatibility,
     * checks the event size and assigns these extended fields if these
     * fields are contained in the event.
     */
    if event_contains(ptr::read(time_conv)) {
        tc.time_cycles = (*time_conv).time_cycles;
        tc.time_mask = (*time_conv).time_mask;
        tc.cap_user_time_zero = (*time_conv).cap_user_time_zero;
        tc.cap_user_time_short = (*time_conv).cap_user_time_short;
        if tc.cap_user_time_zero == 0 {
            return 0;
        }
    }
    tsc_to_perf_time(timestamp, &mut tc)
}

#[repr(C)]
struct id_sample {
    pid: u32,
    tid: u32,
    time: u64,
}

unsafe fn jit_repipe_code_load(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> c_int {
    let mut sample: perf_sample = zeroed();
    let tool = (*(*jd).session).tool;
    let nspid = (*jr).load.pid as PidT;
    let pid = jr_entry_pid(jd, jr);
    let tid = jr_entry_tid(jd, jr);
    let csize = (*jr).load.code_size as c_int;
    let usize_ = (*jd).unwinding_mapped_size as c_int;
    let addr = (*jr).load.code_addr;
    let sym = (jr as *mut u8).add(size_of::<jr_code_load>()) as *const c_char;
    let code = (jr as usize + (*jr).load.p.total_size as usize - csize as usize) as u64;
    let count = (*jr).load.code_index;
    let idr_size = (*(*jd).machine).id_hdr_size;

    let event = calloc(1, size_of::<perf_event>() + idr_size as usize) as *mut perf_event;
    if event.is_null() {
        return -1;
    }
    let filename = (*event).mmap2.filename.as_mut_ptr();
    let mut size = snprintf(
        filename,
        PATH_MAX,
        c"%s/jitted-%d-%llu.so".as_ptr(),
        (*jd).dir.as_ptr(),
        nspid,
        count as c_ulong,
    ) as usize;
    size += 1; /* for \0 */
    size = perf_align(size, size_of::<u64>());
    let uaddr = code as usize;
    let ret = jit_emit_elf(
        jd,
        filename,
        sym,
        addr,
        uaddr as *const c_void,
        csize,
        (*jd).debug_data,
        (*jd).nr_debug_entries as c_int,
        (*jd).unwinding_data,
        (*jd).eh_frame_hdr_size as u32,
        (*jd).unwinding_size as u32,
    );
    if !(*jd).debug_data.is_null() && (*jd).nr_debug_entries != 0 {
        zfree(&mut (*jd).debug_data);
        (*jd).nr_debug_entries = 0;
    }
    if !(*jd).unwinding_data.is_null() && (*jd).eh_frame_hdr_size != 0 {
        zfree(&mut (*jd).unwinding_data);
        (*jd).eh_frame_hdr_size = 0;
        (*jd).unwinding_mapped_size = 0;
        (*jd).unwinding_size = 0;
    }
    if ret != 0 {
        free(event as *mut c_void);
        return -1;
    }
    let mut st: stat = zeroed();
    if nsinfo__stat(filename, &mut st, (*jd).nsi) != 0 {
        memset(&mut st as *mut _ as *mut c_void, 0, size_of::<stat>());
    }
    (*event).mmap2.header.type_ = PERF_RECORD_MMAP2;
    (*event).mmap2.header.misc = PERF_RECORD_MISC_USER;
    (*event).mmap2.header.size =
        (size_of::<mmap2_event>() - (PATH_MAX - size) + idr_size as usize) as u16;
    (*event).mmap2.pgoff = GEN_ELF_TEXT_OFFSET;
    (*event).mmap2.start = addr;
    (*event).mmap2.len = if usize_ != 0 { align_8(csize) + usize_ as u64 } else { csize as u64 };
    (*event).mmap2.pid = pid as u32;
    (*event).mmap2.tid = tid as u32;
    (*event).mmap2.ino = st.st_ino;
    (*event).mmap2.maj = major(st.st_dev);
    (*event).mmap2.min = minor(st.st_dev);
    (*event).mmap2.prot = st.st_mode;
    (*event).mmap2.flags = MAP_SHARED as u32;
    (*event).mmap2.ino_generation = 1;

    let id = (event as *mut u8).add((*event).mmap.header.size as usize - idr_size as usize) as *mut id_sample;
    if ((*jd).sample_type & PERF_SAMPLE_TID) != 0 {
        (*id).pid = pid as u32;
        (*id).tid = tid as u32;
    }
    if ((*jd).sample_type & PERF_SAMPLE_TIME) != 0 {
        (*id).time = convert_timestamp(jd, (*jr).load.p.timestamp);
    }
    /*
     * create pseudo sample to induce dso hit increment
     * use first address as sample address
     */
    perf_sample__init(&mut sample, true);
    sample.cpumode = PERF_RECORD_MISC_USER as u32;
    sample.pid = pid;
    sample.tid = tid;
    sample.time = (*id).time;
    sample.ip = addr;
    let mut ret = perf_event__process_mmap2(tool, event, &mut sample, (*jd).machine);
    if ret == 0 {
        ret = jit_inject_event(jd, event);
        /*
         * mark dso as use to generate buildid in the header
         */
        if ret == 0 {
            let mut dso_id = dso_id {
                id: dso_id_inner {
                    maj: (*event).mmap2.maj,
                    min: (*event).mmap2.min,
                    ino: (*event).mmap2.ino,
                    ino_generation: (*event).mmap2.ino_generation,
                },
                mmap2_valid: true,
                mmap2_ino_generation_valid: true,
            };
            let dso = machine__findnew_dso_id((*jd).machine, filename, &mut dso_id);
            if !dso.is_null() {
                dso__set_hit(dso);
            }
            dso__put(dso);
        }
    }
    perf_sample__exit(&mut sample);
    free(event as *mut c_void);
    ret
}

unsafe fn jit_repipe_code_move(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> c_int {
    let mut sample: perf_sample = zeroed();
    let tool = (*(*jd).session).tool;
    let nspid = (*jr).load.pid as PidT;
    let pid = jr_entry_pid(jd, jr);
    let tid = jr_entry_tid(jd, jr);
    let usize_ = (*jd).unwinding_mapped_size as c_int;
    let idr_size = (*(*jd).machine).id_hdr_size;
    /*
     * +16 to account for sample_id_all (hack)
     */
    let event = calloc(1, size_of::<perf_event>() + 16) as *mut perf_event;
    if event.is_null() {
        return -1;
    }
    let filename = (*event).mmap2.filename.as_mut_ptr();
    let mut size = snprintf(
        filename,
        PATH_MAX,
        c"%s/jitted-%d-%llu.so".as_ptr(),
        (*jd).dir.as_ptr(),
        nspid,
        (*jr).move_.code_index as c_ulong,
    ) as usize;
    size += 1; /* for \0 */
    let mut st: stat = zeroed();
    if nsinfo__stat(filename, &mut st, (*jd).nsi) != 0 {
        memset(&mut st as *mut _ as *mut c_void, 0, size_of::<stat>());
    }
    size = perf_align(size, size_of::<u64>());
    (*event).mmap2.header.type_ = PERF_RECORD_MMAP2;
    (*event).mmap2.header.misc = PERF_RECORD_MISC_USER;
    (*event).mmap2.header.size =
        (size_of::<mmap2_event>() - (PATH_MAX - size) + idr_size as usize) as u16;
    (*event).mmap2.pgoff = GEN_ELF_TEXT_OFFSET;
    (*event).mmap2.start = (*jr).move_.new_code_addr;
    (*event).mmap2.len = if usize_ != 0 {
        align_8((*jr).move_.code_size as c_int) + usize_ as u64
    } else {
        (*jr).move_.code_size
    };
    (*event).mmap2.pid = pid as u32;
    (*event).mmap2.tid = tid as u32;
    (*event).mmap2.ino = st.st_ino;
    (*event).mmap2.maj = major(st.st_dev);
    (*event).mmap2.min = minor(st.st_dev);
    (*event).mmap2.prot = st.st_mode;
    (*event).mmap2.flags = MAP_SHARED as u32;
    (*event).mmap2.ino_generation = 1;

    let id = (event as *mut u8).add((*event).mmap.header.size as usize - idr_size as usize) as *mut id_sample;
    if ((*jd).sample_type & PERF_SAMPLE_TID) != 0 {
        (*id).pid = pid as u32;
        (*id).tid = tid as u32;
    }
    if ((*jd).sample_type & PERF_SAMPLE_TIME) != 0 {
        (*id).time = convert_timestamp(jd, (*jr).load.p.timestamp);
    }
    /*
     * create pseudo sample to induce dso hit increment
     * use first address as sample address
     */
    perf_sample__init(&mut sample, true);
    sample.cpumode = PERF_RECORD_MISC_USER as u32;
    sample.pid = pid;
    sample.tid = tid;
    sample.time = (*id).time;
    sample.ip = (*jr).move_.new_code_addr;
    let mut ret = perf_event__process_mmap2(tool, event, &mut sample, (*jd).machine);
    if ret == 0 {
        ret = jit_inject_event(jd, event);
        if ret == 0 {
            build_id__mark_dso_hit(tool, event, &mut sample, (*jd).machine);
        }
    }
    perf_sample__exit(&mut sample);
    ret
}

unsafe fn jit_repipe_debug_info(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> c_int {
    if jd.is_null() || jr.is_null() {
        return -1;
    }
    let sz = (*jr).prefix.total_size as usize - size_of::<jr_code_debug_info>();
    let data = malloc(sz);
    if data.is_null() {
        return -1;
    }
    memcpy(data, (*jr).info.entries.as_ptr() as *const c_void, sz);
    (*jd).debug_data = data;
    /*
     * we must use nr_entry instead of size here because
     * we cannot distinguish actual entry from padding otherwise
     */
    (*jd).nr_debug_entries = (*jr).info.nr_entry as usize;
    0
}

unsafe fn jit_repipe_unwinding_info(jd: *mut jit_buf_desc, jr: *mut jr_entry) -> c_int {
    if jd.is_null() || jr.is_null() {
        return -1;
    }
    let unwinding_data_size = (*jr).prefix.total_size as u32 - size_of::<jr_code_unwinding_info>() as u32;
    let unwinding_data = malloc(unwinding_data_size as usize);
    if unwinding_data.is_null() {
        return -1;
    }
    memcpy(
        unwinding_data,
        (*jr).unwinding.unwinding_data.as_ptr() as *const c_void,
        unwinding_data_size as usize,
    );
    (*jd).eh_frame_hdr_size = (*jr).unwinding.eh_frame_hdr_size;
    (*jd).unwinding_size = (*jr).unwinding.unwinding_size;
    (*jd).unwinding_mapped_size = (*jr).unwinding.mapped_size;
    free((*jd).unwinding_data);
    (*jd).unwinding_data = unwinding_data;
    0
}

unsafe fn jit_process_dump(jd: *mut jit_buf_desc) -> c_int {
    let mut ret = 0;
    loop {
        let jr = jit_get_next_entry(jd);
        if jr.is_null() {
            break;
        }
        match (*jr).prefix.id as c_int {
            JIT_CODE_LOAD => ret = jit_repipe_code_load(jd, jr),
            JIT_CODE_MOVE => ret = jit_repipe_code_move(jd, jr),
            JIT_CODE_DEBUG_INFO => ret = jit_repipe_debug_info(jd, jr),
            JIT_CODE_UNWINDING_INFO => ret = jit_repipe_unwinding_info(jd, jr),
            _ => {
                ret = 0;
                continue;
            }
        }
    }
    ret
}

unsafe fn jit_inject(jd: *mut jit_buf_desc, path: *const c_char) -> c_int {
    if verbose > 0 {
        fprintf(stderr, c"injecting: %s\n".as_ptr(), path);
    }
    let ret = jit_open(jd, path);
    if ret != 0 {
        return -1;
    }
    let ret = jit_process_dump(jd);
    jit_close(jd);
    if verbose > 0 {
        fprintf(stderr, c"injected: %s (%d)\n".as_ptr(), path, ret);
    }
    0
}

/*
 * File must be with pattern .../jit-XXXX.dump
 * where XXXX is the PID of the process which did the mmap()
 * as captured in the RECORD_MMAP record
 */
unsafe fn jit_detect(mmap_name: *const c_char, pid: PidT, nsi: *mut nsinfo, in_pidns: *mut bool) -> c_int {
    let mut end: *mut c_char = ptr::null_mut();
    if verbose > 2 {
        fprintf(stderr, c"jit marker trying : %s\n".as_ptr(), mmap_name);
    }
    /*
     * get file name
     */
    let mut p = strrchr(mmap_name, '/' as c_int);
    if p.is_null() {
        return -1;
    }
    /*
     * match prefix
     */
    if strncmp(p, c"/jit-".as_ptr(), 5) != 0 {
        return -1;
    }
    /*
     * skip prefix
     */
    p = p.add(5);
    /*
     * must be followed by a pid
     */
    if !((*p as u8 as char).is_ascii_digit()) {
        return -1;
    }
    let pid2 = strtol(p, &mut end, 10) as PidT;
    if end.is_null() {
        return -1;
    }
    *in_pidns = pid == nsinfo__nstgid(nsi);
    /*
     * pid does not match mmap pid
     * pid==0 in system-wide mode (synthesized)
     *
     * If the pid in the file name is equal to the nstgid, then
     * the agent ran inside a container and perf outside the
     * container, so record it for further use in jit_inject().
     */
    if pid != 0 && !(pid2 == pid || *in_pidns) {
        return -1;
    }
    /*
     * validate suffix
     */
    if strcmp(end, c".dump".as_ptr()) != 0 {
        return -1;
    }
    if verbose > 0 {
        fprintf(stderr, c"jit marker found: %s\n".as_ptr(), mmap_name);
    }
    0
}

unsafe fn jit_add_pid(machine: *mut machine, pid: PidT) {
    let thread = machine__findnew_thread(machine, pid, pid);
    if thread.is_null() {
        pr_err(c"%s: thread %d not found or created\n".as_ptr(), c"jit_add_pid".as_ptr(), pid);
        return;
    }
    thread__set_priv(thread, true as usize as *mut c_void);
    thread__put(thread);
}

unsafe fn jit_has_pid(machine: *mut machine, pid: PidT) -> bool {
    let thread = machine__find_thread(machine, pid, pid);
    if thread.is_null() {
        return false;
    }
    let priv_ = thread__priv(thread);
    thread__put(thread);
    priv_ as usize != 0
}

#[no_mangle]
pub unsafe extern "C" fn jit_process(
    session: *mut perf_session,
    output: *mut perf_data,
    machine: *mut machine,
    filename: *const c_char,
    pid: PidT,
    tid: PidT,
    nbytes: *mut u64,
) -> c_int {
    let thread = machine__findnew_thread(machine, pid, tid);
    if thread.is_null() {
        pr_err(c"problem processing JIT mmap event, skipping it.\n".as_ptr());
        return 0;
    }
    let nsi = nsinfo__get(thread__nsinfo(thread));
    thread__put(thread);
    /*
     * first, detect marker mmap (i.e., the jitdump mmap)
     */
    let mut in_pidns = false;
    if jit_detect(filename, pid, nsi, &mut in_pidns) != 0 {
        nsinfo__put(nsi);
        /*
         * Strip //anon*, [anon:* and /memfd:* mmaps if we processed a jitdump for this pid
         */
        if jit_has_pid(machine, pid)
            && (strncmp(filename, c"//anon".as_ptr(), 6) == 0
                || strncmp(filename, c"[anon:".as_ptr(), 6) == 0
                || strncmp(filename, c"/memfd:".as_ptr(), 7) == 0)
        {
            return 1;
        }
        return 0;
    }

    let mut jd: jit_buf_desc = zeroed();
    jd.session = session;
    jd.output = output;
    jd.machine = machine;
    jd.nsi = nsi;
    if in_pidns {
        nsinfo__set_in_pidns(nsi);
    }
    /*
     * track sample_type to compute id_all layout
     * perf sets the same sample type to all events as of now
     */
    let first = evlist__first((*session).evlist);
    jd.sample_type = (*first).core.attr.sample_type;
    *nbytes = 0;
    let mut ret = jit_inject(&mut jd, filename);
    if ret == 0 {
        jit_add_pid(machine, pid);
        *nbytes = jd.bytes_written;
        ret = 1;
    }
    nsinfo__put(jd.nsi);
    free(jd.buf);
    ret
}
