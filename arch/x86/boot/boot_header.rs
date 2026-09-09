/* SPDX-License-Identifier: GPL-2.0-only */
/* Header file for the real-mode kernel code. */

/* STACK_SIZE: minimum number of bytes for stack. */
pub const STACK_SIZE: usize = 1024;

/* Dependencies supplied by the surrounding translated sources. */
extern "C" {
    pub static mut hdr: setup_header;
    pub static mut boot_params: boot_params;
    pub static mut _end: u8;
    pub static mut HEAP: *mut u8;
    pub static mut heap_end: *mut u8;
}

pub type addr_t = ::core::ffi::c_uint;

#[inline(always)]
pub unsafe fn cpu_relax() { ::core::arch::asm!("pause"); }

#[inline(always)]
pub unsafe fn io_delay() { outb(0, 0x80); }

#[inline(always)] pub unsafe fn ds() -> u16 { let mut v: u16; ::core::arch::asm!("movw %ds, {0}", out(reg) v); v }
#[inline(always)] pub unsafe fn set_fs(seg: u16) { ::core::arch::asm!("movw {0}, %fs", in(reg) seg); }
#[inline(always)] pub unsafe fn fs() -> u16 { let mut v: u16; ::core::arch::asm!("movw %fs, {0}", out(reg) v); v }
#[inline(always)] pub unsafe fn set_gs(seg: u16) { ::core::arch::asm!("movw {0}, %gs", in(reg) seg); }
#[inline(always)] pub unsafe fn gs() -> u16 { let mut v: u16; ::core::arch::asm!("movw %gs, {0}", out(reg) v); v }

#[inline(always)] pub unsafe fn rdfs8(addr: addr_t) -> u8 { let p = absolute_pointer(addr) as *const u8; let mut v: u8; ::core::arch::asm!("movb %fs:[{p}], {v}", p = in(reg) p, v = out(reg_byte) v); v }
#[inline(always)] pub unsafe fn rdfs16(addr: addr_t) -> u16 { let p = absolute_pointer(addr) as *const u16; let mut v: u16; ::core::arch::asm!("movw %fs:[{p}], {v}", p = in(reg) p, v = out(reg) v); v }
#[inline(always)] pub unsafe fn rdfs32(addr: addr_t) -> u32 { let p = absolute_pointer(addr) as *const u32; let mut v: u32; ::core::arch::asm!("movl %fs:[{p}], {v}", p = in(reg) p, v = out(reg) v); v }
#[inline(always)] pub unsafe fn wrfs8(v: u8, addr: addr_t) { let p = absolute_pointer(addr) as *mut u8; ::core::arch::asm!("movb {v}, %fs:[{p}]", v = in(reg_byte) v, p = in(reg) p); }
#[inline(always)] pub unsafe fn wrfs16(v: u16, addr: addr_t) { let p = absolute_pointer(addr) as *mut u16; ::core::arch::asm!("movw {v}, %fs:[{p}]", v = in(reg) v, p = in(reg) p); }
#[inline(always)] pub unsafe fn wrfs32(v: u32, addr: addr_t) { let p = absolute_pointer(addr) as *mut u32; ::core::arch::asm!("movl {v}, %fs:[{p}]", v = in(reg) v, p = in(reg) p); }
#[inline(always)] pub unsafe fn rdgs8(addr: addr_t) -> u8 { let p = absolute_pointer(addr) as *const u8; let mut v: u8; ::core::arch::asm!("movb %gs:[{p}], {v}", p = in(reg) p, v = out(reg_byte) v); v }
#[inline(always)] pub unsafe fn rdgs16(addr: addr_t) -> u16 { let p = absolute_pointer(addr) as *const u16; let mut v: u16; ::core::arch::asm!("movw %gs:[{p}], {v}", p = in(reg) p, v = out(reg) v); v }
#[inline(always)] pub unsafe fn rdgs32(addr: addr_t) -> u32 { let p = absolute_pointer(addr) as *const u32; let mut v: u32; ::core::arch::asm!("movl %gs:[{p}], {v}", p = in(reg) p, v = out(reg) v); v }
#[inline(always)] pub unsafe fn wrgs8(v: u8, addr: addr_t) { let p = absolute_pointer(addr) as *mut u8; ::core::arch::asm!("movb {v}, %gs:[{p}]", v = in(reg_byte) v, p = in(reg) p); }
#[inline(always)] pub unsafe fn wrgs16(v: u16, addr: addr_t) { let p = absolute_pointer(addr) as *mut u16; ::core::arch::asm!("movw {v}, %gs:[{p}]", v = in(reg) v, p = in(reg) p); }
#[inline(always)] pub unsafe fn wrgs32(v: u32, addr: addr_t) { let p = absolute_pointer(addr) as *mut u32; ::core::arch::asm!("movl {v}, %gs:[{p}]", v = in(reg) v, p = in(reg) p); }

#[inline(always)] pub unsafe fn memcmp_fs(s1: *const core::ffi::c_void, s2: addr_t, len: usize) -> bool { let mut d: u8; ::core::arch::asm!("fs repe cmpsb", inout("rdi") s1 => _, inout("rsi") s2 => _, inout("rcx") len => _, "setz {d}", d = out(reg_byte) d); d == 0 }
#[inline(always)] pub unsafe fn memcmp_gs(s1: *const core::ffi::c_void, s2: addr_t, len: usize) -> bool { let mut d: u8; ::core::arch::asm!("gs repe cmpsb", inout("rdi") s1 => _, inout("rsi") s2 => _, inout("rcx") len => _, "setz {d}", d = out(reg_byte) d); d == 0 }

#[inline(always)] pub unsafe fn reset_heap() { HEAP = &_end as *const u8 as *mut u8; }
#[inline(always)] pub unsafe fn __get_heap(s: usize, a: usize, n: usize) -> *mut u8 { HEAP = (((HEAP as usize).wrapping_add(a.wrapping_sub(1))) & !a.wrapping_sub(1)) as *mut u8; let t = HEAP; HEAP = HEAP.add(s.wrapping_mul(n)); t }
#[inline(always)] pub unsafe fn heap_free(n: usize) -> bool { (heap_end as isize - HEAP as isize) >= n as isize }

#[repr(C)]
pub union biosregs { pub dword: biosregs_dword, pub word: biosregs_word, pub byte: biosregs_byte }
#[repr(C)] pub struct biosregs_dword { pub edi:u32,pub esi:u32,pub ebp:u32,pub _esp:u32,pub ebx:u32,pub edx:u32,pub ecx:u32,pub eax:u32,pub _fsgs:u32,pub _dses:u32,pub eflags:u32 }
#[repr(C)] pub struct biosregs_word { pub di:u16,pub hdi:u16,pub si:u16,pub hsi:u16,pub bp:u16,pub hbp:u16,pub _sp:u16,pub _hsp:u16,pub bx:u16,pub hbx:u16,pub dx:u16,pub hdx:u16,pub cx:u16,pub hcx:u16,pub ax:u16,pub hax:u16,pub gs:u16,pub fs:u16,pub es:u16,pub ds:u16,pub flags:u16,pub hflags:u16 }
#[repr(C)] pub struct biosregs_byte { pub bytes:[u8;44] }

extern "C" {
    pub fn outb(value: u8, port: u16); pub fn absolute_pointer(addr: addr_t) -> *mut u8;
    pub fn copy_to_fs(dst: addr_t, src: *mut core::ffi::c_void, len: usize); pub fn copy_from_fs(dst:*mut core::ffi::c_void,src:addr_t,len:usize)->*mut core::ffi::c_void;
    pub fn enable_a20()->i32; pub fn query_apm_bios()->i32; pub fn intcall(int_no:u8,ireg:*const biosregs,oreg:*mut biosregs);
    pub fn __cmdline_find_option(p:usize,o:*const i8,b:*mut i8,n:i32)->i32; pub fn __cmdline_find_option_bool(p:usize,o:*const i8)->i32;
    pub fn check_cpu(a:*mut i32,b:*mut i32,c:*mut *mut u32)->i32; pub fn check_knl_erratum()->i32; pub fn validate_cpu()->i32;
    pub static mut early_serial_base:i32; pub fn console_init(); pub fn query_edd(); pub fn die()->!; pub fn detect_memory(); pub fn go_to_protected_mode()->!; pub fn protected_mode_jump(e:u32,b:u32)->!;
    pub fn initregs(r:*mut biosregs); pub fn set_video(); pub fn set_mode(m:u16)->i32; pub fn mode_defined(m:u16)->i32; pub fn probe_cards(u:i32); pub fn vesa_store_edid();
    pub fn sprintf(buf:*mut i8,fmt:*const i8,...)->i32; pub fn vsprintf(buf:*mut i8,fmt:*const i8,args:va_list)->i32; pub fn printf(fmt:*const i8,...)->i32;
    pub fn strcmp(a:*const i8,b:*const i8)->i32; pub fn strncmp(a:*const i8,b:*const i8,n:usize)->i32; pub fn strnlen(s:*const i8,n:usize)->usize; pub fn simple_strtoull(s:*const i8,e:*mut *mut i8,b:u32)->u64; pub fn strlen(s:*const i8)->usize; pub fn strchr(s:*const i8,c:i32)->*mut i8;
    pub fn puts(s:*const i8); pub fn putchar(c:i32); pub fn getchar()->i32; pub fn kbd_flush(); pub fn getchar_timeout()->i32;
}

pub type va_list = *mut core::ffi::c_void;

#[inline(always)] pub unsafe fn cmdline_find_option(option:*const i8, buffer:*mut i8, bufsize:i32)->i32 { let p=boot_params.hdr.cmd_line_ptr as usize; if p >= 0x100000 { -1 } else { __cmdline_find_option(p,option,buffer,bufsize) } }
#[inline(always)] pub unsafe fn cmdline_find_option_bool(option:*const i8)->i32 { let p=boot_params.hdr.cmd_line_ptr as usize; if p >= 0x100000 { -1 } else { __cmdline_find_option_bool(p,option) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
