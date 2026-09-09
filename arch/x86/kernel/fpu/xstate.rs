// SPDX-License-Identifier: GPL-2.0-only
/* xsave/xrstor support. */

// C headers and build-time configuration are supplied by the surrounding kernel crate.

static XFEATURE_NAMES: [&'static [u8]; 21] = [
    b"x87 floating point registers", b"SSE registers", b"AVX registers",
    b"MPX bounds registers", b"MPX CSR", b"AVX-512 opmask", b"AVX-512 Hi256",
    b"AVX-512 ZMM_Hi256", b"Processor Trace (unused)", b"Protection Keys User registers",
    b"PASID state", b"Control-flow User registers", b"Control-flow Kernel registers (KVM only)",
    b"unknown xstate feature", b"unknown xstate feature", b"unknown xstate feature",
    b"unknown xstate feature", b"AMX Tile config", b"AMX Tile data", b"APX registers",
    b"unknown xstate feature",
];

static mut XSAVE_CPUID_FEATURES: [u16; XFEATURE_MAX] = [0; XFEATURE_MAX];
static mut XSTATE_OFFSETS: [u32; XFEATURE_MAX] = [u32::MAX; XFEATURE_MAX];
static mut XSTATE_SIZES: [u32; XFEATURE_MAX] = [u32::MAX; XFEATURE_MAX];
static mut XSTATE_FLAGS: [u32; XFEATURE_MAX] = [0; XFEATURE_MAX];
static mut XFEATURE_UNCOMPACT_ORDER: [u32; XFEATURE_MAX] = [u32::MAX; XFEATURE_MAX];

const XSTATE_FLAG_SUPERVISOR: u32 = 1 << 0;
const XSTATE_FLAG_ALIGNED64: u32 = 1 << 1;

#[inline]
unsafe fn next_xfeature_order(mut i: usize, mask: u64) -> usize {
    while XFEATURE_UNCOMPACT_ORDER[i] != u32::MAX {
        if mask & (1u64 << XFEATURE_UNCOMPACT_ORDER[i]) != 0 { break; }
        i += 1;
    }
    i
}

unsafe fn xfeature_is_aligned64(n: i32) -> bool { XSTATE_FLAGS[n as usize] & XSTATE_FLAG_ALIGNED64 != 0 }
unsafe fn xfeature_is_supervisor(n: i32) -> bool { XSTATE_FLAGS[n as usize] & XSTATE_FLAG_SUPERVISOR != 0 }

unsafe fn xfeature_get_offset(xcomp_bv: u64, xfeature: i32) -> u32 {
    if !cpu_feature_enabled(X86_FEATURE_XCOMPACTED) || xfeature <= XFEATURE_SSE { return XSTATE_OFFSETS[xfeature as usize]; }
    let mut offs = FXSAVE_SIZE + XSAVE_HDR_SIZE;
    let mut i = FIRST_EXTENDED_XFEATURE as usize;
    while i < 8 * core::mem::size_of::<u64>() {
        if xcomp_bv & (1u64 << i) != 0 {
            if xfeature_is_aligned64(i as i32) { offs = ALIGN(offs, 64); }
            if i as i32 == xfeature { break; }
            offs += XSTATE_SIZES[i];
        }
        i += 1;
    }
    offs
}

pub unsafe fn cpu_has_xfeatures(xfeatures_needed: u64, feature_name: *mut *const u8) -> i32 {
    let missing = xfeatures_needed & !fpu_kernel_cfg.max_features;
    if !feature_name.is_null() {
        let print = if missing != 0 { missing } else { xfeatures_needed };
        let mut idx = 63 - print.leading_zeros() as usize;
        if idx >= XFEATURE_NAMES.len() { idx = XFEATURE_NAMES.len() - 1; }
        *feature_name = XFEATURE_NAMES[idx].as_ptr();
    }
    if missing != 0 { 0 } else { 1 }
}

pub unsafe fn fpu__init_cpu_xstate() {
    if !boot_cpu_has(X86_FEATURE_XSAVE) || fpu_kernel_cfg.max_features == 0 { return; }
    cr4_set_bits(X86_CR4_OSXSAVE);
    if cpu_feature_enabled(X86_FEATURE_XFD) { xfd_set_state(init_fpstate.xfd); }
    xsetbv(XCR_XFEATURE_ENABLED_MASK, fpu_user_cfg.max_features);
    if boot_cpu_has(X86_FEATURE_XSAVES) { wrmsrq(MSR_IA32_XSS, xfeatures_mask_supervisor() | xfeatures_mask_independent()); }
}

unsafe fn xfeature_enabled(x: i32) -> bool { fpu_kernel_cfg.max_features & (1u64 << x) != 0 }

unsafe fn compare_xstate_offsets(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    XSTATE_OFFSETS[*(a as *const u32) as usize] as i32 - XSTATE_OFFSETS[*(b as *const u32) as usize] as i32
}

unsafe fn setup_xstate_cache() {
    let mut eax=0; let mut ebx=0; let mut ecx=0; let mut edx=0;
    XSTATE_OFFSETS[XFEATURE_FP as usize]=0;
    XSTATE_SIZES[XFEATURE_FP as usize]=core::mem::offset_of!(fxregs_state, xmm_space) as u32;
    XSTATE_OFFSETS[XFEATURE_SSE as usize]=XSTATE_SIZES[XFEATURE_FP as usize];
    XSTATE_SIZES[XFEATURE_SSE as usize]=core::mem::size_of_val(&(*(core::ptr::null::<fxregs_state>())).xmm_space) as u32;
    let mut i=0usize;
    for x in FIRST_EXTENDED_XFEATURE..XFEATURE_MAX {
        if fpu_kernel_cfg.max_features & (1u64<<x) == 0 { continue; }
        cpuid_count(CPUID_LEAF_XSTATE,x,&mut eax,&mut ebx,&mut ecx,&mut edx);
        XSTATE_SIZES[x as usize]=eax; XSTATE_FLAGS[x as usize]=ecx;
        if xfeature_is_supervisor(x as i32) { continue; }
        XSTATE_OFFSETS[x as usize]=ebx; XFEATURE_UNCOMPACT_ORDER[i]=x; i+=1;
    }
    sort(XFEATURE_UNCOMPACT_ORDER.as_mut_ptr(), i, core::mem::size_of::<u32>(), Some(compare_xstate_offsets), core::ptr::null_mut());
}

unsafe fn xstate_calculate_size(xfeatures: u64, compacted: bool) -> u32 {
    let mut top = (63 - xfeatures.leading_zeros()) as usize;
    if top <= XFEATURE_SSE as usize { return core::mem::size_of::<xregs_state>() as u32; }
    let offset = if compacted { xfeature_get_offset(xfeatures, top as i32) } else {
        let mut i=0usize; while i < XFEATURE_MAX && XFEATURE_UNCOMPACT_ORDER[i] != u32::MAX { if xfeatures & (1u64<<XFEATURE_UNCOMPACT_ORDER[i]) != 0 { top=XFEATURE_UNCOMPACT_ORDER[i] as usize; } i+=1; }
        XSTATE_OFFSETS[top]
    };
    offset + XSTATE_SIZES[top]
}

pub unsafe fn xfeature_size(n: i32) -> i32 { let mut a=0; let mut b=0; let mut c=0; let mut d=0; cpuid_count(CPUID_LEAF_XSTATE,n,&mut a,&mut b,&mut c,&mut d); a as i32 }

unsafe fn __raw_xsave_addr(xsave: *mut xregs_state, n: i32) -> *mut core::ffi::c_void {
    if !xfeature_enabled(n) { return core::ptr::null_mut(); }
    let bv=(*xsave).header.xcomp_bv;
    if cpu_feature_enabled(X86_FEATURE_XCOMPACTED) && bv & (1u64<<n) == 0 { return core::ptr::null_mut(); }
    (xsave as *mut u8).add(xfeature_get_offset(bv,n) as usize) as *mut _
}

pub unsafe fn get_xsave_addr(xsave: *mut xregs_state, n: i32) -> *mut core::ffi::c_void {
    if !boot_cpu_has(X86_FEATURE_XSAVE) || !xfeature_enabled(n) || (*xsave).header.xfeatures & (1u64<<n)==0 { return core::ptr::null_mut(); }
    __raw_xsave_addr(xsave,n)
}
pub unsafe fn get_xsave_addr_user(xsave: *mut xregs_state, n: i32) -> *mut core::ffi::c_void { if !xfeature_enabled(n) { return core::ptr::null_mut(); } (xsave as *mut u8).add(XSTATE_OFFSETS[n as usize] as usize) as *mut _ }

unsafe fn copy_feature(from: bool, to: *mut membuf, x: *const u8, init: *const u8, size: usize) { membuf_write(to, if from{x}else{init}, size); }

pub unsafe fn xsaves(xstate: *mut xregs_state, mask: u64) { if validate_independent_components(mask) { let mut err=0; XSTATE_OP(XSAVES,xstate,mask as u32,(mask>>32) as u32,&mut err); } }
pub unsafe fn xrstors(xstate: *mut xregs_state, mask: u64) { if validate_independent_components(mask) { let mut err=0; XSTATE_OP(XRSTORS,xstate,mask as u32,(mask>>32) as u32,&mut err); } }

unsafe fn validate_independent_components(mask:u64)->bool { if !cpu_feature_enabled(X86_FEATURE_XSAVES){return false;} let xchk=!xfeatures_mask_independent(); mask!=0 && mask&xchk==0 }

pub unsafe fn xstate_get_guest_group_perm()->u64 { xstate_get_group_perm(true) }

pub unsafe fn fpu_xstate_prctl(option:i32,arg2:usize)->isize { let p=arg2 as *mut u64; match option { ARCH_GET_XCOMP_SUPP=>put_user(fpu_user_cfg.max_features|fpu_user_cfg.legacy_features,p), ARCH_GET_XCOMP_PERM=>put_user(xstate_get_host_group_perm()&XFEATURE_MASK_USER_SUPPORTED,p), ARCH_GET_XCOMP_GUEST_PERM=>put_user(xstate_get_guest_group_perm()&XFEATURE_MASK_USER_SUPPORTED,p), _=>-EINVAL as isize } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
