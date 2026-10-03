// SPDX-License-Identifier: GPL-2.0
/*
 * Kernel extraction, ELF loading, relocation, and early console support.
 * Keep this code position independent: no statically initialized pointers to
 * functions or data, because the compressed image relocates without fixups.
 *
 * Native layouts and configuration-dependent constants come from misc.h,
 * asm/bootparam.h, linux/elf.h, and the generated ../voffset.h bindings.
 */

use crate::bindings as b;
use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut, read_unaligned, write_unaligned};

#[cfg(CONFIG_X86_64)]
type Memptr = c_long;
#[cfg(not(CONFIG_X86_64))]
type Memptr = c_uint;

#[cfg(CONFIG_X86_64)]
type ElfHeader = b::Elf64_Ehdr;
#[cfg(CONFIG_X86_64)]
type ProgramHeader = b::Elf64_Phdr;
#[cfg(not(CONFIG_X86_64))]
type ElfHeader = b::Elf32_Ehdr;
#[cfg(not(CONFIG_X86_64))]
type ProgramHeader = b::Elf32_Phdr;

type DecompressIo = Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>;
type DecompressError = unsafe extern "C" fn(*mut c_char);

extern "C" {
    #[link_name = "input_data"]
    static mut INPUT_DATA: u8;
    #[link_name = "input_len"]
    static INPUT_LEN: c_uint;
    #[link_name = "output_len"]
    static OUTPUT_LEN: c_uint;
    #[cfg(CONFIG_EARLY_PRINTK)]
    #[link_name = "early_serial_base"]
    static EARLY_SERIAL_BASE: c_int;
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    #[link_name = "sev_status"]
    static SEV_STATUS: u64;
    #[cfg(all(CONFIG_X86_64, CONFIG_X86_VERBOSE_BOOTUP))]
    #[link_name = "trampoline_32bit"]
    static TRAMPOLINE_32BIT: *mut c_ulong;

    fn memcpy(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, value: c_int, len: usize) -> *mut c_void;
    fn error(message: *mut c_char) -> !;
    fn cmdline_find_option_bool(option: *const c_char) -> c_int;

    // A Rust owner of the selected codec must supply this native entry point.
    // The GZIP dependency closure is not yet complete; do not satisfy this
    // with a C codec and claim the extraction path has Rust ownership.
    fn __decompress(
        input: *mut u8,
        in_len: c_long,
        fill: DecompressIo,
        flush: DecompressIo,
        output: *mut u8,
        out_len: c_long,
        pos: *mut c_long,
        error: DecompressError,
    ) -> c_int;

    #[cfg(CONFIG_EARLY_PRINTK)]
    fn console_init();
    #[cfg(CONFIG_INTEL_TDX_GUEST)]
    fn early_tdx_detect();
    #[cfg(CONFIG_ACPI)]
    fn get_rsdp_addr() -> b::acpi_physical_address;
    #[cfg(CONFIG_RANDOMIZE_BASE)]
    fn choose_random_location(
        input: c_ulong,
        input_size: c_ulong,
        output: *mut c_ulong,
        output_size: c_ulong,
        virt_addr: *mut c_ulong,
    );
    #[cfg(CONFIG_UNACCEPTED_MEMORY)]
    fn init_unaccepted_memory() -> bool;
    #[cfg(CONFIG_UNACCEPTED_MEMORY)]
    fn accept_memory(start: b::phys_addr_t, size: c_ulong);
    #[cfg(CONFIG_X86_64)]
    fn cleanup_exception_handling();
}

#[export_name = "boot_params_ptr"]
static mut BOOT_PARAMS_PTR: *mut b::boot_params = null_mut();
#[export_name = "pio_ops"]
static mut PIO_OPS: b::port_io_ops = b::port_io_ops {
    f_inb: None,
    f_outb: None,
    f_outw: None,
};
#[export_name = "free_mem_ptr"]
static mut FREE_MEM_PTR: Memptr = 0;
#[export_name = "free_mem_end_ptr"]
static mut FREE_MEM_END_PTR: Memptr = 0;
#[export_name = "spurious_nmi_count"]
static mut SPURIOUS_NMI_COUNT: c_int = 0;

static mut VIDMEM: *mut u8 = null_mut();
static mut VIDPORT: c_int = 0;
// Console output may be called before .bss is cleared.
#[link_section = ".data"]
static mut LINES: c_int = 0;
#[link_section = ".data"]
static mut COLS: c_int = 0;

#[repr(C, align(4))]
struct BootHeap([u8; b::BOOT_HEAP_SIZE as usize]);
static mut BOOT_HEAP: BootHeap = BootHeap([0; b::BOOT_HEAP_SIZE as usize]);
static mut MALLOC_PTR: c_ulong = 0;
static mut MALLOC_COUNT: c_int = 0;

#[export_name = "kernel_text_size"]
static KERNEL_TEXT_SIZE: c_ulong =
    (b::VO___start_rodata as c_ulong).wrapping_sub(b::VO__text as c_ulong);
#[export_name = "kernel_inittext_offset"]
static KERNEL_INITTEXT_OFFSET: c_ulong =
    (b::VO__sinittext as c_ulong).wrapping_sub(b::VO__text as c_ulong);
#[export_name = "kernel_inittext_size"]
static KERNEL_INITTEXT_SIZE: c_ulong =
    (b::VO___inittext_end as c_ulong).wrapping_sub(b::VO__sinittext as c_ulong);
#[export_name = "kernel_total_size"]
static KERNEL_TOTAL_SIZE: c_ulong = (b::VO__end as c_ulong).wrapping_sub(b::VO__text as c_ulong);

// The preboot allocator from linux/decompress/mm.h. Failed allocations retain
// the advanced MALLOC_PTR, and the final free resets it to FREE_MEM_PTR.
#[no_mangle]
pub(crate) unsafe extern "C" fn malloc(size: c_int) -> *mut c_void {
    unsafe {
        if size < 0 {
            return null_mut();
        }
        if MALLOC_PTR == 0 {
            MALLOC_PTR = FREE_MEM_PTR as c_ulong;
        }
        MALLOC_PTR = MALLOC_PTR.wrapping_add(7) & !7;
        let result = MALLOC_PTR as *mut c_void;
        MALLOC_PTR = MALLOC_PTR.wrapping_add(size as c_ulong);
        if FREE_MEM_END_PTR != 0 && MALLOC_PTR >= FREE_MEM_END_PTR as c_ulong {
            return null_mut();
        }
        MALLOC_COUNT += 1;
        result
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn free(_where: *mut c_void) {
    unsafe {
        MALLOC_COUNT -= 1;
        if MALLOC_COUNT == 0 {
            MALLOC_PTR = FREE_MEM_PTR as c_ulong;
        }
    }
}

// asm/shared/io.h primitives. The callbacks are installed at runtime, after
// relocation; TDX can replace them before console initialization.
unsafe extern "C" fn native_inb(port: u16) -> u8 {
    unsafe {
        let value: u8;
        asm!("in al, dx", in("dx") port, out("al") value,
         options(nomem, nostack, preserves_flags));
        value
    }
}

unsafe extern "C" fn native_outb(value: u8, port: u16) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value,
         options(nomem, nostack, preserves_flags));
    }
}

unsafe extern "C" fn native_outw(value: u16, port: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value,
         options(nomem, nostack, preserves_flags));
    }
}

unsafe fn init_default_io_ops() {
    unsafe {
        PIO_OPS.f_inb = Some(native_inb);
        PIO_OPS.f_outb = Some(native_outb);
        PIO_OPS.f_outw = Some(native_outw);
    }
}

#[cfg(CONFIG_EARLY_PRINTK)]
unsafe fn inb(port: u16) -> u8 {
    unsafe { (PIO_OPS.f_inb.unwrap_unchecked())(port) }
}

unsafe fn outb(value: u8, port: u16) {
    unsafe {
        (PIO_OPS.f_outb.unwrap_unchecked())(value, port);
    }
}

unsafe fn scroll() {
    unsafe {
        memmove(
            VIDMEM.cast(),
            VIDMEM.add((COLS * 2) as usize).cast(),
            ((LINES - 1) * COLS * 2) as usize,
        );
        let mut i = (LINES - 1) * COLS * 2;
        while i < LINES * COLS * 2 {
            *VIDMEM.add(i as usize) = b' ';
            i += 2;
        }
    }
}

#[cfg(CONFIG_EARLY_PRINTK)]
unsafe fn serial_putchar(ch: u8) {
    unsafe {
        let mut timeout: c_uint = 0xffff;
        while inb(EARLY_SERIAL_BASE.wrapping_add(5) as u16) & 0x20 == 0 {
            timeout -= 1;
            if timeout == 0 {
                break;
            }
            // cpu_relax(), including the compiler memory barrier in the C macro.
            asm!("pause", options(nostack, preserves_flags));
        }
        outb(ch, EARLY_SERIAL_BASE as u16);
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn __putstr(mut s: *const c_char) {
    unsafe {
        #[cfg(CONFIG_EARLY_PRINTK)]
        if EARLY_SERIAL_BASE != 0 {
            let mut serial = s;
            while *serial != 0 {
                if *serial == b'\n' as c_char {
                    serial_putchar(b'\r');
                }
                serial_putchar(*serial as u8);
                serial = serial.add(1);
            }
        }

        if LINES == 0 || COLS == 0 {
            return;
        }
        let mut x = (*BOOT_PARAMS_PTR).screen_info.orig_x as c_int;
        let mut y = (*BOOT_PARAMS_PTR).screen_info.orig_y as c_int;
        while *s != 0 {
            let c = *s as u8;
            s = s.add(1);
            if c == b'\n' {
                x = 0;
                y += 1;
                if y >= LINES {
                    scroll();
                    y -= 1;
                }
            } else {
                *VIDMEM.add(((x + COLS * y) * 2) as usize) = c;
                x += 1;
                if x >= COLS {
                    x = 0;
                    y += 1;
                    if y >= LINES {
                        scroll();
                        y -= 1;
                    }
                }
            }
        }
        (*BOOT_PARAMS_PTR).screen_info.orig_x = x as u8;
        (*BOOT_PARAMS_PTR).screen_info.orig_y = y as u8;
        let pos = (x + COLS * y) * 2;
        outb(14, VIDPORT as u16);
        outb((0xff & (pos >> 9)) as u8, (VIDPORT + 1) as u16);
        outb(15, VIDPORT as u16);
        outb((0xff & (pos >> 1)) as u8, (VIDPORT + 1) as u16);
    }
}

#[inline(never)]
unsafe fn __putnum(mut value: c_ulong, base: c_uint, mut mindig: c_int) {
    unsafe {
        let mut buf = [0u8; 8 * size_of::<c_ulong>() + 1];
        let mut p = buf.as_mut_ptr().add(buf.len());
        p = p.sub(1);
        *p = 0;
        loop {
            let have_digits = mindig > 0;
            mindig -= 1;
            if !have_digits && value == 0 {
                break;
            }
            let digit = (value % base as c_ulong) as u8;
            p = p.sub(1);
            *p = digit + if digit >= 10 { b'a' - 10 } else { b'0' };
            value /= base as c_ulong;
        }
        __putstr(p.cast());
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn __puthex(value: c_ulong) {
    unsafe {
        __putnum(value, 16, (size_of::<c_ulong>() * 2) as c_int);
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn __putdec(value: c_ulong) {
    unsafe {
        __putnum(value, 10, 1);
    }
}

unsafe fn debug_putstr(_s: *const c_char) {
    #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
    unsafe {
        __putstr(_s);
    }
}

unsafe fn debug_puthex(_value: c_ulong) {
    #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
    unsafe {
        __puthex(_value);
    }
}

macro_rules! debug_putaddr {
    ($name:literal, $value:expr) => {
        #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
        {
            __putstr(concat!($name, ": 0x\0").as_ptr().cast());
            __puthex($value as c_ulong);
            __putstr(b"\n\0".as_ptr().cast());
        }
    };
}

#[cfg(CONFIG_X86_NEED_RELOCS)]
unsafe fn handle_relocations(output: *mut u8, len: c_ulong, virt_addr: c_ulong) {
    unsafe {
        let min_addr = output as c_ulong;
        let max_addr = min_addr
            .wrapping_add((b::VO___bss_start as c_ulong).wrapping_sub(b::VO__text as c_ulong));
        let physical_delta = min_addr.wrapping_sub(b::LOAD_PHYSICAL_ADDR as c_ulong);
        let map = physical_delta.wrapping_sub(b::__START_KERNEL_map as c_ulong);
        #[cfg(CONFIG_X86_64)]
        let delta = virt_addr.wrapping_sub(b::LOAD_PHYSICAL_ADDR as c_ulong);
        #[cfg(not(CONFIG_X86_64))]
        let delta = {
            let _ = virt_addr;
            physical_delta
        };

        if delta == 0 {
            debug_putstr(b"No relocation needed... \0".as_ptr().cast());
            return;
        }
        debug_putstr(b"Performing relocations... \0".as_ptr().cast());
        let mut reloc = output
            .wrapping_add(len as usize)
            .wrapping_sub(size_of::<c_int>())
            .cast::<c_int>();
        while read_unaligned(reloc) != 0 {
            // C assigns int to long before adding the unsigned mapping delta.
            let ptr = (read_unaligned(reloc) as c_long as c_ulong).wrapping_add(map);
            if ptr < min_addr || ptr > max_addr {
                error(
                    b"32-bit relocation outside of kernel!\n\0"
                        .as_ptr()
                        .cast_mut()
                        .cast(),
                );
            }
            let word = ptr as *mut u32;
            write_unaligned(word, read_unaligned(word).wrapping_add(delta as u32));
            reloc = reloc.sub(1);
        }
        #[cfg(CONFIG_X86_64)]
        {
            // Step over the 32-bit table's zero terminator before the 64-bit table.
            reloc = reloc.sub(1);
            while read_unaligned(reloc) != 0 {
                let ptr = (read_unaligned(reloc) as c_long as c_ulong).wrapping_add(map);
                if ptr < min_addr || ptr > max_addr {
                    error(
                        b"64-bit relocation outside of kernel!\n\0"
                            .as_ptr()
                            .cast_mut()
                            .cast(),
                    );
                }
                let word = ptr as *mut u64;
                write_unaligned(word, read_unaligned(word).wrapping_add(delta as u64));
                reloc = reloc.sub(1);
            }
        }
    }
}

unsafe fn parse_elf(output: *mut u8) -> usize {
    unsafe {
        // Preserve the header before loading segments can overwrite output[0..].
        let mut ehdr = MaybeUninit::<ElfHeader>::uninit();
        memcpy(
            ehdr.as_mut_ptr().cast(),
            output.cast(),
            size_of::<ElfHeader>(),
        );
        let ehdr = ehdr.assume_init();
        if ehdr.e_ident[b::EI_MAG0 as usize] != b::ELFMAG0 as u8
            || ehdr.e_ident[b::EI_MAG1 as usize] != b::ELFMAG1 as u8
            || ehdr.e_ident[b::EI_MAG2 as usize] != b::ELFMAG2 as u8
            || ehdr.e_ident[b::EI_MAG3 as usize] != b::ELFMAG3 as u8
        {
            error(
                b"Kernel is not a valid ELF file\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        debug_putstr(b"Parsing ELF... \0".as_ptr().cast());
        let phdrs_size = size_of::<ProgramHeader>() * ehdr.e_phnum as usize;
        let phdrs = malloc(phdrs_size as c_int).cast::<ProgramHeader>();
        if phdrs.is_null() {
            error(
                b"Failed to allocate space for phdrs\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        memcpy(
            phdrs.cast(),
            output.wrapping_add(ehdr.e_phoff as usize).cast(),
            phdrs_size,
        );

        for i in 0..ehdr.e_phnum as usize {
            let phdr = &*phdrs.add(i);
            if phdr.p_type != b::PT_LOAD as u32 {
                continue;
            }
            #[cfg(CONFIG_X86_64)]
            if phdr.p_align % 0x200000 != 0 {
                error(
                    b"Alignment of LOAD segment isn't multiple of 2MB\0"
                        .as_ptr()
                        .cast_mut()
                        .cast(),
                );
            }
            #[cfg(CONFIG_RELOCATABLE)]
            let dest = output
                .wrapping_add((phdr.p_paddr as usize).wrapping_sub(b::LOAD_PHYSICAL_ADDR as usize));
            #[cfg(not(CONFIG_RELOCATABLE))]
            let dest = phdr.p_paddr as usize as *mut u8;
            memmove(
                dest.cast(),
                output.wrapping_add(phdr.p_offset as usize).cast(),
                phdr.p_filesz as usize,
            );
        }
        free(phdrs.cast());
        (ehdr.e_entry as usize).wrapping_sub(b::LOAD_PHYSICAL_ADDR as usize)
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn decompress_kernel(
    outbuf: *mut u8,
    virt_addr: c_ulong,
    error_fn: DecompressError,
) -> c_ulong {
    unsafe {
        if FREE_MEM_PTR == 0 {
            FREE_MEM_PTR = addr_of_mut!(BOOT_HEAP.0).cast::<u8>() as Memptr;
            FREE_MEM_END_PTR = FREE_MEM_PTR.wrapping_add(size_of::<BootHeap>() as Memptr);
        }
        if __decompress(
            addr_of_mut!(INPUT_DATA),
            INPUT_LEN as c_long,
            None,
            None,
            outbuf,
            OUTPUT_LEN as c_long,
            null_mut(),
            error_fn,
        ) < 0
        {
            return c_ulong::MAX;
        }
        let entry = parse_elf(outbuf);
        #[cfg(CONFIG_X86_NEED_RELOCS)]
        handle_relocations(outbuf, OUTPUT_LEN as c_ulong, virt_addr);
        #[cfg(not(CONFIG_X86_NEED_RELOCS))]
        let _ = virt_addr;
        entry as c_ulong
    }
}

unsafe fn parse_mem_encrypt(hdr: *mut b::setup_header) {
    unsafe {
        let on = cmdline_find_option_bool(b"mem_encrypt=on\0".as_ptr().cast());
        let off = cmdline_find_option_bool(b"mem_encrypt=off\0".as_ptr().cast());
        if on > off {
            let flag = addr_of_mut!((*hdr).xloadflags);
            write_unaligned(flag, read_unaligned(flag) | b::XLF_MEM_ENCRYPTION as u16);
        }
    }
}

unsafe fn early_sev_detect() {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    unsafe {
        if SEV_STATUS & b::MSR_AMD64_SEV_ES_ENABLED as u64 != 0 {
            LINES = 0;
            COLS = 0;
        }
    }
}

// The bootparam_utils.h preserve list, using native field offsets and sizes.
// Copy raw bytes: boot_params is packed, so never form a reference to a field.
unsafe fn preserve_field<T>(dest: *mut T, source: *const T) {
    unsafe {
        memcpy(dest.cast(), source.cast(), size_of::<T>());
    }
}

unsafe fn sanitize_boot_params(params: *mut b::boot_params) {
    unsafe {
        if (*params).sentinel == 0 {
            return;
        }
        static mut SANITIZE_SCRATCH: MaybeUninit<b::boot_params> = MaybeUninit::uninit();
        let saved = addr_of_mut!(SANITIZE_SCRATCH).cast::<b::boot_params>();
        memset(saved.cast(), 0, size_of::<b::boot_params>());
        macro_rules! preserve {
        ($($field:ident),+ $(,)?) => {
            $(preserve_field(addr_of_mut!((*saved).$field), addr_of!((*params).$field));)+
        };
    }
        preserve!(
            screen_info,
            apm_bios_info,
            tboot_addr,
            ist_info,
            hd0_info,
            hd1_info,
            sys_desc_table,
            olpc_ofw_header,
            efi_info,
            alt_mem_k,
            scratch,
            e820_entries,
            eddbuf_entries,
            edd_mbr_sig_buf_entries,
            edd_mbr_sig_buffer,
            secure_boot,
            hdr,
            e820_table,
            eddbuf,
            cc_blob_address,
        );
        memcpy(params.cast(), saved.cast(), size_of::<b::boot_params>());
    }
}

unsafe extern "C" fn decompress_error(message: *mut c_char) {
    unsafe {
        error(message);
    }
}

// asmlinkage on i386 passes both arguments on the stack. Rust's C ABI is the
// cdecl entry point; no kernel regparm convention applies to this function.
#[no_mangle]
pub(crate) unsafe extern "C" fn extract_kernel(rmode: *mut c_void, output: *mut u8) -> *mut c_void {
    unsafe {
        let virt_addr = b::LOAD_PHYSICAL_ADDR as c_ulong;
        let heap = addr_of_mut!(BOOT_HEAP.0).cast::<u8>() as Memptr;

        BOOT_PARAMS_PTR = rmode.cast();
        (*BOOT_PARAMS_PTR).hdr.loadflags &= !(b::KASLR_FLAG as u8);
        parse_mem_encrypt(addr_of_mut!((*BOOT_PARAMS_PTR).hdr));
        sanitize_boot_params(BOOT_PARAMS_PTR);

        if (*BOOT_PARAMS_PTR).screen_info.orig_video_mode == 7 {
            VIDMEM = 0xb0000 as *mut u8;
            VIDPORT = 0x3b4;
        } else {
            VIDMEM = 0xb8000 as *mut u8;
            VIDPORT = 0x3d4;
        }
        LINES = (*BOOT_PARAMS_PTR).screen_info.orig_video_lines as c_int;
        COLS = (*BOOT_PARAMS_PTR).screen_info.orig_video_cols as c_int;
        init_default_io_ops();
        #[cfg(CONFIG_INTEL_TDX_GUEST)]
        early_tdx_detect();
        early_sev_detect();
        #[cfg(CONFIG_EARLY_PRINTK)]
        console_init();
        #[cfg(CONFIG_ACPI)]
        {
            (*BOOT_PARAMS_PTR).acpi_rsdp_addr = get_rsdp_addr() as u64;
        }
        #[cfg(not(CONFIG_ACPI))]
        {
            (*BOOT_PARAMS_PTR).acpi_rsdp_addr = 0;
        }
        debug_putstr(b"early console in extract_kernel\n\0".as_ptr().cast());
        FREE_MEM_PTR = heap;
        FREE_MEM_END_PTR = heap.wrapping_add(b::BOOT_HEAP_SIZE as Memptr);

        let _needed_size = core::cmp::max(OUTPUT_LEN as c_ulong, KERNEL_TOTAL_SIZE);
        #[cfg(CONFIG_X86_64)]
        let _needed_size = _needed_size.wrapping_add(b::MIN_KERNEL_ALIGN as c_ulong - 1)
            & !(b::MIN_KERNEL_ALIGN as c_ulong - 1);

        debug_putaddr!("input_data", addr_of!(INPUT_DATA));
        debug_putaddr!("input_len", INPUT_LEN);
        debug_putaddr!("output", output);
        debug_putaddr!("output_len", OUTPUT_LEN);
        debug_putaddr!("kernel_total_size", KERNEL_TOTAL_SIZE);
        debug_putaddr!("needed_size", _needed_size);
        #[cfg(CONFIG_X86_64)]
        debug_putaddr!("trampoline_32bit", TRAMPOLINE_32BIT);

        #[cfg(CONFIG_RANDOMIZE_BASE)]
        let (output, virt_addr) = {
            let mut output_addr = output as c_ulong;
            let mut virt_addr = virt_addr;
            choose_random_location(
                addr_of!(INPUT_DATA) as c_ulong,
                INPUT_LEN as c_ulong,
                &mut output_addr,
                _needed_size,
                &mut virt_addr,
            );
            (output_addr as *mut u8, virt_addr)
        };

        if output as c_ulong & (b::MIN_KERNEL_ALIGN as c_ulong - 1) != 0 {
            error(
                b"Destination physical address inappropriately aligned\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        if virt_addr & (b::MIN_KERNEL_ALIGN as c_ulong - 1) != 0 {
            error(
                b"Destination virtual address inappropriately aligned\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        #[cfg(CONFIG_X86_64)]
        {
            if heap as c_ulong > 0x3fffffffffff {
                error(
                    b"Destination address too large\0"
                        .as_ptr()
                        .cast_mut()
                        .cast(),
                );
            }
            if virt_addr.wrapping_add(_needed_size) > b::KERNEL_IMAGE_SIZE as c_ulong {
                error(
                    b"Destination virtual address is beyond the kernel mapping area\0"
                        .as_ptr()
                        .cast_mut()
                        .cast(),
                );
            }
        }
        #[cfg(not(CONFIG_X86_64))]
        if heap as c_ulong
            > (0usize
                .wrapping_sub(b::__PAGE_OFFSET as usize)
                .wrapping_sub(128 << 20)
                .wrapping_sub(1)
                & 0x7fffffff) as c_ulong
        {
            error(
                b"Destination address too large\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        #[cfg(not(CONFIG_RELOCATABLE))]
        if virt_addr != b::LOAD_PHYSICAL_ADDR as c_ulong {
            error(
                b"Destination virtual address changed when not relocatable\0"
                    .as_ptr()
                    .cast_mut()
                    .cast(),
            );
        }
        debug_putstr(b"\nDecompressing Linux... \0".as_ptr().cast());
        #[cfg(CONFIG_UNACCEPTED_MEMORY)]
        if init_unaccepted_memory() {
            debug_putstr(b"Accepting memory... \0".as_ptr().cast());
            // misc.h explicitly defines __pa(x) as identity in the decompressor.
            accept_memory(output as b::phys_addr_t, _needed_size);
        }
        let entry_offset = decompress_kernel(output, virt_addr, decompress_error);
        debug_putstr(
            b"done.\nBooting the kernel (entry_offset: 0x\0"
                .as_ptr()
                .cast(),
        );
        debug_puthex(entry_offset);
        debug_putstr(b").\n\0".as_ptr().cast());
        #[cfg(CONFIG_X86_64)]
        cleanup_exception_handling();
        if SPURIOUS_NMI_COUNT != 0 {
            __putstr(b"Spurious early NMIs ignored: \0".as_ptr().cast());
            __putdec(SPURIOUS_NMI_COUNT as c_ulong);
            __putstr(b"\n\0".as_ptr().cast());
        }
        output.wrapping_add(entry_offset as usize).cast()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
