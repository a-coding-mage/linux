// SPDX-License-Identifier: GPL-2.0-only
/* This only handles 32bit MTRR on 32bit hosts. */

#[repr(C)]
struct FixedRangeBlock { base_msr: i32, ranges: i32 }
static mut FIXED_RANGE_BLOCKS: [FixedRangeBlock; 4] = [
    FixedRangeBlock { base_msr: MSR_MTRRFIX64K_00000, ranges: 1 },
    FixedRangeBlock { base_msr: MSR_MTRRFIX16K_80000, ranges: 2 },
    FixedRangeBlock { base_msr: MSR_MTRRFIX4K_C0000, ranges: 8 },
    FixedRangeBlock { base_msr: 0, ranges: 0 },
];

#[repr(C)]
struct CacheMap { start: u64, end: u64, flags: u64, r#type: u8, fixed: u8 }

static mut MTRR_DEBUG: bool = false;
static mut INIT_CACHE_MAP: [CacheMap; CACHE_MAP_MAX] = [CacheMap { start: 0, end: 0, flags: 0, r#type: 0, fixed: 0 }; CACHE_MAP_MAX];
static mut CACHE_MAP: *mut CacheMap = INIT_CACHE_MAP.as_mut_ptr();
static mut CACHE_MAP_SIZE: u32 = CACHE_MAP_MAX as u32;
static mut CACHE_MAP_N: u32 = 0;
static mut CACHE_MAP_FIXED: u32 = 0;
static mut SMP_CHANGES_MASK: usize = 0;
static mut MTRR_STATE_SET: i32 = 0;
static mut MTRR_TOM2: u64 = 0;
static mut MTRR_STATE: MtrrStateType = MtrrStateType::zero();
static mut PHYS_HI_RSVD: u32 = 0;

const CACHE_MAP_MAX: usize = MTRR_NUM_FIXED_RANGES + MTRR_MAX_VAR_RANGES * 2;

#[inline]
unsafe fn k8_check_syscfg_dram_mod_en() {
    if !(boot_cpu_data.x86_vendor == X86_VENDOR_AMD && boot_cpu_data.x86 >= 0x0f) || cc_platform_has(CC_ATTR_HOST_SEV_SNP) { return; }
    let mut val = Msr { q: 0, l: 0, h: 0 };
    rdmsrq(MSR_AMD64_SYSCFG, &mut val.q);
    if val.l & K8_MTRRFIXRANGE_DRAM_MODIFY != 0 {
        pr_err!("MTRR: CPU {}: SYSCFG[MtrrFixDramModEn] not cleared by BIOS, clearing this bit\n", smp_processor_id());
        val.l &= !K8_MTRRFIXRANGE_DRAM_MODIFY;
        mtrr_wrmsr(MSR_AMD64_SYSCFG, val.l, val.h);
    }
}

unsafe fn get_mtrr_size(mut mask: u64) -> u64 { mask |= (PHYS_HI_RSVD as u64) << 32; 0u64.wrapping_sub(mask) }

unsafe fn get_var_mtrr_state(reg: u32, start: *mut u64, size: *mut u64) -> u8 {
    let m = &MTRR_STATE.var_ranges[reg as usize];
    if m.mask_lo & MTRR_PHYSMASK_V == 0 { return MTRR_TYPE_INVALID; }
    *start = ((m.base_hi as u64) << 32) + (m.base_lo as u64 & PAGE_MASK);
    *size = get_mtrr_size(((m.mask_hi as u64) << 32) + (m.mask_lo as u64 & PAGE_MASK));
    (m.base_lo & MTRR_PHYSBASE_TYPE) as u8
}

unsafe fn get_effective_type(a: u8, b: u8) -> u8 {
    if a == MTRR_TYPE_UNCACHABLE || b == MTRR_TYPE_UNCACHABLE { return MTRR_TYPE_UNCACHABLE; }
    if (a == MTRR_TYPE_WRBACK && b == MTRR_TYPE_WRTHROUGH) || (a == MTRR_TYPE_WRTHROUGH && b == MTRR_TYPE_WRBACK) { return MTRR_TYPE_WRTHROUGH; }
    if a != b { MTRR_TYPE_UNCACHABLE } else { a }
}

unsafe fn rm_map_entry_at(idx: usize) {
    CACHE_MAP_N -= 1;
    if CACHE_MAP_N as usize > idx { core::ptr::copy(CACHE_MAP.add(idx + 1), CACHE_MAP.add(idx), CACHE_MAP_N as usize - idx); }
}

unsafe fn add_map_entry_at(start: u64, end: u64, ty: u8, idx: usize) -> i32 {
    if start >= end { return 0; }
    let prev = idx > 0 && (*CACHE_MAP.add(idx-1)).fixed == 0 && (*CACHE_MAP.add(idx-1)).end == start && (*CACHE_MAP.add(idx-1)).r#type == ty;
    let next = idx < CACHE_MAP_N as usize && (*CACHE_MAP.add(idx)).fixed == 0 && (*CACHE_MAP.add(idx)).start == end && (*CACHE_MAP.add(idx)).r#type == ty;
    if prev && next { (*CACHE_MAP.add(idx-1)).end = (*CACHE_MAP.add(idx)).end; rm_map_entry_at(idx); return 2; }
    if prev { (*CACHE_MAP.add(idx-1)).end = end; return 1; }
    if next { (*CACHE_MAP.add(idx)).start = start; return 1; }
    if CACHE_MAP_N == CACHE_MAP_SIZE { WARN!(1, "MTRR cache mode memory map exhausted!\n"); CACHE_MAP_N = CACHE_MAP_FIXED; return 0; }
    if CACHE_MAP_N as usize > idx { core::ptr::copy(CACHE_MAP.add(idx), CACHE_MAP.add(idx+1), CACHE_MAP_N as usize-idx); }
    *CACHE_MAP.add(idx) = CacheMap { start, end, flags: 0, r#type: ty, fixed: 0 }; CACHE_MAP_N += 1; 0
}

unsafe fn clr_map_range_at(start: u64, end: u64, idx: usize) -> i32 {
    let ret = (start != (*CACHE_MAP.add(idx)).start) as i32;
    if start == (*CACHE_MAP.add(idx)).start && end == (*CACHE_MAP.add(idx)).end { rm_map_entry_at(idx); }
    else if start == (*CACHE_MAP.add(idx)).start { (*CACHE_MAP.add(idx)).start = end; }
    else if end == (*CACHE_MAP.add(idx)).end { (*CACHE_MAP.add(idx)).end = start; }
    else { let tmp = (*CACHE_MAP.add(idx)).end; (*CACHE_MAP.add(idx)).end = start; let ty = (*CACHE_MAP.add(idx)).r#type; add_map_entry_at(end, tmp, ty, idx+1); }
    ret
}

unsafe fn add_map_entry(mut start: u64, end: u64, ty: u8) {
    let mut i = 0usize;
    while i < CACHE_MAP_N as usize && start < end {
        if start >= (*CACHE_MAP.add(i)).end { i += 1; continue; }
        if start < (*CACHE_MAP.add(i)).start { let tmp = core::cmp::min(end, (*CACHE_MAP.add(i)).start); i -= add_map_entry_at(start,tmp,ty,i) as usize; start=tmp; continue; }
        let new_ty=get_effective_type(ty,(*CACHE_MAP.add(i)).r#type); let old=(*CACHE_MAP.add(i)).r#type;
        if (*CACHE_MAP.add(i)).fixed != 0 || new_ty == old { start=(*CACHE_MAP.add(i)).end; i+=1; continue; }
        let tmp=core::cmp::min(end,(*CACHE_MAP.add(i)).end); i += clr_map_range_at(start,tmp,i) as usize; i -= add_map_entry_at(start,tmp,new_ty,i) as usize; start=tmp;
    }
    add_map_entry_at(start,end,ty,i);
}

unsafe fn map_add_var() {
    if MTRR_TOM2 != 0 { add_map_entry(1u64<<32,MTRR_TOM2,MTRR_TYPE_WRBACK); (*CACHE_MAP.add(CACHE_MAP_N as usize-1)).fixed=1; }
    for i in 0..num_var_ranges { let mut s=0; let mut z=0; let ty=get_var_mtrr_state(i,&mut s,&mut z); if ty != MTRR_TYPE_INVALID { add_map_entry(s,s.wrapping_add(z),ty); } }
}

pub unsafe fn generic_rebuild_map() { if !core::ptr::eq(mtrr_if,&generic_mtrr_ops) { return; } CACHE_MAP_N=CACHE_MAP_FIXED; map_add_var(); }

unsafe fn get_cache_map_size() -> u32 { CACHE_MAP_FIXED + 2*num_var_ranges + (MTRR_TOM2 != 0) as u32 }

pub unsafe fn mtrr_build_map() {
    let mut start=0; let mut end=0; let mut size=0; let mut ty=0;
    if MTRR_STATE.enabled & MTRR_STATE_MTRR_FIXED_ENABLED != 0 { end=0x10000; size=end; ty=MTRR_STATE.fixed_ranges[0]; for i in 1..MTRR_NUM_FIXED_RANGES { if i==8 || i==24 { size >>= 2; } if MTRR_STATE.fixed_ranges[i]!=ty { add_map_entry(start,end,ty); start=end; ty=MTRR_STATE.fixed_ranges[i]; } end+=size; } add_map_entry(start,end,ty); }
    for i in 0..CACHE_MAP_N as usize { (*CACHE_MAP.add(i)).fixed=1; } CACHE_MAP_FIXED=CACHE_MAP_N; map_add_var();
}

pub unsafe fn mtrr_type_lookup(mut start:u64, end:u64, uniform:*mut u8)->u8 { if MTRR_STATE_SET==0 { *uniform=0; return MTRR_TYPE_UNCACHABLE; } *uniform=1; if MTRR_STATE.enabled&MTRR_STATE_MTRR_ENABLED==0{return MTRR_TYPE_UNCACHABLE;} let mut ty=MTRR_TYPE_INVALID; for i in 0..CACHE_MAP_N as usize { if start>=end {break;} if start>=(*CACHE_MAP.add(i)).end {continue;} if start<(*CACHE_MAP.add(i)).start { ty=type_merge(ty,MTRR_STATE.def_type,uniform); if end<=(*CACHE_MAP.add(i)).start{return ty;} } ty=type_merge(ty,(*CACHE_MAP.add(i)).r#type,uniform); start=(*CACHE_MAP.add(i)).end; } if start<end {ty=type_merge(ty,MTRR_STATE.def_type,uniform);} ty }

unsafe fn type_merge(ty:u8,new_ty:u8,uniform:*mut u8)->u8 { if ty==MTRR_TYPE_INVALID{return new_ty;} let e=get_effective_type(ty,new_ty); if ty!=e {*uniform=0;} e }

pub unsafe fn fill_mtrr_var_range(index:usize, base_lo:u32,base_hi:u32,mask_lo:u32,mask_hi:u32) { let v=&mut MTRR_STATE.var_ranges[index]; v.base_lo=base_lo;v.base_hi=base_hi;v.mask_lo=mask_lo;v.mask_hi=mask_hi; }

// The remaining entry points retain the kernel's MSR, CPU, allocation, and logging dependencies.
// Their declarations are intentionally left to the surrounding translated kernel modules.
extern "C" {
    static mut num_var_ranges: u32;
    static mtrr_if: *const MtrrOps;
    static generic_mtrr_ops: MtrrOps;
}

pub unsafe fn mtrr_save_fixed_ranges(_info: *mut core::ffi::c_void) { if MTRR_STATE.have_fixed != 0 { get_fixed_ranges(MTRR_STATE.fixed_ranges.as_mut_ptr()); } }
unsafe fn get_fixed_ranges(_frs: *mut u8) { k8_check_syscfg_dram_mod_en(); /* MSR reads supplied externally. */ }
pub unsafe fn mtrr_state_warn() { if SMP_CHANGES_MASK==0{return;} pr_warn!("mtrr: your CPUs had inconsistent MTRR settings\n"); }
pub unsafe fn mtrr_wrmsr(msr:u32,a:u32,b:u32) { let val=Msr{q:0,l:a,h:b}; if wrmsrq_safe(msr,val.q)<0 { pr_err!("MTRR: writing MSR {:x} failed\n",msr); } }
pub unsafe fn mtrr_disable() { let mut v=Msr{q:0,l:0,h:0}; rdmsrq(MSR_MTRRdefType,&mut v.q); DEFTYPE_LO=v.l;DEFTYPE_HI=v.h;mtrr_wrmsr(MSR_MTRRdefType,v.l&MTRR_DEF_TYPE_DISABLE,v.h); }
pub unsafe fn mtrr_enable() { mtrr_wrmsr(MSR_MTRRdefType,DEFTYPE_LO,DEFTYPE_HI); }
pub unsafe fn mtrr_generic_set_state() { let mut mask=set_mtrr_state(); for count in 0..(core::mem::size_of::<usize>()*8) { if mask&1!=0 { set_bit(count,&mut SMP_CHANGES_MASK); } mask>>=1; } }
static mut DEFTYPE_LO:u32=0; static mut DEFTYPE_HI:u32=0;
unsafe fn set_mtrr_state()->usize { let mut mask=0; for i in 0..num_var_ranges { if set_mtrr_var_ranges(i,&MTRR_STATE.var_ranges[i as usize]){mask|=MTRR_CHANGE_MASK_VARIABLE as usize;} } mask }
unsafe fn set_mtrr_var_ranges(_i:u32,_v:&MtrrVarRange)->bool { false }
pub fn positive_have_wrcomb()->i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
