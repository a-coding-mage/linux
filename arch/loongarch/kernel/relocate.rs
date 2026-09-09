// SPDX-License-Identifier: GPL-2.0
/*
 * Support for Kernel relocation at boot time
 *
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

// C dependencies supplied by the kernel and architecture headers are intentionally
// left as external Rust symbols/types.

use core::ffi::{c_char, c_int, c_long, c_void};

type Elf64_Addr = u64;
type U64 = u64;

#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: u64,
    pub r_addend: i64,
}

#[repr(C)]
pub struct RelaLaAbs {
    pub symvalue: c_long,
    pub pc: *mut c_void,
}

#[repr(C)]
pub union LoongarchInstruction {
    pub reg1i20_format: Reg1i20Format,
    pub reg2i12_format: Reg2i12Format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Reg1i20Format {
    pub immediate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Reg2i12Format {
    pub immediate: u32,
}

#[repr(C)]
pub struct NotifierBlock {
    pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, c_ulong, *mut c_void) -> c_int>,
}

type c_ulong = usize;

extern "C" {
    static mut __rela_dyn_begin: Elf64_Rela;
    static mut __rela_dyn_end: Elf64_Rela;
    static mut __la_abs_begin: RelaLaAbs;
    static mut __la_abs_end: RelaLaAbs;
    static mut _text: c_void;
    static mut _end: c_void;
    static mut _sdata: c_void;
    static mut __bss_start: c_void;
    static mut boot_command_line: *mut c_char;
    static mut current_thread_pointer: *mut c_void;
    static mut panic_notifier_list: NotifierBlock;
    static linux_banner: *const c_char;
    static mut fw_arg1: usize;
    static mut reloc_offset: usize;
    static VMLINUX_LOAD_ADDRESS: usize;
    static CONFIG_CMDLINE: *const c_char;
    static CONFIG_RANDOMIZE_BASE_MAX_OFFSET: usize;

    fn random_get_entropy() -> usize;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> usize;
    fn PHYSADDR(addr: *mut c_void) -> usize;
    fn early_memremap_ro(addr: usize, size: usize) -> *mut c_char;
    fn early_memunmap(addr: *mut c_char, size: usize);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn printk(level: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn atomic_notifier_chain_register(chain: *mut NotifierBlock, block: *mut NotifierBlock) -> c_int;
}

const R_LARCH_RELATIVE: u64 = 3;
const COMMAND_LINE_SIZE: usize = 2048;
const NOTIFY_DONE: c_int = 0;
const KERN_EMERG: *const c_char = b"<0>\0".as_ptr() as *const c_char;

#[inline]
unsafe fn relocated<T>(x: *mut T) -> *mut T {
    (x as isize).wrapping_add(reloc_offset as isize) as *mut T
}

#[inline]
unsafe fn relocated_kaslr<T>(x: *mut T, random_offset: c_long) -> *mut T {
    (x as isize).wrapping_add(random_offset as isize) as *mut T
}

#[inline]
unsafe fn relocate_relative() {
    let mut rela = &mut __rela_dyn_begin as *mut Elf64_Rela;
    let rela_end = &mut __rela_dyn_end as *mut Elf64_Rela;

    while rela < rela_end {
        let addr = (*rela).r_offset;
        let mut relocated_addr = (*rela).r_addend as Elf64_Addr;

        if (*rela).r_info != R_LARCH_RELATIVE {
            rela = rela.add(1);
            continue;
        }

        relocated_addr = relocated(relocated_addr as *mut c_void) as Elf64_Addr;
        *(relocated(addr as *mut c_void) as *mut Elf64_Addr) = relocated_addr;
        rela = rela.add(1);
    }

    // CONFIG_RELR
    #[cfg(CONFIG_RELR)]
    {
        let mut addr: *mut U64 = core::ptr::null_mut();
        let mut relr = &mut __relr_dyn_begin as *mut U64;
        let relr_end = &mut __relr_dyn_end as *mut U64;

        while relr < relr_end {
            if (*relr & 1) == 0 {
                addr = ((*relr).wrapping_add(reloc_offset as U64)) as *mut U64;
                *addr = (*addr).wrapping_add(reloc_offset as U64);
                addr = addr.add(1);
            } else {
                let mut p = addr;
                let mut r = *relr >> 1;
                while r != 0 {
                    if (r & 1) != 0 {
                        *p = (*p).wrapping_add(reloc_offset as U64);
                    }
                    p = p.add(1);
                    r >>= 1;
                }
                addr = addr.add(63);
            }
            relr = relr.add(1);
        }
    }
}

#[inline]
unsafe fn relocate_absolute(random_offset: c_long) {
    let begin = relocated_kaslr(&mut __la_abs_begin, random_offset);
    let end = relocated_kaslr(&mut __la_abs_end, random_offset);
    let mut p = begin;

    while (p as *mut c_void) < (end as *mut c_void) {
        let v = (*p).symvalue;
        let lu12iw = ((v >> 12) as u32) & 0xfffff;
        let ori = (v as u32) & 0xfff;
        // CONFIG_64BIT
        #[cfg(CONFIG_64BIT)]
        let lu32id = ((v >> 32) as u32) & 0xfffff;
        #[cfg(CONFIG_64BIT)]
        let lu52id = (v >> 52) as u32;
        let insn = (*p).pc as *mut LoongarchInstruction;

        (*insn).reg1i20_format.immediate = lu12iw;
        (*insn.add(1)).reg2i12_format.immediate = ori;
        #[cfg(CONFIG_64BIT)]
        {
            (*insn.add(2)).reg1i20_format.immediate = lu32id;
            (*insn.add(3)).reg2i12_format.immediate = lu52id;
        }
        p = p.add(1);
    }
}

// CONFIG_RANDOMIZE_BASE
#[cfg(CONFIG_RANDOMIZE_BASE)]
#[inline]
unsafe fn rotate_xor(mut hash: usize, area: *const c_void, mut size: usize) -> usize {
    let ptr = ((area as usize + core::mem::size_of::<usize>() - 1)
        & !(core::mem::size_of::<usize>() - 1)) as *const usize;
    let diff = ptr as usize - area as usize;
    if size < diff + core::mem::size_of::<usize>() { return hash; }
    size = (size - diff) & !(core::mem::size_of::<usize>() - 1);
    for i in 0..(size / core::mem::size_of::<usize>()) {
        hash = hash.rotate_left(core::mem::size_of::<usize>() as u32 * 8 - 7);
        hash ^= *ptr.add(i);
    }
    hash
}

// CONFIG_RANDOMIZE_BASE
#[cfg(CONFIG_RANDOMIZE_BASE)]
#[inline]
unsafe fn get_random_boot() -> usize {
    let mut hash = 0;
    let entropy = random_get_entropy();
    hash = rotate_xor(hash, linux_banner as *const c_void, strlen(linux_banner));
    rotate_xor(hash, &entropy as *const usize as *const c_void, core::mem::size_of_val(&entropy))
}

// CONFIG_RANDOMIZE_BASE and CONFIG_HIBERNATION retain their source conditional intent below.
#[cfg(CONFIG_RANDOMIZE_BASE)]
unsafe fn nokaslr(_p: *mut c_char) -> c_int { 0 }

#[cfg(CONFIG_RANDOMIZE_BASE)]
#[inline]
unsafe fn kaslr_disabled() -> bool {
    let builtin_cmdline = CONFIG_CMDLINE;
    let mut str_ = strstr(builtin_cmdline, b"nokaslr\0".as_ptr() as *const c_char);
    if str_ == builtin_cmdline as *mut c_char || (str_ > builtin_cmdline as *mut c_char && *str_.offset(-1) == b' ' as c_char) { return true; }
    str_ = strstr(boot_command_line, b"nokaslr\0".as_ptr() as *const c_char);
    if str_ == boot_command_line || (str_ > boot_command_line && *str_.offset(-1) == b' ' as c_char) { return true; }
    str_ = strstr(boot_command_line, b"kexec_file\0".as_ptr() as *const c_char);
    if str_ == boot_command_line || (str_ > boot_command_line && *str_.offset(-1) == b' ' as c_char) { return true; }
    false
}

#[cfg(CONFIG_RANDOMIZE_BASE)]
#[inline]
unsafe fn determine_relocation_address() -> *mut c_void {
    let destination = &mut _text as *mut c_void;
    if kaslr_disabled() { return destination; }
    let kernel_length = (&_end as *const c_void as usize).wrapping_sub(&_text as *const c_void as usize);
    let mut random_offset = get_random_boot() << 16;
    random_offset &= CONFIG_RANDOMIZE_BASE_MAX_OFFSET - 1;
    if random_offset < kernel_length { random_offset = random_offset.wrapping_add((kernel_length + 0xffff) & !0xffff); }
    relocated_kaslr(destination, random_offset as c_long)
}

#[cfg(CONFIG_RANDOMIZE_BASE)]
unsafe fn determine_initrd_address(size: *mut usize) -> usize {
    let mut start = 0;
    let mut endp: *mut c_char = core::ptr::null_mut();
    let mut key = b"initrd=\0".as_ptr() as *const c_char;
    let mut key_length = strlen(key);
    let mut p = strstr(boot_command_line, key);
    if p.is_null() {
        key = b"initrdmem=\0".as_ptr() as *const c_char;
        key_length = strlen(key);
        p = strstr(boot_command_line, key);
    }
    if p == boot_command_line || (p > boot_command_line && *p.offset(-1) == b' ' as c_char) {
        p = p.add(key_length);
        start = memparse(p, &mut endp);
        if !endp.is_null() && *endp == b',' as c_char { *size = memparse(endp.add(1), core::ptr::null_mut()); }
    }
    start
}

#[cfg(CONFIG_RANDOMIZE_BASE)]
#[inline]
unsafe fn relocation_addr_valid(location_new: *mut c_void) -> c_int {
    let mut initrd_size = 0usize;
    if (location_new as usize & 0x00000ffff) != 0 { return 0; }
    if (location_new as usize) < (&_end as *const c_void as usize) { return 0; }
    let initrd_start = determine_initrd_address(&mut initrd_size);
    if initrd_start != 0 && initrd_size != 0 {
        let kernel_start = PHYSADDR(location_new);
        let kernel_size = (&_end as *const c_void as usize).wrapping_sub(&_text as *const c_void as usize);
        if kernel_start < initrd_start.wrapping_add(initrd_size) && initrd_start < kernel_start.wrapping_add(kernel_size) { return 0; }
    }
    1
}

#[inline]
unsafe fn update_reloc_offset(addr: *mut usize, random_offset: c_long) {
    let new_addr = relocated_kaslr(addr, random_offset);
    *new_addr = reloc_offset;
}

pub unsafe fn relocate_kernel() -> usize {
    let mut kernel_length;
    let mut random_offset = 0usize;
    let mut location_new = &mut _text as *mut c_void;
    let cmdline = early_memremap_ro(fw_arg1, COMMAND_LINE_SIZE);
    strscpy(boot_command_line, cmdline, COMMAND_LINE_SIZE);

    #[cfg(CONFIG_RANDOMIZE_BASE)]
    {
        location_new = determine_relocation_address();
        if relocation_addr_valid(location_new) { random_offset = (location_new as usize).wrapping_sub(&_text as *const c_void as usize); }
    }
    reloc_offset = (&_text as *const c_void as usize).wrapping_sub(VMLINUX_LOAD_ADDRESS);
    early_memunmap(cmdline, COMMAND_LINE_SIZE);

    if random_offset != 0 {
        kernel_length = (&_end as *const c_void as usize).wrapping_sub(&_text as *const c_void as usize);
        memcpy(location_new, &_text as *const c_void, kernel_length);
        core::arch::asm!("ibar 0", "dbar 0", options(nostack));
        reloc_offset = reloc_offset.wrapping_add(random_offset);
        current_thread_pointer = relocated_kaslr(current_thread_pointer, random_offset as c_long);
        update_reloc_offset(&mut reloc_offset, random_offset as c_long);
    }
    if reloc_offset != 0 { relocate_relative(); }
    relocate_absolute(random_offset as c_long);
    random_offset
}

unsafe fn show_kernel_relocation(level: *const c_char) {
    if reloc_offset > 0 {
        printk(level);
        pr_cont(b"Kernel relocated by 0x%lx\n\0".as_ptr() as *const c_char, reloc_offset);
        pr_cont(b" .text @ 0x%px\n\0".as_ptr() as *const c_char, &_text as *const c_void);
        pr_cont(b" .data @ 0x%px\n\0".as_ptr() as *const c_char, &_sdata as *const c_void);
        pr_cont(b" .bss  @ 0x%px\n\0".as_ptr() as *const c_char, &__bss_start as *const c_void);
    }
}

unsafe extern "C" fn kernel_location_notifier_fn(_self: *mut NotifierBlock, _v: c_ulong, _p: *mut c_void) -> c_int {
    show_kernel_relocation(KERN_EMERG);
    NOTIFY_DONE
}

static mut KERNEL_LOCATION_NOTIFIER: NotifierBlock = NotifierBlock { notifier_call: Some(kernel_location_notifier_fn) };

unsafe fn register_kernel_offset_dumper() -> c_int {
    atomic_notifier_chain_register(&mut panic_notifier_list, &mut KERNEL_LOCATION_NOTIFIER);
    0
}

// arch_initcall(register_kernel_offset_dumper);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
