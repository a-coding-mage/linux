/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available under either the GNU General Public License
 * (GPL) Version 2 or the OpenIB.org BSD license.
 */

// Kernel and RDS types, constants, and functions are supplied by the
// surrounding translation unit and headers.

#[repr(C)]
pub struct rds_info_iterator {
    pub pages: *mut *mut page,
    pub addr: *mut core::ffi::c_void,
    pub offset: usize,
}

extern "C" {
    static mut rds_info_srcu: srcu_struct;
    static mut rds_info_lock: spinlock_t;
    static mut rds_info_funcs: [rds_info_func; (RDS_INFO_LAST - RDS_INFO_FIRST + 1) as usize];
}

pub unsafe fn rds_info_register_func(optname: i32, func: rds_info_func) {
    let offset = optname - RDS_INFO_FIRST;
    BUG_ON(optname < RDS_INFO_FIRST || optname > RDS_INFO_LAST);
    spin_lock(&mut rds_info_lock);
    if WARN_ON_ONCE(!rds_info_funcs[offset as usize].is_none()) {
        spin_unlock(&mut rds_info_lock);
        return;
    }
    WRITE_ONCE(&mut rds_info_funcs[offset as usize], func);
    spin_unlock(&mut rds_info_lock);
}

pub unsafe fn rds_info_deregister_func(optname: i32, func: rds_info_func) {
    let offset = optname - RDS_INFO_FIRST;
    BUG_ON(optname < RDS_INFO_FIRST || optname > RDS_INFO_LAST);
    spin_lock(&mut rds_info_lock);
    if WARN_ON_ONCE(rds_info_funcs[offset as usize] != func) {
        spin_unlock(&mut rds_info_lock);
        return;
    }
    WRITE_ONCE(&mut rds_info_funcs[offset as usize], None);
    spin_unlock(&mut rds_info_lock);
    synchronize_srcu(&mut rds_info_srcu);
}

// Typically an atomic kmap is held across multiple rds_info_copy calls.
pub unsafe fn rds_info_iter_unmap(iter: *mut rds_info_iterator) {
    if !(*iter).addr.is_null() {
        kunmap_atomic((*iter).addr);
        (*iter).addr = core::ptr::null_mut();
    }
}

// get_user_pages() flushes the dcache on the pages for us.
pub unsafe fn rds_info_copy(
    iter: *mut rds_info_iterator,
    mut data: *mut core::ffi::c_void,
    mut bytes: usize,
) {
    while bytes != 0 {
        if (*iter).addr.is_null() {
            (*iter).addr = kmap_atomic(*(*iter).pages);
        }

        let this = core::cmp::min(bytes, PAGE_SIZE - (*iter).offset);

        rdsdebug(
            "page %p addr %p offset %lu this %lu data %p bytes %lu\n",
            *(*iter).pages,
            (*iter).addr,
            (*iter).offset,
            this,
            data,
            bytes,
        );

        core::ptr::copy_nonoverlapping(
            data as *const u8,
            ((*iter).addr as *mut u8).add((*iter).offset),
            this,
        );

        data = (data as *mut u8).add(this) as *mut core::ffi::c_void;
        bytes -= this;
        (*iter).offset += this;

        if (*iter).offset == PAGE_SIZE {
            kunmap_atomic((*iter).addr);
            (*iter).addr = core::ptr::null_mut();
            (*iter).offset = 0;
            (*iter).pages = (*iter).pages.add(1);
        }
    }
}

pub unsafe fn rds_info_getsockopt(
    sock: *mut socket,
    optname: i32,
    opt: *mut sockopt_t,
) -> i32 {
    let mut iter = rds_info_iterator {
        pages: core::ptr::null_mut(),
        addr: core::ptr::null_mut(),
        offset: 0,
    };
    let mut lens = rds_info_lengths { each: 0, nr: 0 };
    let mut nr_pages: usize = 0;
    let mut func: rds_info_func;
    let mut pages: *mut *mut page = core::ptr::null_mut();
    let mut offset0: usize = 0;
    let mut srcu_idx: i32;
    let mut npages: i32 = 0;
    let mut ret: i32;
    let mut len: i32;
    let mut total: i32;

    len = (*opt).optlen;

    if len < 0 || len > i32::MAX - PAGE_SIZE as i32 + 1 {
        ret = -EINVAL;
        return ret;
    }

    if !user_backed_iter(&mut (*opt).iter_out) {
        ret = -EOPNOTSUPP;
        return ret;
    }

    if len != 0 {
        npages = iov_iter_npages(&(*opt).iter_out, i32::MAX);
        pages = kvmalloc_array(npages as usize, core::mem::size_of::<*mut page>(), GFP_KERNEL)
            as *mut *mut page;
        if pages.is_null() {
            ret = -ENOMEM;
            return ret;
        }

        ret = iov_iter_extract_pages(&mut (*opt).iter_out, &mut pages, len as usize,
                                     npages, 0, &mut offset0);
        if ret < 0 {
            kvfree(pages as *mut core::ffi::c_void);
            return ret;
        }
        nr_pages = (offset0 + ret as usize).div_ceil(PAGE_SIZE);
        if ret != len {
            ret = -EAGAIN;
            kvfree(pages as *mut core::ffi::c_void);
            return ret;
        }
    }

    srcu_idx = srcu_read_lock(&rds_info_srcu);
    func = READ_ONCE(rds_info_funcs[(optname - RDS_INFO_FIRST) as usize]);
    if func.is_none() {
        srcu_read_unlock(&rds_info_srcu, srcu_idx);
        ret = -ENOPROTOOPT;
        if !pages.is_null() { kvfree(pages as *mut core::ffi::c_void); }
        return ret;
    }

    iter.pages = pages;
    iter.addr = core::ptr::null_mut();
    iter.offset = offset0;
    func.unwrap()(sock, len, &mut iter, &mut lens);
    srcu_read_unlock(&rds_info_srcu, srcu_idx);

    BUG_ON(lens.each == 0);
    total = lens.nr * lens.each;
    rds_info_iter_unmap(&mut iter);

    if total > len {
        len = total;
        ret = -ENOSPC;
    } else {
        len = total;
        ret = lens.each;
    }
    (*opt).optlen = len;

    if !pages.is_null() && iov_iter_extract_will_pin(&(*opt).iter_out) {
        unpin_user_pages_dirty_lock(pages, nr_pages, true);
    }
    kvfree(pages as *mut core::ffi::c_void);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
