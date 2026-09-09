// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common Performance counter support functions for PowerISA v2.07 processors.
 *
 * Copyright 2009 Paul Mackerras, IBM Corporation.
 * Copyright 2013 Michael Ellerman, IBM Corporation.
 * Copyright 2016 Madhavan Srinivasan, IBM Corporation.
 */

// Symbols, constants, macros, and types referenced below are supplied by the
// surrounding PowerPC performance-monitoring implementation.

pub const ISA207_PMU_FORMAT_ATTRS: [&str; 12] = [
    "event=config:0-49", "pmcxsel=config:0-7", "mark=config:8",
    "combine=config:11", "unit=config:12-15", "pmc=config:16-19",
    "cache_sel=config:20-23", "sample_mode=config:24-28",
    "thresh_sel=config:29-31", "thresh_stop=config:32-35",
    "thresh_start=config:36-39", "thresh_cmp=config:40-49",
];

#[inline]
unsafe fn event_is_fab_match(mut event: u64) -> bool {
    event &= 0xff0fe;
    event == 0x30056 || event == 0x4f052
}

unsafe fn is_event_valid(event: u64) -> bool {
    let mut valid_mask = EVENT_VALID_MASK;
    if cpu_has_feature(CPU_FTR_ARCH_31) { valid_mask = p10_EVENT_VALID_MASK; }
    else if cpu_has_feature(CPU_FTR_ARCH_300) { valid_mask = p9_EVENT_VALID_MASK; }
    (event & !valid_mask) == 0
}

#[inline]
unsafe fn is_event_marked(event: u64) -> bool { (event & EVENT_IS_MARKED) != 0 }

unsafe fn sdar_mod_val(event: u64) -> u64 {
    if cpu_has_feature(CPU_FTR_ARCH_31) { p10_SDAR_MODE(event) } else { p9_SDAR_MODE(event) }
}

unsafe fn mmcra_sdar_mode(event: u64, mmcra: *mut u64) {
    if cpu_has_feature(CPU_FTR_ARCH_300) {
        if is_event_marked(event) || (*mmcra & MMCRA_SAMPLE_ENABLE) != 0 {
            *mmcra &= MMCRA_SDAR_MODE_NO_UPDATES;
        } else if sdar_mod_val(event) != 0 {
            *mmcra |= sdar_mod_val(event) << MMCRA_SDAR_MODE_SHIFT;
        } else { *mmcra |= MMCRA_SDAR_MODE_DCACHE; }
    } else { *mmcra |= MMCRA_SDAR_MODE_TLB; }
}

unsafe fn p10_thresh_cmp_val(mut value: u64) -> i32 {
    let mut exp = 0;
    let mut result = value as i32;
    if value == 0 { return 0; }
    if cpu_has_feature(CPU_FTR_ARCH_31) {
        if value > 261120 { value = 261120; }
        while (64 - value.leading_zeros()) > 8 { exp += 1; value >>= 2; }
        if (value & 0xc0) == 0 && exp != 0 { result = -1; }
        else { result = ((exp << 8) | value as i32); }
    }
    result
}

unsafe fn thresh_cmp_val(mut value: u64) -> u64 {
    if cpu_has_feature(CPU_FTR_ARCH_31) { value = p10_thresh_cmp_val(value) as u64; }
    if cpu_has_feature(CPU_FTR_ARCH_300) { value << p9_MMCRA_THR_CMP_SHIFT } else { value << MMCRA_THR_CMP_SHIFT }
}

unsafe fn combine_from_event(event: u64) -> u64 { if cpu_has_feature(CPU_FTR_ARCH_300) { p9_EVENT_COMBINE(event) } else { EVENT_COMBINE(event) } }
unsafe fn combine_shift(pmc: u64) -> u64 { if cpu_has_feature(CPU_FTR_ARCH_300) { p9_MMCR1_COMBINE_SHIFT(pmc) } else { MMCR1_COMBINE_SHIFT(pmc) } }
#[inline] unsafe fn event_is_threshold(event: u64) -> bool { ((event >> EVENT_THR_SEL_SHIFT) & EVENT_THR_SEL_MASK) != 0 }

unsafe fn is_thresh_cmp_valid(event: u64) -> bool {
    if cpu_has_feature(CPU_FTR_ARCH_31) { return p10_thresh_cmp_val(event) >= 0; }
    let cmp = ((event >> EVENT_THR_CMP_SHIFT) & EVENT_THR_CMP_MASK) as u32;
    let exp = cmp >> 7;
    !(exp != 0 && (cmp & 0x60) == 0)
}

unsafe fn dc_ic_rld_quad_l1_sel(event: u64) -> u32 { ((event >> EVENT_CACHE_SEL_SHIFT) & MMCR1_DC_IC_QUAL_MASK) as u32 }

unsafe fn isa207_find_source(idx: u64, sub_idx: u32) -> u64 {
    let mut ret = PERF_MEM_NA;
    match idx {
        0 => {},
        1 => ret = PH!(LVL, L1) | LEVEL!(L1) | P!(SNOOP, HIT),
        2 => ret = PH!(LVL, L2) | LEVEL!(L2) | P!(SNOOP, HIT),
        3 => ret = PH!(LVL, L3) | LEVEL!(L3) | P!(SNOOP, HIT),
        4 => {
            if cpu_has_feature(CPU_FTR_ARCH_31) {
                ret = P!(SNOOP, HIT);
                if sub_idx == 1 { ret |= PH!(LVL, LOC_RAM) | LEVEL!(RAM); }
                else if sub_idx == 2 || sub_idx == 3 { ret |= P!(LVL, HIT) | LEVEL!(PMEM); }
                else if sub_idx == 4 { ret |= PH!(LVL, REM_RAM1) | REM | LEVEL!(RAM) | P!(HOPS, 2); }
                else if sub_idx == 5 || sub_idx == 7 { ret |= P!(LVL, HIT) | LEVEL!(PMEM) | REM; }
                else if sub_idx == 6 { ret |= PH!(LVL, REM_RAM2) | REM | LEVEL!(RAM) | P!(HOPS, 3); }
            } else {
                if sub_idx <= 1 { ret = PH!(LVL, LOC_RAM); }
                else if sub_idx <= 2 { ret = PH!(LVL, REM_RAM1); } else { ret = PH!(LVL, REM_RAM2); }
                ret |= P!(SNOOP, HIT);
            }
        },
        5 => {
            if cpu_has_feature(CPU_FTR_ARCH_31) {
                ret = REM | P!(HOPS, 0);
                if sub_idx == 0 || sub_idx == 4 { ret |= PH!(LVL, L2) | LEVEL!(L2) | P!(SNOOP, HIT); }
                else if sub_idx == 1 || sub_idx == 5 { ret |= PH!(LVL, L2) | LEVEL!(L2) | P!(SNOOP, HITM); }
                else if sub_idx == 2 || sub_idx == 6 { ret |= PH!(LVL, L3) | LEVEL!(L3) | P!(SNOOP, HIT); }
                else if sub_idx == 3 || sub_idx == 7 { ret |= PH!(LVL, L3) | LEVEL!(L3) | P!(SNOOP, HITM); }
            } else {
                if sub_idx == 0 { ret = PH!(LVL,L2)|LEVEL!(L2)|REM|P!(SNOOP,HIT)|P!(HOPS,0); }
                else if sub_idx == 1 { ret = PH!(LVL,L2)|LEVEL!(L2)|REM|P!(SNOOP,HITM)|P!(HOPS,0); }
                else if sub_idx == 2 || sub_idx == 4 { ret = PH!(LVL,L3)|LEVEL!(L3)|REM|P!(SNOOP,HIT)|P!(HOPS,0); }
                else if sub_idx == 3 || sub_idx == 5 { ret = PH!(LVL,L3)|LEVEL!(L3)|REM|P!(SNOOP,HITM)|P!(HOPS,0); }
            }
        },
        6 => {
            if cpu_has_feature(CPU_FTR_ARCH_31) {
                if sub_idx == 0 { ret=PH!(LVL,REM_CCE1)|LEVEL!(ANY_CACHE)|REM|P!(SNOOP,HIT)|P!(HOPS,2); }
                else if sub_idx == 1 { ret=PH!(LVL,REM_CCE1)|LEVEL!(ANY_CACHE)|REM|P!(SNOOP,HITM)|P!(HOPS,2); }
                else if sub_idx == 2 { ret=PH!(LVL,REM_CCE2)|LEVEL!(ANY_CACHE)|REM|P!(SNOOP,HIT)|P!(HOPS,3); }
                else if sub_idx == 3 { ret=PH!(LVL,REM_CCE2)|LEVEL!(ANY_CACHE)|REM|P!(SNOOP,HITM)|P!(HOPS,3); }
            } else { ret=PH!(LVL,REM_CCE2); if sub_idx==0||sub_idx==2 { ret|=P!(SNOOP,HIT); } else if sub_idx==1||sub_idx==3 { ret|=P!(SNOOP,HITM); } }
        },
        7 => ret = PM!(LVL, L1),
        _ => {}
    }
    ret
}

// The remaining exported routines retain the C ABI and use the external
// kernel structures/macros directly.
pub unsafe fn isa207_get_mem_data_src(dsrc: *mut perf_mem_data_src, flags: u32, regs: *mut pt_regs) {
    if flags & PPMU_HAS_SIER == 0 { (*dsrc).val = 0; return; }
    let sier = (*regs).dar;
    let val = (sier & ISA207_SIER_TYPE_MASK) >> ISA207_SIER_TYPE_SHIFT;
    if val != 1 && val != 2 && !(val == 7 && cpu_has_feature(CPU_FTR_ARCH_31)) { (*dsrc).val=0; return; }
    let idx=(sier&ISA207_SIER_LDST_MASK)>>ISA207_SIER_LDST_SHIFT;
    let sub_idx=((sier&ISA207_SIER_DATA_SRC_MASK)>>ISA207_SIER_DATA_SRC_SHIFT) as u32;
    (*dsrc).val=isa207_find_source(idx,sub_idx);
    if val==7 { let op_type=(((*regs).dsisr>>MMCRA_SAMP_ELIG_SHIFT)&MMCRA_SAMP_ELIG_MASK) as u32; (*dsrc).val |= match op_type {5=>P!(OP,LOAD),7=>P!(OP,STORE),_=>P!(OP,NA)}; }
    else { (*dsrc).val |= if val==1 {P!(OP,LOAD)} else {P!(OP,STORE)}; }
}

pub unsafe fn isa207_get_mem_weight(weight: *mut u64, typ: u64) {
    let mmcra=mfspr(SPRN_MMCRA); let exp=MMCRA_THR_CTR_EXP(mmcra); let mut mantissa=MMCRA_THR_CTR_MANT(mmcra); let sier=mfspr(SPRN_SIER); let val=(sier&ISA207_SIER_TYPE_MASK)>>ISA207_SIER_TYPE_SHIFT;
    if cpu_has_feature(CPU_FTR_ARCH_31) { mantissa=P10_MMCRA_THR_CTR_MANT(mmcra); }
    let weight_lat=if val==0 || (val==7&&!cpu_has_feature(CPU_FTR_ARCH_31)){0}else{mantissa<<(2*exp)};
    let fields=weight as *mut perf_sample_weight;
    if typ & PERF_SAMPLE_WEIGHT != 0 { (*fields).full=weight_lat; } else { (*fields).var1_dw=weight_lat as u32; if cpu_has_feature(CPU_FTR_ARCH_31) { let s=mfspr(SPRN_SIER2); (*fields).var2_w=P10_SIER2_FINISH_CYC(s); (*fields).var3_w=P10_SIER2_DISPATCH_CYC(s); } }
}

pub unsafe fn isa207_get_constraint(event:u64, maskp:*mut u64, valp:*mut u64, event_config1:u64)->i32 {
    let mut mask=0; let mut value=0;
    if !is_event_valid(event){return -1;}
    let pmc=(event>>EVENT_PMC_SHIFT)&EVENT_PMC_MASK; let unit=(event>>EVENT_UNIT_SHIFT)&EVENT_UNIT_MASK;
    let cache=(event>>EVENT_CACHE_SEL_SHIFT)&if cpu_has_feature(CPU_FTR_ARCH_31){p10_EVENT_CACHE_SEL_MASK}else{EVENT_CACHE_SEL_MASK}; let ebb=(event>>EVENT_EBB_SHIFT)&EVENT_EBB_MASK;
    if pmc!=0 { if pmc>6{return -1;} let base=event&!EVENT_LINUX_MASK; if pmc>=5&&base!=0x500fa&&base!=0x600f4{return -1;} mask|=CNST_PMC_MASK(pmc); value|=CNST_PMC_VAL(pmc); if pmc>=5{ return isa207_get_constraint_ebb(event,maskp,valp,mask,value,ebb); } }
    if pmc<=4 {mask|=CNST_NC_MASK;value|=CNST_NC_VAL;}
    if unit>=6&&unit<=9 { if cpu_has_feature(CPU_FTR_ARCH_31) {if unit==6{mask|=CNST_L2L3_GROUP_MASK;value|=CNST_L2L3_GROUP_VAL(event>>p10_L2L3_EVENT_SHIFT);}} else if cpu_has_feature(CPU_FTR_ARCH_300){mask|=CNST_CACHE_GROUP_MASK;value|=CNST_CACHE_GROUP_VAL(event&0xff);mask|=CNST_CACHE_PMC4_MASK;if pmc==4{value|=CNST_CACHE_PMC4_VAL;}} else if cache&7!=0{return -1;} } else if cpu_has_feature(CPU_FTR_ARCH_300)||(event&EVENT_IS_L1)!=0 {mask|=CNST_L1_QUAL_MASK;value|=CNST_L1_QUAL_VAL(cache);}
    if cpu_has_feature(CPU_FTR_ARCH_31){mask|=CNST_RADIX_SCOPE_GROUP_MASK;value|=CNST_RADIX_SCOPE_GROUP_VAL(event>>p10_EVENT_RADIX_SCOPE_QUAL_SHIFT);}
    if is_event_marked(event){mask|=CNST_SAMPLE_MASK;value|=CNST_SAMPLE_VAL(event>>EVENT_SAMPLE_SHIFT);}
    if cpu_has_feature(CPU_FTR_ARCH_31){if event_is_threshold(event)&&is_thresh_cmp_valid(event_config1){mask|=CNST_THRESH_CTL_SEL_MASK;value|=CNST_THRESH_CTL_SEL_VAL(event>>EVENT_THRESH_SHIFT);mask|=p10_CNST_THRESH_CMP_MASK;value|=p10_CNST_THRESH_CMP_VAL(p10_thresh_cmp_val(event_config1) as u64);}else if event_is_threshold(event){return -1;}} else if cpu_has_feature(CPU_FTR_ARCH_300){if event_is_threshold(event)&&is_thresh_cmp_valid(event){mask|=CNST_THRESH_MASK;value|=CNST_THRESH_VAL(event>>EVENT_THRESH_SHIFT);}else if event_is_threshold(event){return -1;}} else {if event_is_fab_match(event){mask|=CNST_FAB_MATCH_MASK;value|=CNST_FAB_MATCH_VAL(event>>EVENT_THR_CTL_SHIFT);}else{if !is_thresh_cmp_valid(event){return -1;}mask|=CNST_THRESH_MASK;value|=CNST_THRESH_VAL(event>>EVENT_THRESH_SHIFT);}}
    isa207_get_constraint_ebb(event,maskp,valp,mask,value,ebb)
}

unsafe fn isa207_get_constraint_ebb(event:u64,maskp:*mut u64,valp:*mut u64,mut mask:u64,mut value:u64,ebb:u64)->i32 { if (event>>EVENT_PMC_SHIFT)&EVENT_PMC_MASK==0&&ebb!=0{return -1;} if event&EVENT_WANTS_BHRB!=0 {if ebb==0{return -1;}mask|=CNST_IFM_MASK;value|=CNST_IFM_VAL(event>>EVENT_IFM_SHIFT);} mask|=CNST_EBB_MASK;value|=CNST_EBB_VAL(ebb);*maskp=mask;*valp=value;0 }

pub unsafe fn isa207_compute_mmcr(event:*const u64,n_ev:i32,hwc:*mut u32,mmcr:*mut mmcr_regs,pevents:*mut *mut perf_event,flags:u32)->i32 {
    let mut inuse=0u64; for i in 0..n_ev {let e=*event.offset(i as isize);let p=(e>>EVENT_PMC_SHIFT)&EVENT_PMC_MASK;if p!=0{inuse|=1<<p;}}
    let mut mmcra=0u64;let mut mmcr1=0u64;let mut mmcr2=0u64;let mut mmcr3=0u64;if cpu_has_feature(CPU_FTR_ARCH_31){mmcra|=MMCRA_BHRB_DISABLE;}
    for i in 0..n_ev {let e=*event.offset(i as isize);let mut p=((e>>EVENT_PMC_SHIFT)&EVENT_PMC_MASK) as u32;let unit=(e>>EVENT_UNIT_SHIFT)&EVENT_UNIT_MASK;let combine=combine_from_event(e);let psel=e&EVENT_PSEL_MASK;if p==0{p=1;while p<=4&&(inuse&(1<<p))!=0{p+=1;}inuse|=1<<p;}if p<=4{mmcr1|=unit<<MMCR1_UNIT_SHIFT(p);mmcr1|=combine<<combine_shift(p);mmcr1|=psel<<MMCR1_PMCSEL_SHIFT(p);}mmcra_sdar_mode(e,&mut mmcra);if is_event_marked(e){mmcra|=MMCRA_SAMPLE_ENABLE;}if e&EVENT_WANTS_BHRB!=0{mmcra|=((e>>EVENT_IFM_SHIFT)&EVENT_IFM_MASK)<<MMCRA_IFM_SHIFT;}(*hwc.offset(i as isize))=p-1;}
    (*mmcr).mmcr0=0;if inuse&2!=0{(*mmcr).mmcr0|=MMCR0_PMC1CE;}if inuse&0x7c!=0{(*mmcr).mmcr0|=MMCR0_PMCjCE;}if inuse&0x60==0{(*mmcr).mmcr0|=MMCR0_FC56;}if cpu_has_feature(CPU_FTR_ARCH_31){(*mmcr).mmcr0|=MMCR0_PMCCEXT;}(*mmcr).mmcr1=mmcr1;(*mmcr).mmcra=mmcra;(*mmcr).mmcr2=mmcr2;(*mmcr).mmcr3=mmcr3;let _=(pevents,flags);0
}

pub unsafe fn isa207_disable_pmc(pmc:u32,mmcr:*mut mmcr_regs){if pmc<=3{(*mmcr).mmcr1&=!(0xffu64<<MMCR1_PMCSEL_SHIFT(pmc+1));}}
unsafe fn find_alternative(event:u64,ev_alt:*const [u32;MAX_ALT],size:i32)->i32{for i in 0..size{let a=(*ev_alt.offset(i as isize));if event<a[0] as u64{break;}for j in 0..MAX_ALT{if a[j]==0{break;}if event==a[j] as u64{return i;}}} -1}
pub unsafe fn isa207_get_alternatives(event:u64,alt:*mut u64,size:i32,flags:u32,ev_alt:*const [u32;MAX_ALT])->i32{let mut n=0;*alt=event;n+=1;let i=find_alternative(event,ev_alt,size);if i>=0{for j in 0..MAX_ALT{let x=(*ev_alt.offset(i as isize))[j] as u64;if x!=0&&x!=event{*alt.offset(n as isize)=x;n+=1;}}}if flags&PPMU_ONLY_COUNT_RUN!=0{let old=n;for i in 0..old{let x=*alt.offset(i as isize);let y=match x{0x1e=>0x600f4,0x600f4=>0x1e,0x2=>0x500fa,0x500fa=>0x2,_=>continue};*alt.offset(n as isize)=y;n+=1;}}n}

pub unsafe fn isa3XX_check_attr_config(ev:*mut perf_event)->i32{let event=(*ev).attr.config;let val=(event>>EVENT_SAMPLE_SHIFT)&EVENT_SAMPLE_MASK;if val&3==3{return -EINVAL;}match val{5|9|13|25|29|26|30=>return -EINVAL,_=>{}}let v=(event>>EVENT_THR_CTL_SHIFT)&EVENT_THR_CTL_MASK;if v&0xf0==0xf0||v&0xf==0xf{-EINVAL}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
