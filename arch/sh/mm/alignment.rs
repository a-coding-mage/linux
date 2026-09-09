/*
 * Alignment access counters and corresponding user-space interfaces.
 *
 * Copyright (C) 2009 ST Microelectronics
 * Copyright (C) 2009 - 2010 Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* C dependencies: linux/module.h, linux/kernel.h, linux/seq_file.h,
 * linux/proc_fs.h, linux/uaccess.h, linux/ratelimit.h, asm/alignment.h,
 * and asm/processor.h. */

use core::ffi::{c_char, c_int, c_void};

type SizeT = usize;
type SsizeT = isize;
type LoffT = i64;
type InsnSizeT = u16;

const UM_WARN: c_int = 1;
const UM_FIXUP: c_int = 2;
const UM_SIGNAL: c_int = 4;
const SH_THREAD_UAC_SIGBUS: u64 = 1 << 0;
const SH_THREAD_UAC_NOPRINT: u64 = 1 << 1;
const SH_THREAD_UAC_MASK: u64 = SH_THREAD_UAC_SIGBUS | SH_THREAD_UAC_NOPRINT;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const S_IWUSR: u32 = 0o200;
const S_IRUGO: u32 = 0o444;

#[repr(C)]
pub struct ThreadStruct {
    pub flags: u64,
}

#[repr(C)]
pub struct TaskStruct {
    pub thread: ThreadStruct,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ProcDirEntry {
    _private: [u8; 0],
}

extern "C" {
    static mut current: *mut TaskStruct;
    fn user_mode(regs: *mut PtRegs) -> bool;
    fn instruction_pointer(regs: *mut PtRegs) -> usize;
    fn task_pid_nr(tsk: *mut TaskStruct) -> c_int;
    fn put_user<T>(value: T, ptr: *mut T) -> c_int;
    fn get_user<T>(value: *mut T, ptr: *const T) -> c_int;
    fn seq_printf(m: *mut SeqFile, fmt: *const c_char, ...) -> c_int;
    fn single_open(file: *mut File, show: Option<unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn seq_read(file: *mut File, buf: *mut c_void, count: SizeT, pos: *mut LoFF) -> SsizeT;
    fn seq_lseek(file: *mut File, pos: LoFF, whence: c_int) -> LoFF;
    fn single_release(inode: *mut Inode, file: *mut File) -> c_int;
    fn pde_data(inode: *mut Inode) -> *mut c_void;
    fn file_inode(file: *mut File) -> *mut Inode;
    fn proc_mkdir(name: *const c_char, parent: *mut ProcDirEntry) -> *mut ProcDirEntry;
    fn proc_create_data(name: *const c_char, mode: u32, parent: *mut ProcDirEntry, ops: *const ProcOps, data: *mut c_void) -> *mut ProcDirEntry;
    fn pr_notice_ratelimited(fmt: *const c_char, ...);
}

type LoFF = LoFF_t;
type LoFF_t = i64;

#[repr(C)]
pub struct ProcOps {
    pub proc_open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
    pub proc_read: Option<unsafe extern "C" fn(*mut File, *mut c_void, SizeT, *mut LoFF) -> SsizeT>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut File, LoFF, c_int) -> LoFF>,
    pub proc_release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
    pub proc_write: Option<unsafe extern "C" fn(*mut File, *const c_char, SizeT, *mut LoFF) -> SsizeT>,
}

static mut SE_USER: usize = 0;
static mut SE_SYS: usize = 0;
static mut SE_HALF: usize = 0;
static mut SE_WORD: usize = 0;
static mut SE_DWORD: usize = 0;
static mut SE_MULTI: usize = 0;
/* bitfield: 1: warn 2: fixup 4: signal -> combinations 2|4 && 1|2|4 are not
   valid! */
static mut SE_USERMODE: c_int = UM_WARN | UM_FIXUP;
/* 0: no warning 1: print a warning message, disabled by default */
static mut SE_KERNMODE_WARN: c_int = 0;

pub unsafe extern "C" fn inc_unaligned_byte_access() { SE_HALF += 1; }
pub unsafe extern "C" fn inc_unaligned_word_access() { SE_WORD += 1; }
pub unsafe extern "C" fn inc_unaligned_dword_access() { SE_DWORD += 1; }
pub unsafe extern "C" fn inc_unaligned_multi_access() { SE_MULTI += 1; }
pub unsafe extern "C" fn inc_unaligned_user_access() { SE_USER += 1; }
pub unsafe extern "C" fn inc_unaligned_kernel_access() { SE_SYS += 1; }

pub unsafe extern "C" fn unaligned_user_action() -> u32 {
    let mut action = SE_USERMODE;
    if ((*current).thread.flags & SH_THREAD_UAC_SIGBUS) != 0 {
        action &= !UM_FIXUP;
        action |= UM_SIGNAL;
    }
    if ((*current).thread.flags & SH_THREAD_UAC_NOPRINT) != 0 {
        action &= !UM_WARN;
    }
    action as u32
}

pub unsafe extern "C" fn get_unalign_ctl(tsk: *mut TaskStruct, addr: usize) -> c_int {
    put_user((*tsk).thread.flags & SH_THREAD_UAC_MASK, addr as *mut u64)
}

pub unsafe extern "C" fn set_unalign_ctl(tsk: *mut TaskStruct, val: u32) -> c_int {
    (*tsk).thread.flags = ((*tsk).thread.flags & !SH_THREAD_UAC_MASK) | (val as u64 & SH_THREAD_UAC_MASK);
    0
}

pub unsafe extern "C" fn unaligned_fixups_notify(tsk: *mut TaskStruct, insn: InsnSizeT, regs: *mut PtRegs) {
    if user_mode(regs) && (SE_USERMODE & UM_WARN) != 0 {
        pr_notice_ratelimited(b"Fixing up unaligned userspace access in \"%s\" pid=%d pc=0x%p ins=0x%04hx\n\0".as_ptr() as *const c_char, (*tsk).comm.as_ptr(), task_pid_nr(tsk), instruction_pointer(regs) as *mut c_void, insn);
    } else if SE_KERNMODE_WARN != 0 {
        pr_notice_ratelimited(b"Fixing up unaligned kernel access in \"%s\" pid=%d pc=0x%p ins=0x%04hx\n\0".as_ptr() as *const c_char, (*tsk).comm.as_ptr(), task_pid_nr(tsk), instruction_pointer(regs) as *mut c_void, insn);
    }
}

static SE_USERMODE_ACTION: [&[u8]; 6] = [b"ignored\0", b"warn\0", b"fixup\0", b"fixup+warn\0", b"signal\0", b"signal+warn\0"];

unsafe extern "C" fn alignment_proc_show(m: *mut SeqFile, _v: *mut c_void) -> c_int {
    seq_printf(m, b"User:\t\t%lu\n\0".as_ptr() as *const c_char, SE_USER);
    seq_printf(m, b"System:\t\t%lu\n\0".as_ptr() as *const c_char, SE_SYS);
    seq_printf(m, b"Half:\t\t%lu\n\0".as_ptr() as *const c_char, SE_HALF);
    seq_printf(m, b"Word:\t\t%lu\n\0".as_ptr() as *const c_char, SE_WORD);
    seq_printf(m, b"DWord:\t\t%lu\n\0".as_ptr() as *const c_char, SE_DWORD);
    seq_printf(m, b"Multi:\t\t%lu\n\0".as_ptr() as *const c_char, SE_MULTI);
    seq_printf(m, b"User faults:\t%i (%s)\n\0".as_ptr() as *const c_char, SE_USERMODE, SE_USERMODE_ACTION[SE_USERMODE as usize].as_ptr());
    seq_printf(m, b"Kernel faults:\t%i (fixup%s)\n\0".as_ptr() as *const c_char, SE_KERNMODE_WARN, if SE_KERNMODE_WARN != 0 { b"+warn\0".as_ptr() } else { b"\0".as_ptr() });
    0
}

unsafe extern "C" fn alignment_proc_open(_inode: *mut Inode, file: *mut File) -> c_int { single_open(file, Some(alignment_proc_show), core::ptr::null_mut()) }

unsafe extern "C" fn alignment_proc_write(file: *mut File, buffer: *const c_char, count: SizeT, _pos: *mut LoFF) -> SsizeT {
    let data = pde_data(file_inode(file)) as *mut c_int;
    if count > 0 {
        let mode = *(buffer as *const u8);
        if mode >= b'0' && mode <= b'5' { *data = (mode - b'0') as c_int; }
    }
    count as SsizeT
}

static ALIGNMENT_PROC_OPS: ProcOps = ProcOps { proc_open: Some(alignment_proc_open), proc_read: Some(seq_read), proc_lseek: Some(seq_lseek), proc_release: Some(single_release), proc_write: Some(alignment_proc_write) };

unsafe extern "C" fn alignment_init() -> c_int {
    let dir = proc_mkdir(b"cpu\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if dir.is_null() { return -ENOMEM; }
    let res = proc_create_data(b"alignment\0".as_ptr() as *const c_char, S_IWUSR | S_IRUGO, dir, &ALIGNMENT_PROC_OPS, &mut SE_USERMODE as *mut _ as *mut c_void);
    if res.is_null() { return -ENOMEM; }
    let res = proc_create_data(b"kernel_alignment\0".as_ptr() as *const c_char, S_IWUSR | S_IRUGO, dir, &ALIGNMENT_PROC_OPS, &mut SE_KERNMODE_WARN as *mut _ as *mut c_void);
    if res.is_null() { return -ENOMEM; }
    0
}

/* C registration declarations: core_param(alignment, se_usermode, int, 0600)
 * and fs_initcall(alignment_init). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
