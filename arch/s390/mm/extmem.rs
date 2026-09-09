// SPDX-License-Identifier: GPL-2.0
/*
 * Author(s)......: Carsten Otte <cotte@de.ibm.com>
 *                  Rob M van der Heij <rvdheij@nl.ibm.com>
 *                  Steven Shultz <shultzss@us.ibm.com>
 * Bugreports.to..: <Linux390@de.ibm.com>
 * Copyright IBM Corp. 2002, 2004
 */

// C dependencies supplied by the kernel and architecture headers are intentionally
// referenced by name here rather than reimplemented in this translation unit.

const DCSS_PURGESEG: i32 = 0x08;
const DCSS_LOADSHRX: i32 = 0x20;
const DCSS_LOADNSRX: i32 = 0x24;
const DCSS_FINDSEGX: i32 = 0x2c;
const DCSS_SEGEXTX: i32 = 0x38;
const DCSS_FINDSEGA: i8 = 0x0c;

#[repr(C)]
struct qrange {
    start: usize,
    end: usize,
}

#[repr(C)]
struct qout64 {
    segstart: usize,
    segend: usize,
    segcnt: i32,
    segrcnt: i32,
    range: [qrange; 6],
}

#[repr(C)]
struct qin64 {
    qopcode: i8,
    rsrv1: [i8; 3],
    qrcode: i8,
    rsrv2: [i8; 3],
    qname: [i8; 8],
    qoutptr: u32,
    qoutlen: i16,
}

#[repr(C)]
struct dcss_segment {
    list: list_head,
    dcss_name: [i8; 8],
    res_name: [i8; 16],
    start_addr: usize,
    end: usize,
    ref_count: refcount_t,
    do_nonshared: i32,
    vm_segtype: u32,
    range: [qrange; 6],
    segcnt: i32,
    res: *mut resource,
}

static mut dcss_lock: mutex = DEFINE_MUTEX!();
static mut dcss_list: list_head = LIST_HEAD!();
static mut segtype_string: [&'static [u8]; 8] = [
    b"SW\0", b"EW\0", b"SR\0", b"ER\0", b"SN\0", b"EN\0", b"SC\0",
    b"EW/EN-MIXED\0",
];
static mut loadshr_scode: i32 = DCSS_LOADSHRX;
static mut loadnsr_scode: i32 = DCSS_LOADNSRX;
static mut purgeseg_scode: i32 = DCSS_PURGESEG;
static mut segext_scode: i32 = DCSS_SEGEXTX;

unsafe fn dcss_mkname(name: *mut i8, dcss_name: *mut i8) {
    let mut i = 0;
    while i < 8 {
        if *name.add(i) == 0 { break; }
        *dcss_name.add(i) = toupper(*name.add(i));
        i += 1;
    }
    while i < 8 { *dcss_name.add(i) = b' ' as i8; i += 1; }
    ASCEBC(dcss_name, 8);
}

unsafe fn segment_by_name(name: *mut i8) -> *mut dcss_segment {
    BUG_ON(!mutex_is_locked(&dcss_lock));
    let mut dcss_name = [0i8; 9];
    dcss_mkname(name, dcss_name.as_mut_ptr());
    let mut l = (*dcss_list.next).next;
    while l != &dcss_list as *const _ as *mut list_head {
        let tmp = list_entry(l, dcss_segment, list);
        if memcmp((*tmp).dcss_name.as_ptr() as *const _, dcss_name.as_ptr() as *const _, 8) == 0 { return tmp; }
        l = (*l).next;
    }
    core::ptr::null_mut()
}

unsafe fn dcss_diag(func: *mut i32, parameter: *mut core::ffi::c_void, ret1: *mut usize, ret2: *mut usize) -> i32 {
    let mut rx = virt_to_phys(parameter);
    let mut ry = *func as usize;
    diag_stat_inc(DIAG_STAT_X064);
    let cc: i32;
    core::arch::asm!("diag {rx}, {ry}, 0x64", rx = inout(reg) rx, ry = inout(reg) ry, lateout("cc") cc);
    *ret1 = rx; *ret2 = ry; CC_TRANSFORM(cc)
}

unsafe fn dcss_diag_translate_rc(vm_rc: i32) -> i32 { if vm_rc == 44 { -ENOENT } else { -EIO } }

unsafe fn query_segment_type(seg: *mut dcss_segment) -> i32 {
    let qin = kmalloc_obj::<qin64>(GFP_KERNEL | GFP_DMA);
    let qout = kmalloc_obj::<qout64>(GFP_KERNEL | GFP_DMA);
    if qin.is_null() || qout.is_null() { kfree(qin as *mut _); kfree(qout as *mut _); return -ENOMEM; }
    (*qin).qopcode = DCSS_FINDSEGA;
    (*qin).qoutptr = virt_to_phys(qout as *mut _ ) as u32;
    (*qin).qoutlen = core::mem::size_of::<qout64>() as i16;
    memcpy((*qin).qname.as_mut_ptr() as *mut _, (*seg).dcss_name.as_ptr() as *const _, 8);
    let mut dummy = 0usize; let mut vmrc = 0usize;
    let diag_cc = dcss_diag(&mut segext_scode, qin as *mut _, &mut dummy, &mut vmrc);
    let mut rc = 0;
    if diag_cc < 0 { rc = diag_cc; }
    else if diag_cc > 1 { pr_warn!("Querying a DCSS type failed with rc={}\n", vmrc); rc = dcss_diag_translate_rc(vmrc as i32); }
    else if (*qout).segcnt > 6 { rc = -EOPNOTSUPP; }
    else if (*qout).segcnt == 1 { (*seg).vm_segtype = ((*qout).range[0].start & 0xff) as u32; }
    else {
        let mut start = (*qout).segstart >> PAGE_SHIFT;
        let mut i = 0;
        while i < (*qout).segcnt {
            let ty = (*qout).range[i as usize].start & 0xff;
            if ty != SEG_TYPE_EW as usize && ty != SEG_TYPE_EN as usize || start != (*qout).range[i as usize].start >> PAGE_SHIFT { rc = -EOPNOTSUPP; break; }
            start = ((*qout).range[i as usize].end >> PAGE_SHIFT) + 1; i += 1;
        }
        if rc == 0 { (*seg).vm_segtype = SEG_TYPE_EWEN as u32; }
    }
    if rc == 0 { (*seg).start_addr = (*qout).segstart; (*seg).end = (*qout).segend; memcpy((*seg).range.as_mut_ptr() as *mut _, (*qout).range.as_ptr() as *const _, 6 * core::mem::size_of::<qrange>()); (*seg).segcnt = (*qout).segcnt; }
    kfree(qin as *mut _); kfree(qout as *mut _); rc
}

pub unsafe fn segment_type(name: *mut i8) -> i32 {
    if !machine_is_vm() { return -ENOSYS; }
    let mut seg: dcss_segment = core::mem::zeroed(); dcss_mkname(name, seg.dcss_name.as_mut_ptr());
    let rc = query_segment_type(&mut seg); if rc < 0 { rc } else { seg.vm_segtype as i32 }
}

unsafe fn segment_overlaps_others(seg: *mut dcss_segment) -> i32 {
    BUG_ON(!mutex_is_locked(&dcss_lock));
    let mut l = (*dcss_list.next).next;
    while l != &dcss_list as *const _ as *mut list_head {
        let tmp = list_entry(l, dcss_segment, list);
        if ((*tmp).start_addr >> 20) <= ((*seg).end >> 20) && ((*tmp).end >> 20) >= ((*seg).start_addr >> 20) && tmp != seg { return 1; }
        l = (*l).next;
    }
    0
}

unsafe fn __segment_load(name: *mut i8, do_nonshared: i32, addr: *mut usize, end: *mut usize) -> i32 {
    let mut start_addr = 0usize; let mut end_addr = 0usize; let mut dummy = 0usize; let mut segtype = -1;
    let seg = kmalloc_obj::<dcss_segment>(GFP_KERNEL | GFP_DMA);
    if seg.is_null() { return -ENOMEM; }
    dcss_mkname(name, (*seg).dcss_name.as_mut_ptr());
    let mut rc = query_segment_type(seg);
    if rc < 0 { kfree(seg as *mut _); return rc; }
    if segment_overlaps_others(seg) != 0 { kfree(seg as *mut _); return -EBUSY; }
    (*seg).res = kzalloc_obj::<resource>();
    if (*seg).res.is_null() { kfree(seg as *mut _); return -ENOMEM; }
    (*(*seg).res).flags = IORESOURCE_BUSY | IORESOURCE_MEM; (*(*seg).res).start = (*seg).start_addr; (*(*seg).res).end = (*seg).end;
    memcpy((*seg).res_name.as_mut_ptr() as *mut _, (*seg).dcss_name.as_ptr() as *const _, 8); EBCASC((*seg).res_name.as_mut_ptr(), 8); (*seg).res_name[8] = 0; strlcat((*seg).res_name.as_mut_ptr(), b" (DCSS)\0".as_ptr() as *const _, 16); (*(*seg).res).name = (*seg).res_name.as_mut_ptr();
    segtype = (*seg).vm_segtype as i32;
    if segtype == SEG_TYPE_SC || ((segtype == SEG_TYPE_SR || segtype == SEG_TYPE_ER) && do_nonshared == 0) { (*(*seg).res).flags |= IORESOURCE_READONLY; }
    if request_resource(&mut iomem_resource, (*seg).res) != 0 { kfree((*seg).res as *mut _); kfree(seg as *mut _); return -EBUSY; }
    rc = vmem_add_mapping((*seg).start_addr, (*seg).end - (*seg).start_addr + 1);
    if rc != 0 { release_resource((*seg).res); kfree((*seg).res as *mut _); kfree(seg as *mut _); return rc; }
    let diag_cc = if do_nonshared != 0 { dcss_diag(&mut loadnsr_scode, (*seg).dcss_name.as_mut_ptr() as *mut _, &mut start_addr, &mut end_addr) } else { dcss_diag(&mut loadshr_scode, (*seg).dcss_name.as_mut_ptr() as *mut _, &mut start_addr, &mut end_addr) };
    if diag_cc < 0 { dcss_diag(&mut purgeseg_scode, (*seg).dcss_name.as_mut_ptr() as *mut _, &mut dummy, &mut dummy); vmem_remove_mapping((*seg).start_addr, (*seg).end - (*seg).start_addr + 1); release_resource((*seg).res); kfree((*seg).res as *mut _); kfree(seg as *mut _); return diag_cc; }
    if diag_cc > 1 { pr_warn!("Loading DCSS {} failed with rc={}\n", name, end_addr); rc = dcss_diag_translate_rc(end_addr as i32); dcss_diag(&mut purgeseg_scode, (*seg).dcss_name.as_mut_ptr() as *mut _, &mut dummy, &mut dummy); vmem_remove_mapping((*seg).start_addr, (*seg).end - (*seg).start_addr + 1); release_resource((*seg).res); kfree((*seg).res as *mut _); kfree(seg as *mut _); return rc; }
    (*seg).start_addr = start_addr; (*seg).end = end_addr; (*seg).do_nonshared = do_nonshared; refcount_set(&mut (*seg).ref_count, 1); list_add(&mut (*seg).list, &mut dcss_list); *addr = start_addr; *end = end_addr; rc = (*seg).vm_segtype as i32; rc
}

pub unsafe fn segment_load(name: *mut i8, do_nonshared: i32, addr: *mut usize, end: *mut usize) -> i32 {
    if !machine_is_vm() { return -ENOSYS; } mutex_lock(&mut dcss_lock); let seg = segment_by_name(name); let rc;
    if seg.is_null() { rc = __segment_load(name, do_nonshared, addr, end); } else if do_nonshared == (*seg).do_nonshared { refcount_inc(&mut (*seg).ref_count); *addr = (*seg).start_addr; *end = (*seg).end; rc = (*seg).vm_segtype as i32; } else { *addr = 0; *end = 0; rc = -EPERM; }
    mutex_unlock(&mut dcss_lock); rc
}

pub unsafe fn segment_unload(name: *mut i8) { if !machine_is_vm() { return; } mutex_lock(&mut dcss_lock); let seg = segment_by_name(name); if seg.is_null() { pr_err!("Unloading unknown DCSS failed\n"); mutex_unlock(&mut dcss_lock); return; } if !refcount_dec_and_test(&mut (*seg).ref_count) { mutex_unlock(&mut dcss_lock); return; } release_resource((*seg).res); kfree((*seg).res as *mut _); vmem_remove_mapping((*seg).start_addr, (*seg).end - (*seg).start_addr + 1); list_del(&mut (*seg).list); dcss_diag(&mut purgeseg_scode, (*seg).dcss_name.as_mut_ptr() as *mut _, &mut 0, &mut 0); kfree(seg as *mut _); mutex_unlock(&mut dcss_lock); }

pub unsafe fn segment_modify_shared(_name: *mut i8, _do_nonshared: i32) -> i32 { -EINVAL }
pub unsafe fn segment_save(_name: *mut i8) {}
pub unsafe fn segment_warning(_rc: i32, _seg_name: *mut i8) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
