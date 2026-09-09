/* SPDX-License-Identifier: GPL-2.0 */

pub const IPL_START: usize = 0x200;

/* Types and symbols supplied by the included kernel headers. */
#[repr(C)] pub struct machine_info { _private: [u8; 0] }
#[repr(C)] pub struct reserved_range_type { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct psw_t { _private: [u8; 0] }
#[repr(C)]
pub struct vmlinux_info {
    pub entry: usize,
    pub image_size: usize, /* does not include .bss */
    pub bss_size: usize, /* uncompressed image .bss size */
    pub bootdata_off: usize,
    pub bootdata_size: usize,
    pub bootdata_preserved_off: usize,
    pub bootdata_preserved_size: usize,
    pub got_start: usize,
    pub got_end: usize,
    pub amode31_size: usize,
    pub init_mm_off: usize,
    pub swapper_pg_dir_off: usize,
    pub invalid_pg_dir_off: usize,
    pub alt_instructions: usize,
    pub alt_instructions_end: usize,
    /* CONFIG_STACKPROTECTOR fields, when enabled, are present in the C layout. */
    #[cfg(feature = "CONFIG_STACKPROTECTOR")]
    pub stack_prot_start: usize,
    #[cfg(feature = "CONFIG_STACKPROTECTOR")]
    pub stack_prot_end: usize,
    /* CONFIG_KASAN fields, when enabled, are present in the C layout. */
    #[cfg(feature = "CONFIG_KASAN")]
    pub kasan_early_shadow_page_off: usize,
    #[cfg(feature = "CONFIG_KASAN")]
    pub kasan_early_shadow_pte_off: usize,
    #[cfg(feature = "CONFIG_KASAN")]
    pub kasan_early_shadow_pmd_off: usize,
    #[cfg(feature = "CONFIG_KASAN")]
    pub kasan_early_shadow_pud_off: usize,
    #[cfg(feature = "CONFIG_KASAN")]
    pub kasan_early_shadow_p4d_off: usize,
}

extern "C" {
    pub fn startup_kernel();
    pub fn detect_max_physmem_end() -> usize;
    pub fn detect_physmem_online_ranges(max_physmem_end: usize);
    pub fn physmem_set_usable_limit(limit: usize);
    pub fn physmem_reserve(ty: reserved_range_type, addr: usize, size: usize);
    pub fn physmem_free(ty: reserved_range_type);
    pub fn physmem_alloc_or_die(ty: reserved_range_type, size: usize, align: usize) -> usize;
    pub fn physmem_alloc(ty: reserved_range_type, size: usize, align: usize, die_on_oom: bool) -> usize;
    pub fn physmem_alloc_range(ty: reserved_range_type, size: usize, align: usize, min: usize, max: usize, die_on_oom: bool) -> usize;
    pub fn get_physmem_alloc_pos() -> usize;
    pub fn dump_physmem_reserved();
    pub fn ipl_report_certs_intersects(addr: usize, size: usize, intersection_start: *mut usize) -> bool;
    pub fn is_ipl_block_dump() -> bool;
    pub fn store_ipl_parmblock();
    pub fn read_ipl_report() -> i32;
    pub fn save_ipl_cert_comp_list();
    pub fn setup_boot_command_line();
    pub fn parse_boot_command_line();
    pub fn verify_facilities();
    pub fn print_missing_facilities();
    pub fn sclp_early_setup_buffer();
    pub fn alt_debug_setup(s: *mut i8);
    pub fn do_pgm_check(regs: *mut pt_regs);
    pub fn randomize_within_range(size: usize, align: usize, min: usize, max: usize) -> usize;
    pub fn setup_vmem(kernel_start: usize, kernel_end: usize, asce_limit: usize);
    pub fn boot_printk(fmt: *const i8, ...) -> i32;
    pub fn print_stacktrace(sp: usize);
    pub fn error(m: *mut i8);
    pub fn get_random(limit: usize, value: *mut usize) -> i32;
    pub fn boot_rb_dump();
    pub fn jump_to_kernel(psw: *mut psw_t) -> !;
}

#[macro_export]
macro_rules! boot_fmt { ($fmt:expr) => { $fmt }; }
#[macro_export] macro_rules! boot_emerg { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_EMERG, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_alert { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_ALERT, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_crit { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_CRIT, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_err { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_ERR, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_warn { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_WARNING, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_notice { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_NOTICE, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_info { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_INFO, boot_fmt!($($arg)*)),) } }; }
#[macro_export] macro_rules! boot_debug { ($($arg:tt)*) => { unsafe { boot_printk(concat!(KERN_DEBUG, boot_fmt!($($arg)*)),) } }; }

extern "C" {
    pub static mut machine: machine_info;
    pub static mut boot_console_loglevel: i32;
    pub static mut boot_ignore_loglevel: bool;
    pub static kernel_version: i8;
    pub static mut __boot_data_start: i8;
    pub static mut __boot_data_end: i8;
    pub static mut __boot_data_preserved_start: i8;
    pub static mut __boot_data_preserved_end: i8;
    pub static mut __vmlinux_relocs_64_start: i8;
    pub static mut __vmlinux_relocs_64_end: i8;
    pub static mut _decompressor_syms_start: i8;
    pub static mut _decompressor_syms_end: i8;
    pub static mut _stack_start: i8;
    pub static mut _stack_end: i8;
    pub static mut _end: i8;
    pub static mut _decompressor_end: i8;
    pub static mut _compressed_start: u8;
    pub static mut _compressed_end: u8;
    pub static mut memory_limit: usize;
    pub static mut vmalloc_size: usize;
    pub static mut vmalloc_size_set: i32;
    pub static mut _vmlinux_info: vmlinux_info;
}

pub use _vmlinux_info as vmlinux;

#[inline]
pub fn intersects(addr0: usize, size0: usize, addr1: usize, size1: usize) -> bool {
    addr0.wrapping_add(size0) > addr1 && addr1.wrapping_add(size1) > addr0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
