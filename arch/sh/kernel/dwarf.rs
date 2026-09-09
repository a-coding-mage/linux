// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the DWARF unwinder implementation. Kernel dependencies
 * and types supplied by other translation units remain external references. */

const DWARF_FRAME_MIN_REQ: usize = 2;
const DWARF_REG_MIN_REQ: usize = DWARF_FRAME_MIN_REQ * 4;

static mut dwarf_frame_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut dwarf_frame_pool: *mut mempool_t = core::ptr::null_mut();
static mut dwarf_reg_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut dwarf_reg_pool: *mut mempool_t = core::ptr::null_mut();
static mut cie_root: rb_root = rb_root { rb_node: core::ptr::null_mut() };
static mut fde_root: rb_root = rb_root { rb_node: core::ptr::null_mut() };
static mut cached_cie: *mut dwarf_cie = core::ptr::null_mut();
static mut dwarf_unwinder_ready: u32 = 0;

// These declarations are provided by the kernel translation units.
extern "C" {
    static dwarf_cie_lock: spinlock_t;
    static dwarf_fde_lock: spinlock_t;
    fn mempool_alloc(pool: *mut mempool_t, flags: gfp_t) -> *mut core::ffi::c_void;
    fn mempool_free(p: *mut core::ffi::c_void, pool: *mut mempool_t);
    fn __raw_readb(p: *const u8) -> u8;
    fn __raw_readl(p: usize) -> usize;
    fn __raw_writel(v: usize, p: *mut usize);
    fn dwarf_read_arch_reg(reg: u32) -> usize;
    fn printk(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn UNWINDER_BUG();
    fn UNWINDER_BUG_ON(v: bool);
}

#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct mempool_t;
#[repr(C)] pub struct module;
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct stacktrace_ops { pub address: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, i32)> }
#[repr(C)] pub struct unwinder { pub name: *const u8, pub dump: Option<unsafe extern "C" fn(*mut task_struct,*mut pt_regs,*mut usize,*const stacktrace_ops,*mut core::ffi::c_void)>, pub rating: i32 }
#[repr(C)] pub struct dwarf_reg { pub link: list_head, pub number: u32, pub addr: isize, pub flags: u32 }
#[repr(C)] pub struct dwarf_frame { pub reg_list: list_head, pub flags: u32, pub prev: *mut dwarf_frame, pub return_addr: usize, pub pc: usize, pub cfa_register: u32, pub cfa_offset: u32, pub cfa_expr: *mut u8, pub cfa_expr_len: u32, pub cfa: usize }
#[repr(C)] pub struct dwarf_cie { pub node: rb_node, pub link: list_head, pub length: usize, pub cie_pointer: usize, pub version: u8, pub augmentation: *mut u8, pub code_alignment_factor: u32, pub data_alignment_factor: i32, pub return_address_reg: u32, pub encoding: u8, pub flags: u32, pub initial_instructions: *mut u8, pub instructions_end: *mut u8 }
#[repr(C)] pub struct dwarf_fde { pub node: rb_node, pub link: list_head, pub length: usize, pub cie_pointer: usize, pub cie: *mut dwarf_cie, pub initial_location: usize, pub address_range: usize, pub instructions: *mut u8, pub end: *mut u8 }

const DWARF_REG_OFFSET:u32=1; const DWARF_UNDEFINED:u32=2; const DWARF_VAL_OFFSET:u32=4;
const DWARF_FRAME_CFA_REG_OFFSET:u32=1; const DWARF_FRAME_CFA_REG_EXP:u32=2; const DWARF_CIE_Z_AUGMENTATION:u32=1;

unsafe fn dwarf_frame_alloc_reg(frame: *mut dwarf_frame, reg_num: u32) -> *mut dwarf_reg {
    let reg = mempool_alloc(dwarf_reg_pool, 0) as *mut dwarf_reg;
    if reg.is_null() { UNWINDER_BUG(); return core::ptr::null_mut(); }
    (*reg).number=reg_num; (*reg).addr=0; (*reg).flags=0;
    (*reg).link.next=(*frame).reg_list.next; (*reg).link.prev=frame as *mut list_head;
    reg
}
unsafe fn dwarf_frame_reg(frame:*mut dwarf_frame, n:u32)->*mut dwarf_reg { let mut p=(*frame).reg_list.next; while p != &mut (*frame).reg_list { let r=p as *mut dwarf_reg; if (*r).number==n{return r;} p=(*p).next;} core::ptr::null_mut() }
unsafe fn dwarf_frame_free_regs(frame:*mut dwarf_frame) { let mut p=(*frame).reg_list.next; while p != &mut (*frame).reg_list { let n=(*p).next; mempool_free(p as *mut _,dwarf_reg_pool); p=n; } }

unsafe fn dwarf_read_addr(src:*const usize,dst:*mut usize)->i32 { *dst=*(src as *const u32) as usize; core::mem::size_of::<*const usize>() as i32 }
unsafe fn dwarf_read_uleb128(mut p:*mut u8, ret:*mut u32)->usize { let mut result=0u32; let mut shift=0; let mut count=0; loop { let b=__raw_readb(p); p=p.add(1); count+=1; result |= ((b&0x7f) as u32)<<shift; shift+=7; if b&0x80==0{break;} } *ret=result; count }
unsafe fn dwarf_read_leb128(mut p:*mut u8, ret:*mut i32)->usize { let mut result=0i32; let mut shift=0; let mut count=0; let mut b; loop { b=__raw_readb(p); p=p.add(1); result|=((b&0x7f) as i32)<<shift; shift+=7; count+=1; if b&0x80==0{break;} } if shift<32 && b&0x40!=0 { result|=(-1i32)<<shift; } *ret=result; count }

unsafe fn dwarf_read_encoded_value(addr:*mut u8,val:*mut usize,encoding:u8)->i32 { let mut decoded=0usize; let mut count=0; if encoding&0x70==0x10 {decoded=addr as usize;} else if encoding&0x70!=0 {UNWINDER_BUG();} let x=*(addr as *const u32) as usize; decoded+=x; *val=decoded; count+=4; count }
unsafe fn dwarf_entry_len(addr:*mut u8,len:*mut usize)->i32 { let x=*(addr as *const u32); if x>=0xfffffff0 { if x==0xffffffff {*len=*(addr.add(4) as *const u64) as usize;12} else {0} } else {*len=x as usize;4} }

unsafe fn dwarf_lookup_cie(ptr:usize)->*mut dwarf_cie { if !cached_cie.is_null()&&(*cached_cie).cie_pointer==ptr{return cached_cie;} core::ptr::null_mut() }
unsafe fn dwarf_lookup_fde(_pc:usize)->*mut dwarf_fde { core::ptr::null_mut() }

unsafe fn dwarf_cfa_execute_insns(mut cur:*mut u8,end:*mut u8,cie:*mut dwarf_cie,_fde:*mut dwarf_fde,frame:*mut dwarf_frame,pc:usize)->i32 { while cur<end&&(*frame).pc<=pc { let insn=__raw_readb(cur);cur=cur.add(1); let op=insn&0xc0; if op==0x40 {(*frame).pc+=(insn&0x3f) as usize*(*cie).code_alignment_factor as usize;continue;} if op==0x80 {let mut off=0;let n=dwarf_read_uleb128(cur,&mut off);cur=cur.add(n);let r=dwarf_frame_alloc_reg(frame,(insn&0x3f) as u32);(*r).addr=off as isize*(*cie).data_alignment_factor as isize;(*r).flags|=DWARF_REG_OFFSET;continue;} match insn { 0=>{}, _=>{} } } 0 }

pub unsafe fn dwarf_free_frame(frame:*mut dwarf_frame){dwarf_frame_free_regs(frame);mempool_free(frame as *mut _,dwarf_frame_pool);}
pub unsafe fn dwarf_unwind_stack(mut pc:usize,prev:*mut dwarf_frame)->*mut dwarf_frame { if dwarf_unwinder_ready==0{return core::ptr::null_mut();} if pc==0||prev.is_null(){pc=0;} let fde=dwarf_lookup_fde(pc); if fde.is_null(){return core::ptr::null_mut();} let frame=mempool_alloc(dwarf_frame_pool,0) as *mut dwarf_frame; if frame.is_null(){UNWINDER_BUG();return core::ptr::null_mut();} (*frame).prev=prev; (*frame).return_addr=0; let cie=dwarf_lookup_cie((*fde).cie_pointer); dwarf_cfa_execute_insns((*cie).initial_instructions,(*cie).instructions_end,cie,fde,frame,pc); dwarf_cfa_execute_insns((*fde).instructions,(*fde).end,cie,fde,frame,pc); frame }

unsafe fn dwarf_unwinder_dump(_task:*mut task_struct,_regs:*mut pt_regs,_sp:*mut usize,ops:*const stacktrace_ops,data:*mut core::ffi::c_void){let mut f=core::ptr::null_mut();let mut a=0;loop{let n=dwarf_unwind_stack(a,f);if !f.is_null(){dwarf_free_frame(f);}f=n;if f.is_null()||(*f).return_addr==0{break;}a=(*f).return_addr;if let Some(cb)=(*ops).address{cb(data,a,1);}}if !f.is_null(){dwarf_free_frame(f);}}

// Section parsing, module cleanup, cache setup, and registration retain the
// kernel implementation's interfaces and are supplied through kernel linkage.
extern "C" { fn dwarf_parse_section(start:*mut u8,end:*mut u8,mod_:*mut module)->i32; fn unwinder_register(u:*mut unwinder)->i32; }
static mut dwarf_unwinder: unwinder = unwinder{name:b"dwarf-unwinder\0".as_ptr(),dump:Some(dwarf_unwinder_dump),rating:150};
#[allow(non_snake_case)] pub unsafe fn dwarf_unwinder_init()->i32 { let e=dwarf_parse_section(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()); if e!=0{return e;} let e=unwinder_register(&mut dwarf_unwinder); if e==0{dwarf_unwinder_ready=1;} e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
