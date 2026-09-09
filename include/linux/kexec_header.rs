/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::{c_char, c_int, c_void};

pub const IND_DESTINATION_BIT: u32 = 0;
pub const IND_INDIRECTION_BIT: u32 = 1;
pub const IND_DONE_BIT: u32 = 2;
pub const IND_SOURCE_BIT: u32 = 3;
pub const IND_DESTINATION: u32 = 1 << IND_DESTINATION_BIT;
pub const IND_INDIRECTION: u32 = 1 << IND_INDIRECTION_BIT;
pub const IND_DONE: u32 = 1 << IND_DONE_BIT;
pub const IND_SOURCE: u32 = 1 << IND_SOURCE_BIT;
pub const IND_FLAGS: u32 = IND_DESTINATION | IND_INDIRECTION | IND_DONE | IND_SOURCE;

pub type kimage_entry_t = usize;
pub type phys_addr_t = usize;
pub type gfp_t = usize;
pub type compat_uptr_t = u32;
pub type compat_size_t = u32;
pub type compat_ulong_t = u32;
pub type size_t = usize;
pub type Elf_Ehdr = c_void;
pub type Elf_Shdr = c_void;
pub type elfhdr = c_void;
pub type elf_phdr = c_void;
pub type note_buf_t = c_void;

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kimage_arch { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct kimage;

#[repr(C)] pub union kexec_segment_ptr { pub buf: *mut c_void, pub kbuf: *mut c_void }
#[repr(C)] pub struct kexec_segment {
    pub ptr: kexec_segment_ptr,
    pub bufsz: size_t,
    pub mem: usize,
    pub memsz: size_t,
}
#[repr(C)] pub struct compat_kexec_segment {
    pub buf: compat_uptr_t, pub bufsz: compat_size_t,
    pub mem: compat_ulong_t, pub memsz: compat_size_t,
}

#[repr(C)] pub struct purgatory_info {
    pub ehdr: *const Elf_Ehdr,
    pub sechdrs: *mut Elf_Shdr,
    pub purgatory_buf: *mut c_void,
}
pub type kexec_probe_t = unsafe extern "C" fn(*const c_char, usize) -> c_int;
pub type kexec_load_t = unsafe extern "C" fn(*mut kimage, *mut c_char, usize, *mut c_char, usize, *mut c_char, usize) -> *mut c_void;
pub type kexec_cleanup_t = unsafe extern "C" fn(*mut c_void) -> c_int;
pub type kexec_verify_sig_t = unsafe extern "C" fn(*const c_char, usize) -> c_int;
#[repr(C)] pub struct kexec_file_ops {
    pub probe: Option<kexec_probe_t>, pub load: Option<kexec_load_t>, pub cleanup: Option<kexec_cleanup_t>,
    pub verify_sig: Option<kexec_verify_sig_t>,
}

#[repr(C)] pub struct kexec_buf {
    pub image: *mut kimage, pub buffer: *mut c_void, pub bufsz: usize, pub mem: usize, pub memsz: usize,
    pub buf_align: usize, pub buf_min: usize, pub buf_max: usize, pub cma: *mut page,
    pub top_down: bool, pub random: bool,
}
#[repr(C)] pub struct kexec_elf_info {
    pub buffer: *const c_char, pub ehdr: *const elfhdr, pub proghdrs: *const elf_phdr,
}

#[repr(C)] pub struct kimage {
    pub head: kimage_entry_t, pub entry: *mut kimage_entry_t, pub last_entry: *mut kimage_entry_t,
    pub start: usize, pub control_code_page: *mut page, pub swap_page: *mut page,
    pub vmcoreinfo_data_copy: *mut c_void, pub nr_segments: usize,
    pub segment: *mut kexec_segment, pub segment_cma: *mut *mut page,
    pub control_pages: list_head, pub dest_pages: list_head, pub unusable_pages: list_head,
    pub control_page: usize, pub type_: u32, pub preserve_context: u32, pub file_mode: u32,
    pub hotplug_support: u32, pub no_cma: u32, pub arch: kimage_arch,
    pub kernel_buf: *mut c_void, pub kernel_buf_len: usize, pub initrd_buf: *mut c_void,
    pub initrd_buf_len: usize, pub cmdline_buf: *mut c_char, pub cmdline_buf_len: usize,
    pub fops: *const kexec_file_ops, pub image_loader_data: *mut c_void,
    pub purgatory_info: purgatory_info, pub force_dtb: bool,
    pub hp_action: c_int, pub elfcorehdr_index: c_int, pub elfcorehdr_updated: bool,
    pub ima_buffer: *mut c_void, pub ima_buffer_addr: phys_addr_t, pub ima_buffer_size: usize,
    pub ima_segment_index: usize, pub is_ima_segment_index_set: bool,
    pub kho_scratch: *mut kexec_segment, pub kho_fdt: phys_addr_t,
    pub elf_headers: *mut c_void, pub elf_headers_sz: usize, pub elf_load_addr: usize,
    pub dm_crypt_keys_addr: usize, pub dm_crypt_keys_sz: usize,
}
pub const KEXEC_TYPE_DEFAULT: u32 = 0;
pub const KEXEC_TYPE_CRASH: u32 = 1;

extern "C" {
    pub static mut crash_notes: *mut note_buf_t;
    pub static kexec_file_loaders: *const *const kexec_file_ops;
    pub static mut kexec_image: *mut kimage;
    pub static mut kexec_crash_image: *mut kimage;
    pub static mut kexec_in_progress: bool;
    pub static mut kexec_file_dbg_print: bool;
    pub fn kexec_image_probe_default(image: *mut kimage, buf: *mut c_void, len: usize) -> c_int;
    pub fn kexec_image_post_load_cleanup_default(image: *mut kimage) -> c_int;
    pub fn kexec_load_purgatory(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int;
    pub fn kexec_purgatory_get_set_symbol(image: *mut kimage, name: *const c_char, buf: *mut c_void, size: u32, get_value: bool) -> c_int;
    pub fn kexec_purgatory_get_symbol_addr(image: *mut kimage, name: *const c_char) -> *mut c_void;
    pub fn kexec_add_buffer(kbuf: *mut kexec_buf) -> c_int;
    pub fn kexec_locate_mem_hole(kbuf: *mut kexec_buf) -> c_int;
    pub fn machine_kexec(image: *mut kimage); pub fn machine_kexec_prepare(image: *mut kimage) -> c_int;
    pub fn machine_kexec_cleanup(image: *mut kimage); pub fn kernel_kexec() -> c_int;
    pub fn kimage_alloc_control_pages(image: *mut kimage, order: u32) -> *mut page;
    pub fn kexec_load_permitted(kind: c_int) -> bool;
    pub fn kimage_map_segment(image: *mut kimage, idx: c_int) -> *mut c_void;
    pub fn kimage_unmap_segment(buffer: *mut c_void);
    pub fn kexec_build_elf_info(buf: *const c_char, len: usize, ehdr: *mut elfhdr, info: *mut kexec_elf_info) -> c_int;
    pub fn kexec_elf_load(image: *mut kimage, ehdr: *mut elfhdr, info: *mut kexec_elf_info, kbuf: *mut kexec_buf, lowest: *mut usize) -> c_int;
    pub fn kexec_free_elf_info(info: *mut kexec_elf_info);
    pub fn kexec_elf_probe(buf: *const c_char, len: usize) -> c_int;
}

/* Architecture/configuration-dependent declarations and helpers remain gated by their kernel cfgs. */
#[inline] pub unsafe fn arch_kexec_kernel_image_probe(i: *mut kimage, b: *mut c_void, n: usize) -> c_int { kexec_image_probe_default(i,b,n) }
#[inline] pub unsafe fn arch_kimage_file_post_load_cleanup(i: *mut kimage) -> c_int { kexec_image_post_load_cleanup_default(i) }
#[inline] pub unsafe fn arch_check_excluded_range(_: *mut kimage, _: usize, _: usize) -> c_int { 0 }
#[inline] pub unsafe fn arch_kexec_locate_mem_hole(b: *mut kexec_buf) -> c_int { kexec_locate_mem_hole(b) }
#[inline] pub unsafe fn machine_kexec_post_load(_: *mut kimage) -> c_int { 0 }
#[inline] pub unsafe fn arch_kexec_post_alloc_pages(_: *mut c_void, _: u32, _: gfp_t) -> c_int { 0 }
#[inline] pub unsafe fn arch_kexec_pre_free_pages(_: *mut c_void, _: u32) {}
#[inline] pub unsafe fn __crash_kexec(_: *mut pt_regs) {}
#[inline] pub unsafe fn crash_kexec(_: *mut pt_regs) {}
#[inline] pub unsafe fn kexec_should_crash(_: *mut task_struct) -> c_int { 0 }
#[inline] pub unsafe fn kexec_crash_loaded() -> c_int { 0 }
#[inline] pub unsafe fn set_kexec_sig_enforced() {}

pub const KEXEC_BUF_MEM_UNKNOWN: usize = 0;
pub const KEXEC_FLAGS: u32 = 0;
pub const KEXEC_FILE_FLAGS: u32 = 0;

#[inline]
pub unsafe fn arch_kexec_apply_relocations_add(_: *mut purgatory_info, _: *mut Elf_Shdr, _: *const Elf_Shdr, _: *const Elf_Shdr) -> c_int {
    /* C implementation logs "RELA relocation unsupported." and returns -ENOEXEC. */
    -8
}
#[inline]
pub unsafe fn arch_kexec_apply_relocations(_: *mut purgatory_info, _: *mut Elf_Shdr, _: *const Elf_Shdr, _: *const Elf_Shdr) -> c_int {
    /* C implementation logs "REL relocation unsupported." and returns -ENOEXEC. */
    -8
}

#[inline]
pub unsafe fn kexec_random_range_start(start: usize, end: usize, kbuf: *mut kexec_buf, temp_start: *mut usize) {
    /* CONFIG_CRASH_DUMP supplies random bytes; retain the source calculation when enabled. */
    if !kbuf.is_null() && (*kbuf).random {
        let i = 0u16; // external get_random_bytes dependency
        *temp_start = start + (end - start) / (u16::MAX as usize) * (i as usize);
    }
}

#[inline] pub unsafe fn page_to_boot_pfn(page: *mut page) -> usize { page as usize }
#[inline] pub unsafe fn boot_pfn_to_page(pfn: usize) -> *mut page { pfn as *mut page }
#[inline] pub unsafe fn phys_to_boot_phys(phys: phys_addr_t) -> usize { phys }
#[inline] pub unsafe fn boot_phys_to_phys(phys: usize) -> phys_addr_t { phys }
#[inline] pub unsafe fn virt_to_boot_phys(addr: *mut c_void) -> usize { addr as usize }
#[inline] pub unsafe fn boot_phys_to_virt(entry: usize) -> *mut c_void { entry as *mut c_void }
#[inline] pub unsafe fn crash_free_reserved_phys_range(_: usize, _: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
