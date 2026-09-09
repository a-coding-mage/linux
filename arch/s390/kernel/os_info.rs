// SPDX-License-Identifier: GPL-2.0
/*
 * OS info memory interface
 *
 * Copyright IBM Corp. 2012
 * Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut OS_INFO: os_info = os_info { };

/*
 * Compute checksum over OS info structure
 */
pub unsafe fn os_info_csum(os_info: *mut os_info) -> u32 {

    let size: i32 = core::mem::size_of::<os_info>() as i32
        - core::mem::offset_of!(os_info, version_major) as i32;
    cksm(
        (os_info as *mut u8).add(core::mem::offset_of!(os_info, version_major)),
        size as u64,
        0,
    ) as u32
}

/*
 * Add crashkernel info to OS info and update checksum
 */
pub unsafe fn os_info_crashkernel_add(base: usize, size: usize) {
    OS_INFO.crashkernel_addr = base as u64;
    OS_INFO.crashkernel_size = size as u64;
    OS_INFO.csum = os_info_csum(&raw mut OS_INFO);
}

/*
 * Add OS info data entry and update checksum
 */
pub unsafe fn os_info_entry_add_data(nr: i32, ptr: *mut core::ffi::c_void, size: u64) {
    OS_INFO.entry[nr as usize].addr = __pa(ptr);
    OS_INFO.entry[nr as usize].size = size;
    OS_INFO.entry[nr as usize].csum = cksm(ptr as *mut u8, size, 0) as u32;
    OS_INFO.csum = os_info_csum(&raw mut OS_INFO);
}

/*
 * Add OS info value entry and update checksum
 */
pub unsafe fn os_info_entry_add_val(nr: i32, value: u64) {
    OS_INFO.entry[nr as usize].val = value;
    OS_INFO.entry[nr as usize].size = 0;
    OS_INFO.entry[nr as usize].csum = 0;
    OS_INFO.csum = os_info_csum(&raw mut OS_INFO);
}

/*
 * Initialize OS info structure and set lowcore pointer
 */
pub unsafe fn os_info_init() {
    // BUILD_BUG_ON(sizeof(struct os_info) != PAGE_SIZE);
    OS_INFO.version_major = OS_INFO_VERSION_MAJOR;
    OS_INFO.version_minor = OS_INFO_VERSION_MINOR;
    OS_INFO.magic = OS_INFO_MAGIC;
    os_info_entry_add_val(OS_INFO_IDENTITY_BASE, __identity_base as u64);
    os_info_entry_add_val(OS_INFO_KASLR_OFFSET, kaslr_offset());
    os_info_entry_add_val(OS_INFO_KASLR_OFF_PHYS, __kaslr_offset_phys);
    os_info_entry_add_val(OS_INFO_VMEMMAP, vmemmap as usize as u64);
    os_info_entry_add_val(OS_INFO_AMODE31_START, AMODE31_START);
    os_info_entry_add_val(OS_INFO_AMODE31_END, AMODE31_END);
    os_info_entry_add_val(OS_INFO_IMAGE_START, _stext as usize as u64);
    os_info_entry_add_val(OS_INFO_IMAGE_END, _end as usize as u64);
    os_info_entry_add_val(OS_INFO_IMAGE_PHYS, __pa_symbol(_stext));
    OS_INFO.csum = os_info_csum(&raw mut OS_INFO);
    let abs_lc = get_abs_lowcore();
    (*abs_lc).os_info = __pa(&raw mut OS_INFO);
    put_abs_lowcore(abs_lc);
}

#[cfg(CONFIG_CRASH_DUMP)]
static mut os_info_old: *mut os_info = core::ptr::null_mut();

#[cfg(CONFIG_CRASH_DUMP)]
/*
 * Allocate and copy OS info entry from oldmem
 */
unsafe fn os_info_old_alloc(nr: i32, align: usize) {
    let addr: usize = (*os_info_old).entry[nr as usize].addr as usize;
    let mut size: usize = 0;
    let mut msg: *const u8;
    if addr == 0 {
        msg = b"not available\0".as_ptr();
        goto_fail!(msg, addr, size);
    }
    size = (*os_info_old).entry[nr as usize].size as usize;
    let buf = kmalloc(size + align - 1, GFP_KERNEL);
    if buf.is_null() {
        msg = b"alloc failed\0".as_ptr();
        goto_fail!(msg, addr, size);
    }
    let buf_align = PTR_ALIGN(buf, align);
    if copy_oldmem_kernel(buf_align, addr, size) != 0 {
        msg = b"copy failed\0".as_ptr();
        kfree(buf);
        goto_fail!(msg, addr, size);
    }
    let csum = cksm(buf_align, size as u64, 0) as u32;
    if csum != (*os_info_old).entry[nr as usize].csum {
        msg = b"checksum failed\0".as_ptr();
        kfree(buf);
        goto_fail!(msg, addr, size);
    }
    (*os_info_old).entry[nr as usize].addr = buf_align as usize as u64;
    msg = b"copied\0".as_ptr();
    pr_info_entry(nr, msg, addr, size);
    return;
}

#[cfg(CONFIG_CRASH_DUMP)]
/*
 * Initialize os info and os info entries from oldmem
 */
unsafe fn os_info_old_init() {
    static mut os_info_init: i32 = 0;
    let mut addr: usize = 0;
    if os_info_init != 0 { return; }
    if !oldmem_data.start && !is_ipl_type_dump() { goto_fail_init!(); }
    if copy_oldmem_kernel(&mut addr as *mut usize as *mut u8, __LC_OS_INFO, core::mem::size_of::<usize>()) != 0 { goto_fail_init!(); }
    if addr == 0 || addr % PAGE_SIZE != 0 { goto_fail_init!(); }
    os_info_old = kzalloc_os_info();
    if os_info_old.is_null() { goto_fail_init!(); }
    if copy_oldmem_kernel(os_info_old as *mut u8, addr, core::mem::size_of::<os_info>()) != 0 { kfree(os_info_old); goto_fail_init!(); }
    if (*os_info_old).magic != OS_INFO_MAGIC || (*os_info_old).csum != os_info_csum(os_info_old) || (*os_info_old).version_major > OS_INFO_VERSION_MAJOR { kfree(os_info_old); goto_fail_init!(); }
    os_info_old_alloc(OS_INFO_VMCOREINFO, 1);
    os_info_old_alloc(OS_INFO_REIPL_BLOCK, 1);
    pr_info_crashkernel((*os_info_old).crashkernel_addr as usize, (*os_info_old).crashkernel_size as usize);
    os_info_init = 1;
    return;
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn os_info_old_entry(nr: i32, size: *mut usize) -> *mut core::ffi::c_void {
    os_info_old_init();
    if os_info_old.is_null() || (*os_info_old).entry[nr as usize].addr == 0 { return core::ptr::null_mut(); }
    *size = (*os_info_old).entry[nr as usize].size as usize;
    (*os_info_old).entry[nr as usize].addr as usize as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
