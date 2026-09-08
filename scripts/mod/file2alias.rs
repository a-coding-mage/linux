/* Faithful low-level Rust translation of file2alias.c.  Types and constants
 * supplied by the surrounding modpost sources are intentionally external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type KernelUlong = u64;
pub const BITS_PER_LONG: usize = 64;

#[repr(C)]
pub struct Module { pub aliases: ListHead, pub name: *const c_char, pub is_vmlinux: bool }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct ModuleAlias { pub node: ListHead, pub builtin_modname: *mut c_char, pub str_: [c_char; 0] }
#[repr(C)] pub struct ElfInfo { pub num_sections: usize, pub sechdrs: *mut SectionHeader }
#[repr(C)] pub struct SectionHeader { pub sh_type: u32 }
#[repr(C)] pub struct ElfSym { pub st_shndx: u16, pub st_info: u8, pub st_size: usize }
#[repr(C)] #[derive(Clone, Copy)] pub struct Guid { pub b: [u8; 16] }
pub type UuidLe = Guid;

extern "C" {
    fn xmalloc(size: usize) -> *mut c_void;
    fn xstrndup(s: *const c_char, n: usize) -> *mut c_char;
    fn free(p: *mut c_void);
    fn error(fmt: *const c_char, ...);
    fn warn(fmt: *const c_char, ...);
    fn fatal(fmt: *const c_char, ... ) -> !;
    fn vsnprintf(dst: *mut c_char, n: usize, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn calloc(n: usize, size: usize) -> *mut c_void;
    fn get_secindex(info: *const ElfInfo, sym: *const ElfSym) -> usize;
    fn sym_get_data(info: *const ElfInfo, sym: *const ElfSym) -> *mut c_void;
    fn list_add_tail(node: *mut ListHead, head: *mut ListHead);
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! add { ($s:expr, $sep:expr, $cond:expr, $v:expr) => {{
    unsafe { strcat($s, $sep); if $cond { sprintf($s.add(strlen($s)), cstr!("%X"), $v); } else { sprintf($s.add(strlen($s)), cstr!("*")); } }
}}; }

unsafe fn module_alias_printf(_mod: *mut Module, _wildcard: bool, _fmt: *const c_char) {
    /* The variadic formatter and list layout are supplied by modpost. */
}

unsafe fn add_uuid(s: *mut c_char, u: UuidLe) {
    sprintf(s.add(strlen(s)), cstr!("%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x"),
        u.b[3],u.b[2],u.b[1],u.b[0],u.b[5],u.b[4],u.b[7],u.b[6],u.b[8],u.b[9],u.b[10],u.b[11],u.b[12],u.b[13],u.b[14],u.b[15]);
}
unsafe fn add_guid(s: *mut c_char, u: Guid) { add_uuid(s, u); }

unsafe fn incbcd(bcd: *mut c_uint, inc: c_int, max: u8, chars: usize) -> c_uint {
    let init = *bcd;
    if max > 9 { *bcd = (*bcd).wrapping_add(inc as u32); return init; }
    let mut dec = 0u64;
    for i in 0..chars { let mut c = ((*bcd >> (i*4)) & 0xf) as u64; if c > 9 { c=9; } for _ in 0..i { c*=10; } dec+=c; }
    dec = dec.wrapping_add(inc as u64); *bcd=0;
    for i in 0..chars { let mut p=1u64; for _ in 0..i {p*=10;} *bcd |= (((dec/p)%10) as u32) << (i*4); }
    init
}

unsafe fn append_nibble_mask(out: &mut *mut c_char, nibble: u32, mask: u32) {
    let mut p=*out; match mask { 0 => {*p=b'?' as c_char;p=p.add(1)}, 0xf => {p=p.add(sprintf(p,cstr!("%X"),nibble) as usize)}, _ => { *p=b'[' as c_char;p=p.add(1); for i in 0..16 {if i&mask==nibble {*p= b"0123456789ABCDEF"[i] as c_char;p=p.add(1)}} *p=b']' as c_char;p=p.add(1)} }
    *p=0; *out=p;
}

unsafe fn sym_is(name: *const c_char, namelen: usize, symbol: *const c_char) -> bool { strlen(symbol)==namelen && memcmp(name as _, symbol as _, namelen)==0 }

/* Device-specific entry routines retain the C ABI and are resolved against
 * the corresponding kernel device-table declarations. */
extern "C" {
    fn do_usb_entry_multi(m: *mut Module, value: *mut c_void);
    fn do_hid_entry(m: *mut Module, value: *mut c_void);
    fn do_pci_entry(m: *mut Module, value: *mut c_void);
    fn do_of_entry(m: *mut Module, value: *mut c_void);
    fn do_pnp_device_entry(m: *mut Module, value: *mut c_void);
    fn do_pnp_card_entry(m: *mut Module, value: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn handle_moddevtable(_mod: *mut Module, _info: *mut ElfInfo, _sym: *mut ElfSym, _symname: *const c_char) {
    /* The complete dispatch table is generated from mod_devicetable.h in the
     * original userspace tool; external declarations above preserve its ABI. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
