// SPDX-License-Identifier: GPL-2.0-only
// x86_64 production owner of arch/x86/kernel/setup.c (aec86a8).
// Canonical header types are generated for the selected kernel configuration.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/x86_setup_generated.rs"));
}
use bindings as b;
use core::arch::global_asm;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, write_bytes};
use kernel::ffi::{c_char, c_int, c_ulong, c_void};
#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

macro_rules! cstr { ($s:expr) => { concat!($s, "\0").as_ptr().cast::<c_char>() }; }
#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkIndexEntry(b::pi_entry);
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkIndexEntry {}
#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkIndexPointer(*const PrintkIndexEntry);
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkIndexPointer {}
// Canonical printk_index_wrap: full level-prefixed format, NULL level/prefix,
// a packed pi_entry and a retained pointer in .printk_index.
macro_rules! log { ($function:literal, $level:literal, $format:literal $(, $arg:expr)* $(,)?) => {{
    #[cfg(CONFIG_PRINTK_INDEX)]
    #[used]
    static ENTRY: PrintkIndexEntry = PrintkIndexEntry(b::pi_entry {
        fmt: cstr!(concat!($level, $format)), func: cstr!($function),
        file: cstr!(file!()), line: line!(), level: core::ptr::null(),
        subsys_fmt_prefix: core::ptr::null(),
    });
    #[cfg(CONFIG_PRINTK_INDEX)]
    #[used]
    #[link_section = ".printk_index"]
    static POINTER: PrintkIndexPointer = PrintkIndexPointer(addr_of!(ENTRY));
    #[cfg(CONFIG_PRINTK)]
    { b::_printk(cstr!(concat!($level, $format)) $(, $arg)* ) }
    #[cfg(not(CONFIG_PRINTK))]
    { $(let _ = $arg;)* 0 }
}}; }

#[no_mangle]
pub static mut max_low_pfn_mapped: c_ulong = 0;
#[no_mangle]
pub static mut max_pfn_mapped: c_ulong = 0;
#[cfg(CONFIG_DMI)]
#[used]
#[link_section = ".bss..brk"]
static mut __brk_dmi_alloc: [u8; 65536] = [0; 65536];
// Link-time integer-valued pointer relocations cannot be Rust const casts.
// These two .quad initializers are exactly the C unsigned-long initializers.
global_asm!(
    ".pushsection .data,\"aw\"\n.balign 8",
    ".globl _brk_start\n.type _brk_start,@object\n.size _brk_start,8",
    "_brk_start: .quad __brk_base",
    ".globl _brk_end\n.type _brk_end,@object\n.size _brk_end,8",
    "_brk_end: .quad __brk_base\n.popsection",
);
#[no_mangle]
pub static mut boot_params: b::boot_params = unsafe { zeroed() };
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut boot_cpu_data: b::cpuinfo_x86 = unsafe { zeroed() };
global_asm!(".globl __pi_boot_cpu_data\n.set __pi_boot_cpu_data, {cpu}", cpu = sym boot_cpu_data);
ffi_export::export_symbol!(boot_cpu_data, boot_cpu_data, "", "");
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut mmu_cr4_features: c_ulong = 0;
#[cfg(CONFIG_IMA)]
static mut ima_kexec_buffer_phys: b::phys_addr_t = 0;
#[cfg(CONFIG_IMA)]
static mut ima_kexec_buffer_size: usize = 0;
#[no_mangle]
pub static mut bootloader_type: c_int = 0;
#[no_mangle]
pub static mut bootloader_version: c_int = 0;
#[no_mangle]
pub static mut sysfb_primary_display: b::sysfb_display_info = unsafe { zeroed() };
ffi_export::export_symbol!(sysfb_primary_display, sysfb_primary_display, "", "");
#[no_mangle]
pub static mut saved_video_mode: c_ulong = 0;
#[link_section = ".init.data"]
static mut command_line: [c_char; b::RUST_SETUP_COMMAND_LINE_SIZE as usize] =
    [0; b::RUST_SETUP_COMMAND_LINE_SIZE as usize];
#[cfg(CONFIG_CMDLINE_BOOL)]
const fn builtin_command_line() -> [c_char; b::RUST_SETUP_COMMAND_LINE_SIZE as usize] {
    let mut value = [0; b::RUST_SETUP_COMMAND_LINE_SIZE as usize];
    // Kconfig emits the string-valued CONFIG_CMDLINE cfg alongside autoconf.h.
    // Bindgen can omit empty string macros; the original initializer is all zero.
    #[cfg(not(CONFIG_CMDLINE = ""))]
    {
        let bytes = b::RUST_SETUP_BUILTIN_CMDLINE;
        let mut i = 0;
        while i < bytes.len() && i < value.len() {
            value[i] = bytes[i] as c_char;
            i += 1;
        }
    }
    value
}
#[cfg(CONFIG_CMDLINE_BOOL)]
#[no_mangle]
pub static mut builtin_cmdline: [c_char; b::RUST_SETUP_COMMAND_LINE_SIZE as usize] = builtin_command_line();
#[cfg(CONFIG_CMDLINE_BOOL)]
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut builtin_cmdline_added: bool = false;
#[cfg(CONFIG_EDD)]
#[no_mangle]
pub static mut edd: b::edd = unsafe { zeroed() };
#[cfg(CONFIG_EDD = "m")]
ffi_export::export_symbol!(edd, edd, "", "");

const fn resource(name: *const c_char, start: u64, end: u64, flags: c_ulong) -> b::resource {
    let mut value: b::resource = unsafe { zeroed() };
    value.name = name;
    value.start = start;
    value.end = end;
    value.flags = flags;
    value
}
const RAM_FLAGS: c_ulong = b::RUST_SETUP_RESOURCE_RAM_FLAGS as c_ulong;
const IO_FLAGS: c_ulong = b::RUST_SETUP_RESOURCE_IO_FLAGS as c_ulong;
static mut code_resource: b::resource = resource(cstr!("Kernel code"), 0, 0, RAM_FLAGS);
static mut rodata_resource: b::resource = resource(cstr!("Kernel rodata"), 0, 0, RAM_FLAGS);
static mut data_resource: b::resource = resource(cstr!("Kernel data"), 0, 0, RAM_FLAGS);
static mut bss_resource: b::resource = resource(cstr!("Kernel bss"), 0, 0, RAM_FLAGS);
static mut standard_io_resources: [b::resource; 10] = [
    resource(cstr!("dma1"), 0x00, 0x1f, IO_FLAGS),
    resource(cstr!("pic1"), 0x20, 0x21, IO_FLAGS),
    resource(cstr!("timer0"), 0x40, 0x43, IO_FLAGS),
    resource(cstr!("timer1"), 0x50, 0x53, IO_FLAGS),
    resource(cstr!("keyboard"), 0x60, 0x60, IO_FLAGS),
    resource(cstr!("keyboard"), 0x64, 0x64, IO_FLAGS),
    resource(cstr!("dma page reg"), 0x80, 0x8f, IO_FLAGS),
    resource(cstr!("pic2"), 0xa0, 0xa1, IO_FLAGS),
    resource(cstr!("dma2"), 0xc0, 0xdf, IO_FLAGS),
    resource(cstr!("fpu"), 0xf0, 0xff, IO_FLAGS),
];

const fn sysctl_int(name: *const c_char, data: *mut c_int, mode: u16) -> b::ctl_table {
    let mut value: b::ctl_table = unsafe { zeroed() };
    value.procname = name;
    value.data = data.cast();
    value.maxlen = size_of::<c_int>() as c_int;
    value.mode = mode;
    value.proc_handler = Some(b::proc_dointvec);
    value
}
#[cfg(CONFIG_ACPI_SLEEP)]
const fn sysctl_acpi() -> b::ctl_table {
    let mut value: b::ctl_table = unsafe { zeroed() };
    value.procname = cstr!("acpi_video_flags");
    value.data = addr_of_mut!(b::acpi_realmode_flags).cast();
    value.maxlen = size_of::<c_ulong>() as c_int;
    value.mode = 0o644;
    value.proc_handler = Some(b::proc_doulongvec_minmax);
    value
}
#[repr(transparent)]
struct SysctlTable([b::ctl_table; 6 + cfg!(CONFIG_ACPI_SLEEP) as usize]);
// Immutable descriptor array; pointees are synchronized by the sysctl API.
unsafe impl Sync for SysctlTable {}
static x86_sysctl_table: SysctlTable = SysctlTable([
    sysctl_int(cstr!("unknown_nmi_panic"), addr_of_mut!(b::unknown_nmi_panic), 0o644),
    sysctl_int(cstr!("panic_on_unrecovered_nmi"), addr_of_mut!(b::panic_on_unrecovered_nmi), 0o644),
    sysctl_int(cstr!("panic_on_io_nmi"), addr_of_mut!(b::panic_on_io_nmi), 0o644),
    sysctl_int(cstr!("bootloader_type"), addr_of_mut!(bootloader_type), 0o444),
    sysctl_int(cstr!("bootloader_version"), addr_of_mut!(bootloader_version), 0o444),
    sysctl_int(cstr!("io_delay_type"), addr_of_mut!(b::io_delay_type), 0o644),
    #[cfg(CONFIG_ACPI_SLEEP)]
    sysctl_acpi(),
]);
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_setup_init_x86_sysctl() -> c_int {
    #[cfg(CONFIG_SYSCTL)]
    b::__register_sysctl_init(cstr!("kernel"), addr_of!(x86_sysctl_table).cast(),
        cstr!("x86_sysctl_table"), 6 + cfg!(CONFIG_ACPI_SLEEP) as usize);
    0
}

#[inline]
unsafe fn pa_symbol(p: c_ulong) -> u64 { b::rust_setup_pa_symbol(p) as u64 }
#[inline]
unsafe fn va(p: u64) -> c_ulong { p.wrapping_add(b::page_offset_base as u64) as c_ulong }
#[inline]
fn page_align(p: u64) -> u64 {
    p.wrapping_add(b::RUST_SETUP_PAGE_SIZE as u64 - 1) & !(b::RUST_SETUP_PAGE_SIZE as u64 - 1)
}
#[inline]
unsafe fn reserve_kern(base: u64, size: u64) {
    b::__memblock_reserve(base, size, b::RUST_SETUP_NUMA_NO_NODE as c_int, b::MEMBLOCK_RSRV_KERN);
}
#[inline]
unsafe fn copy_edd() {
    #[cfg(CONFIG_EDD)]
    {
        core::ptr::copy_nonoverlapping(addr_of!(boot_params.edd_mbr_sig_buffer).cast::<u8>(),
            addr_of_mut!(edd.mbr_signature).cast::<u8>(), size_of::<[u32; b::EDD_MBR_SIG_MAX as usize]>());
        core::ptr::copy_nonoverlapping(addr_of!(boot_params.eddbuf).cast::<u8>(),
            addr_of_mut!(edd.edd_info).cast::<u8>(), size_of::<[b::edd_info; b::EDDMAXNR as usize]>());
        edd.mbr_signature_nr = boot_params.edd_mbr_sig_buf_entries;
        edd.edd_info_nr = boot_params.eddbuf_entries;
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn extend_brk(size: usize, align: usize) -> *mut c_void {
    let mask = align.wrapping_sub(1);
    if b::_brk_start == 0 || align & mask != 0 { b::rust_setup_bug(); }
    b::_brk_end = b::_brk_end.wrapping_add(mask) & !mask;
    if b::_brk_end.wrapping_add(size) > addr_of!(b::__brk_limit) as usize { b::rust_setup_bug(); }
    let ret = b::_brk_end as *mut c_void;
    b::_brk_end = b::_brk_end.wrapping_add(size);
    write_bytes(ret.cast::<u8>(), 0, size);
    ret
}
#[link_section = ".init.text"]
unsafe fn reserve_brk() {
    if b::_brk_end > b::_brk_start {
        reserve_kern(pa_symbol(b::_brk_start), b::_brk_end.wrapping_sub(b::_brk_start) as u64);
    }
    b::_brk_start = 0;
}
#[cfg(CONFIG_BLK_DEV_INITRD)]
#[link_section = ".init.text"]
unsafe fn get_ramdisk_image() -> u64 {
    let image = boot_params.hdr.ramdisk_image as u64 | (boot_params.ext_ramdisk_image as u64) << 32;
    if image == 0 { b::phys_initrd_start as u64 } else { image }
}
#[cfg(CONFIG_BLK_DEV_INITRD)]
#[link_section = ".init.text"]
unsafe fn get_ramdisk_size() -> u64 {
    let size = boot_params.hdr.ramdisk_size as u64 | (boot_params.ext_ramdisk_size as u64) << 32;
    if size == 0 { b::phys_initrd_size as u64 } else { size }
}
#[cfg(CONFIG_BLK_DEV_INITRD)]
#[link_section = ".init.text"]
unsafe fn relocate_initrd() {
    let image = get_ramdisk_image();
    let size = get_ramdisk_size();
    let relocated = b::memblock_phys_alloc_range(page_align(size), b::RUST_SETUP_PAGE_SIZE as u64,
        0, (max_pfn_mapped as u64) << b::RUST_SETUP_PAGE_SHIFT);
    if relocated == 0 { b::panic(cstr!("Cannot find place for new RAMDISK of size %lld\n"), size); }
    b::initrd_start = va(relocated);
    b::initrd_end = b::initrd_start.wrapping_add(size as usize);
    log!("relocate_initrd", "\x016", "Allocated new RAMDISK: [mem %#010llx-%#010llx]\n", relocated, relocated.wrapping_add(size).wrapping_sub(1));
    if b::copy_from_early_mem(b::initrd_start as *mut c_void, image, size as usize) != 0 {
        b::panic(cstr!("Copy RAMDISK failed\n"));
    }
    log!("relocate_initrd", "\x016", "Move RAMDISK from [mem %#010llx-%#010llx] to [mem %#010llx-%#010llx]\n",
        image, image.wrapping_add(size).wrapping_sub(1), relocated, relocated.wrapping_add(size).wrapping_sub(1));
}
#[link_section = ".init.text"]
unsafe fn early_reserve_initrd() {
    #[cfg(CONFIG_BLK_DEV_INITRD)]
    {
        let image = get_ramdisk_image();
        let size = get_ramdisk_size();
        let end = page_align(image.wrapping_add(size));
        if boot_params.hdr.type_of_loader == 0 || image == 0 || size == 0 { return; }
        reserve_kern(image, end.wrapping_sub(image));
    }
}
#[link_section = ".init.text"]
unsafe fn reserve_initrd() {
    #[cfg(CONFIG_BLK_DEV_INITRD)]
    {
        let image = get_ramdisk_image();
        let size = get_ramdisk_size();
        let end = page_align(image.wrapping_add(size));
        if boot_params.hdr.type_of_loader == 0 || image == 0 || size == 0 { return; }
        b::initrd_start = 0;
        log!("reserve_initrd", "\x016", "RAMDISK: [mem %#010llx-%#010llx]\n", image, end.wrapping_sub(1));
        if b::pfn_range_is_mapped((image >> b::RUST_SETUP_PAGE_SHIFT) as usize,
            (end >> b::RUST_SETUP_PAGE_SHIFT) as usize) {
            b::initrd_start = va(image);
            b::initrd_end = b::initrd_start.wrapping_add(size as usize);
            return;
        }
        relocate_initrd();
        b::memblock_phys_free(image, end.wrapping_sub(image));
    }
}
#[link_section = ".init.text"]
unsafe fn add_early_ima_buffer(phys_addr: u64) {
    #[cfg(CONFIG_IMA)]
    {
        let data = b::early_memremap(phys_addr.wrapping_add(size_of::<b::setup_data>() as u64),
            size_of::<b::ima_setup_data>() as usize).cast::<b::ima_setup_data>();
        if data.is_null() { log!("add_early_ima_buffer", "\x014", "setup: failed to memremap ima_setup_data entry\n"); return; }
        if (*data).size != 0 {
            reserve_kern((*data).addr, (*data).size);
            ima_kexec_buffer_phys = (*data).addr;
            ima_kexec_buffer_size = (*data).size as usize;
        }
        b::early_memunmap(data.cast(), size_of::<b::ima_setup_data>());
    }
    #[cfg(not(CONFIG_IMA))]
    { let _ = phys_addr; log!("add_early_ima_buffer", "\x014", "Passed IMA kexec data, but CONFIG_IMA not set. Ignoring.\n"); }
}
#[cfg(all(CONFIG_HAVE_IMA_KEXEC, not(CONFIG_OF_FLATTREE)))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn ima_free_kexec_buffer() -> c_int {
    if ima_kexec_buffer_size == 0 { return -(b::ENOENT as c_int); }
    b::memblock_phys_free(ima_kexec_buffer_phys, ima_kexec_buffer_size as u64);
    ima_kexec_buffer_phys = 0;
    ima_kexec_buffer_size = 0;
    0
}
#[cfg(all(CONFIG_HAVE_IMA_KEXEC, not(CONFIG_OF_FLATTREE)))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn ima_get_kexec_buffer(addr: *mut *mut c_void, size: *mut usize) -> c_int {
    if ima_kexec_buffer_size == 0 { return -(b::ENOENT as c_int); }
    let ret = b::ima_validate_range(ima_kexec_buffer_phys, ima_kexec_buffer_size);
    if ret != 0 { return ret; }
    *addr = va(ima_kexec_buffer_phys) as *mut c_void;
    *size = ima_kexec_buffer_size;
    0
}
#[link_section = ".init.text"]
unsafe fn add_kho(phys_addr: u64, data_len: u32) {
    #[cfg(not(CONFIG_KEXEC_HANDOVER))]
    { let _ = (phys_addr, data_len); log!("add_kho", "\x014", "Passed KHO data, but CONFIG_KEXEC_HANDOVER not set. Ignoring.\n"); }
    #[cfg(CONFIG_KEXEC_HANDOVER)]
    {
        let addr = phys_addr.wrapping_add(size_of::<b::setup_data>() as u64);
        let size = (data_len as u64).wrapping_sub(size_of::<b::setup_data>() as u64);
        let kho = b::early_memremap(addr, size as usize).cast::<b::kho_data>();
        if kho.is_null() { log!("add_kho", "\x014", "setup: failed to memremap kho data (0x%llx, 0x%llx)\n", addr, size); return; }
        b::kho_populate((*kho).fdt_addr, (*kho).fdt_size, (*kho).scratch_addr, (*kho).scratch_size);
        b::early_memunmap(kho.cast(), size as usize);
    }
}
#[link_section = ".init.text"]
unsafe fn parse_setup_data() {
    let mut pa = boot_params.hdr.setup_data;
    while pa != 0 {
        let mut data = b::early_memremap(pa, size_of::<b::setup_data>()).cast::<b::setup_data>();
        let len = (*data).len.wrapping_add(size_of::<b::setup_data>() as u32);
        let kind = (*data).type_;
        let next = (*data).next;
        b::early_memunmap(data.cast(), size_of::<b::setup_data>());
        match kind {
            b::SETUP_E820_EXT => b::e820__memory_setup_extended(pa, len),
            b::SETUP_DTB => b::add_dtb(pa),
            b::SETUP_EFI => b::parse_efi_setup(pa, len),
            b::SETUP_IMA => add_early_ima_buffer(pa),
            b::SETUP_KEXEC_KHO => add_kho(pa, len),
            b::SETUP_RNG_SEED => {
                data = b::early_memremap(pa, len as usize).cast();
                let payload = data.add(1).cast::<c_void>();
                b::add_bootloader_randomness(payload, (*data).len as usize);
                b::rust_setup_memzero_explicit(payload, (*data).len as usize);
                b::rust_setup_memzero_explicit(addr_of_mut!((*data).len).cast(), size_of::<u32>());
                b::early_memunmap(data.cast(), len as usize);
            }
            _ => {}
        }
        pa = next;
    }
}
#[link_section = ".init.text"]
unsafe fn parse_boot_params() {
    // old_decode_dev(), kdev_t.h: MKDEV((val >> 8) & 255, val & 255).
    let root = boot_params.hdr.root_dev as u32;
    b::ROOT_DEV = (((root >> 8) & 255) << b::RUST_SETUP_MINORBITS) | (root & 255);
    sysfb_primary_display.screen = boot_params.screen_info;
    #[cfg(CONFIG_FIRMWARE_EDID)]
    { sysfb_primary_display.edid = boot_params.edid_info; }
    saved_video_mode = boot_params.hdr.vid_mode as c_ulong;
    bootloader_type = boot_params.hdr.type_of_loader as c_int;
    if bootloader_type >> 4 == 0xe {
        bootloader_type &= 0xf;
        bootloader_type |= ((boot_params.hdr.ext_loader_type as c_int) + 0x10) << 4;
    }
    bootloader_version = (bootloader_type & 0xf) | ((boot_params.hdr.ext_loader_ver as c_int) << 4);
    #[cfg(CONFIG_BLK_DEV_RAM = "y")]
    { b::rd_image_start = (boot_params.hdr.ram_size & 0x07ff) as c_int; }
    #[cfg(CONFIG_EFI)]
    {
        let sig = addr_of!(boot_params.efi_info.efi_loader_signature).cast::<c_char>();
        if b::strncmp(sig, b::EFI32_LOADER_SIGNATURE.as_ptr().cast(), 4) == 0 {
            b::rust_setup_set_efi_flag(b::EFI_BOOT);
        } else if b::strncmp(sig, b::EFI64_LOADER_SIGNATURE.as_ptr().cast(), 4) == 0 {
            b::rust_setup_set_efi_flag(b::EFI_BOOT);
            b::rust_setup_set_efi_flag(b::EFI_64BIT);
        }
    }
    if boot_params.hdr.root_flags == 0 { b::root_mountflags &= !(b::MS_RDONLY as c_int); }
}
#[link_section = ".init.text"]
unsafe fn memblock_x86_reserve_range_setup_data() {
    let mut pa = boot_params.hdr.setup_data;
    while pa != 0 {
        let mut data = b::early_memremap(pa, size_of::<b::setup_data>()).cast::<b::setup_data>();
        if data.is_null() { log!("memblock_x86_reserve_range_setup_data", "\x014", "setup: failed to memremap setup_data entry\n"); return; }
        let mut len = size_of::<b::setup_data>() as u32;
        let next = (*data).next;
        reserve_kern(pa, size_of::<b::setup_data>() as u64 + (*data).len as u64);
        if (*data).type_ == b::SETUP_INDIRECT {
            len = len.wrapping_add((*data).len);
            b::early_memunmap(data.cast(), size_of::<b::setup_data>());
            data = b::early_memremap(pa, len as usize).cast();
            if data.is_null() { log!("memblock_x86_reserve_range_setup_data", "\x014", "setup: failed to memremap indirect setup_data\n"); return; }
            let indirect = data.add(1).cast::<b::setup_indirect>();
            if (*indirect).type_ != b::SETUP_INDIRECT { reserve_kern((*indirect).addr, (*indirect).len); }
        }
        pa = next;
        b::early_memunmap(data.cast(), len as usize);
    }
}
#[link_section = ".init.text"]
unsafe fn arch_reserve_crashkernel() {
    #[cfg(CONFIG_CRASH_RESERVE)]
    {
        let (mut base, mut size, mut low, mut cma) = (0, 0, 0, 0);
        let mut high = false;
        if b::parse_crashkernel(addr_of_mut!(b::boot_command_line).cast(), b::memblock_phys_mem_size(),
            &mut size, &mut base, &mut low, &mut cma, &mut high) != 0 { return; }
        if b::rust_setup_xen_pv_domain() { log!("arch_reserve_crashkernel", "\x016", "Ignoring crashkernel for a Xen PV domain\n"); return; }
        b::reserve_crashkernel_generic(size, base, low, high);
        b::reserve_crashkernel_cma(cma);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn reserve_standard_io_resources() {
    for i in 0..10 {
        b::request_resource(addr_of_mut!(b::ioport_resource), addr_of_mut!(standard_io_resources).cast::<b::resource>().add(i));
    }
}
#[link_section = ".init.text"]
unsafe fn setup_kernel_resources() {
    code_resource.start = pa_symbol(addr_of!(b::_text) as usize);
    code_resource.end = pa_symbol(addr_of!(b::_etext) as usize).wrapping_sub(1);
    rodata_resource.start = pa_symbol(addr_of!(b::__start_rodata) as usize);
    rodata_resource.end = pa_symbol(addr_of!(b::__end_rodata) as usize).wrapping_sub(1);
    data_resource.start = pa_symbol(addr_of!(b::_sdata) as usize);
    data_resource.end = pa_symbol(addr_of!(b::_edata) as usize).wrapping_sub(1);
    bss_resource.start = pa_symbol(addr_of!(b::__bss_start) as usize);
    bss_resource.end = pa_symbol(addr_of!(b::__bss_stop) as usize).wrapping_sub(1);
    b::insert_resource(addr_of_mut!(b::iomem_resource), addr_of_mut!(code_resource));
    b::insert_resource(addr_of_mut!(b::iomem_resource), addr_of_mut!(rodata_resource));
    b::insert_resource(addr_of_mut!(b::iomem_resource), addr_of_mut!(data_resource));
    b::insert_resource(addr_of_mut!(b::iomem_resource), addr_of_mut!(bss_resource));
}
#[link_section = ".init.text"]
unsafe fn snb_gfx_workaround_needed() -> bool {
    #[cfg(CONFIG_PCI)]
    {
        if b::early_pci_allowed() == 0 { return false; }
        if b::read_pci_config_16(0, 2, 0, b::PCI_VENDOR_ID as u8) != 0x8086 { return false; }
        let id = b::read_pci_config_16(0, 2, 0, b::PCI_DEVICE_ID as u8);
        for known in [0x0102, 0x0112, 0x0122, 0x0106, 0x0116, 0x0126, 0x010a] {
            if id == known { return true; }
        }
    }
    false
}
#[link_section = ".init.text"]
unsafe fn trim_snb_memory() {
    if !snb_gfx_workaround_needed() { return; }
    log!("trim_snb_memory", "\x017", "reserving inaccessible SNB gfx pages\n");
    for page in [0x20050000u64, 0x20110000, 0x20130000, 0x20138000, 0x40004000] {
        if b::memblock_reserve(page, b::RUST_SETUP_PAGE_SIZE as u64) != 0 {
            log!("trim_snb_memory", "\x014", "failed to reserve 0x%08lx\n", page as c_ulong);
        }
    }
}
#[link_section = ".init.text"]
unsafe fn trim_bios_range() {
    b::e820__range_update(0, b::RUST_SETUP_PAGE_SIZE as u64, b::E820_TYPE_RAM, b::E820_TYPE_RESERVED);
    b::e820__range_remove(b::RUST_SETUP_BIOS_BEGIN as u64,
        (b::RUST_SETUP_BIOS_END - b::RUST_SETUP_BIOS_BEGIN) as u64, b::E820_TYPE_RAM);
    b::e820__update_table(b::e820_table);
}
#[link_section = ".init.text"]
unsafe fn e820_add_kernel_range() {
    let start = pa_symbol(addr_of!(b::_text) as usize);
    let size = pa_symbol(addr_of!(b::_end) as usize).wrapping_sub(start);
    if b::e820__mapped_all(start, start.wrapping_add(size), b::E820_TYPE_RAM) { return; }
    log!("e820_add_kernel_range", "\x014", ".text .data .bss are not marked as E820_TYPE_RAM!\n");
    b::e820__range_remove(start, size, 0);
    b::e820__range_add(start, size, b::E820_TYPE_RAM);
}
#[link_section = ".init.text"]
unsafe fn early_reserve_memory() {
    reserve_kern(pa_symbol(addr_of!(b::_text) as usize),
        (addr_of!(b::__end_of_kernel_reserve) as u64).wrapping_sub(addr_of!(b::_text) as u64));
    b::memblock_reserve(0, b::SZ_64K as u64);
    early_reserve_initrd();
    memblock_x86_reserve_range_setup_data();
    b::reserve_bios_regions();
    trim_snb_memory();
}
unsafe extern "C" fn dump_kernel_offset(_: *mut b::notifier_block, _: c_ulong, _: *mut c_void) -> c_int {
    if cfg!(CONFIG_RANDOMIZE_MEMORY) && (boot_params.hdr.loadflags & b::KASLR_FLAG as u8) != 0 {
        log!("dump_kernel_offset", "\x010", "Kernel Offset: 0x%lx from 0x%lx (relocation range: 0x%lx-0x%lx)\n",
            (addr_of!(b::_text) as c_ulong).wrapping_sub(b::RUST_SETUP_START_KERNEL as c_ulong), b::RUST_SETUP_START_KERNEL as c_ulong,
            b::RUST_SETUP_START_KERNEL_MAP as c_ulong, (b::RUST_SETUP_MODULES_VADDR as c_ulong).wrapping_sub(1));
    } else { log!("dump_kernel_offset", "\x010", "Kernel Offset: disabled\n"); }
    0
}
#[no_mangle]
pub unsafe extern "C" fn x86_configure_nx() {
    if b::rust_setup_boot_cpu_has_nx() { b::__supported_pte_mask |= b::RUST_SETUP_PAGE_NX as c_ulong; }
    else { b::__supported_pte_mask &= !(b::RUST_SETUP_PAGE_NX as c_ulong); }
}
#[link_section = ".init.text"]
unsafe fn x86_report_nx() {
    if !b::rust_setup_boot_cpu_has_nx() {
        log!("x86_report_nx", "\x015", "Notice: NX (Execute Disable) protection missing in CPU!\n");
    } else { log!("x86_report_nx", "\x016", "NX (Execute Disable) protection: active\n"); }
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut c_char) {
    let bootline = addr_of_mut!(b::boot_command_line).cast::<c_char>();
    let command = addr_of_mut!(command_line).cast::<c_char>();
    let command_size = b::RUST_SETUP_COMMAND_LINE_SIZE as usize;
    log!("setup_arch", "\x016", "Command line: %s\n", bootline);
    boot_cpu_data.x86_phys_bits = b::rust_setup_max_physmem_bits() as u8;
    #[cfg(CONFIG_CMDLINE_BOOL)]
    {
        let builtin = addr_of_mut!(builtin_cmdline).cast::<c_char>();
        #[cfg(CONFIG_CMDLINE_OVERRIDE)]
        { b::sized_strscpy(bootline, builtin, command_size); }
        #[cfg(not(CONFIG_CMDLINE_OVERRIDE))]
        if *builtin != 0 {
            b::strlcat(builtin, cstr!(" "), command_size);
            b::strlcat(builtin, bootline, command_size);
            b::sized_strscpy(bootline, builtin, command_size);
        }
        builtin_cmdline_added = true;
    }
    #[cfg(CONFIG_CMDLINE_FROM_BOOTCONFIG)]
    if b::bootconfig_cmdline_requested(bootline, core::ptr::null_mut()) || cfg!(CONFIG_BOOT_CONFIG_FORCE) {
        b::xbc_prepend_embedded_cmdline(bootline, command_size);
    }
    b::sized_strscpy(command, bootline, command_size);
    *cmdline_p = command;

    b::olpc_ofw_detect();
    b::idt_setup_early_traps();
    b::early_cpu_init();
    b::jump_label_init();
    b::static_call_init();
    b::early_ioremap_init();
    b::setup_olpc_ofw_pgd();
    parse_boot_params();
    (b::x86_init.oem.arch_setup.unwrap_unchecked())();

    // Reservations precede e820 setup, including Xen dom0 memory setup.
    early_reserve_memory();
    b::iomem_resource.end = (1u64 << boot_cpu_data.x86_phys_bits).wrapping_sub(1);
    b::e820__memory_setup();
    parse_setup_data();
    copy_edd();
    b::setup_initial_init_mm(addr_of!(b::_text).cast_mut().cast(),
        addr_of!(b::_etext).cast_mut().cast(), addr_of!(b::_edata).cast_mut().cast(), b::_brk_end as *mut c_void);
    x86_configure_nx();
    b::parse_early_param();
    if b::rust_setup_efi_enabled(b::EFI_BOOT) { b::efi_memblock_x86_reserve_range(); }
    x86_report_nx();
    b::apic_setup_apic_calls();
    if b::acpi_mps_check() != 0 {
        #[cfg(CONFIG_X86_LOCAL_APIC)]
        { b::apic_is_disabled = true; }
        b::setup_clear_cpu_cap(b::X86_FEATURE_APIC);
    }
    b::e820__finish_early_params();
    if b::rust_setup_efi_enabled(b::EFI_BOOT) { b::efi_init(); }
    b::reserve_ibft_region();
    (b::x86_init.resources.dmi_setup.unwrap_unchecked())();
    b::init_hypervisor_platform();
    b::tsc_early_init();
    (b::x86_init.resources.probe_roms.unwrap_unchecked())();
    setup_kernel_resources();
    e820_add_kernel_range();
    trim_bios_range();
    b::early_gart_iommu_check();
    b::max_pfn = b::e820__end_of_ram_pfn();
    b::cache_bp_init();
    if b::mtrr_trim_uncached_memory(b::max_pfn) != 0 { b::max_pfn = b::e820__end_of_ram_pfn(); }
    b::max_possible_pfn = b::max_pfn as u64;
    b::kernel_randomize_memory();
    b::check_x2apic();
    b::max_low_pfn = if b::max_pfn > (1usize << (32 - b::RUST_SETUP_PAGE_SHIFT)) {
        b::e820__end_of_low_ram_pfn()
    } else { b::max_pfn };
    (b::x86_init.mpparse.find_mptable.unwrap_unchecked())();
    b::early_alloc_pgt_buf();
    reserve_brk();
    b::cleanup_highmap();
    b::e820__memblock_setup();
    b::mem_encrypt_setup_arch();
    b::cc_random_init();
    b::efi_find_mirror();
    b::efi_esrt_init();
    b::efi_mokvar_table_init();
    b::efi_reserve_boot_services();
    b::e820__memblock_alloc_reserved_mpc_new();
    #[cfg(CONFIG_X86_CHECK_BIOS_CORRUPTION)]
    b::setup_bios_corruption_check();
    (b::x86_platform.realmode_reserve.unwrap_unchecked())();
    b::init_mem_mapping();
    b::cpu_init_replace_early_idt();
    mmu_cr4_features = b::rust_setup_read_cr4() & !(b::RUST_SETUP_CR4_PCIDE as c_ulong);
    b::memblock_set_current_limit((max_pfn_mapped as u64) << b::RUST_SETUP_PAGE_SHIFT);
    #[cfg(CONFIG_PROVIDE_OHCI1394_DMA_INIT)]
    if b::init_ohci1394_dma_early != 0 { b::init_ohci1394_dma_on_all_controllers(); }
    b::setup_log_buf(1);
    if b::rust_setup_efi_enabled(b::EFI_BOOT) {
        match boot_params.secure_boot as b::rust_setup_constants {
            b::RUST_SETUP_EFI_SECURE_DISABLED => { log!("setup_arch", "\x016", "Secure boot disabled\n"); }
            b::RUST_SETUP_EFI_SECURE_ENABLED => { log!("setup_arch", "\x016", "Secure boot enabled\n"); }
            _ => { log!("setup_arch", "\x016", "Secure boot could not be determined\n"); }
        }
    }
    reserve_initrd();
    b::acpi_table_upgrade();
    b::acpi_boot_table_init();
    b::vsmp_init();
    b::io_delay_init();
    b::early_platform_quirks();
    b::early_acpi_boot_init();
    (b::x86_init.mpparse.early_parse_smp_cfg.unwrap_unchecked())();
    b::x86_flattree_get_config();
    b::initmem_init();
    b::dma_contiguous_reserve((max_pfn_mapped as u64) << b::RUST_SETUP_PAGE_SHIFT);
    arch_reserve_crashkernel();
    if b::early_xdbc_setup_hardware() == 0 { b::early_xdbc_register_console(); }
    (b::x86_init.paging.pagetable_init.unwrap_unchecked())();
    b::kasan_init();
    b::sync_initial_page_table();
    #[cfg(CONFIG_INTEL_TXT)]
    b::tboot_probe();
    b::map_vsyscall();
    b::x86_32_probe_apic();
    b::early_quirks();
    b::topology_apply_cmdline_limits_early();
    b::acpi_boot_init();
    (b::x86_init.mpparse.parse_smp_cfg.unwrap_unchecked())();
    b::init_apic_mappings();
    b::topology_init_possible_cpus();
    b::init_cpu_to_node();
    b::init_gi_nodes();
    b::io_apic_init_mappings();
    (b::x86_init.hyper.guest_late_init.unwrap_unchecked())();
    b::e820__reserve_resources();
    b::e820__register_nosave_regions(b::max_pfn);
    (b::x86_init.resources.reserve_resources.unwrap_unchecked())();
    b::e820__setup_pci_gap();
    #[cfg(all(CONFIG_VT, CONFIG_VGA_CONSOLE))]
    if !b::rust_setup_efi_enabled(b::EFI_BOOT) || b::efi_mem_type(0xa0000) != b::EFI_CONVENTIONAL_MEMORY as c_int {
        b::vgacon_register_screen(addr_of_mut!(sysfb_primary_display.screen));
    }
    (b::x86_init.oem.banner.unwrap_unchecked())();
    (b::x86_init.timers.wallclock_init.unwrap_unchecked())();
    b::therm_lvt_init();
    b::mcheck_init();
    b::register_refined_jiffies(b::PIT_TICK_RATE as isize);
    #[cfg(CONFIG_EFI)]
    if b::rust_setup_efi_enabled(b::EFI_BOOT) { b::efi_apply_memmap_quirks(); }
    b::unwind_init();
}

static mut kernel_offset_notifier: b::notifier_block = b::notifier_block {
    notifier_call: Some(dump_kernel_offset), next: core::ptr::null_mut(), priority: 0,
};
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_setup_register_kernel_offset_dumper() -> c_int {
    b::atomic_notifier_chain_register(addr_of_mut!(b::panic_notifier_list), addr_of_mut!(kernel_offset_notifier));
    0
}
#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool { cpu > 0 }
