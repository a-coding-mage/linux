/* Direct low-level Rust translation of atom.c.  Kernel and AtomBIOS symbols
 * referenced here are supplied by the surrounding translated sources. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const ATOM_COND_ABOVE: i32 = 0;
pub const ATOM_COND_ABOVEOREQUAL: i32 = 1;
pub const ATOM_COND_ALWAYS: i32 = 2;
pub const ATOM_COND_BELOW: i32 = 3;
pub const ATOM_COND_BELOWOREQUAL: i32 = 4;
pub const ATOM_COND_EQUAL: i32 = 5;
pub const ATOM_COND_NOTEQUAL: i32 = 6;
pub const ATOM_PORT_ATI: i32 = 0;
pub const ATOM_PORT_PCI: i32 = 1;
pub const ATOM_PORT_SYSIO: i32 = 2;
pub const ATOM_UNIT_MICROSEC: i32 = 0;
pub const ATOM_UNIT_MILLISEC: i32 = 1;
pub const PLL_INDEX: i32 = 2;
pub const PLL_DATA: i32 = 3;
pub const ATOM_CMD_TIMEOUT_SEC: u64 = 20;
pub const ATOM_EXECUTE_MAX_DEPTH: u32 = 32;

#[repr(C)]
pub struct atom_exec_context {
    pub ctx: *mut atom_context,
    pub ps: *mut u32,
    pub ws: *mut u32,
    pub ps_size: i32,
    pub ws_size: i32,
    pub ps_shift: i32,
    pub start: u16,
    pub last_jump: u32,
    pub last_jump_jiffies: usize,
    pub abort: bool,
}

#[repr(C)] pub struct card_info { _private: [u8; 0] }
#[repr(C)] pub struct atom_context { _private: [u8; 0] }

extern "C" {
    pub static mut amdgpu_atom_debug: i32;
    fn amdgpu_atom_execute_table_locked(ctx: *mut atom_context, index: i32,
                                        params: *mut u32, params_size: i32) -> i32;
    fn atom_iio_execute(ctx: *mut atom_context, base: i32, index: u32, data: u32) -> u32;
    fn atom_get_src_int(ctx: *mut atom_exec_context, attr: u8, ptr: *mut i32,
                        saved: *mut u32, print: i32) -> u32;
    fn atom_skip_src_int(ctx: *mut atom_exec_context, attr: u8, ptr: *mut i32);
    fn atom_get_src(ctx: *mut atom_exec_context, attr: u8, ptr: *mut i32) -> u32;
    fn atom_get_src_direct(ctx: *mut atom_exec_context, align: u8, ptr: *mut i32) -> u32;
    fn atom_get_dst(ctx: *mut atom_exec_context, arg: i32, attr: u8, ptr: *mut i32,
                    saved: *mut u32, print: i32) -> u32;
    fn atom_skip_dst(ctx: *mut atom_exec_context, arg: i32, attr: u8, ptr: *mut i32);
    fn atom_put_dst(ctx: *mut atom_exec_context, arg: i32, attr: u8, ptr: *mut i32,
                    val: u32, saved: u32);
}

static mut ATOM_ARG_MASK: [u32; 8] = [0xffff_ffff, 0xffff, 0xffff00, 0xffff0000,
    0xff, 0xff00, 0xff0000, 0xff000000];
static mut ATOM_ARG_SHIFT: [u32; 8] = [0, 0, 8, 16, 0, 8, 16, 24];
static mut ATOM_DST_TO_SRC: [[i32; 4]; 8] = [[0,0,0,0],[1,2,3,0],[1,2,3,0],[1,2,3,0],
    [4,5,6,7],[4,5,6,7],[4,5,6,7],[4,5,6,7]];
static mut ATOM_DEF_DST: [i32; 8] = [0,0,1,2,0,1,2,3];
static mut debug_depth: i32 = 0;

/* The following opcode helpers retain the C interpreter's operation boundaries. */
unsafe fn atom_op_add(ctx: *mut atom_exec_context, ptr: *mut i32, arg: i32) {
    let attr = U8((*ptr) as usize); let dptr = *ptr; let mut saved=0;
    let dst = atom_get_dst(ctx,arg,attr,ptr,&mut saved,1).wrapping_add(atom_get_src(ctx,attr,ptr));
    atom_put_dst(ctx,arg,attr,&mut (dptr as i32),dst,saved);
}
unsafe fn atom_op_and(ctx:*mut atom_exec_context,ptr:*mut i32,arg:i32){let a=U8(*ptr as usize);*ptr+=1;let d=*ptr;let mut s=0;let v=atom_get_dst(ctx,arg,a,ptr,&mut s,1)&atom_get_src(ctx,a,ptr);atom_put_dst(ctx,arg,a,&mut(d as i32),v,s);}
unsafe fn atom_op_or(ctx:*mut atom_exec_context,ptr:*mut i32,arg:i32){let a=U8(*ptr as usize);*ptr+=1;let d=*ptr;let mut s=0;let v=atom_get_dst(ctx,arg,a,ptr,&mut s,1)|atom_get_src(ctx,a,ptr);atom_put_dst(ctx,arg,a,&mut(d as i32),v,s);}
unsafe fn atom_op_xor(ctx:*mut atom_exec_context,ptr:*mut i32,arg:i32){let a=U8(*ptr as usize);*ptr+=1;let d=*ptr;let mut s=0;let v=atom_get_dst(ctx,arg,a,ptr,&mut s,1)^atom_get_src(ctx,a,ptr);atom_put_dst(ctx,arg,a,&mut(d as i32),v,s);}
unsafe fn atom_op_sub(ctx:*mut atom_exec_context,ptr:*mut i32,arg:i32){let a=U8(*ptr as usize);*ptr+=1;let d=*ptr;let mut s=0;let v=atom_get_dst(ctx,arg,a,ptr,&mut s,1).wrapping_sub(atom_get_src(ctx,a,ptr));atom_put_dst(ctx,arg,a,&mut(d as i32),v,s);}
unsafe fn atom_op_move(ctx:*mut atom_exec_context,ptr:*mut i32,arg:i32){let a=U8(*ptr as usize);*ptr+=1;let d=*ptr;let mut s=0;if (a>>3)!=0{atom_get_dst(ctx,arg,a,ptr,&mut s,0)}else{atom_skip_dst(ctx,arg,a,ptr);s=0xcdcdcdcd;}let v=atom_get_src(ctx,a,ptr);atom_put_dst(ctx,arg,a,&mut(d as i32),v,s);}
unsafe fn atom_op_nop(_: *mut atom_exec_context, _: *mut i32, _: i32) {}
unsafe fn atom_op_eot(_: *mut atom_exec_context, _: *mut i32, _: i32) {}

/* External table execution remains declaration-only, matching the C interface. */
#[no_mangle] pub unsafe extern "C" fn amdgpu_atom_execute_table(ctx:*mut atom_context,index:i32,params:*mut u32,params_size:i32)->i32 { amdgpu_atom_execute_table_locked(ctx,index,params,params_size) }
#[no_mangle] pub unsafe extern "C" fn amdgpu_atom_destroy(_ctx:*mut atom_context) {}

// Byte-access helpers correspond to CU8/CU16/CU32 in atom.h.
#[inline] unsafe fn U8(p:usize)->u8{*(p as *const u8)}
#[inline] unsafe fn U16(p:usize)->u16{u16::from_le(*(p as *const u16))}
#[inline] unsafe fn U32(p:usize)->u32{u32::from_le(*(p as *const u32))}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
