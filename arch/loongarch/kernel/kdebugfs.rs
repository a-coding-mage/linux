// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and LoongArch architecture headers.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
    pub open: Option<unsafe extern "C" fn(*mut file, *mut c_void) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
}

extern "C" {
    fn csr_xchg32(val: i32, mask: u32, reg: u32);
    fn csr_read32(reg: u32) -> u32;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn kstrtoint_from_user(buf: *const c_char, count: usize, base: u32, res: *mut i32) -> c_int;
    fn copy_to_user(to: *mut c_char, from: *const c_char, n: usize) -> usize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn simple_open(file: *mut file, inode: *mut c_void) -> c_int;
    fn default_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn read_cpucfg(reg: u32) -> u32;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
}

#[no_mangle]
pub static mut arch_debugfs_dir: *mut dentry = core::ptr::null_mut();

static mut SFB_STATE: i32 = 0;
static mut TSO_STATE: i32 = 0;

unsafe extern "C" fn set_sfb_state(info: *mut c_void) {
    let val = (*(info as *mut i32)).wrapping_shl(CSR_STFILL_SHIFT);
    csr_xchg32(val, CSR_STFILL, LOONGARCH_CSR_IMPCTL1);
}

unsafe extern "C" fn sfb_read(
    _file: *mut file,
    buf: *mut c_char,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let state = ((csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_STFILL) >> CSR_STFILL_SHIFT) as i32;
    let mut str_buf = [0i8; 32];
    let s = snprintf(
        str_buf.as_mut_ptr(),
        str_buf.len(),
        b"Boot State: %x\nCurrent State: %x\n\0".as_ptr() as *const c_char,
        SFB_STATE,
        state,
    );
    if *ppos >= s as i64 {
        return 0;
    }
    let remaining = (s as i64 - *ppos) as usize;
    let n = core::cmp::min(remaining, count);
    if copy_to_user(buf, str_buf.as_ptr().add(*ppos as usize), n) != 0 {
        return -14;
    }
    *ppos += n as i64;
    n as isize
}

unsafe extern "C" fn sfb_write(
    _file: *mut file,
    buf: *const c_char,
    count: usize,
    _ppos: *mut i64,
) -> isize {
    let mut state = 0i32;
    if kstrtoint_from_user(buf, count, 10, &mut state) != 0 {
        return -14;
    }
    match state {
        0 | 1 => on_each_cpu(set_sfb_state, &mut state as *mut _ as *mut c_void, 1),
        _ => return -22,
    }
    count as isize
}

static SFB_FOPS: file_operations = file_operations {
    read: Some(sfb_read), write: Some(sfb_write), open: Some(simple_open), llseek: Some(default_llseek),
};

static TSO_HINTS: [&[u8]; 8] = [
    b"No Load No Store", b"All Load No Store", b"Invalid Config", b"Same Load No Store",
    b"No Load All Store", b"All Load All Store", b"Invalid Config", b"Same Load All Store",
];

unsafe extern "C" fn set_tso_state(info: *mut c_void) {
    let val = (*(info as *mut i32)).wrapping_shl(CSR_LDSTORDER_SHIFT);
    csr_xchg32(val, CSR_LDSTORDER_MASK, LOONGARCH_CSR_IMPCTL1);
}

unsafe extern "C" fn tso_read(_file: *mut file, buf: *mut c_char, count: usize, ppos: *mut i64) -> isize {
    let state = ((csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_LDSTORDER_MASK) >> CSR_LDSTORDER_SHIFT) as usize;
    let mut str_buf = [0i8; 240];
    let s = snprintf(
        str_buf.as_mut_ptr(), str_buf.len(),
        b"Boot State: %d (%s)\nCurrent State: %d (%s)\n\nAvailable States:\n0 (%s)\t1 (%s)\t3 (%s)\n4 (%s)\t5 (%s)\t7 (%s)\n\0".as_ptr() as *const c_char,
        TSO_STATE, TSO_HINTS[TSO_STATE as usize].as_ptr(), state as i32, TSO_HINTS[state].as_ptr(),
        TSO_HINTS[0].as_ptr(), TSO_HINTS[1].as_ptr(), TSO_HINTS[3].as_ptr(),
        TSO_HINTS[4].as_ptr(), TSO_HINTS[5].as_ptr(), TSO_HINTS[7].as_ptr(),
    );
    if *ppos >= s as i64 { return 0; }
    let n = core::cmp::min((s as i64 - *ppos) as usize, count);
    if copy_to_user(buf, str_buf.as_ptr().add(*ppos as usize), n) != 0 { return -14; }
    *ppos += n as i64;
    n as isize
}

unsafe extern "C" fn tso_write(_file: *mut file, buf: *const c_char, count: usize, _ppos: *mut i64) -> isize {
    let mut state = 0i32;
    if kstrtoint_from_user(buf, count, 10, &mut state) != 0 { return -14; }
    match state {
        0 | 1 | 3 | 4 | 5 | 7 => on_each_cpu(set_tso_state, &mut state as *mut _ as *mut c_void, 1),
        _ => return -22,
    }
    count as isize
}

static TSO_FOPS: file_operations = file_operations {
    read: Some(tso_read), write: Some(tso_write), open: Some(simple_open), llseek: Some(default_llseek),
};

unsafe extern "C" fn arch_kdebugfs_init() -> c_int {
    let config = read_cpucfg(LOONGARCH_CPUCFG3);
    arch_debugfs_dir = debugfs_create_dir(b"loongarch\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if config & CPUCFG3_SFB != 0 {
        debugfs_create_file(b"sfb_state\0".as_ptr() as *const c_char, S_IRUGO | S_IWUSR, arch_debugfs_dir, &mut SFB_STATE as *mut _ as *mut c_void, &SFB_FOPS);
        SFB_STATE = ((csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_STFILL) >> CSR_STFILL_SHIFT) as i32;
    }
    if config & (CPUCFG3_ALDORDER_CAP | CPUCFG3_ASTORDER_CAP) != 0 {
        debugfs_create_file(b"tso_state\0".as_ptr() as *const c_char, S_IRUGO | S_IWUSR, arch_debugfs_dir, &mut TSO_STATE as *mut _ as *mut c_void, &TSO_FOPS);
        TSO_STATE = ((csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_LDSTORDER_MASK) >> CSR_LDSTORDER_SHIFT) as i32;
    }
    0
}

// postcore_initcall(arch_kdebugfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
