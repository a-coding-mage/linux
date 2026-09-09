// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of nvram.c; kernel-provided symbols remain external. */

const NVRAM_VERSION: &str = "1.3";

// C includes and build-system configuration are supplied by the surrounding kernel crate.

static mut NVRAM_OPEN_CNT: i32 = 0;
static mut NVRAM_OPEN_MODE: i32 = 0;
static mut NVRAM_SIZE: isize = 0;
const NVRAM_WRITE: i32 = 1;
const NVRAM_EXCL: i32 = 2;

#[cfg(target_arch = "x86")]
const NVRAM_BYTES: i32 = 128 - NVRAM_FIRST_BYTE;

#[cfg(target_arch = "x86")]
unsafe fn __nvram_read_byte(i: i32) -> u8 {
    CMOS_READ(NVRAM_FIRST_BYTE + i)
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_read_byte(i: i32) -> u8 {
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut rtc_lock, &mut flags);
    let c = __nvram_read_byte(i);
    spin_unlock_irqrestore(&raw mut rtc_lock, flags);
    c
}

#[cfg(target_arch = "x86")]
unsafe fn __nvram_write_byte(c: u8, i: i32) {
    CMOS_WRITE(c, NVRAM_FIRST_BYTE + i);
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_write_byte(c: u8, i: i32) {
    let mut flags: usize = 0;
    spin_lock_irqsave(&raw mut rtc_lock, &mut flags);
    __nvram_write_byte(c, i);
    spin_unlock_irqrestore(&raw mut rtc_lock, flags);
}

#[cfg(target_arch = "x86")]
const PC_CKS_RANGE_START: i32 = 2;
#[cfg(target_arch = "x86")]
const PC_CKS_RANGE_END: i32 = 31;
#[cfg(target_arch = "x86")]
const PC_CKS_LOC: i32 = 32;

#[cfg(target_arch = "x86")]
unsafe fn __nvram_check_checksum() -> bool {
    let mut sum: u16 = 0;
    let mut i = PC_CKS_RANGE_START;
    while i <= PC_CKS_RANGE_END {
        sum = sum.wrapping_add(__nvram_read_byte(i) as u16);
        i += 1;
    }
    let expect = ((__nvram_read_byte(PC_CKS_LOC) as u16) << 8)
        | __nvram_read_byte(PC_CKS_LOC + 1) as u16;
    (sum & 0xffff) == expect
}

#[cfg(target_arch = "x86")]
unsafe fn __nvram_set_checksum() {
    let mut sum: u16 = 0;
    let mut i = PC_CKS_RANGE_START;
    while i <= PC_CKS_RANGE_END {
        sum = sum.wrapping_add(__nvram_read_byte(i) as u16);
        i += 1;
    }
    __nvram_write_byte((sum >> 8) as u8, PC_CKS_LOC);
    __nvram_write_byte((sum & 0xff) as u8, PC_CKS_LOC + 1);
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_set_checksum() -> isize {
    spin_lock_irq(&raw mut rtc_lock);
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    0
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_initialize() -> isize {
    spin_lock_irq(&raw mut rtc_lock);
    let mut i: isize = 0;
    while i < NVRAM_BYTES as isize {
        __nvram_write_byte(0, i as i32);
        i += 1;
    }
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    0
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_get_size() -> isize { NVRAM_BYTES as isize }

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_read(buf: *mut u8, count: usize, ppos: *mut i64) -> isize {
    spin_lock_irq(&raw mut rtc_lock);
    if !__nvram_check_checksum() {
        spin_unlock_irq(&raw mut rtc_lock);
        return -EIO;
    }
    let mut i = *ppos;
    let mut n = count;
    while n > 0 && i < NVRAM_BYTES as i64 {
        *buf.offset((count - n) as isize) = __nvram_read_byte(i as i32);
        n -= 1;
        i += 1;
    }
    spin_unlock_irq(&raw mut rtc_lock);
    *ppos = i;
    (count - n) as isize
}

#[cfg(target_arch = "x86")]
unsafe fn pc_nvram_write(buf: *const u8, count: usize, ppos: *mut i64) -> isize {
    spin_lock_irq(&raw mut rtc_lock);
    if !__nvram_check_checksum() {
        spin_unlock_irq(&raw mut rtc_lock);
        return -EIO;
    }
    let mut i = *ppos;
    let mut n = count;
    while n > 0 && i < NVRAM_BYTES as i64 {
        __nvram_write_byte(*buf.offset((count - n) as isize), i as i32);
        n -= 1;
        i += 1;
    }
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    *ppos = i;
    (count - n) as isize
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct nvram_ops {
    pub read: Option<unsafe fn(*mut u8, usize, *mut i64) -> isize>,
    pub write: Option<unsafe fn(*const u8, usize, *mut i64) -> isize>,
    pub read_byte: Option<unsafe fn(i32) -> u8>,
    pub write_byte: Option<unsafe fn(u8, i32)>,
    pub get_size: Option<unsafe fn() -> isize>,
    pub set_checksum: Option<unsafe fn() -> isize>,
    pub initialize: Option<unsafe fn() -> isize>,
}

#[cfg(target_arch = "x86")]
#[no_mangle]
pub static arch_nvram_ops: nvram_ops = nvram_ops {
    read: Some(pc_nvram_read), write: Some(pc_nvram_write),
    read_byte: Some(pc_nvram_read_byte), write_byte: Some(pc_nvram_write_byte),
    get_size: Some(pc_nvram_get_size), set_checksum: Some(pc_nvram_set_checksum),
    initialize: Some(pc_nvram_initialize),
};

// The remaining file operations and module registration retain their C kernel ABI;
// referenced kernel types/functions are intentionally left as external dependencies.
unsafe fn nvram_misc_llseek(file: *mut file, offset: i64, origin: i32) -> i64 {
    generic_file_llseek_size(file, offset, origin, MAX_LFS_FILESIZE, NVRAM_SIZE)
}

unsafe fn nvram_misc_read(file: *mut file, buf: *mut u8, mut count: usize, ppos: *mut i64) -> isize {
    if *ppos >= NVRAM_SIZE { return 0; }
    count = core::cmp::min(count, (NVRAM_SIZE - *ppos) as usize);
    count = core::cmp::min(count, PAGE_SIZE);
    let tmp = kmalloc(count, GFP_KERNEL);
    if tmp.is_null() { return -ENOMEM; }
    let mut ret = nvram_read(tmp, count, ppos);
    if ret > 0 && copy_to_user(buf, tmp, ret as usize) != 0 {
        *ppos -= ret; ret = -EFAULT;
    }
    kfree(tmp);
    ret
}

unsafe fn nvram_misc_write(file: *mut file, buf: *const u8, mut count: usize, ppos: *mut i64) -> isize {
    if *ppos >= NVRAM_SIZE { return 0; }
    count = core::cmp::min(count, (NVRAM_SIZE - *ppos) as usize);
    count = core::cmp::min(count, PAGE_SIZE);
    let tmp = memdup_user(buf, count);
    if is_err(tmp) { return ptr_err(tmp); }
    let ret = nvram_write(tmp, count, ppos);
    kfree(tmp);
    ret
}

unsafe fn nvram_misc_open(inode: *mut inode, file: *mut file) -> i32 {
    spin_lock(&raw mut nvram_state_lock);
    if (NVRAM_OPEN_CNT != 0 && ((*file).f_flags & O_EXCL) != 0)
        || (NVRAM_OPEN_MODE & NVRAM_EXCL) != 0 {
        spin_unlock(&raw mut nvram_state_lock); return -EBUSY;
    }
    if (arch_nvram_ops.set_checksum.is_some()) && ((*file).f_mode & FMODE_WRITE) != 0
        && (NVRAM_OPEN_MODE & NVRAM_WRITE) != 0 {
        spin_unlock(&raw mut nvram_state_lock); return -EBUSY;
    }
    if ((*file).f_flags & O_EXCL) != 0 { NVRAM_OPEN_MODE |= NVRAM_EXCL; }
    if ((*file).f_mode & FMODE_WRITE) != 0 { NVRAM_OPEN_MODE |= NVRAM_WRITE; }
    NVRAM_OPEN_CNT += 1;
    spin_unlock(&raw mut nvram_state_lock);
    0
}

unsafe fn nvram_misc_release(inode: *mut inode, file: *mut file) -> i32 {
    spin_lock(&raw mut nvram_state_lock);
    NVRAM_OPEN_CNT -= 1;
    if (NVRAM_OPEN_MODE & NVRAM_EXCL) != 0 { NVRAM_OPEN_MODE &= !NVRAM_EXCL; }
    if ((*file).f_mode & FMODE_WRITE) != 0 { NVRAM_OPEN_MODE &= !NVRAM_WRITE; }
    spin_unlock(&raw mut nvram_state_lock);
    0
}

unsafe fn nvram_module_init() -> i32 {
    NVRAM_SIZE = nvram_get_size();
    if NVRAM_SIZE < 0 { return NVRAM_SIZE as i32; }
    let ret = misc_register(&raw mut nvram_misc);
    if ret != 0 { pr_err("nvram: can't misc_register on minor=%d\n", NVRAM_MINOR); return ret; }
    pr_info("Non-volatile memory driver v{}\n", NVRAM_VERSION);
    0
}

unsafe fn nvram_module_exit() { misc_deregister(&raw mut nvram_misc); }

// module_init/module_exit and metadata are supplied by the kernel module integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
