// SPDX-License-Identifier: GPL-2.0
/*
 * Access to PCI I/O memory from user space programs.
 *
 * Copyright IBM Corp. 2014
 * Author(s): Alexey Ishchuk <aishchuk@linux.vnet.ibm.com>
 */

// Kernel and s390 architecture dependencies supplied by the surrounding tree.

#[inline]
unsafe fn zpci_err_mmio(cc: u8, status: u8, offset: u64) {
    #[repr(C)]
    struct Data {
        offset: u64,
        cc: u8,
        status: u8,
    }
    let data = Data { offset, cc, status };
    zpci_err_hex((&data as *const Data).cast(), core::mem::size_of::<Data>());
}

#[inline]
unsafe fn __pcistb_mio_inuser(
    ioaddr: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut len: u64,
    status: *mut u8,
) -> i32 {
    let mut cc: i32;
    let mut exception: i32 = 1;
    let sacf_flag = enable_sacf_uaccess();
    // s390 PCISTB user-access sequence, including exception-table fixups.
    core::arch::asm!(
        "sacf 256",
        "0: .insn rsy,0xeb00000000d4,{len},{ioaddr},{src}",
        "1: lhi {exc},0",
        "2: sacf 768",
        len = inout(reg) len,
        ioaddr = in(reg) ioaddr,
        src = in(reg) src,
        exc = inout(reg) exception,
        lateout("r0") cc,
        options(nostack)
    );
    disable_sacf_uaccess(sacf_flag);
    *status = ((len >> 24) & 0xff) as u8;
    if exception != 0 { -ENXIO } else { CC_TRANSFORM(cc) }
}

#[inline]
unsafe fn __pcistg_mio_inuser(
    ioaddr: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    ulen: u64,
    status: *mut u8,
) -> i32 {
    let mut ioaddr_len = RegisterPair { even: ioaddr as u64, odd: ulen };
    let mut cc: i32;
    let mut exception: i32 = 1;
    let sacf_flag = enable_sacf_uaccess();
    let mut val: u64 = 0;
    let mut cnt = ulen;
    let mut tmp: u8;
    // Copy 0 < len <= 8 bytes from src into the rightmost bytes of a
    // register, then store it to PCI at ioaddr in secondary address space.
    core::arch::asm!(
        "sacf 256",
        "0: llgc {tmp},0({src})",
        "4: sllg {val},{val},8",
        "aghi {src},1",
        "ogr {val},{tmp}",
        "brctg {cnt},0b",
        "1: .insn rre,0xb9d40000,{val},{ioaddr_len}",
        "2: lhi {exc},0",
        "3: sacf 768",
        src = inout(reg) src => _, cnt = inout(reg) cnt,
        val = inout(reg) val, tmp = lateout(reg) tmp,
        exc = inout(reg) exception, ioaddr_len = inout(reg) ioaddr_len.pair,
        lateout("r0") cc, options(nostack)
    );
    disable_sacf_uaccess(sacf_flag);
    *status = ((ioaddr_len.odd >> 24) & 0xff) as u8;
    cc = if exception != 0 { -ENXIO } else { CC_TRANSFORM(cc) };
    if cc == 0 && cnt != 0 { -EFAULT } else { cc }
}

#[inline]
unsafe fn __memcpy_toio_inuser(mut dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, mut n: usize) -> i32 {
    let mut rc = 0;
    let mut status: u8 = 0;
    if src.is_null() { return -EINVAL; }
    while n > 0 {
        let size = zpci_get_max_io_size(dst as u64, src as u64, n, ZPCI_MAX_WRITE_SIZE);
        rc = if size > 8 { __pcistb_mio_inuser(dst, src, size as u64, &mut status) }
             else { __pcistg_mio_inuser(dst, src, size as u64, &mut status) };
        if rc != 0 { break; }
        src = src.add(size); dst = dst.add(size); n -= size;
    }
    if rc != 0 { zpci_err_mmio(rc as u8, status, dst as u64); }
    rc
}

pub unsafe fn s390_pci_mmio_write(mmio_addr: usize, user_buffer: *const core::ffi::c_void, length: usize) -> isize {
    let mut args = FollowPfnmapArgs::default();
    let mut local_buf = [0u8; 64];
    let mut io_addr: *mut core::ffi::c_void;
    let mut buf: *mut core::ffi::c_void;
    let mut vma: *mut VmAreaStruct;
    let mut ret: isize;
    if !zpci_is_enabled() { return -ENODEV as isize; }
    if length == 0 || PAGE_SIZE - (mmio_addr & !PAGE_MASK) < length { return -EINVAL as isize; }
    if static_branch_likely(&have_mio) { return __memcpy_toio_inuser(mmio_addr as *mut _, user_buffer, length) as isize; }
    if length > 64 { buf = kmalloc(length, GFP_KERNEL); if buf.is_null() { return -ENOMEM as isize; } } else { buf = local_buf.as_mut_ptr().cast(); }
    ret = -EFAULT as isize;
    if copy_from_user(buf, user_buffer, length) != 0 { goto_out_free!(buf, local_buf, ret); }
    mmap_read_lock(current_mm());
    ret = -EINVAL as isize;
    vma = vma_lookup(current_mm(), mmio_addr);
    if vma.is_null() || ((*vma).vm_flags & (VM_IO | VM_PFNMAP)) == 0 { goto_out_unlock_mmap!(buf, local_buf, ret); }
    ret = -EACCES as isize;
    if ((*vma).vm_flags & VM_WRITE) == 0 { goto_out_unlock_mmap!(buf, local_buf, ret); }
    args.address = mmio_addr; args.vma = vma;
    ret = follow_pfnmap_start(&mut args);
    if ret != 0 { fixup_user_fault(current_mm(), mmio_addr, FAULT_FLAG_WRITE, core::ptr::null_mut()); ret = follow_pfnmap_start(&mut args); if ret != 0 { goto_out_unlock_mmap!(buf, local_buf, ret); } }
    ret = -EFAULT as isize;
    io_addr = (((args.pfn << PAGE_SHIFT) | (mmio_addr & !PAGE_MASK)) as *mut core::ffi::c_void);
    if io_addr as usize >= ZPCI_IOMAP_ADDR_BASE { ret = zpci_memcpy_toio(io_addr, buf, length); }
    follow_pfnmap_end(&mut args); mmap_read_unlock(current_mm());
    if buf != local_buf.as_mut_ptr().cast() { kfree(buf); } ret
}

#[inline]
unsafe fn __pcilg_mio_inuser(dst: *mut core::ffi::c_void, ioaddr: *const core::ffi::c_void, ulen: u64, status: *mut u8) -> i32 {
    let mut ioaddr_len = RegisterPair { even: ioaddr as u64, odd: ulen };
    let mut exception = 1i32;
    let mut cnt = ulen;
    let mut shift = (ulen * 8) as i32;
    let mut cc: i32;
    let mut val: u64;
    let mut tmp: u64;
    let sacf_flag = enable_sacf_uaccess();
    // s390 PCILG user-access sequence, including exception-table fixups.
    core::arch::asm!(
        "sacf 256",
        "0: .insn rre,0xb9d60000,{val},{ioaddr_len}",
        "1: lhi {exc},0", "jne 4f", "2: ahi {shift},-8",
        "srlg {tmp},{val},0({shift})", "3: stc {tmp},0({dst})",
        "5: aghi {dst},1", "brctg {cnt},2b", "xr {exc},{exc}",
        "4: sacf 768",
        ioaddr_len = inout(reg) ioaddr_len.pair, exc = inout(reg) exception,
        val = lateout(reg) val, dst = inout(reg) dst, cnt = inout(reg) cnt,
        tmp = lateout(reg) tmp, shift = inout(reg) shift, lateout("r0") cc,
        options(nostack)
    );
    disable_sacf_uaccess(sacf_flag);
    cc = if exception != 0 { -ENXIO } else { CC_TRANSFORM(cc) };
    if cc == 0 && cnt != 0 { cc = -EFAULT; }
    *status = ((ioaddr_len.odd >> 24) & 0xff) as u8;
    cc
}

#[inline]
unsafe fn __memcpy_fromio_inuser(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void, mut n: usize) -> i32 {
    let mut rc = 0; let mut status: u8 = 0;
    while n > 0 {
        let size = zpci_get_max_io_size(src as u64, dst as u64, n, ZPCI_MAX_READ_SIZE);
        rc = __pcilg_mio_inuser(dst, src, size as u64, &mut status);
        if rc != 0 { break; }
        src = src.add(size); dst = dst.add(size); n -= size;
    }
    if rc != 0 { zpci_err_mmio(rc as u8, status, dst as u64); } rc
}

pub unsafe fn s390_pci_mmio_read(mmio_addr: usize, user_buffer: *mut core::ffi::c_void, length: usize) -> isize {
    let mut args = FollowPfnmapArgs::default(); let mut local_buf = [0u8; 64];
    let mut buf: *mut core::ffi::c_void; let mut vma: *mut VmAreaStruct; let mut ret: isize;
    if !zpci_is_enabled() { return -ENODEV as isize; }
    if length == 0 || PAGE_SIZE - (mmio_addr & !PAGE_MASK) < length { return -EINVAL as isize; }
    if static_branch_likely(&have_mio) { return __memcpy_fromio_inuser(user_buffer, mmio_addr as *const _, length) as isize; }
    if length > 64 { buf = kmalloc(length, GFP_KERNEL); if buf.is_null() { return -ENOMEM as isize; } } else { buf = local_buf.as_mut_ptr().cast(); }
    mmap_read_lock(current_mm()); ret = -EINVAL as isize; vma = vma_lookup(current_mm(), mmio_addr);
    if vma.is_null() || ((*vma).vm_flags & (VM_IO | VM_PFNMAP)) == 0 { goto_out_unlock_mmap!(buf, local_buf, ret); }
    ret = -EACCES as isize; if ((*vma).vm_flags & VM_READ) == 0 { goto_out_unlock_mmap!(buf, local_buf, ret); }
    args.vma = vma; args.address = mmio_addr; ret = follow_pfnmap_start(&mut args);
    if ret != 0 { fixup_user_fault(current_mm(), mmio_addr, 0, core::ptr::null_mut()); ret = follow_pfnmap_start(&mut args); if ret != 0 { goto_out_unlock_mmap!(buf, local_buf, ret); } }
    let io_addr = ((args.pfn << PAGE_SHIFT) | (mmio_addr & !PAGE_MASK)) as *mut core::ffi::c_void;
    if io_addr as usize < ZPCI_IOMAP_ADDR_BASE { ret = -EFAULT as isize; } else { ret = zpci_memcpy_fromio(buf, io_addr, length); }
    follow_pfnmap_end(&mut args); mmap_read_unlock(current_mm());
    if ret == 0 && copy_to_user(user_buffer, buf, length) != 0 { ret = -EFAULT as isize; }
    if buf != local_buf.as_mut_ptr().cast() { kfree(buf); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
