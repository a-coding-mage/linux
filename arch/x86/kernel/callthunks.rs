// SPDX-License-Identifier: GPL-2.0-only

// Translated from callthunks.c. Kernel includes, configuration, and assembly
// symbols are supplied by the surrounding kernel environment.

const MAX_PATCH_LEN: usize = 255 - 1;

static mut DEBUG_CALLTHUNKS: i32 = 0;

#[repr(C)]
pub struct CoreText {
    pub base: usize,
    pub end: usize,
    pub name: *const core::ffi::c_char,
}

static mut THUNKS_INITIALIZED: bool = false;

// The following symbols and types are external kernel dependencies.
extern "C" {
    static __call_sites: i32;
    static __call_sites_end: i32;
    static _text: u8;
    static _etext: u8;
    static skl_call_thunk_template: u8;
    static skl_call_thunk_tail: u8;

    fn error_entry();
    fn xen_error_entry();
    fn paranoid_entry();
    fn __switch_to_asm();
    fn ret_from_fork();
    fn insn_decode_kernel(insn: *mut Insn, addr: *mut core::ffi::c_void) -> i32;
    fn text_poke_apply_relocation(dst: *mut u8, src: *mut u8, len: u32,
                                  template: *const u8, template_len: u32);
    fn text_poke_copy_locked(dst: *mut u8, src: *const u8, len: u32, cross_modify: bool);
    fn __text_gen_insn(buf: *mut u8, opcode: u8, addr: *mut u8, target: *mut u8, len: u32);
    fn text_poke_early(addr: *mut u8, bytes: *const u8, len: u32);
    fn mutex_lock(mutex: *mut core::ffi::c_void);
    fn mutex_unlock(mutex: *mut core::ffi::c_void);
    static text_mutex: core::ffi::c_void;
}

#[repr(C)]
struct Insn {
    opcode: [u8; 4],
    length: u8,
    immediate_value: i64,
}

#[repr(C)]
pub struct CallthunkSites {
    pub call_start: *mut i32,
    pub call_end: *mut i32,
}

static BUILTIN_CORETEXT: CoreText = CoreText {
    base: unsafe { &_text as *const u8 as usize },
    end: unsafe { &_etext as *const u8 as usize },
    name: b"builtin\0".as_ptr() as *const core::ffi::c_char,
};

#[inline]
unsafe fn within_coretext(ct: *const CoreText, addr: *mut core::ffi::c_void) -> bool {
    let p = addr as usize;
    !ct.is_null() && (*ct).base <= p && p < (*ct).end
}

#[inline]
unsafe fn within_module_coretext(_addr: *mut core::ffi::c_void) -> bool {
    // CONFIG_MODULES implementation is provided by the kernel build.
    false
}

unsafe fn is_coretext(ct: *const CoreText, addr: *mut core::ffi::c_void) -> bool {
    if within_coretext(ct, addr) || within_coretext(&BUILTIN_CORETEXT, addr) { return true; }
    within_module_coretext(addr)
}

unsafe fn skip_addr(dest: *mut core::ffi::c_void) -> bool {
    if dest == error_entry as *mut _ || dest == paranoid_entry as *mut _ ||
       dest == xen_error_entry as *mut _ || dest == __switch_to_asm as *mut _ ||
       dest == ret_from_fork as *mut _ { return true; }
    // CONFIG_HOTPLUG_CPU, CONFIG_AMD_MEM_ENCRYPT, CONFIG_FUNCTION_TRACER,
    // CONFIG_KEXEC_CORE, and architecture-specific symbols are build-time dependencies.
    false
}

unsafe fn call_get_dest(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut insn = core::mem::zeroed::<Insn>();
    let ret = insn_decode_kernel(&mut insn, addr);
    if ret != 0 { return ret as isize as *mut _; }
    if insn.opcode[0] != 0xe8 { return core::ptr::null_mut(); }
    let dest = (addr as usize).wrapping_add(insn.length as usize)
        .wrapping_add(insn.immediate_value as usize) as *mut _;
    if skip_addr(dest) { core::ptr::null_mut() } else { dest }
}

static NOPS: [u8; 32] = [0x90; 32];

unsafe fn patch_dest(dest: *mut core::ffi::c_void, direct: bool) -> *mut core::ffi::c_void {
    let tsize = (&skl_call_thunk_tail as *const u8 as usize)
        .wrapping_sub(&skl_call_thunk_template as *const u8 as usize) as u32;
    let mut insn_buff = [0u8; MAX_PATCH_LEN];
    let pad = (dest as usize).wrapping_sub(tsize as usize) as *mut u8;
    core::ptr::copy_nonoverlapping(&skl_call_thunk_template, insn_buff.as_mut_ptr(), tsize as usize);
    text_poke_apply_relocation(insn_buff.as_mut_ptr(), pad, tsize, &skl_call_thunk_template, tsize);
    if core::slice::from_raw_parts(pad, tsize as usize) == &insn_buff[..tsize as usize] { return pad as *mut _; }
    if core::slice::from_raw_parts(pad, tsize as usize) != &NOPS[..tsize as usize] { return core::ptr::null_mut(); }
    if direct { core::ptr::copy_nonoverlapping(insn_buff.as_ptr(), pad, tsize as usize); }
    else { text_poke_copy_locked(pad, insn_buff.as_ptr(), tsize, true); }
    pad as *mut _
}

unsafe fn patch_call(addr: *mut core::ffi::c_void, ct: *const CoreText) {
    if !within_coretext(ct, addr) { return; }
    let dest = call_get_dest(addr);
    if dest.is_null() || !is_coretext(ct, dest) { return; }
    let pad = patch_dest(dest, within_coretext(ct, dest));
    if pad.is_null() { return; }
    let mut bytes = [0u8; 8];
    __text_gen_insn(bytes.as_mut_ptr(), 0xe8, addr as *mut u8, pad as *mut u8, 5);
    text_poke_early(addr as *mut u8, bytes.as_ptr(), 5);
}

unsafe fn patch_call_sites(start: *mut i32, end: *mut i32, ct: *const CoreText) {
    let mut s = start;
    while s < end {
        patch_call((s as usize).wrapping_add((*s) as isize as usize) as *mut _, ct);
        s = s.add(1);
    }
}

unsafe fn callthunks_setup(cs: *mut CallthunkSites, ct: *const CoreText) {
    patch_call_sites((*cs).call_start, (*cs).call_end, ct);
}

pub unsafe fn callthunks_patch_builtin_calls() {
    // cpu_feature_enabled(X86_FEATURE_CALL_DEPTH) is a kernel build dependency.
    let cs = CallthunkSites { call_start: &__call_sites as *const _ as *mut _, call_end: &__call_sites_end as *const _ as *mut _ };
    mutex_lock(&text_mutex as *const _ as *mut _);
    callthunks_setup(&cs as *const _ as *mut _, &BUILTIN_CORETEXT);
    THUNKS_INITIALIZED = true;
    mutex_unlock(&text_mutex as *const _ as *mut _);
}

pub unsafe fn callthunks_translate_call_dest(dest: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if !THUNKS_INITIALIZED || skip_addr(dest) || !is_coretext(core::ptr::null(), dest) { return dest; }
    let target = patch_dest(dest, false);
    if target.is_null() { dest } else { target }
}

#[cfg(feature = "bpf_jit")]
unsafe fn is_callthunk(addr: *mut core::ffi::c_void) -> bool {
    let tmpl_size = (&skl_call_thunk_tail as *const u8 as usize)
        .wrapping_sub(&skl_call_thunk_template as *const u8 as usize);
    let dest = (addr as usize + CONFIG_FUNCTION_ALIGNMENT - 1)
        & !(CONFIG_FUNCTION_ALIGNMENT - 1);
    if !THUNKS_INITIALIZED || skip_addr(dest as *mut _) { return false; }
    let pad = (dest - tmpl_size) as *mut u8;
    let mut insn_buff = [0u8; MAX_PATCH_LEN];
    core::ptr::copy_nonoverlapping(&skl_call_thunk_template, insn_buff.as_mut_ptr(), tmpl_size);
    text_poke_apply_relocation(insn_buff.as_mut_ptr(), pad, tmpl_size as u32,
                               &skl_call_thunk_template, tmpl_size as u32);
    core::slice::from_raw_parts(pad, tmpl_size) == &insn_buff[..tmpl_size]
}

#[cfg(feature = "bpf_jit")]
pub unsafe fn x86_call_depth_emit_accounting(
    pprog: *mut *mut u8, func: *mut core::ffi::c_void, ip: *mut core::ffi::c_void,
) -> u32 {
    let tmpl_size = (&skl_call_thunk_tail as *const u8 as usize)
        .wrapping_sub(&skl_call_thunk_template as *const u8 as usize);
    if !THUNKS_INITIALIZED || (!func.is_null() && is_callthunk(func)) { return 0; }
    let mut insn_buff = [0u8; MAX_PATCH_LEN];
    core::ptr::copy_nonoverlapping(&skl_call_thunk_template, insn_buff.as_mut_ptr(), tmpl_size);
    text_poke_apply_relocation(insn_buff.as_mut_ptr(), ip as *mut u8, tmpl_size as u32,
                               &skl_call_thunk_template, tmpl_size as u32);
    core::ptr::copy_nonoverlapping(insn_buff.as_ptr(), *pprog, tmpl_size);
    *pprog = (*pprog).add(tmpl_size);
    tmpl_size as u32
}

#[cfg(feature = "modules")]
pub unsafe fn callthunks_patch_module_calls(cs: *mut CallthunkSites, mod_ptr: *mut Module) {
    let ct = CoreText { base: (*mod_ptr).text_base, end: (*mod_ptr).text_base + (*mod_ptr).text_size, name: (*mod_ptr).name };
    if !THUNKS_INITIALIZED { return; }
    mutex_lock(&text_mutex as *const _ as *mut _);
    callthunks_setup(cs, &ct);
    mutex_unlock(&text_mutex as *const _ as *mut _);
}

#[cfg(feature = "modules")]
#[repr(C)]
pub struct Module { pub text_base: usize, pub text_size: usize, pub name: *const core::ffi::c_char }

// CONFIG_CALL_THUNKS_DEBUG and CONFIG_DEBUG_FS provide the per-CPU counters,
// seq_file operations, debugfs_create_dir/debugfs_create_file, and initcall
// registration used by callthunks_debug_show/open and callthunks_debugfs_init.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
