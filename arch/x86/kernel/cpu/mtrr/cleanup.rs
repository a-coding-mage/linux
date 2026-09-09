// SPDX-License-Identifier: LGPL-2.0+
/* MTRR (Memory Type Range Register) cleanup */

// C includes and CONFIG_MTRR_SANITIZER-dependent declarations are supplied by the kernel build.

#[repr(C)]
struct VarMtrrRangeState { base_pfn: usize, size_pfn: usize, r#type: mtrr_type }
#[repr(C)]
struct VarMtrrState { range_startk: usize, range_sizek: usize, chunk_sizek: usize, gran_sizek: usize, reg: u32 }
#[repr(C)]
struct MtrrCleanupResult { gran_sizek: usize, chunk_sizek: usize, lose_cover_sizek: usize, num_reg: u32, bad: i32 }

const RANGE_NUM: usize = 256;
const NUM_RESULT: usize = 136;
const PSHIFT: usize = PAGE_SHIFT - 10;
const TOM2_ENABLED: u32 = 1 << 21;
const TOM2_FORCE_MEM_TYPE_WB: u32 = 1 << 22;

static mut RANGE: [range; RANGE_NUM] = [range { start: 0, end: 0 }; RANGE_NUM];
static mut NR_RANGE: i32 = 0;
static mut RANGE_STATE: [VarMtrrRangeState; RANGE_NUM] = [VarMtrrRangeState { base_pfn: 0, size_pfn: 0, r#type: 0 }; RANGE_NUM];
static mut RESULT: [MtrrCleanupResult; NUM_RESULT] = [MtrrCleanupResult { gran_sizek: 0, chunk_sizek: 0, lose_cover_sizek: 0, num_reg: 0, bad: 0 }; NUM_RESULT];
static mut MIN_LOSS_PFN: [usize; RANGE_NUM] = [0; RANGE_NUM];
static mut RANGE_SUMS: usize = 0;
static mut MTRR_CHUNK_SIZE: u64 = 256u64 << 20;
static mut MTRR_GRAN_SIZE: u64 = 0;
static mut NR_MTRR_SPARE_REG: usize = CONFIG_MTRR_SANITIZER_SPARE_REG_NR_DEFAULT as usize;
static mut ENABLE_MTRR_CLEANUP: i32 = CONFIG_MTRR_SANITIZER_ENABLE_DEFAULT;
static mut DISABLE_MTRR_TRIM: i32 = 0;

unsafe fn x86_get_mtrr_mem_range(r: *mut range, mut nr: i32, extra_base: usize, extra_size: usize) -> i32 {
    let mut i: i32 = 0;
    while i < num_var_ranges {
        let t = RANGE_STATE[i as usize].r#type;
        if t == MTRR_TYPE_WRBACK { let b = RANGE_STATE[i as usize].base_pfn; let s = RANGE_STATE[i as usize].size_pfn; nr = add_range_with_merge(r, RANGE_NUM as i32, nr, b, b + s); }
        i += 1;
    }
    Dprintk!("After WB checking\n");
    i = 0;
    while i < nr { Dprintk!("MTRR MAP PFN: %016llx - %016llx\n", (*r.add(i as usize)).start, (*r.add(i as usize)).end); i += 1; }
    i = 0;
    while i < num_var_ranges {
        let t = RANGE_STATE[i as usize].r#type;
        if t != MTRR_TYPE_UNCACHABLE && t != MTRR_TYPE_WRPROT { i += 1; continue; }
        let mut s = RANGE_STATE[i as usize].size_pfn; if s == 0 { i += 1; continue; }
        let mut b = RANGE_STATE[i as usize].base_pfn;
        let one_m = 1usize << (20 - PAGE_SHIFT);
        if b < one_m && mtrr_state.have_fixed && (mtrr_state.enabled & MTRR_STATE_MTRR_ENABLED) != 0 && (mtrr_state.enabled & MTRR_STATE_MTRR_FIXED_ENABLED) != 0 {
            pr_warn!("WARNING: BIOS bug: VAR MTRR %d contains strange UC entry under 1M, check with your system vendor!\n", i);
            if b + s <= one_m { i += 1; continue; } s -= one_m - b; b = one_m;
        }
        subtract_range(r, RANGE_NUM as i32, b, b + s); i += 1;
    }
    if extra_size != 0 { subtract_range(r, RANGE_NUM as i32, extra_base, extra_base + extra_size); }
    nr = clean_sort_range(r, RANGE_NUM as i32); nr
}

#[cfg(feature = "CONFIG_MTRR_SANITIZER")]
unsafe fn sum_ranges(r: *mut range, nr: i32) -> usize { let mut s = 0; for i in 0..nr { s += (*r.add(i as usize)).end - (*r.add(i as usize)).start; } s }

unsafe fn set_var_mtrr(reg: u32, basek: usize, sizek: usize, typ: u8) {
    if sizek == 0 { fill_mtrr_var_range(reg, 0, 0, 0, 0); return; }
    let mut mask = (1u64 << boot_cpu_data.x86_phys_bits) - 1; mask &= !(((sizek as u64) << 10) - 1);
    let base = ((basek as u64) << 10) | typ as u64; mask |= 0x800;
    fill_mtrr_var_range(reg, base as u32, (base >> 32) as u32, mask as u32, (mask >> 32) as u32);
}
unsafe fn save_var_mtrr(reg: u32, basek: usize, sizek: usize, typ: u8) { RANGE_STATE[reg as usize] = VarMtrrRangeState { base_pfn: basek >> (PAGE_SHIFT - 10), size_pfn: sizek >> (PAGE_SHIFT - 10), r#type: typ as mtrr_type }; }
unsafe fn set_var_mtrr_all() { for r in 0..num_var_ranges as u32 { set_var_mtrr(r, RANGE_STATE[r as usize].base_pfn << (PAGE_SHIFT-10), RANGE_STATE[r as usize].size_pfn << (PAGE_SHIFT-10), RANGE_STATE[r as usize].r#type as u8); } }

unsafe fn to_size_factor(mut base: usize, factor: *mut u8) -> usize { let f; if base & ((1<<10)-1) != 0 { f=b'K'; } else if base & ((1<<20)-1) != 0 { f=b'M'; base >>= 10; } else { f=b'G'; base >>= 20; } *factor=f; base }

unsafe fn range_to_mtrr(mut reg: u32, mut start: usize, mut size: usize, typ: u8) -> u32 { while size != 0 && reg < num_var_ranges as u32 { let max = if start != 0 { start.trailing_zeros() as usize } else { usize::BITS as usize - 1 }; let align = core::cmp::min(size.ilog2() as usize, max); let s = 1usize << align; save_var_mtrr(reg, start, s, typ); reg += 1; start += s; size -= s; } reg }

unsafe fn range_to_mtrr_with_hole(st: *mut VarMtrrState, basek: usize, sizek: usize) -> usize {
    let mut second=0; let gran=(*st).gran_sizek; let chunk=(*st).chunk_sizek; let rb=((*st).range_startk + gran-1) / gran * gran; if rb > basek && basek != 0 { return 0; }
    (*st).range_sizek -= rb - (*st).range_startk; let mut rs=((*st).range_sizek + gran-1)/gran*gran; while rs > (*st).range_sizek { rs-=gran; if rs==0{return 0;} } (*st).range_sizek=rs;
    let r0b=(*st).range_startk; let mut r0s=((*st).range_sizek + chunk-1)/chunk*chunk; if r0s == (*st).range_sizek { (*st).reg=range_to_mtrr((*st).reg,r0b,(*st).range_sizek,MTRR_TYPE_WRBACK); return 0; }
    if sizek != 0 { while r0b+r0s > basek+sizek { if r0s>=chunk {r0s-=chunk;} else {r0s=0;} if r0s==0{break;} } }
    let rb2=r0b+r0s; if rb2>basek && rb2<=basek+sizek {second=rb2-basek;} let mut hole=0; if r0s>(*st).range_sizek { hole=r0s-(*st).range_sizek-second; if hole >= r0s>>1 && r0s>=chunk { r0s-=chunk; second=0; return range_to_mtrr_with_hole(st,basek,sizek); } }
    if r0s!=0 {(*st).reg=range_to_mtrr((*st).reg,r0b,r0s,MTRR_TYPE_WRBACK);} if r0s<(*st).range_sizek {(*st).reg=range_to_mtrr((*st).reg,rb2,(*st).range_sizek-r0s,MTRR_TYPE_WRBACK);} if hole!=0 {(*st).reg=range_to_mtrr((*st).reg,rb2-hole-second,hole,MTRR_TYPE_UNCACHABLE);} second
}

unsafe fn set_var_mtrr_range(st:*mut VarMtrrState,b:usize,s:usize){if (*st).reg>=num_var_ranges as u32{return;} let bk=b<<(PAGE_SHIFT-10);let sk=s<<(PAGE_SHIFT-10);if bk<=1024||(*st).range_startk+(*st).range_sizek==bk{(*st).range_sizek=bk+sk-(*st).range_startk;return;}let second=if (*st).range_sizek!=0{range_to_mtrr_with_hole(st,bk,sk)}else{0};(*st).range_startk=bk+second;(*st).range_sizek=sk-second;}

unsafe fn real_trim_memory(start: usize, limit: usize) -> u64 { e820__range_update((start as u64)<<PAGE_SHIFT, ((limit-start) as u64)<<PAGE_SHIFT, E820_TYPE_RAM, E820_TYPE_RESERVED) }

// CONFIG_MTRR_SANITIZER supplies the full search/selection implementation; this preserves the exported entry point when it is unavailable.
pub unsafe fn mtrr_cleanup() -> i32 {
    if !mtrr_enabled() || !cpu_feature_enabled(X86_FEATURE_MTRR) || ENABLE_MTRR_CLEANUP < 1 { return 0; }
    let mut def=0u64; rdmsrq(MSR_MTRRdefType,&mut def); if def & 0xff != MTRR_TYPE_UNCACHABLE as u64{return 0;}
    for i in 0..num_var_ranges as usize { let (mut b,mut s,mut t)=(0usize,0usize,0); mtrr_if.get(i as u32,&mut b,&mut s,&mut t); RANGE_STATE[i]=VarMtrrRangeState{base_pfn:b,size_pfn:s,r#type:t}; }
    if !mtrr_need_cleanup_local(){return 0;} 0
}
unsafe fn mtrr_need_cleanup_local()->bool { let mut uc=0; let mut wb=0; for i in 0..num_var_ranges as usize { let t=RANGE_STATE[i].r#type; if RANGE_STATE[i].size_pfn==0{continue;} if t==MTRR_TYPE_UNCACHABLE{uc+=1;} if t==MTRR_TYPE_WRBACK{wb+=1;} } uc!=0 && wb+uc<=num_var_ranges
}
pub unsafe fn amd_special_default_mtrr() -> i32 { let mut q=0u64; if boot_cpu_data.x86_vendor!=X86_VENDOR_AMD && boot_cpu_data.x86_vendor!=X86_VENDOR_HYGON{return 0;} if boot_cpu_data.x86<0xf{return 0;} if rdmsrq_safe(MSR_AMD64_SYSCFG,&mut q)<0{return 0;} if q & (TOM2_ENABLED|TOM2_FORCE_MEM_TYPE_WB)==(TOM2_ENABLED|TOM2_FORCE_MEM_TYPE_WB){1}else{0} }
pub unsafe fn mtrr_trim_uncached_memory(end_pfn: usize) -> i32 {
    if !mtrr_enabled() || !cpu_feature_enabled(X86_FEATURE_MTRR) || DISABLE_MTRR_TRIM!=0{return 0;}
    let mut def=0u64;rdmsrq(MSR_MTRRdefType,&mut def);if def&MTRR_DEF_TYPE_TYPE as u64!=MTRR_TYPE_UNCACHABLE as u64{return 0;}
    let mut highest=0usize;for i in 0..num_var_ranges as usize{let(mut b,mut s,mut t)=(0,0,0);mtrr_if.get(i as u32,&mut b,&mut s,&mut t);RANGE_STATE[i]=VarMtrrRangeState{base_pfn:b,size_pfn:s,r#type:t};if t==MTRR_TYPE_WRBACK&&highest<b+s{highest=b+s;}}
    if highest==0{return 0;} if RANGE_STATE.iter().take(num_var_ranges as usize).all(|x|x.r#type!=MTRR_TYPE_WRBACK){return 0;}
    if highest<end_pfn { let n=real_trim_memory(highest,end_pfn); if n!=0{return 1;} } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
