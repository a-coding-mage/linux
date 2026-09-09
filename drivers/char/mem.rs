// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/drivers/char/mem.c. Kernel includes and configuration
 * symbols are supplied by the surrounding Rust translation. */

const DEVMEM_MINOR: i32 = 1;
const DEVPORT_MINOR: i32 = 4;

#[inline]
unsafe fn size_inside_page(start: usize, size: usize) -> usize {
    let sz = PAGE_SIZE - (start & (PAGE_SIZE - 1));
    core::cmp::min(sz, size)
}

#[cfg(not(ARCH_HAS_VALID_PHYS_ADDR_RANGE))]
#[inline]
unsafe fn valid_phys_addr_range(addr: phys_addr_t, count: usize) -> i32 {
    (addr.wrapping_add(count as phys_addr_t) <= __pa(high_memory)) as i32
}

#[cfg(not(ARCH_HAS_VALID_PHYS_ADDR_RANGE))]
#[inline]
unsafe fn valid_mmap_phys_addr_range(_pfn: usize, _size: usize) -> i32 { 1 }

#[cfg(CONFIG_STRICT_DEVMEM)]
#[inline]
unsafe fn page_is_allowed(pfn: usize) -> i32 { devmem_is_allowed(pfn) }

#[cfg(not(CONFIG_STRICT_DEVMEM))]
#[inline]
unsafe fn page_is_allowed(_pfn: usize) -> i32 { 1 }

#[inline]
unsafe fn should_stop_iteration() -> bool {
    if need_resched() != 0 { cond_resched(); }
    signal_pending(current) != 0
}

unsafe fn read_mem(file: *mut file, mut buf: *mut u8, mut count: usize, ppos: *mut loff_t) -> isize {
    let mut p: phys_addr_t = *ppos as phys_addr_t;
    let mut read: isize = 0;
    let mut sz: usize;
    let mut ptr: *mut core::ffi::c_void;
    let bounce = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut u8;
    if p != *ppos as phys_addr_t { return 0; }
    if valid_phys_addr_range(p, count) == 0 { return -EFAULT; }
    #[cfg(__ARCH_HAS_NO_PAGE_ZERO_MAPPED)]
    if p < PAGE_SIZE as phys_addr_t {
        sz = size_inside_page(p as usize, count);
        if sz > 0 {
            if clear_user(buf, sz) != 0 { return -EFAULT; }
            buf = buf.add(sz); p += sz as phys_addr_t; count -= sz; read += sz as isize;
        }
    }
    if bounce.is_null() { return -ENOMEM; }
    while count > 0 {
        let allowed;
        let remaining;
        sz = size_inside_page(p as usize, count);
        allowed = page_is_allowed((p >> PAGE_SHIFT) as usize);
        if allowed == 0 { kfree(bounce as *mut _); return -EPERM; }
        if allowed == 2 { remaining = clear_user(buf, sz); }
        else {
            ptr = xlate_dev_mem_ptr(p);
            if ptr.is_null() { kfree(bounce as *mut _); return -EFAULT; }
            let probe = copy_from_kernel_nofault(bounce as *mut _, ptr, sz);
            unxlate_dev_mem_ptr(p, ptr);
            if probe != 0 { kfree(bounce as *mut _); return -EFAULT; }
            remaining = copy_to_user(buf, bounce as *const _, sz);
        }
        if remaining != 0 { kfree(bounce as *mut _); return -EFAULT; }
        buf = buf.add(sz); p += sz as phys_addr_t; count -= sz; read += sz as isize;
        if should_stop_iteration() { break; }
    }
    kfree(bounce as *mut _); *ppos += read as loff_t; read
}

unsafe fn write_mem(file: *mut file, mut buf: *const u8, mut count: usize, ppos: *mut loff_t) -> isize {
    let mut p = *ppos as phys_addr_t; let mut written: isize = 0;
    if p != *ppos as phys_addr_t { return -EFBIG; }
    if valid_phys_addr_range(p, count) == 0 { return -EFAULT; }
    #[cfg(__ARCH_HAS_NO_PAGE_ZERO_MAPPED)]
    if p < PAGE_SIZE as phys_addr_t { let sz = size_inside_page(p as usize, count); buf = buf.add(sz); p += sz as phys_addr_t; count -= sz; written += sz as isize; }
    while count > 0 {
        let sz = size_inside_page(p as usize, count); let allowed = page_is_allowed((p >> PAGE_SHIFT) as usize);
        if allowed == 0 { return -EPERM; }
        if allowed == 1 {
            let ptr = xlate_dev_mem_ptr(p);
            if ptr.is_null() { if written != 0 { break; } return -EFAULT; }
            let copied = copy_from_user(ptr, buf, sz); unxlate_dev_mem_ptr(p, ptr);
            if copied != 0 { written += (sz - copied) as isize; if written != 0 { break; } return -EFAULT; }
        }
        buf = buf.add(sz); p += sz as phys_addr_t; count -= sz; written += sz as isize;
        if should_stop_iteration() { break; }
    }
    *ppos += written as loff_t; written
}

#[no_mangle]
unsafe extern "C" fn phys_mem_access_prot_allowed(_file: *mut file, _pfn: usize, _size: usize, _vma_prot: *mut pgprot_t) -> i32 { 1 }

#[cfg(not(HAVE_PHYS_MEM_ACCESS_PROT))]
unsafe fn phys_mem_access_prot(file: *mut file, pfn: usize, _size: usize, vma_prot: pgprot_t) -> pgprot_t {
    #[cfg(pgprot_noncached)] { let offset = (pfn as phys_addr_t) << PAGE_SHIFT; if uncached_access(file, offset) != 0 { return pgprot_noncached(vma_prot); } }
    vma_prot
}

#[cfg(not(CONFIG_MMU))]
unsafe fn get_unmapped_area_mem(_file: *mut file, _addr: usize, len: usize, pgoff: usize, _flags: usize) -> usize { if valid_mmap_phys_addr_range(pgoff, len) == 0 { return (-EINVAL) as usize; } pgoff << PAGE_SHIFT }
#[cfg(not(CONFIG_MMU))]
unsafe fn memory_mmap_capabilities(_file: *mut file) -> u32 { NOMMU_MAP_DIRECT | NOMMU_MAP_READ | NOMMU_MAP_WRITE | NOMMU_MAP_EXEC }
#[cfg(not(CONFIG_MMU))]
unsafe fn zero_mmap_capabilities(_file: *mut file) -> u32 { NOMMU_MAP_COPY }
#[cfg(not(CONFIG_MMU))]
unsafe fn private_mapping_ok(desc: *mut vm_area_desc) -> i32 { is_nommu_shared_vma_flags(&(*desc).vma_flags) }
#[cfg(CONFIG_MMU)]
unsafe fn private_mapping_ok(_desc: *mut vm_area_desc) -> i32 { 1 }

unsafe fn mmap_mem_prepare(desc: *mut vm_area_desc) -> i32 {
    let file = (*desc).file; let size = vma_desc_size(desc); let offset = ((*desc).pgoff as phys_addr_t) << PAGE_SHIFT;
    if offset >> PAGE_SHIFT != (*desc).pgoff as phys_addr_t || offset.wrapping_add(size as phys_addr_t).wrapping_sub(1) < offset { return -EINVAL; }
    if valid_mmap_phys_addr_range((*desc).pgoff, size) == 0 { return -EINVAL; }
    if private_mapping_ok(desc) == 0 { return -ENOSYS; }
    if range_is_allowed((*desc).pgoff, size) == 0 { return -EPERM; }
    if phys_mem_access_prot_allowed(file, (*desc).pgoff, size, &mut (*desc).page_prot) == 0 { return -EINVAL; }
    (*desc).page_prot = phys_mem_access_prot(file, (*desc).pgoff, size, (*desc).page_prot);
    (*desc).vm_ops = &mmap_mem_ops; mmap_action_remap_full(desc, (*desc).pgoff); (*desc).action.error_override = -EAGAIN; 0
}

unsafe fn read_null(_file: *mut file, _buf: *mut u8, _count: usize, _ppos: *mut loff_t) -> isize { 0 }
unsafe fn write_null(_file: *mut file, _buf: *const u8, count: usize, _ppos: *mut loff_t) -> isize { count as isize }
unsafe fn read_iter_null(_iocb: *mut kiocb, _to: *mut iov_iter) -> isize { 0 }
unsafe fn write_iter_null(_iocb: *mut kiocb, from: *mut iov_iter) -> isize { let count = iov_iter_count(from); iov_iter_advance(from, count); count as isize }
unsafe fn pipe_to_null(_info: *mut pipe_inode_info, _buf: *mut pipe_buffer, sd: *mut splice_desc) -> i32 { (*sd).len as i32 }
unsafe fn splice_write_null(pipe: *mut pipe_inode_info, out: *mut file, ppos: *mut loff_t, len: usize, flags: u32) -> isize { splice_from_pipe(pipe, out, ppos, len, flags, pipe_to_null) }
unsafe fn uring_cmd_null(_ioucmd: *mut io_uring_cmd, _issue_flags: u32) -> i32 { 0 }

unsafe fn read_iter_zero(_iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let mut written = 0usize;
    while iov_iter_count(iter) != 0 { let mut chunk = iov_iter_count(iter); if chunk > PAGE_SIZE { chunk = PAGE_SIZE; } let n = iov_iter_zero(chunk, iter); if n == 0 && iov_iter_count(iter) != 0 { return if written != 0 { written as isize } else { -EFAULT }; } written += n; if signal_pending(current) != 0 { return if written != 0 { written as isize } else { -ERESTARTSYS }; } if need_resched() == 0 { continue; } if (*_iocb).ki_flags & IOCB_NOWAIT != 0 { return if written != 0 { written as isize } else { -EAGAIN }; } cond_resched(); }
    written as isize
}
unsafe fn read_zero(_file: *mut file, buf: *mut u8, mut count: usize, _ppos: *mut loff_t) -> isize { let mut cleared = 0usize; while count != 0 { let chunk = core::cmp::min(count, PAGE_SIZE); let left = clear_user(buf.add(cleared), chunk); if left != 0 { cleared += chunk-left; if cleared == 0 { return -EFAULT; } break; } cleared += chunk; count -= chunk; if signal_pending(current) != 0 { break; } cond_resched(); } cleared as isize }
unsafe fn mmap_zero_prepare(desc: *mut vm_area_desc) -> i32 { #[cfg(not(CONFIG_MMU))] { return -ENOSYS; } if vma_desc_test(desc, VMA_SHARED_BIT) != 0 { return shmem_zero_setup_desc(desc); } vma_desc_set_anonymous(desc); 0 }
#[cfg(not(CONFIG_MMU))]
unsafe fn get_unmapped_area_zero(_file: *mut file, _addr: usize, _len: usize, _pgoff: usize, _flags: usize) -> usize { (-ENOSYS) as usize }
#[cfg(CONFIG_MMU)]
unsafe fn get_unmapped_area_zero(file: *mut file, addr: usize, len: usize, pgoff: usize, flags: usize) -> usize { if flags & MAP_SHARED != 0 { shmem_get_unmapped_area(core::ptr::null_mut(), addr, len, pgoff, flags) } else { mm_get_unmapped_area(file, addr, len, pgoff, flags) } }
unsafe fn write_full(_file: *mut file, _buf: *const u8, _count: usize, _ppos: *mut loff_t) -> isize { -ENOSPC }
unsafe fn null_lseek(file: *mut file, _offset: loff_t, _orig: i32) -> loff_t { (*file).f_pos = 0; 0 }
unsafe fn memory_lseek(file: *mut file, offset: loff_t, orig: i32) -> loff_t { inode_lock(file_inode(file)); let mut off = offset; let ret = match orig { SEEK_CUR => { off += (*file).f_pos; if (off as u64) >= (-MAX_ERRNO) as u64 { -EOVERFLOW } else { (*file).f_pos=off; force_successful_syscall_return(); off } }, SEEK_SET => { if (off as u64) >= (-MAX_ERRNO) as u64 { -EOVERFLOW } else { (*file).f_pos=off; force_successful_syscall_return(); off } }, _ => -EINVAL }; inode_unlock(file_inode(file)); ret }
unsafe fn open_port(inode: *mut inode, filp: *mut file) -> i32 { if capable(CAP_SYS_RAWIO)==0 { return -EPERM; } let rc=security_locked_down(LOCKDOWN_DEV_MEM); if rc!=0{return rc;} if iminor(inode)!=DEVMEM_MINOR{return 0;} (*filp).f_mapping=iomem_get_mapping(); 0 }

// The remaining file-operation tables and device registration are direct C
// declarations/initializers; their concrete kernel types are supplied here.
const zero_lseek: unsafe fn(*mut file, loff_t, i32) -> loff_t = null_lseek;
const full_lseek: unsafe fn(*mut file, loff_t, i32) -> loff_t = null_lseek;
const write_zero: unsafe fn(*mut file, *const u8, usize, *mut loff_t) -> isize = write_null;
const write_iter_zero: unsafe fn(*mut kiocb, *mut iov_iter) -> isize = write_iter_null;
const splice_write_zero: unsafe fn(*mut pipe_inode_info,*mut file,*mut loff_t,usize,u32)->isize = splice_write_null;
const open_mem: unsafe fn(*mut inode,*mut file)->i32 = open_port;

#[cfg(CONFIG_DEVPORT)]
unsafe fn read_port(_file:*mut file, mut buf:*mut u8, mut count:usize, ppos:*mut loff_t)->isize { let mut i=*ppos as usize; let start=buf; if access_ok(buf,count)==0{return -EFAULT;} while count>0 && i<65536 { if __put_user(inb(i),buf)<0{return -EFAULT;} i+=1; count-=1; buf=buf.add(1); } *ppos=i as loff_t; buf.offset_from(start) as isize }
#[cfg(CONFIG_DEVPORT)]
unsafe fn write_port(_file:*mut file, mut buf:*const u8, mut count:usize, ppos:*mut loff_t)->isize { let mut i=*ppos as usize; let start=buf; if access_ok(buf,count)==0{return -EFAULT;} while count>0 && i<65536 { let mut c=0u8; if __get_user(&mut c,buf)!=0 { if buf>start {break;} return -EFAULT;} outb(c,i); i+=1; count-=1; buf=buf.add(1); } *ppos=i as loff_t; buf.offset_from(start) as isize }

#[repr(C)]
struct memdev { name:*const u8, fops:*const file_operations, fmode:fmode_t, mode:umode_t }
// Conditional entries preserve the original CONFIG_* device list.
static mut devlist: [memdev; 12] = [
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:b"null\0".as_ptr(),fops:&null_fops,fmode:FMODE_NOWAIT,mode:0o666},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:b"zero\0".as_ptr(),fops:&zero_fops,fmode:FMODE_NOWAIT,mode:0o666},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:b"full\0".as_ptr(),fops:&full_fops,fmode:0,mode:0o666},
    memdev{name:b"random\0".as_ptr(),fops:&random_fops,fmode:FMODE_NOWAIT,mode:0o666},
    memdev{name:b"urandom\0".as_ptr(),fops:&urandom_fops,fmode:FMODE_NOWAIT,mode:0o666},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
    memdev{name:core::ptr::null(),fops:core::ptr::null(),fmode:0,mode:0},
];

unsafe fn memory_open(inode:*mut inode, filp:*mut file)->i32 { let minor=iminor(inode) as usize; if minor>=devlist.len(){return -ENXIO;} let dev=&devlist[minor]; if dev.fops.is_null(){return -ENXIO;} (*filp).f_op=dev.fops; (*filp).f_mode|=dev.fmode; if (*dev.fops).open.is_some(){return ((*dev.fops).open.unwrap())(inode,filp);} 0 }
unsafe fn mem_devnode(dev:*const device, mode:*mut umode_t)->*mut i8 { if !mode.is_null() && devlist[MINOR((*dev).devt) as usize].mode!=0 {*mode=devlist[MINOR((*dev).devt) as usize].mode;} core::ptr::null_mut() }
unsafe fn chr_dev_init()->i32 { let mut retval=0; if register_chrdev(MEM_MAJOR,b"mem\0".as_ptr() as *const i8,&memory_fops)!=0 {printk(b"unable to get major %d for memory devs\n\0".as_ptr() as *const i8,MEM_MAJOR);} retval=class_register(&mem_class); if retval!=0{return retval;} let mut minor=1; while minor<devlist.len(){if !devlist[minor].name.is_null(){if minor==DEVPORT_MINOR as usize && arch_has_dev_port()==0 {minor+=1;continue;} device_create(&mem_class,core::ptr::null_mut(),MKDEV(MEM_MAJOR,minor as i32),core::ptr::null_mut(),devlist[minor].name);} minor+=1;} tty_init() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
