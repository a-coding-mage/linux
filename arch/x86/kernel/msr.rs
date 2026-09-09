// SPDX-License-Identifier: GPL-2.0-or-later
/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2000-2008 H. Peter Anvin - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author: H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * x86 MSR access device
 *
 * This device is accessed by lseek() to the appropriate register number
 * and then read/write in chunks of 8 bytes.  A larger size means multiple
 * reads or writes of the same register.
 *
 * Writing the same register multiple times can be useful for MSRs with
 * I/O-like semantics, e.g. a virtual MSR that accepts logging information.
 *
 * This driver uses /dev/cpu/%d/msr where %d is the minor number, and on
 * an SMP box will direct the access to CPU %d.
 */

// Dependencies supplied by the kernel and architecture-specific code are
// intentionally left as external symbols.

static mut cpuhp_msr_state: enum_cpuhp_state = 0;

#[repr(C)]
#[derive(Copy, Clone)]
enum allow_write_msrs {
    MSR_WRITES_ON,
    MSR_WRITES_OFF,
    MSR_WRITES_DEFAULT,
}

static mut allow_writes: allow_write_msrs = allow_write_msrs::MSR_WRITES_DEFAULT;

unsafe fn msr_read(file: *mut file, buf: *mut u8, mut count: usize, ppos: *mut loff_t) -> ssize_t {
    let mut tmp = buf as *mut u32;
    let mut data: u64 = 0;
    let reg = *ppos as u32;
    let cpu = iminor(file_inode(file));
    let mut err: i32 = 0;
    let mut bytes: ssize_t = 0;

    if count % 8 != 0 {
        return -EINVAL as ssize_t; // Invalid chunk size
    }

    while count != 0 {
        err = rdmsrq_safe_on_cpu(cpu, reg, &mut data);
        if err != 0 {
            break;
        }
        if copy_to_user(tmp as *mut u8, &data as *const u64 as *const u8, 8) != 0 {
            err = -EFAULT;
            break;
        }
        tmp = tmp.add(2);
        bytes += 8;
        count -= 8;
    }

    if bytes != 0 { bytes } else { err as ssize_t }
}

unsafe fn filter_write(reg: u32) -> i32 {
    /* MSRs writes usually happen all at once, and can easily saturate kmsg. */
    static mut fw_rs: ratelimit_state = ratelimit_state { _private: 0 };

    match allow_writes {
        allow_write_msrs::MSR_WRITES_ON => return 0,
        allow_write_msrs::MSR_WRITES_OFF => return -EPERM,
        allow_write_msrs::MSR_WRITES_DEFAULT => {}
    }

    if __ratelimit(&mut fw_rs) == 0 {
        return 0;
    }
    pr_warn("Write to unrecognized MSR 0x%x by %s (pid: %d), tainting CPU_OUT_OF_SPEC.\n", reg, (*current).comm, (*current).pid);
    pr_warn("See https://git.kernel.org/pub/scm/linux/kernel/git/tip/tip.git/about for details.\n");
    0
}

unsafe fn msr_write(file: *mut file, buf: *const u8, mut count: usize, ppos: *mut loff_t) -> ssize_t {
    let mut tmp = buf as *const u32;
    let mut data: u64 = 0;
    let reg = *ppos as u32;
    let cpu = iminor(file_inode(file));
    let mut err = security_locked_down(LOCKDOWN_MSR);
    let mut bytes: ssize_t = 0;

    if err != 0 { return err as ssize_t; }
    err = filter_write(reg);
    if err != 0 { return err as ssize_t; }
    if count % 8 != 0 { return -EINVAL as ssize_t; }

    while count != 0 {
        if copy_from_user(&mut data as *mut u64 as *mut u8, tmp as *const u8, 8) != 0 {
            err = -EFAULT;
            break;
        }
        add_taint(TAINT_CPU_OUT_OF_SPEC, LOCKDEP_STILL_OK);
        err = wrmsrq_safe_on_cpu(cpu, reg, data);
        if err != 0 { break; }
        tmp = tmp.add(2);
        bytes += 8;
        count -= 8;
    }
    if bytes != 0 { bytes } else { err as ssize_t }
}

unsafe fn msr_ioctl(file: *mut file, ioc: c_uint, arg: c_ulong) -> c_long {
    let uregs = arg as *mut u32;
    let mut regs = [0u32; 8];
    let cpu = iminor(file_inode(file));
    let mut err: i32;

    match ioc {
        X86_IOC_RDMSR_REGS => {
            if (*file).f_mode & FMODE_READ == 0 { return -EBADF as c_long; }
            if copy_from_user(regs.as_mut_ptr() as *mut u8, uregs as *const u8, core::mem::size_of_val(&regs)) != 0 { return -EFAULT as c_long; }
            err = rdmsr_safe_regs_on_cpu(cpu, regs.as_mut_ptr());
            if err != 0 { return err as c_long; }
            if copy_to_user(uregs as *mut u8, regs.as_ptr() as *const u8, core::mem::size_of_val(&regs)) != 0 { -EFAULT as c_long } else { 0 }
        }
        X86_IOC_WRMSR_REGS => {
            if (*file).f_mode & FMODE_WRITE == 0 { return -EBADF as c_long; }
            if copy_from_user(regs.as_mut_ptr() as *mut u8, uregs as *const u8, core::mem::size_of_val(&regs)) != 0 { return -EFAULT as c_long; }
            err = security_locked_down(LOCKDOWN_MSR);
            if err != 0 { return err as c_long; }
            err = filter_write(regs[1]);
            if err != 0 { return err as c_long; }
            add_taint(TAINT_CPU_OUT_OF_SPEC, LOCKDEP_STILL_OK);
            err = wrmsr_safe_regs_on_cpu(cpu, regs.as_mut_ptr());
            if err != 0 { return err as c_long; }
            if copy_to_user(uregs as *mut u8, regs.as_ptr() as *const u8, core::mem::size_of_val(&regs)) != 0 { -EFAULT as c_long } else { 0 }
        }
        _ => -ENOTTY as c_long,
    }
}

unsafe fn msr_open(inode: *mut inode, _file: *mut file) -> i32 {
    let cpu = iminor(file_inode_from_inode(inode));
    if capable(CAP_SYS_RAWIO) == 0 { return -EPERM; }
    if cpu >= nr_cpu_ids || cpu_online(cpu) == 0 { return -ENXIO; }
    let c = &mut cpu_data(cpu);
    if !cpu_has(c, X86_FEATURE_MSR) { return -EIO; }
    0
}

// File operations and device/module registration are supplied by the kernel ABI.
// The C initializer maps directly to the corresponding external file_operations.
extern "C" {
    static msr_fops: file_operations;
    fn msr_init() -> i32;
    fn msr_exit();
}

unsafe fn set_allow_writes(val: *const c_char, _cp: *const kernel_param) -> i32 {
    let s = strstrip(val as *mut c_char);
    if strcmp(s, b"on\0".as_ptr() as *const c_char) == 0 { allow_writes = allow_write_msrs::MSR_WRITES_ON; }
    else if strcmp(s, b"off\0".as_ptr() as *const c_char) == 0 { allow_writes = allow_write_msrs::MSR_WRITES_OFF; }
    else { allow_writes = allow_write_msrs::MSR_WRITES_DEFAULT; }
    0
}

unsafe fn get_allow_writes(buf: *mut c_char, _kp: *const kernel_param) -> i32 {
    let res = match allow_writes {
        allow_write_msrs::MSR_WRITES_ON => b"on\0",
        allow_write_msrs::MSR_WRITES_OFF => b"off\0",
        allow_write_msrs::MSR_WRITES_DEFAULT => b"default\0",
    };
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, res.as_ptr());
    0
}

// MODULE_AUTHOR("H. Peter Anvin <hpa@zytor.com>");
// MODULE_DESCRIPTION("x86 generic MSR driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
