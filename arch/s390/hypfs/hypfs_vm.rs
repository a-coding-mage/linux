// SPDX-License-Identifier: GPL-2.0
/*
 *    Hypervisor filesystem for Linux on s390. z/VM implementation.
 *
 *    Copyright IBM Corp. 2006
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

use core::ffi::c_void;

// Declarations supplied by the surrounding kernel translation.
extern "C" {
    static mut diag2fc_guest_query: *mut i8;
    fn diag_stat_inc(stat: i32);
    fn ASCEBC(data: *mut i8, len: usize);
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *const c_void);
    fn store_tod_clock_ext(clock: *mut tod_clock);
    fn machine_is_vm() -> bool;
    fn hypfs_dbfs_create_file(file: *mut hypfs_dbfs_file);
    fn hypfs_dbfs_remove_file(file: *mut hypfs_dbfs_file);
}

const DBFS_D2FC_HDR_VERSION: u16 = 0;
const DIAG2FC_NAME_LEN: usize = 8;
const DIAG_STAT_X2FC: i32 = 0;

static mut local_guest: [i8; 9] = [b' ' as i8; 9];
static mut all_guests: [i8; 9] = [b'*' as i8, b' ' as i8, b' ' as i8, b' ' as i8,
    b' ' as i8, b' ' as i8, b' ' as i8, b' ' as i8, 0];
static mut all_groups: *mut i8 = unsafe { all_guests.as_mut_ptr() };

#[repr(C)]
struct diag2fc_parm_list {
    userid: [i8; DIAG2FC_NAME_LEN],
    aci_grp: [i8; DIAG2FC_NAME_LEN],
    addr: u64,
    size: i32,
    fmt: i32,
}

#[repr(C)]
struct diag2fc_data {
    _opaque: [u8; 0],
}

#[repr(C)]
union tod_clock {
    _opaque: u64,
}

#[repr(C, packed)]
struct dbfs_d2fc_hdr {
    len: u64,
    version: u16,
    tod_ext: tod_clock,
    count: u64,
    reserved: [i8; 30],
}

#[repr(C, packed)]
struct dbfs_d2fc {
    hdr: dbfs_d2fc_hdr,
    buf: [i8; 0],
}

#[repr(C)]
struct hypfs_dbfs_file {
    name: *const i8,
    data_create: Option<unsafe extern "C" fn(*mut *mut c_void, *mut *mut c_void, *mut usize) -> i32>,
    data_free: Option<unsafe extern "C" fn(*const c_void)>,
}

unsafe fn diag2fc(size: i32, query: *mut i8, addr: *mut c_void) -> i32 {
    let mut residual_cnt: u64 = 0;
    let mut rc: i64 = -1;
    let mut parm_list: diag2fc_parm_list = core::mem::zeroed();

    core::ptr::copy_nonoverlapping(query, parm_list.userid.as_mut_ptr(), DIAG2FC_NAME_LEN);
    ASCEBC(parm_list.userid.as_mut_ptr(), DIAG2FC_NAME_LEN);
    core::ptr::copy_nonoverlapping(all_groups, parm_list.aci_grp.as_mut_ptr(), DIAG2FC_NAME_LEN);
    ASCEBC(parm_list.aci_grp.as_mut_ptr(), DIAG2FC_NAME_LEN);
    parm_list.addr = addr as u64;
    parm_list.size = size;
    parm_list.fmt = 0x02;

    diag_stat_inc(DIAG_STAT_X2FC);
    // The s390 DIAG 0x2fc instruction and exception-table entry are external
    // assembly semantics; preserve their outputs and memory clobber here.
    core::arch::asm!(
        "diag {0}, {1}, 0x2fc",
        "0: nopr %r7",
        inout("r2") &mut parm_list => residual_cnt,
        inout("r3") rc,
        options(nostack, preserves_flags)
    );

    if rc != 0 && rc != -2 {
        rc as i32
    } else {
        -(residual_cnt as i32)
    }
}

/*
 * Allocate buffer for "query" and store diag 2fc at "offset"
 */
#[no_mangle]
pub unsafe extern "C" fn diag2fc_store(query: *mut i8, count: *mut u32, offset: i32) -> *mut c_void {
    let data: *mut c_void;
    let size: i32;
    loop {
        size = diag2fc(0, query, core::ptr::null_mut());
        if size < 0 {
            return (-13isize) as *mut c_void;
        }
        data = vmalloc((size + offset) as usize);
        if data.is_null() {
            return (-12isize) as *mut c_void;
        }
        if diag2fc(size, query, (data as *mut u8).add(offset as usize) as *mut c_void) == 0 {
            break;
        }
        vfree(data);
    }
    *count = (size as usize / core::mem::size_of::<diag2fc_data>()) as u32;
    data
}

#[no_mangle]
pub unsafe extern "C" fn diag2fc_free(data: *const c_void) {
    vfree(data);
}

unsafe extern "C" fn dbfs_diag2fc_create(data: *mut *mut c_void, data_free_ptr: *mut *mut c_void, size: *mut usize) -> i32 {
    let mut count: u32 = 0;
    let d2fc = diag2fc_store(diag2fc_guest_query, &mut count, core::mem::size_of::<dbfs_d2fc_hdr>() as i32) as *mut dbfs_d2fc;
    if d2fc as isize == -13 || d2fc as isize == -12 {
        return d2fc as isize as i32;
    }
    store_tod_clock_ext(core::ptr::addr_of_mut!((*d2fc).hdr.tod_ext));
    (*d2fc).hdr.len = (count as usize * core::mem::size_of::<diag2fc_data>()) as u64;
    (*d2fc).hdr.version = DBFS_D2FC_HDR_VERSION;
    (*d2fc).hdr.count = count as u64;
    (*d2fc).hdr.reserved = [0; 30];
    *data = d2fc as *mut c_void;
    *data_free_ptr = d2fc as *mut c_void;
    *size = (*d2fc).hdr.len as usize + core::mem::size_of::<dbfs_d2fc_hdr>();
    0
}

static mut dbfs_file_2fc: hypfs_dbfs_file = hypfs_dbfs_file {
    name: b"diag_2fc\0".as_ptr() as *const i8,
    data_create: Some(dbfs_diag2fc_create),
    data_free: Some(diag2fc_free),
};

#[no_mangle]
pub unsafe extern "C" fn hypfs_vm_init() -> i32 {
    if !machine_is_vm() {
        return 0;
    }
    if diag2fc(0, all_guests.as_mut_ptr(), core::ptr::null_mut()) > 0 {
        diag2fc_guest_query = all_guests.as_mut_ptr();
    } else if diag2fc(0, local_guest.as_mut_ptr(), core::ptr::null_mut()) > 0 {
        diag2fc_guest_query = local_guest.as_mut_ptr();
    } else {
        return -13;
    }
    hypfs_dbfs_create_file(&mut dbfs_file_2fc);
    0
}

#[no_mangle]
pub unsafe extern "C" fn hypfs_vm_exit() {
    if !machine_is_vm() {
        return;
    }
    hypfs_dbfs_remove_file(&mut dbfs_file_2fc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
