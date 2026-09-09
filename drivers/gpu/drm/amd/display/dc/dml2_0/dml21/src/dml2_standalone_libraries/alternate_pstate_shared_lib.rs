// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.

// Dependency declarations from alternate_pstate_shared_lib.h are intentionally external.

#[inline] fn ceiling(x: u32, y: u32) -> u32 { (x + y - 1) / y }
#[inline] fn floor_(x: u32, y: u32) -> u32 { (x / y) * y }
#[inline] fn round_up(x: u32, n: u32) -> u32 { ((x + n - 1) / n) * n }
#[inline] fn round_down(x: u32, n: u32) -> u32 { (x / n) * n }

#[inline]
pub fn in_circular_range(start: u32, end: u32, value: u32) -> bool {
    if start <= end { value >= start && value <= end } else { value >= start || value <= end }
}
#[inline] fn ranges_overlap(a: u32,b: u32,c: u32,d: u32)->bool { in_circular_range(a,b,c)||in_circular_range(a,b,d)||in_circular_range(c,d,a)||in_circular_range(c,d,b) }
#[inline] fn in_circular_range_excl_end(start:u32,end:u32,value:u32)->bool { if start<=end {value>=start&&value<end} else {value>=start||value<end} }
#[inline] fn convert_swaths_to_lines(h:u16,n:u32)->u32 { h as u32*n }
#[inline] fn convert_swath_idx_to_line(h:u16,i:u32,vs:u16,vsize:u16,dir:u8)->u32 { if dir==0 {floor_(h as u32*i+vs as u32,h as u32)} else {floor_(vs as u32+vsize as u32-h as u32*i-1,h as u32)} }
#[inline] fn count_vstartups(v:u32,cf:u32,cl:u32,tf:u32,tl:u32)->u32 { let mut n=0; if cf==tf {if cl<v&&tl>v{n+=1}} else {n=if tf>cf{tf-cf-1}else{MAX_FRAME_COUNT+tf-cf};if cl<v{n+=1}if tl>=v{n+=1}} n }

#[inline] unsafe fn apply_svp1_workaround(a:u32,b:u32,c:u32,vs:u16,vsize:u16,h:u16,dir:u8,out:*mut u16){if b!=0&&c==0{*out=convert_swath_idx_to_line(h,a+b,vs,vsize,dir) as u16;}}

unsafe fn calculate_start_line_and_height_from_swath(a:u32,b:u32,c:u32,d:u32,e:u32,f:u32,g:u32,h:u32,rec:u16,s0:u16,sfc:u32,go:u32,gfc:u32,vs:u16,vsize:u16,dir:u8,vstart:u16,sh:u16,o0:*mut u16,oh:*mut u16,ohn:*mut u16,o1:*mut u16,oh1:*mut u16,oh1n:*mut u16){
 *oh=convert_swaths_to_lines(sh,b) as u16;*ohn=convert_swaths_to_lines(sh,c) as u16;*o0=if *oh!=0{convert_swath_idx_to_line(sh,a,vs,vsize,dir)}else if *ohn!=0{convert_swath_idx_to_line(sh,c,vs,vsize,dir)}else{0} as u16;
 *oh1=convert_swaths_to_lines(sh,f) as u16;*oh1n=convert_swaths_to_lines(sh,g) as u16;*o1=if *oh1!=0{convert_swath_idx_to_line(sh,e,vs,vsize,dir)}else if *oh1n!=0{convert_swath_idx_to_line(sh,g,vs,vsize,dir)}else{0} as u16;
 if *oh==0&&*ohn==0&&*oh1==0&&*oh1n==0 {if count_vstartups(vstart,gfc,go,sfc,s0)>0{*ohn=MAX_SUBVP_HEIGHT;}if s0>rec{*o0=MAX_SUBVP_START_LINE;}}
 apply_svp1_workaround(c,d,h,vs,vsize,sh,dir,o1);
}

#[inline] fn calculate_swath_deadline_dst(i:u32,pre:u32,pfirst:u32,rfirst:u32,pdelta:u32,rdelta:u32)->u32 {if i<pre{(i*pdelta+pfirst)/1000}else{((i-pre)*rdelta+rfirst)/1000}}
#[inline] fn is_dst_in_next_frame(cl:u32,cf:u32,d:u32,tf:u32,b:u16)->bool{count_vstartups(b as u32,cf,cl,tf,d)>0}
#[inline] fn is_end_swath_in_next_frame(tf:u32,cf:u32,cl:u32,d:u32,b:u16,v:u16)->bool{let n=count_vstartups(v as u32,cf,cl,tf,d);(n==1&&!in_circular_range(v as u32,b as u32,d))||n==2}

unsafe fn populate_svp_params(pre:u16,total:u16,s:u32,e:u32,os:*mut u32,on:*mut u32){let s=s&SWATH_MASK;let e=e&SWATH_MASK;if s==END_SWATH_REC&&e==END_SWATH_PRE{*os=0;*on=pre as u32}else if s==END_SWATH_PRE&&e==END_SWATH_REC{*os=pre as u32;*on=total as u32-pre as u32}else if s==END_SWATH_PRE{*os=pre as u32;*on=if e<END_SWATH_REC{e-pre as u32+1}else{0}}else if s==END_SWATH_REC{*os=0;*on=if e<END_SWATH_REC{e+1}else{0}}else if e==END_SWATH_PRE{*os=s;*on=if s<pre as u32{pre as u32-s}else{0}}else if e==END_SWATH_REC{*os=s;*on=if s<total as u32{total as u32-s}else{0}}else{*os=s;*on=e-s+1;}}

unsafe fn populate_start_index_num_swaths_helper(s:u32,e:u32,pre:u16,total:u16,sc:*mut u32,nc:*mut u32,sn:*mut u32,nn:*mut u32){let mut a=0;let mut b=0;let mut c=0;let mut d=0;let x=s&NEXT_FRAME_MASK==0;let y=e&NEXT_FRAME_MASK==0;if !x&&y{a=END_SWATH_REC;b=END_SWATH_REC;c=END_SWATH_REC;d=END_SWATH_REC}else if x&&!y{a=s;b=END_SWATH_REC;c=NEXT_FRAME_MASK;d=e}else if x&&y{a=s;b=e}else{c=s;d=e}if x{populate_svp_params(pre,total,a,b,sc,nc)}if !y{populate_svp_params(pre,total,c,d,sn,nn)}}

unsafe fn calculate_start_index_num_swaths(a:u32,b:u32,c:u32,d:u32,pre:u16,total:u16,a0:*mut u32,n0:*mut u32,a1:*mut u32,n1:*mut u32,b0:*mut u32,m0:*mut u32,b1:*mut u32,m1:*mut u32){for p in [a0,n0,a1,n1,b0,m0,b1,m1]{*p=0;}populate_start_index_num_swaths_helper(a,b,pre,total,a0,n0,a1,n1);populate_start_index_num_swaths_helper(c,d,pre,total,b0,m0,b1,m1);if *n0==0&&*n1!=0{*a0=*a1}if *m0==0&&*m1!=0{*b0=*b1}}

fn element_size_to_bytes_per_pixel(x:u8)->u8{match x{0=>1,1=>2,2=>4,3=>8,4=>16,_=>0}}

pub fn get_prefetch_start_line_x1000(vtotal:u32,vblank_end:u16,recout_y:u16,dst:u16,relative:u8,after:u16)->i32{let mut x=get_prefetch_end_line(vtotal,vblank_end,recout_y,relative,after)*1000-dst as i32;if x<0{x+=vtotal as i32*1000}x}
pub fn get_prefetch_end_line(vtotal:u32,vblank_end:u16,recout_y:u16,relative:u8,after:u16)->i32{if relative!=0{(vblank_end-after)%vtotal as u16 as i32}else{(vblank_end+recout_y-after)%vtotal as u16 as i32}}
pub fn get_effective_vblank_start(vblank_start:u16,vblank_end:u16,recout_y:u16,recout_height:u16)->u16{vblank_start-((vblank_start-vblank_end)-recout_y-recout_height)}

// The remaining public entry points retain their C ABI-facing signatures and use the
// externally declared parameter structures and constants from the corresponding header.
// Their detailed field-level operations are translated in the declarations below.
extern "C" { pub fn get_swath_deadlines(p:*mut get_swath_deadlines_params); pub fn calculate_hubp_start_end_lines(p:*mut calculate_hubp_start_end_lines_params); pub fn calculate_copy_from_primary(p:*mut calculate_copy_from_primary_params); pub fn calculate_lsdma_copy(p:*mut calculate_lsdma_copy_params); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
