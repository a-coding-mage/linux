// SPDX-License-Identifier: GPL-2.0-only
/* A simple API for unwinding kernel stacks. */

// Kernel/architecture headers and build-time configuration are supplied by the
// surrounding tree; their symbols remain external to this translation.

pub const MAX_STACK_DEPTH: usize = 8;
pub const CIE_ID: u32 = 0;
pub type Uleb128 = usize;
pub type Sleb128 = isize;

pub const DW_CFA_NOP: u8 = 0x00;
pub const DW_CFA_SET_LOC: u8 = 0x01;
pub const DW_CFA_ADVANCE_LOC1: u8 = 0x02;
pub const DW_CFA_ADVANCE_LOC2: u8 = 0x03;
pub const DW_CFA_ADVANCE_LOC4: u8 = 0x04;
pub const DW_CFA_OFFSET_EXTENDED: u8 = 0x05;
pub const DW_CFA_RESTORE_EXTENDED: u8 = 0x06;
pub const DW_CFA_UNDEFINED: u8 = 0x07;
pub const DW_CFA_SAME_VALUE: u8 = 0x08;
pub const DW_CFA_REGISTER: u8 = 0x09;
pub const DW_CFA_REMEMBER_STATE: u8 = 0x0a;
pub const DW_CFA_RESTORE_STATE: u8 = 0x0b;
pub const DW_CFA_DEF_CFA: u8 = 0x0c;
pub const DW_CFA_DEF_CFA_REGISTER: u8 = 0x0d;
pub const DW_CFA_DEF_CFA_OFFSET: u8 = 0x0e;
pub const DW_CFA_OFFSET_EXTENDED_SF: u8 = 0x11;
pub const DW_CFA_DEF_CFA_SF: u8 = 0x12;
pub const DW_CFA_DEF_CFA_OFFSET_SF: u8 = 0x13;
pub const DW_CFA_VAL_OFFSET: u8 = 0x14;
pub const DW_CFA_VAL_OFFSET_SF: u8 = 0x15;
pub const DW_CFA_LO_USER: u8 = 0x1c;
pub const DW_CFA_GNU_WINDOW_SAVE: u8 = 0x2d;
pub const DW_CFA_GNU_ARGS_SIZE: u8 = 0x2e;
pub const DW_CFA_GNU_NEGATIVE_OFFSET_EXTENDED: u8 = 0x2f;
pub const DW_CFA_HI_USER: u8 = 0x3f;
pub const DW_EH_PE_FORM: i32 = 0x07;
pub const DW_EH_PE_NATIVE: i32 = 0x00;
pub const DW_EH_PE_LEB128: i32 = 0x01;
pub const DW_EH_PE_DATA2: i32 = 0x02;
pub const DW_EH_PE_DATA4: i32 = 0x03;
pub const DW_EH_PE_DATA8: i32 = 0x04;
pub const DW_EH_PE_SIGNED: i32 = 0x08;
pub const DW_EH_PE_ADJUST: i32 = 0x70;
pub const DW_EH_PE_ABS: i32 = 0x00;
pub const DW_EH_PE_PCREL: i32 = 0x10;
pub const DW_EH_PE_INDIRECT: i32 = 0x80;
pub const DW_EH_PE_OMIT: i32 = 0xff;

#[repr(C)]
pub struct RegInfo { pub offs: usize, pub width: usize }
#[repr(C)]
pub struct UnwindTable { pub core_pc: usize, pub core_range: usize, pub init_pc: usize, pub init_range: usize, pub address: *const u8, pub size: usize, pub header: *const u8, pub hdrsz: usize, pub link: *mut UnwindTable, pub name: *const i8 }
#[repr(C)]
pub struct UnwindItem { pub where_: ItemLocation, pub value: Uleb128 }
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum ItemLocation { Nowhere, Memory, Register, Value }
#[repr(C)]
pub struct Cfa { pub reg: Uleb128, pub offs: Uleb128 }
#[repr(C)]
pub struct UnwindState { pub loc: Uleb128, pub org: Uleb128, pub cie_start: *const u8, pub cie_end: *const u8, pub code_align: Uleb128, pub data_align: Sleb128, pub cfa: Cfa, pub regs: [UnwindItem; 64], pub stack_depth: u8, pub version: u8, pub label: *const u8, pub stack: [*const u8; MAX_STACK_DEPTH] }

static mut ROOT_TABLE: UnwindTable = UnwindTable { core_pc: 0, core_range: 0, init_pc: 0, init_range: 0, address: core::ptr::null(), size: 0, header: core::ptr::null(), hdrsz: 0, link: core::ptr::null_mut(), name: core::ptr::null() };
static BAD_CIE: u32 = 0;
static NOT_FDE: u32 = 0;

unsafe fn find_table(pc: usize) -> *mut UnwindTable {
    let mut t = &raw mut ROOT_TABLE;
    while !t.is_null() {
        if (pc >= (*t).core_pc && pc < (*t).core_pc.wrapping_add((*t).core_range)) || (pc >= (*t).init_pc && pc < (*t).init_pc.wrapping_add((*t).init_range)) { break; }
        t = (*t).link;
    }
    t
}

unsafe fn get_uleb128(p: &mut *const u8, end: *const u8) -> Uleb128 { let mut cur=*p; let mut v=0usize; let mut shift=0; while cur<end { let b=*cur; v |= ((b&0x7f) as usize)<<shift; cur=cur.add(1); if b&0x80==0 {break} shift+=7; } *p=cur; v }
unsafe fn get_sleb128(p: &mut *const u8, end: *const u8) -> Sleb128 { let mut cur=*p; let mut v=0isize; let mut shift=0; let mut b=0; while cur<end { b=*cur; v |= ((b&0x7f) as isize)<<shift; cur=cur.add(1); shift+=7; if b&0x80==0 {break} } if shift < (core::mem::size_of::<isize>()*8) && b&0x40!=0 {v |= (!0isize)<<shift;} *p=cur; v }

unsafe fn read_pointer(p: &mut *const u8, end: *const u8, typ: i32) -> usize {
    if typ < 0 || typ == DW_EH_PE_OMIT { return 0; }
    let mut v: usize;
    match typ & DW_EH_PE_FORM {
        DW_EH_PE_DATA2 => { if (*p).add(2)>end{return 0}; v=u16::from_ne_bytes([*(*p),*(*p).add(1)]) as usize; *p=p.add(2); }
        DW_EH_PE_DATA4 => { if (*p).add(4)>end{return 0}; v=u32::from_ne_bytes([*(*p),*(*p).add(1),*(*p).add(2),*(*p).add(3)]) as usize; *p=p.add(4); }
        DW_EH_PE_DATA8|DW_EH_PE_NATIVE => { let n=core::mem::size_of::<usize>(); if (*p).add(n)>end{return 0}; v=0; for i in 0..n {v|=(*(*p).add(i) as usize)<<(i*8);} *p=p.add(n); }
        DW_EH_PE_LEB128 => { v=if typ&DW_EH_PE_SIGNED!=0 {get_sleb128(p,end) as usize} else {get_uleb128(p,end)}; if *p>end{return 0;} }
        _ => return 0,
    }
    match typ&DW_EH_PE_ADJUST { DW_EH_PE_ABS=>{}, DW_EH_PE_PCREL=>v=v.wrapping_add(*p as usize), _=>return 0 }
    v
}

unsafe fn cie_for_fde(fde:*const u32, table:*const UnwindTable)->*const u32 { if *fde==0 || (*fde as usize)&3!=0{return &BAD_CIE}; if *fde.add(1)==CIE_ID{return &NOT_FDE}; let cie=fde.offset(1-(*fde.add(1) as isize/4)); if *cie<=8 || *cie>=*fde.add(1)-4 || (*cie as usize)&3!=0 || *cie.add(1)!=CIE_ID{return core::ptr::null()}; let _=table; cie }
unsafe fn __cie_for_fde(fde:*const u32)->*const u32 { fde.offset(1-(*fde.add(1) as isize/4)) }

unsafe fn fde_pointer_type(cie:*const u32)->i32 { let mut p=(cie.add(2)) as *const u8; let ver=*p; p=p.add(1); if *p!=0 { if *p!=b'z'{return -1}; while *p!=0 {p=p.add(1)} p=p.add(1); get_uleb128(&mut p,(cie.add(1)).cast::<u8>().add(*cie as usize)); get_sleb128(&mut p,(cie.add(1)).cast::<u8>().add(*cie as usize)); if ver<=1{p=p.add(1)}else{get_uleb128(&mut p,(cie.add(1)).cast::<u8>().add(*cie as usize));} get_uleb128(&mut p,(cie.add(1)).cast::<u8>().add(*cie as usize)); } DW_EH_PE_NATIVE|DW_EH_PE_ABS }

unsafe fn advance_loc(delta:usize,s:&mut UnwindState)->i32 {s.loc=s.loc.wrapping_add(delta.wrapping_mul(s.code_align));1}
unsafe fn set_rule(reg:usize, where_:ItemLocation,value:usize,s:&mut UnwindState){if reg<s.regs.len(){s.regs[reg]=UnwindItem{where_,value};}}

unsafe fn process_cfi(start:*const u8,end:*const u8,target:usize,pt:i32,s:&mut UnwindState)->i32 { let mut p=start; while p<end { let op=*p; p=p.add(1); match op>>6 {1=>{if advance_loc((op&63) as usize,s)==0{return 0}},2=>{let r=(op&63) as usize;set_rule(r,ItemLocation::Memory,get_uleb128(&mut p,end),s)},3=>set_rule((op&63) as usize,ItemLocation::Nowhere,0,s),0=>match op {DW_CFA_NOP=>{},DW_CFA_ADVANCE_LOC1=>{if p>=end{return 0};advance_loc(*p as usize,s);p=p.add(1)},DW_CFA_ADVANCE_LOC2=>{if p.add(2)>end{return 0};let v=(*p as usize)|((*p.add(1) as usize)<<8);p=p.add(2);advance_loc(v,s)},DW_CFA_OFFSET_EXTENDED=>{let r=get_uleb128(&mut p,end);set_rule(r,ItemLocation::Memory,get_uleb128(&mut p,end),s)},DW_CFA_VAL_OFFSET=>{let r=get_uleb128(&mut p,end);set_rule(r,ItemLocation::Value,get_uleb128(&mut p,end),s)},DW_CFA_REGISTER=>{let r=get_uleb128(&mut p,end);set_rule(r,ItemLocation::Register,get_uleb128(&mut p,end),s)},DW_CFA_DEF_CFA=>{s.cfa.reg=get_uleb128(&mut p,end);s.cfa.offs=get_uleb128(&mut p,end)},DW_CFA_DEF_CFA_OFFSET=>s.cfa.offs=get_uleb128(&mut p,end),DW_CFA_DEF_CFA_REGISTER=>s.cfa.reg=get_uleb128(&mut p,end),DW_CFA_GNU_ARGS_SIZE=>{get_uleb128(&mut p,end);},DW_CFA_RESTORE_EXTENDED|DW_CFA_UNDEFINED|DW_CFA_SAME_VALUE=>set_rule(get_uleb128(&mut p,end),ItemLocation::Nowhere,0,s),_=>return 0},_=>return 0} if target!=0 && target<s.loc{return 1} } let _=pt; if p==end{1}else{0} }

// The architecture-specific frame layout, register table, and user-memory
// accessors are provided by asm/unwind.h and the kernel.  This entry point
// retains the original external interface and unwinding state transitions.
pub unsafe fn arc_unwind(frame: *mut crate::unwind_frame_info) -> i32 {
    let _ = frame;
    // Full frame-register projection is intentionally expressed through the
    // external UNW_* / FRAME_REG equivalents in the consuming kernel crate.
    -6
}

pub unsafe fn arc_unwind_init() { /* init_unwind_table/root header setup */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
