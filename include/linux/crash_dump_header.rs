/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kexec.h, linux/proc_fs.h, linux/elf.h, linux/pgtable.h,
// uapi/linux/vmcore.h

pub const ELFCORE_ADDR_MAX: u64 = u64::MAX;
pub const ELFCORE_ADDR_ERR: u64 = u64::MAX - 1;

extern "C" {
    pub static mut elfcorehdr_addr: u64;
    pub static mut elfcorehdr_size: u64;
    pub static mut dm_crypt_keys_addr: u64;

    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn elfcorehdr_alloc(addr: *mut u64, size: *mut u64) -> i32;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn elfcorehdr_free(addr: u64);
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn elfcorehdr_read(buf: *mut i8, count: usize, ppos: *mut u64) -> isize;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn elfcorehdr_read_notes(buf: *mut i8, count: usize, ppos: *mut u64) -> isize;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn elfcorehdr_fill_device_ram_ptload_elf64(
        phdr: *mut Elf64_Phdr, paddr: u64, size: u64,
    );
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn remap_oldmem_pfn_range(
        vma: *mut vm_area_struct, from: usize, pfn: usize, size: usize, prot: pgprot_t,
    ) -> i32;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn copy_oldmem_page(i: *mut iov_iter, pfn: usize, csize: usize, offset: usize) -> isize;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn copy_oldmem_page_encrypted(
        iter: *mut iov_iter, pfn: usize, csize: usize, offset: usize,
    ) -> isize;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn vmcore_cleanup();

    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn register_vmcore_cb(cb: *mut vmcore_cb);
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    pub fn unregister_vmcore_cb(cb: *mut vmcore_cb);
}

#[inline]
#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe fn is_kdump_kernel() -> bool {
    elfcorehdr_addr != ELFCORE_ADDR_MAX
}

#[inline]
#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe fn is_vmcore_usable() -> i32 {
    if elfcorehdr_addr != ELFCORE_ADDR_ERR && elfcorehdr_addr != ELFCORE_ADDR_MAX { 1 } else { 0 }
}

#[inline]
#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe fn vmcore_unusable() {
    elfcorehdr_addr = ELFCORE_ADDR_ERR;
}

#[inline]
#[cfg(not(feature = "CONFIG_CRASH_DUMP"))]
pub fn is_kdump_kernel() -> bool { false }

#[repr(C)]
pub struct vmcore_cb {
    pub pfn_is_ram: Option<unsafe extern "C" fn(*mut vmcore_cb, usize) -> bool>,
    pub get_device_ram: Option<unsafe extern "C" fn(*mut vmcore_cb, *mut list_head) -> i32>,
    pub next: list_head,
}

#[repr(C)]
pub struct vmcore_range {
    pub list: list_head,
    pub paddr: u64,
    pub size: u64,
    pub offset: loff_t,
}

#[inline]
pub unsafe fn vmcore_alloc_add_range(list: *mut list_head, paddr: u64, size: u64) -> i32 {
    // kzalloc_obj(), list_add_tail(), and -ENOMEM are supplied by kernel headers.
    let m: *mut vmcore_range = todo!("translate kernel kzalloc_obj dependency");
    if m.is_null() { return -12; }
    (*m).paddr = paddr;
    (*m).size = size;
    todo!("translate kernel list_add_tail dependency")
}

#[inline]
pub unsafe fn vmcore_free_ranges(_list: *mut list_head) {
    // list_for_each_entry_safe(), list_del(), and kfree() are kernel macros.
    todo!("translate kernel list iteration and freeing dependencies")
}

#[repr(C)]
pub struct vmcoredd_data {
    pub dump_name: [i8; VMCOREDD_MAX_NAME_BYTES],
    pub size: u32,
    pub vmcoredd_callback: Option<unsafe extern "C" fn(*mut vmcoredd_data, *mut core::ffi::c_void) -> i32>,
}

#[cfg(feature = "CONFIG_PROC_VMCORE_DEVICE_DUMP")]
extern "C" { pub fn vmcore_add_device_dump(data: *mut vmcoredd_data) -> i32; }

#[cfg(not(feature = "CONFIG_PROC_VMCORE_DEVICE_DUMP"))]
#[inline]
pub unsafe fn vmcore_add_device_dump(_data: *mut vmcoredd_data) -> i32 { -95 }

#[cfg(feature = "CONFIG_PROC_VMCORE")]
extern "C" { pub fn read_from_oldmem(iter: *mut iov_iter, count: usize, ppos: *mut u64, encrypted: bool) -> isize; }

#[cfg(not(feature = "CONFIG_PROC_VMCORE"))]
#[inline]
pub unsafe fn read_from_oldmem(_iter: *mut iov_iter, _count: usize, _ppos: *mut u64, _encrypted: bool) -> isize { -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
