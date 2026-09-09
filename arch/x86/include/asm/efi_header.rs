/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

extern "C" {
    pub static mut efi_fw_vendor: usize;
    pub static mut efi_config_table: usize;
    pub static mut efi_mixed_mode_stack_pa: usize;
}

pub const EFI32_LOADER_SIGNATURE: &[u8; 4] = b"EL32";
pub const EFI64_LOADER_SIGNATURE: &[u8; 4] = b"EL64";
pub const ARCH_EFI_IRQ_FLAGS_MASK: u64 = X86_EFLAGS_IF;
pub const EFI_UNACCEPTED_UNIT_SIZE: usize = PMD_SIZE;

pub const EFI_X86_KERNEL_ALLOC_LIMIT: usize = EFI_ALLOC_LIMIT;

extern "C" {
    pub fn __efi_call(fp: *mut core::ffi::c_void, ...) -> u64;
    pub static mut efi_disable_ibt_for_runtime: bool;
}

#[inline]
pub unsafe fn efi_fpu_begin() {
    /* UEFI requires FCW and MXCSR to be initialized before entering UEFI code. */
    kernel_fpu_begin_mask(KFPU_387 | KFPU_MXCSR);
}

#[inline]
pub unsafe fn efi_fpu_end() {
    kernel_fpu_end();
}

#[macro_export]
macro_rules! efi_call {
    ($($arg:expr),* $(,)?) => {{
        __efi_call($($arg),*)
    }};
}

#[macro_export]
macro_rules! arch_efi_call_virt {
    ($p:expr, $f:ident $(, $arg:expr)*) => {{
        let ibt = ibt_save(efi_disable_ibt_for_runtime);
        let ret = efi_call!((&*$p).$f $(, $arg)*);
        ibt_restore(ibt);
        ret
    }};
}

extern "C" {
    pub fn efi_memblock_x86_reserve_range() -> i32;
    pub fn efi_print_memmap();
    pub fn efi_map_region(md: *mut efi_memory_desc_t);
    pub fn efi_map_region_fixed(md: *mut efi_memory_desc_t);
    pub fn efi_sync_low_kernel_mappings();
    pub fn efi_alloc_page_tables() -> i32;
    pub fn efi_setup_page_tables(pa_memmap: usize, num_pages: u32) -> i32;
    pub fn efi_runtime_update_mappings();
    pub fn efi_dump_pagetable();
    pub fn efi_apply_memmap_quirks();
    pub fn efi_reuse_config(tables: u64, nr_tables: i32) -> i32;
    pub fn efi_delete_dummy_variable();
    pub fn efi_crash_gracefully_on_page_fault(phys_addr: usize, regs: *const pt_regs);
    pub fn efi_unmap_boot_services();
    pub fn arch_efi_call_virt_setup();
    pub fn arch_efi_call_virt_teardown();
    pub static mut efi_setup: u64;
}

extern "C" {
    pub fn __efi64_thunk(...) -> u64;
}

#[macro_export]
macro_rules! efi64_thunk {
    ($($arg:expr),* $(,)?) => {{
        let __pad = [0u64; 3];
        __efi64_thunk($($arg),*, __pad.as_ptr())
    }};
}

#[inline]
pub fn efi_is_mixed() -> bool {
    IS_ENABLED_EFI_MIXED && IS_ENABLED_X86_64 && unsafe { !efi_enabled(EFI_64BIT) }
}

#[inline]
pub fn efi_runtime_supported() -> bool {
    if IS_ENABLED_X86_64 == unsafe { efi_enabled(EFI_64BIT) } { true } else { IS_ENABLED_EFI_MIXED }
}

extern "C" {
    pub fn parse_efi_setup(phys_addr: u64, data_len: u32);
    pub fn efi_thunk_runtime_setup();
    pub fn efi_set_virtual_address_map(
        memory_map_size: usize, descriptor_size: usize, descriptor_version: u32,
        virtual_map: *mut efi_memory_desc_t, systab_phys: usize,
    ) -> efi_status_t;
}

#[cfg(feature = "efi-mixed")]
pub const EFI_ALLOC_LIMIT: usize = if efi_is_64bit() { usize::MAX } else { u32::MAX as usize };

#[inline]
pub fn efi_is_64bit() -> bool {
    #[cfg(feature = "efi-mixed")]
    unsafe { efi_is64 }
    #[cfg(not(feature = "efi-mixed"))]
    { IS_ENABLED_X86_64 }
}

#[inline]
pub fn efi_is_native() -> bool { efi_is_64bit() }

#[cfg(feature = "efi-mixed")]
extern "C" { pub static efi_is64: bool; }

#[cfg(feature = "efi-mixed")]
#[inline]
pub unsafe fn efi64_zero_upper(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if !p.is_null() { *(p as *mut u32).add(1) = 0; }
    p
}

#[cfg(feature = "efi-mixed")]
#[inline]
pub fn efi64_convert_status(status: u64) -> u32 { (status | (status >> 32)) as u32 }

#[macro_export]
macro_rules! __efi64_split { ($v:expr) => { (($v) & u32::MAX as u64, ($v) >> 32) }; }
#[macro_export]
macro_rules! __efi64_argmap_free_pages { ($addr:expr, $size:expr) => { ($addr, 0, $size) }; }
#[macro_export]
macro_rules! __efi64_argmap_get_memory_map { ($mm_size:expr,$mm:expr,$key:expr,$size:expr,$ver:expr) => { ($mm_size,$mm,efi64_zero_upper($key),efi64_zero_upper($size),$ver) }; }
#[macro_export]
macro_rules! __efi64_argmap_allocate_pool { ($ty:expr,$size:expr,$buf:expr) => { ($ty,$size,efi64_zero_upper($buf)) }; }
#[macro_export]
macro_rules! __efi64_argmap_locate_handle_buffer { ($ty:expr,$proto:expr,$key:expr,$num:expr,$buf:expr) => { ($ty,$proto,$key,efi64_zero_upper($num),efi64_zero_upper($buf)) }; }
#[macro_export]
macro_rules! __efi64_argmap_create_event { ($ty:expr,$tpl:expr,$f:expr,$c:expr,$event:expr) => { ($ty,$tpl,$f,$c,efi64_zero_upper($event)) }; }
#[macro_export]
macro_rules! __efi64_argmap_set_timer { ($event:expr,$ty:expr,$time:expr) => { ($event,$ty,lower_32_bits($time),upper_32_bits($time)) }; }
#[macro_export]
macro_rules! __efi64_argmap_wait_for_event { ($num:expr,$event:expr,$idx:expr) => { ($num,$event,efi64_zero_upper($idx)) }; }
#[macro_export]
macro_rules! __efi64_argmap_handle_protocol { ($h:expr,$p:expr,$i:expr) => { ($h,$p,efi64_zero_upper($i)) }; }
#[macro_export]
macro_rules! __efi64_argmap_locate_protocol { ($p:expr,$r:expr,$i:expr) => { ($p,$r,efi64_zero_upper($i)) }; }
#[macro_export]
macro_rules! __efi64_argmap_locate_device_path { ($p:expr,$path:expr,$h:expr) => { ($p,$path,efi64_zero_upper($h)) }; }
#[macro_export]
macro_rules! __efi64_argmap_exit { ($h:expr,$s:expr,$sz:expr,$d:expr) => { ($h,efi64_convert_status($s),$sz,$d) }; }
#[macro_export]
macro_rules! __efi64_argmap_get_location { ($p:expr,$seg:expr,$bus:expr,$dev:expr,$func:expr) => { ($p,efi64_zero_upper($seg),efi64_zero_upper($bus),efi64_zero_upper($dev),efi64_zero_upper($func)) }; }
#[macro_export]
macro_rules! __efi64_argmap_load_file { ($p:expr,$path:expr,$pol:expr,$sz:expr,$buf:expr) => { ($p,$path,$pol,efi64_zero_upper($sz),$buf) }; }
#[macro_export]
macro_rules! __efi64_argmap_query_mode { ($g:expr,$m:expr,$sz:expr,$i:expr) => { ($g,$m,efi64_zero_upper($sz),efi64_zero_upper($i)) }; }
#[macro_export]
macro_rules! __efi64_argmap_set_position { ($p:expr) => { __efi64_split!($p) }; }
#[macro_export]
macro_rules! __efi64_argmap_open_volume { ($p:expr,$f:expr) => { ($p,efi64_zero_upper($f)) }; }
#[macro_export]
macro_rules! __efi64_argmap_get_next { ($p:expr,$h:expr,$t:expr,$r:expr,$ph:expr) => { ($p,$h,$t,efi64_zero_upper($r),efi64_zero_upper($ph)) }; }

#[cfg(feature = "efi-mixed")]
#[inline]
pub fn __efi64_widen_efi_status(status: u64) -> efi_status_t { ror64(rol32(status, 1), 1) as efi_status_t }

extern "C" {
    pub fn efi_reboot_required() -> bool;
    pub fn efi_is_table_address(phys_addr: usize) -> bool;
    pub fn efi_reserve_boot_services();
    pub fn efi_memmap_alloc(num_entries: u32, data: *mut efi_memory_map_data) -> i32;
    pub fn efi_memmap_install(data: *mut efi_memory_map_data) -> i32;
    pub fn efi_memmap_split_count(md: *mut efi_memory_desc_t, range: *mut range) -> i32;
    pub fn efi_memmap_insert(old_memmap: *mut efi_memory_map, buf: *mut core::ffi::c_void, mem: *mut efi_mem_range);
    pub fn __x86_efi_boot_mode() -> efi_secureboot_mode;
}

pub const arch_efi_boot_mode: efi_secureboot_mode = unsafe { __x86_efi_boot_mode() };

#[cfg(not(feature = "efi"))]
#[inline] pub fn parse_efi_setup(_phys_addr: u64, _data_len: u32) {}
#[cfg(not(feature = "efi"))]
#[inline] pub fn efi_reboot_required() -> bool { false }
#[cfg(not(feature = "efi"))]
#[inline] pub fn efi_is_table_address(_phys_addr: usize) -> bool { false }
#[cfg(not(feature = "efi"))]
#[inline] pub fn efi_reserve_boot_services() {}

#[cfg(feature = "efi-runtime-map")]
extern "C" {
    pub fn efi_get_runtime_map_size() -> i32;
    pub fn efi_get_runtime_map_desc_size() -> i32;
    pub fn efi_runtime_map_copy(buf: *mut core::ffi::c_void, bufsz: usize) -> i32;
}

#[cfg(not(feature = "efi-runtime-map"))]
#[inline] pub fn efi_get_runtime_map_size() -> i32 { 0 }
#[cfg(not(feature = "efi-runtime-map"))]
#[inline] pub fn efi_get_runtime_map_desc_size() -> i32 { 0 }
#[cfg(not(feature = "efi-runtime-map"))]
#[inline] pub fn efi_runtime_map_copy(_buf: *mut core::ffi::c_void, _bufsz: usize) -> i32 { 0 }

/* External kernel types, constants, and helpers are provided by other headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
