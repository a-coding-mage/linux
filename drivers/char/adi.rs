// SPDX-License-Identifier: GPL-2.0
/*
 * Privileged ADI driver for sparc64
 *
 * Author: Tom Hromatka <tom.hromatka@oracle.com>
 */

// Linux kernel dependencies supplied by other translation units.

const MAX_BUF_SZ: usize = PAGE_SIZE;

unsafe fn read_mcd_tag(addr: ::core::ffi::c_ulong) -> i32 {
    let mut err: isize;
    let mut ver: i32;

    core::arch::asm!(
        "1: ldxa [{addr}] {asi}, {ver}",
        "   mov 0, {err}",
        "2:",
        "   .section .fixup,#alloc,#execinstr",
        "   .align 4",
        "3: sethi %hi(2b), %g1",
        "   jmpl %g1 + %lo(2b), %g0",
        "   mov {invalid}, {err}",
        "   .previous",
        "   .section __ex_table, \"a\"",
        "   .align 4",
        "   .word 1b, 3b",
        "   .previous",
        addr = in(reg) addr,
        ver = out(reg) ver,
        err = out(reg) err,
        invalid = const EFAULT,
        asi = const ASI_MCD_REAL,
        lateout("g1") _,
        options(nostack)
    );

    if err != 0 { -EFAULT } else { ver }
}

unsafe fn adi_read(
    file: *mut file,
    buf: *mut u8,
    count: usize,
    offp: *mut loff_t,
) -> isize {
    let mut ver_buf_sz: usize;
    let mut bytes_read: usize = 0;
    let mut ver_buf_idx: i32 = 0;
    let mut offset: loff_t;
    let ver_buf: *mut u8;
    let mut ret: isize;

    ver_buf_sz = core::cmp::min(count, MAX_BUF_SZ);
    ver_buf = kmalloc(ver_buf_sz, GFP_KERNEL);
    if ver_buf.is_null() { return -ENOMEM; }

    offset = (*offp) * adi_blksize();

    while bytes_read < count {
        ret = read_mcd_tag(offset as ::core::ffi::c_ulong) as isize;
        if ret < 0 {
            kfree(ver_buf);
            return ret;
        }

        *ver_buf.add(ver_buf_idx as usize) = ret as u8;
        ver_buf_idx += 1;
        offset += adi_blksize();

        if ver_buf_idx as usize >= ver_buf_sz {
            if copy_to_user(buf.add(bytes_read), ver_buf, ver_buf_sz) != 0 {
                ret = -EFAULT;
                kfree(ver_buf);
                return ret;
            }

            bytes_read += ver_buf_sz;
            ver_buf_idx = 0;
            ver_buf_sz = core::cmp::min(count - bytes_read, MAX_BUF_SZ);
        }
    }

    *offp += bytes_read as loff_t;
    ret = bytes_read as isize;
    kfree(ver_buf);
    ret
}

unsafe fn set_mcd_tag(addr: ::core::ffi::c_ulong, ver: u8) -> i32 {
    let mut err: isize;

    core::arch::asm!(
        "1: stxa {ver}, [{addr}] {asi}",
        "   mov 0, {err}",
        "2:",
        "   .section .fixup,#alloc,#execinstr",
        "   .align 4",
        "3: sethi %hi(2b), %g1",
        "   jmpl %g1 + %lo(2b), %g0",
        "   mov {invalid}, {err}",
        "   .previous",
        "   .section __ex_table, \"a\"",
        "   .align 4",
        "   .word 1b, 3b",
        "   .previous",
        ver = in(reg) ver,
        addr = in(reg) addr,
        err = out(reg) err,
        invalid = const EFAULT,
        asi = const ASI_MCD_REAL,
        lateout("g1") _,
        options(nostack)
    );

    if err != 0 { -EFAULT } else { ver as i32 }
}

unsafe fn adi_write(
    file: *mut file,
    buf: *const u8,
    count: usize,
    offp: *mut loff_t,
) -> isize {
    if count == 0 { return -EINVAL; }

    let mut ver_buf_sz = core::cmp::min(count, MAX_BUF_SZ);
    let mut bytes_written: usize = 0;
    let mut offset = (*offp) * adi_blksize();
    let ver_buf = kmalloc(ver_buf_sz, GFP_KERNEL);
    if ver_buf.is_null() { return -ENOMEM; }
    let mut ret: isize;

    loop {
        if copy_from_user(ver_buf, buf.add(bytes_written), ver_buf_sz) != 0 {
            ret = -EFAULT;
            break;
        }

        for i in 0..ver_buf_sz {
            ret = set_mcd_tag(offset as ::core::ffi::c_ulong, *ver_buf.add(i)) as isize;
            if ret < 0 { break; }
            offset += adi_blksize();
        }
        if ret < 0 { break; }

        bytes_written += ver_buf_sz;
        ver_buf_sz = core::cmp::min(count - bytes_written, MAX_BUF_SZ);
        if bytes_written >= count {
            *offp += bytes_written as loff_t;
            ret = bytes_written as isize;
            break;
        }
    }

    core::arch::asm!("membar #Sync", options(nostack));
    kfree(ver_buf);
    ret
}

unsafe fn adi_llseek(file: *mut file, mut offset: loff_t, whence: i32) -> loff_t {
    let mut ret = -EINVAL;

    match whence {
        SEEK_END | SEEK_DATA | SEEK_HOLE => return -EINVAL,
        SEEK_CUR => {
            if offset == 0 { return (*file).f_pos; }
            offset += (*file).f_pos;
        }
        SEEK_SET => {}
        _ => {}
    }

    if offset != (*file).f_pos {
        (*file).f_pos = offset;
        ret = offset;
    }
    ret
}

static adi_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    llseek: Some(adi_llseek),
    read: Some(adi_read),
    write: Some(adi_write),
    fop_flags: FOP_UNSIGNED_OFFSET,
};

static mut adi_miscdev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: KBUILD_MODNAME,
    fops: &adi_fops,
};

unsafe fn adi_init() -> i32 {
    if !adi_capable() { return -EPERM; }
    misc_register(&mut adi_miscdev)
}

unsafe fn adi_exit() {
    misc_deregister(&mut adi_miscdev);
}

module_init!(adi_init);
module_exit!(adi_exit);

module_author!("Tom Hromatka <tom.hromatka@oracle.com>");
module_description!("Privileged interface to ADI");
module_version!("1.0");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
