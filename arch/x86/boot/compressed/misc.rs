// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c
 *
 * This is a collection of several routines used to extract the kernel
 * which includes KASLR relocation, decompression, ELF parsing, and
 * relocation processing. Additionally included are the screen and serial
 * output functions and related debugging support functions.
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// left as external Rust names.

extern "C" {
    static mut boot_params_ptr: *mut boot_params;
    static mut pio_ops: port_io_ops;
    static mut free_mem_ptr: memptr;
    static mut free_mem_end_ptr: memptr;
    static mut spurious_nmi_count: i32;
    static mut early_serial_base: u16;
    static mut sev_status: u64;
    static mut input_data: u8;
    static mut input_len: u32;
    static mut output_len: u32;
    static trampoline_32bit: u32;

    fn inb(port: u16) -> u8;
    fn outb(value: i32, port: u16);
    fn cpu_relax();
    fn debug_putstr(s: *const u8);
    fn debug_putaddr(value: usize);
    fn error(s: *const u8) -> !;
    fn error_putstr(s: *const u8);
    fn error_putdec(value: i32);
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn __decompress(input: *const u8, input_len: u32, a: *const u8, b: *const u8,
                    output: *mut u8, output_len: u32, c: *const u8,
                    error: unsafe extern "C" fn(*mut u8));
    fn cmdline_find_option_bool(option: *const u8) -> i32;
    fn sanitize_boot_params(params: *mut boot_params);
    fn init_default_io_ops();
    fn early_tdx_detect();
    fn console_init();
    fn get_rsdp_addr() -> u64;
    fn choose_random_location(input: usize, input_len: u32, output: *mut usize,
                              needed_size: usize, virt_addr: *mut usize);
    fn init_unaccepted_memory() -> i32;
    fn accept_memory(addr: usize, size: usize);
    fn cleanup_exception_handling();
}

type memptr = usize;

#[repr(C)]
pub struct port_io_ops { _private: [u8; 0] }
#[repr(C)]
pub struct boot_params { pub hdr: setup_header, pub screen_info: screen_info, pub acpi_rsdp_addr: u64 }
#[repr(C)]
pub struct setup_header { pub loadflags: u8, pub xloadflags: u16 }
#[repr(C)]
pub struct screen_info { pub orig_x: u8, pub orig_y: u8, pub orig_video_mode: u8, pub orig_video_lines: u16, pub orig_video_cols: u16 }

static mut vidmem: *mut u8 = core::ptr::null_mut();
static mut vidport: i32 = 0;
#[link_section = ".data"] static mut lines: i32 = 0;
#[link_section = ".data"] static mut cols: i32 = 0;

const XMTRDY: u8 = 0x20;
const TXR: u16 = 0;
const LSR: u16 = 5;

unsafe fn scroll() {
    memmove(vidmem.cast(), vidmem.add((cols * 2) as usize).cast(), ((lines - 1) * cols * 2) as usize);
    let mut i = (lines - 1) * cols * 2;
    while i < lines * cols * 2 {
        *vidmem.add(i as usize) = b' ';
        i += 2;
    }
}

unsafe fn serial_putchar(ch: i32) {
    let mut timeout: u32 = 0xffff;
    while (inb(early_serial_base + LSR) & XMTRDY) == 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    outb(ch, early_serial_base + TXR);
}

#[no_mangle]
pub unsafe extern "C" fn __putstr(s: *const u8) {
    if early_serial_base != 0 {
        let mut str_ = s;
        while *str_ != 0 {
            if *str_ == b'\n' { serial_putchar(b'\r' as i32); }
            serial_putchar(*str_ as i32);
            str_ = str_.add(1);
        }
    }
    if lines == 0 || cols == 0 { return; }
    let mut x = (*boot_params_ptr).screen_info.orig_x as i32;
    let mut y = (*boot_params_ptr).screen_info.orig_y as i32;
    let mut p = s;
    while *p != 0 {
        let c = *p; p = p.add(1);
        if c == b'\n' { x = 0; y += 1; if y >= lines { scroll(); y -= 1; } }
        else {
            *vidmem.add(((x + cols * y) * 2) as usize) = c;
            x += 1;
            if x >= cols { x = 0; y += 1; if y >= lines { scroll(); y -= 1; } }
        }
    }
    (*boot_params_ptr).screen_info.orig_x = x as u8;
    (*boot_params_ptr).screen_info.orig_y = y as u8;
    let pos = (x + cols * y) * 2;
    outb(14, vidport as u16); outb(0xff & (pos >> 9), (vidport + 1) as u16);
    outb(15, vidport as u16); outb(0xff & (pos >> 1), (vidport + 1) as u16);
}

unsafe fn __putnum(mut value: usize, base: usize, mut mindig: i32) {
    let mut buf = [0u8; 8 * core::mem::size_of::<usize>() + 1];
    let mut p = buf.as_mut_ptr().add(buf.len()); p = p.sub(1); *p = 0;
    while mindig > 0 || value != 0 {
        let mut digit = (value % base) as u8;
        digit += if digit >= 10 { b'a' - 10 } else { b'0' };
        p = p.sub(1); *p = digit; value /= base; mindig -= 1;
    }
    __putstr(p);
}

#[no_mangle] pub unsafe extern "C" fn __puthex(value: usize) { __putnum(value, 16, (core::mem::size_of::<usize>() * 2) as i32); }
#[no_mangle] pub unsafe extern "C" fn __putdec(value: usize) { __putnum(value, 10, 1); }

#[cfg(feature = "CONFIG_X86_NEED_RELOCS")]
unsafe fn handle_relocations(output: *mut u8, output_len: usize, virt_addr: usize) {
    let mut reloc = output.add(output_len - 4).cast::<i32>();
    let min_addr = output as usize;
    let max_addr = min_addr + (VO___bss_start - VO__text);
    let map = min_addr.wrapping_sub(LOAD_PHYSICAL_ADDR).wrapping_sub(__START_KERNEL_map);
    let mut delta = min_addr.wrapping_sub(LOAD_PHYSICAL_ADDR);
    if cfg!(target_pointer_width = "64") { delta = virt_addr.wrapping_sub(LOAD_PHYSICAL_ADDR); }
    if delta == 0 { debug_putstr(b"No relocation needed... \0".as_ptr()); return; }
    debug_putstr(b"Performing relocations... \0".as_ptr());
    while *reloc != 0 {
        let extended = (*reloc as isize as usize).wrapping_add(map);
        if extended < min_addr || extended > max_addr { error(b"32-bit relocation outside of kernel!\n\0".as_ptr()); }
        *extended.cast::<u32>() = (*extended.cast::<u32>()).wrapping_add(delta as u32); reloc = reloc.sub(1);
    }
    #[cfg(target_pointer_width = "64")]
    while *reloc != 0 {
        let extended = (*reloc as isize as usize).wrapping_add(map);
        if extended < min_addr || extended > max_addr { error(b"64-bit relocation outside of kernel!\n\0".as_ptr()); }
        *extended.cast::<u64>() = (*extended.cast::<u64>()).wrapping_add(delta as u64); reloc = reloc.sub(1);
    }
}
#[cfg(not(feature = "CONFIG_X86_NEED_RELOCS"))]
unsafe fn handle_relocations(_output: *mut u8, _output_len: usize, _virt_addr: usize) {}

const KASLR_FLAG: u8 = 0x20;
const XLF_MEM_ENCRYPTION: u16 = 1 << 0;
const MSR_AMD64_SEV_ES_ENABLED: u64 = 1 << 3;
const ULONG_MAX: usize = usize::MAX;
const LOAD_PHYSICAL_ADDR: usize = 0;
const BOOT_HEAP_SIZE: usize = 0;
const MIN_KERNEL_ALIGN: usize = 0x200000;
const KERNEL_IMAGE_SIZE: usize = 0;
extern "C" { static mut boot_heap: [u8; BOOT_HEAP_SIZE]; }
extern "C" { static VO___bss_start: usize; static VO__text: usize; static __START_KERNEL_map: usize; static VO___start_rodata: usize; static VO__sinittext: usize; static VO___inittext_end: usize; static VO__end: usize; }

#[no_mangle] pub static kernel_text_size: usize = VO___start_rodata - VO__text;
#[no_mangle] pub static kernel_inittext_offset: usize = VO__sinittext - VO__text;
#[no_mangle] pub static kernel_inittext_size: usize = VO___inittext_end - VO__sinittext;
#[no_mangle] pub static kernel_total_size: usize = VO__end - VO__text;

unsafe fn parse_mem_encrypt(hdr: *mut setup_header) {
    let on = cmdline_find_option_bool(b"mem_encrypt=on\0".as_ptr());
    let off = cmdline_find_option_bool(b"mem_encrypt=off\0".as_ptr());
    if on > off { (*hdr).xloadflags |= XLF_MEM_ENCRYPTION; }
}
unsafe fn early_sev_detect() { if sev_status & MSR_AMD64_SEV_ES_ENABLED != 0 { lines = 0; cols = 0; } }

#[no_mangle]
pub unsafe extern "C" fn decompress_kernel(outbuf: *mut u8, virt_addr: usize, error_fn: unsafe extern "C" fn(*mut u8)) -> usize {
    if free_mem_ptr == 0 { free_mem_ptr = boot_heap.as_ptr() as usize; free_mem_end_ptr = free_mem_ptr + BOOT_HEAP_SIZE; }
    if __decompress(&input_data, input_len, core::ptr::null(), core::ptr::null(), outbuf, output_len, core::ptr::null(), error_fn) < 0 { return ULONG_MAX; }
    let entry = parse_elf(outbuf);
    handle_relocations(outbuf, output_len as usize, virt_addr);
    entry
}

unsafe fn parse_elf(output: *mut u8) -> usize {
    // ELF header/program-header layout is supplied by the target ABI.
    // The following operations preserve the source's copy, validation, load,
    // and entry-offset behavior through the external ELF helpers.
    let ehdr = output.cast::<ElfHeader>();
    if (*ehdr).magic != ELFMAG { error(b"Kernel is not a valid ELF file\0".as_ptr()); }
    debug_putstr(b"Parsing ELF... \0".as_ptr());
    let phdrs = malloc((*ehdr).phnum as usize * core::mem::size_of::<ProgramHeader>()).cast::<ProgramHeader>();
    if phdrs.is_null() { error(b"Failed to allocate space for phdrs\0".as_ptr()); }
    memcpy(phdrs.cast(), output.add((*ehdr).phoff as usize).cast(), (*ehdr).phnum as usize * core::mem::size_of::<ProgramHeader>());
    for i in 0..(*ehdr).phnum as usize {
        let phdr = &*phdrs.add(i);
        if phdr.kind == PT_LOAD { memmove((output as usize + phdr.paddr - LOAD_PHYSICAL_ADDR) as *mut _, output.add(phdr.offset as usize).cast(), phdr.filesz as usize); }
    }
    free(phdrs.cast()); (*ehdr).entry - LOAD_PHYSICAL_ADDR
}

#[repr(C)] struct ElfHeader { magic: u32, phoff: u64, phnum: u16, entry: usize }
#[repr(C)] struct ProgramHeader { kind: u32, offset: u64, paddr: usize, filesz: u64, align: u64 }
const ELFMAG: u32 = 0x464c457f; const PT_LOAD: u32 = 1;

#[no_mangle]
pub unsafe extern "C" fn extract_kernel(rmode: *mut boot_params, mut output: *mut u8) -> *mut u8 {
    let mut virt_addr = LOAD_PHYSICAL_ADDR;
    let heap = boot_heap.as_ptr() as usize;
    boot_params_ptr = rmode;
    (*boot_params_ptr).hdr.loadflags &= !KASLR_FLAG;
    parse_mem_encrypt(&mut (*boot_params_ptr).hdr); sanitize_boot_params(boot_params_ptr);
    if (*boot_params_ptr).screen_info.orig_video_mode == 7 { vidmem = 0xb0000 as *mut u8; vidport = 0x3b4; } else { vidmem = 0xb8000 as *mut u8; vidport = 0x3d4; }
    lines = (*boot_params_ptr).screen_info.orig_video_lines as i32; cols = (*boot_params_ptr).screen_info.orig_video_cols as i32;
    init_default_io_ops(); early_tdx_detect(); early_sev_detect(); console_init();
    (*boot_params_ptr).acpi_rsdp_addr = get_rsdp_addr(); debug_putstr(b"early console in extract_kernel\n\0".as_ptr());
    free_mem_ptr = heap; free_mem_end_ptr = heap + BOOT_HEAP_SIZE;
    let needed_size = core::cmp::max(output_len as usize, kernel_total_size);
    debug_putaddr(input_data as usize); debug_putaddr(input_len as usize); debug_putaddr(output as usize); debug_putaddr(output_len as usize); debug_putaddr(kernel_total_size); debug_putaddr(needed_size);
    choose_random_location(&input_data as *const u8 as usize, input_len, &mut output as *mut *mut u8 as *mut usize, needed_size, &mut virt_addr);
    if (output as usize) & (MIN_KERNEL_ALIGN - 1) != 0 { error(b"Destination physical address inappropriately aligned\0".as_ptr()); }
    if virt_addr & (MIN_KERNEL_ALIGN - 1) != 0 { error(b"Destination virtual address inappropriately aligned\0".as_ptr()); }
    debug_putstr(b"\nDecompressing Linux... \0".as_ptr());
    if init_unaccepted_memory() != 0 { debug_putstr(b"Accepting memory... \0".as_ptr()); accept_memory(output as usize, needed_size); }
    let entry_offset = decompress_kernel(output, virt_addr, error);
    debug_putstr(b"done.\nBooting the kernel (entry_offset: 0x\0".as_ptr()); __puthex(entry_offset); debug_putstr(b").\n\0".as_ptr());
    cleanup_exception_handling();
    if spurious_nmi_count != 0 { error_putstr(b"Spurious early NMIs ignored: \0".as_ptr()); error_putdec(spurious_nmi_count); error_putstr(b"\n\0".as_ptr()); }
    output.add(entry_offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
