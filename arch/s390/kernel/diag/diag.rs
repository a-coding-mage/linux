// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of s390 diagnose codes
 *
 * Copyright IBM Corp. 2007
 * Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// Kernel and architecture dependencies are supplied by other translation units.

#[repr(C)]
pub struct diag_stat {
    pub counter: [core::ffi::c_uint; NR_DIAG_STAT],
}

// DEFINE_PER_CPU(struct diag_stat, diag_stat);
extern "C" {
    static mut diag_stat: diag_stat;
}

#[repr(C)]
pub struct diag_desc {
    pub code: core::ffi::c_int,
    pub name: *mut core::ffi::c_char,
}

extern "C" {
    static diag_map: [diag_desc; NR_DIAG_STAT];
}

const DIAG_MAP_CODES: [core::ffi::c_int; 26] = [0x008, 0x00c, 0x010, 0x014, 0x044, 0x064, 0x08c, 0x09c, 0x0dc, 0x204, 0x210, 0x224, 0x250, 0x258, 0x26c, 0x288, 0x2c4, 0x2fc, 0x304, 0x308, 0x310, 0x318, 0x320, 0x324, 0x49c, 0x500];
const DIAG_MAP_NAMES: [&str; 26] = ["Console Function", "Pseudo Timer", "Release Pages", "Spool File Services", "Voluntary Timeslice End", "NSS Manipulation", "Access 3270 Display Device Information", "Relinquish Timeslice", "Appldata Control", "Logical-CPU Utilization", "Device Information", "EBCDIC-Name Table", "Block I/O", "Page-Reference Services", "Certain System Information", "Time Bomb", "FTP Services", "Guest Performance Data", "Partition-Resource Service", "List-Directed IPL", "Memory Topology Information", "CP Name and Version Codes", "Certificate Store", "Power Information Block", "Warning-Track Interruption", "Virtio Service"];

pub unsafe extern "C" fn diag_stat_inc(nr: diag_stat_enum) {
    // this_cpu_inc(diag_stat.counter[nr]);
    trace_s390_diagnose(diag_map[nr as usize].code);
}

pub unsafe extern "C" fn diag_stat_inc_norecursion(nr: diag_stat_enum) {
    // this_cpu_inc(diag_stat.counter[nr]);
    trace_s390_diagnose_norecursion(diag_map[nr as usize].code);
}

extern "C" {
    static mut diag_amode31_ops: diag_ops;
    static mut __diag210_tmp_amode31: *mut diag210;
}

extern "C" {
    static mut __diag8c_tmp_amode31: *mut diag8c;
}

// The following declarations are provided by the kernel and architecture headers.
extern "C" {
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn seq_puts(m: *mut seq_file, s: *const core::ffi::c_char);
    fn seq_putc(m: *mut seq_file, c: core::ffi::c_int);
    fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...);
    fn virt_to_phys(addr: *const core::ffi::c_void) -> core::ffi::c_ulong;
    fn is_vmalloc_addr(addr: *const core::ffi::c_void) -> bool;
    fn vmalloc_to_pfn(addr: *const core::ffi::c_void) -> core::ffi::c_ulong;
    fn pfn_to_phys(pfn: core::ffi::c_ulong) -> core::ffi::c_ulong;
    fn trace_s390_diagnose(code: core::ffi::c_int);
    fn trace_s390_diagnose_norecursion(code: core::ffi::c_int);
}

extern "C" {
    fn _diag210_amode31(addr: *mut diag210) -> core::ffi::c_int;
    fn _diag26c_amode31(req: core::ffi::c_ulong, resp: core::ffi::c_ulong, subcode: diag26c_sc) -> core::ffi::c_int;
    fn _diag14_amode31(rx: core::ffi::c_ulong, ry1: core::ffi::c_ulong, subcode: core::ffi::c_ulong) -> core::ffi::c_int;
    fn _diag0c_amode31(addr: core::ffi::c_ulong);
    fn _diag8c_amode31(addr: *mut diag8c, devno: *mut ccw_dev_id, size: usize) -> core::ffi::c_int;
    fn _diag308_reset_amode31();
}

#[repr(C)]
pub struct diag_ops {
    pub diag210: Option<unsafe extern "C" fn(*mut diag210) -> core::ffi::c_int>,
    pub diag26c: Option<unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_ulong, diag26c_sc) -> core::ffi::c_int>,
    pub diag14: Option<unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_ulong, core::ffi::c_ulong) -> core::ffi::c_int>,
    pub diag0c: Option<unsafe extern "C" fn(core::ffi::c_ulong)>,
    pub diag8c: Option<unsafe extern "C" fn(*mut diag8c, *mut ccw_dev_id, usize) -> core::ffi::c_int>,
    pub diag308_reset: Option<unsafe extern "C" fn()>,
}

pub unsafe fn show_diag_stat_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    if *pos <= NR_DIAG_STAT as loff_t { (*pos as usize + 1) as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}

pub unsafe fn show_diag_stat_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    show_diag_stat_start(m, pos)
}

pub unsafe fn show_diag_stat_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

pub unsafe fn show_diag_stat(_m: *mut seq_file, _v: *mut core::ffi::c_void) -> core::ffi::c_int { 0 }

// DEFINE_SEQ_ATTRIBUTE(show_diag_stat);
// device_initcall(show_diag_stat_init);

#[no_mangle]
pub unsafe extern "C" fn diag0c(data: *mut hypfs_diag0c_entry) {
    diag_stat_inc(DIAG_STAT_X00C);
    if let Some(f) = (*core::ptr::addr_of!(diag_amode31_ops)).diag0c { f(virt_to_phys(data.cast())); }
}

#[no_mangle]
pub unsafe extern "C" fn diag14(mut rx: core::ffi::c_ulong, ry1: core::ffi::c_ulong, subcode: core::ffi::c_ulong) -> core::ffi::c_int {
    diag_stat_inc(DIAG_STAT_X014);
    match subcode { 0x0 | 0xfff => rx = virt_to_phys(rx as *const core::ffi::c_void), _ => {} }
    (*core::ptr::addr_of!(diag_amode31_ops)).diag14.unwrap()(rx, ry1, subcode)
}

const DIAG204_BUSY_RC: core::ffi::c_int = 8;

unsafe fn __diag204(subcode: *mut core::ffi::c_ulong, size: core::ffi::c_ulong, _addr: *mut core::ffi::c_void) -> core::ffi::c_ulong {
    // The C implementation issues `diag ...,0x204` with an asm register pair and exception table.
    let _ = size;
    *subcode = *subcode;
    0
}

#[no_mangle]
pub unsafe extern "C" fn diag204(mut subcode: core::ffi::c_ulong, size: core::ffi::c_ulong, mut addr: *mut core::ffi::c_void) -> core::ffi::c_int {
    if !addr.is_null() {
        if !is_vmalloc_addr(addr) { return -22; }
        // IS_ALIGNED(addr, PAGE_SIZE)
        if (addr as usize) % PAGE_SIZE != 0 { return -22; }
    }
    if (subcode & DIAG204_SUBCODE_MASK) == DIAG204_SUBC_STIB4 { addr = pfn_to_phys(vmalloc_to_pfn(addr)) as *mut core::ffi::c_void; }
    diag_stat_inc(DIAG_STAT_X204);
    let result = __diag204(&mut subcode, size, addr);
    if subcode == DIAG204_BUSY_RC as _ { -16 } else if subcode != 0 { -95 } else { result as core::ffi::c_int }
}

#[no_mangle]
pub unsafe extern "C" fn diag210(addr: *mut diag210) -> core::ffi::c_int {
    let tmp = __diag210_tmp_amode31;
    *tmp = *addr;
    diag_stat_inc(DIAG_STAT_X210);
    let ccode = diag_amode31_ops.diag210.unwrap()(tmp);
    *addr = *tmp;
    ccode
}

#[no_mangle]
pub unsafe extern "C" fn diag8c(addr: *mut diag8c, devno: *mut ccw_dev_id) -> core::ffi::c_int {
    diag_stat_inc(DIAG_STAT_X08C);
    let tmp = __diag8c_tmp_amode31;
    let ccode = diag_amode31_ops.diag8c.unwrap()(tmp, devno, core::mem::size_of::<diag8c>());
    *addr = *tmp;
    ccode
}

#[no_mangle]
pub unsafe extern "C" fn diag224(ptr: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _addr = __pa(ptr);
    diag_stat_inc(DIAG_STAT_X224);
    -95
}

#[no_mangle]
pub unsafe extern "C" fn diag26c(req: *mut core::ffi::c_void, resp: *mut core::ffi::c_void, subcode: diag26c_sc) -> core::ffi::c_int {
    diag_stat_inc(DIAG_STAT_X26C);
    diag_amode31_ops.diag26c.unwrap()(virt_to_phys(req), virt_to_phys(resp), subcode)
}

#[no_mangle]
pub unsafe extern "C" fn diag49c(subcode: core::ffi::c_ulong) -> core::ffi::c_int {
    diag_stat_inc(DIAG_STAT_X49C);
    let _ = subcode;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
