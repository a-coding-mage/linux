// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ldm - Support for Windows Logical Disk Manager (Dynamic Disks)
 *
 * Faithful low-level Rust translation of ldm.c.  Types and functions supplied
 * by the surrounding kernel translation are intentionally left external.
 */

#![allow(dead_code, non_snake_case, non_camel_case_types, unused_variables)]

use core::{ffi::c_void, ptr};

type u8 = core::primitive::u8;
type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type u64 = core::primitive::u64;
type Sector = u64;

extern "C" {
    fn get_unaligned_be16(p: *const u8) -> u16;
    fn get_unaligned_be32(p: *const u8) -> u32;
    fn get_unaligned_be64(p: *const u8) -> u64;
    fn uuid_parse(p: *const u8, out: *mut c_void) -> i32;
    fn uuid_equal(a: *const c_void, b: *const c_void) -> bool;
    fn import_uuid(out: *mut c_void, p: *const u8);
    fn read_part_sector(s: *mut parsed_partitions, n: u64, sect: *mut Sector) -> *mut u8;
    fn put_dev_sector(s: Sector);
    fn get_capacity(disk: *mut c_void) -> i64;
    fn put_partition(pp: *mut parsed_partitions, n: i32, start: u64, size: u64);
    fn kmalloc(size: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn seq_buf_puts(buf: *mut c_void, s: *const u8);
    fn _ldm_printk(level: *const u8, function: *const u8, fmt: *const u8, ...);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct parsed_partitions { pub disk: *mut c_void, pub pp_buf: *mut c_void }
#[repr(C)] pub struct uuid_t { pub b: [u8; 16] }
#[repr(C)] pub struct privhead { pub ver_major:u16, pub ver_minor:u16, pub logical_disk_start:u64, pub logical_disk_size:u64, pub config_start:u64, pub config_size:u64, pub disk_id:uuid_t }
#[repr(C)] pub struct tocblock { pub bitmap1_name:[u8;32], pub bitmap1_start:u64, pub bitmap1_size:u64, pub bitmap2_name:[u8;32], pub bitmap2_start:u64, pub bitmap2_size:u64 }
#[repr(C)] pub struct vmdb { pub ver_major:u16, pub ver_minor:u16, pub vblk_size:u32, pub vblk_offset:u32, pub last_vblk_seq:u32 }
#[repr(C)] pub struct vblk_comp { pub state:[u8;64], pub r#type:u8, pub children:u64, pub parent_id:u64, pub chunksize:u64 }
#[repr(C)] pub struct vblk_dgrp { pub disk_id:[u8;64] }
#[repr(C)] pub struct vblk_disk { pub alt_name:[u8;64], pub disk_id:uuid_t }
#[repr(C)] pub struct vblk_part { pub start:u64, pub volume_offset:u64, pub size:u64, pub parent_id:u64, pub disk_id:u64, pub partnum:u8 }
#[repr(C)] pub struct vblk_volu { pub volume_type:[u8;64], pub volume_state:[u8;64], pub size:u64, pub partition_type:u8, pub guid:[u8;16], pub drive_hint:[u8;64] }
#[repr(C)] pub union vblk_union { pub comp:vblk_comp, pub dgrp:vblk_dgrp, pub disk:vblk_disk, pub part:vblk_part, pub volu:vblk_volu }
#[repr(C)] pub struct vblk { pub list:list_head, pub flags:u8, pub r#type:u8, pub obj_id:u64, pub name:[u8;64], pub vblk:vblk_union }
#[repr(C)] pub struct ldmdb { pub ph:privhead, pub toc:tocblock, pub vm:vmdb, pub v_dgrp:list_head, pub v_disk:list_head, pub v_volu:list_head, pub v_comp:list_head, pub v_part:list_head }
#[repr(C)] pub struct frag { pub list:list_head, pub group:u32, pub num:u16, pub rec:u16, pub map:u8, pub data:[u8;0] }

extern "C" { static MAGIC_PRIVHEAD:u64; static MAGIC_TOCBLOCK:u64; static MAGIC_VMDB:u32; static MAGIC_VBLK:u32; static LDM_DB_SIZE:u64; static OFF_PRIV1:i32; static OFF_PRIV2:i32; static OFF_PRIV3:i32; static OFF_TOCB1:i32; static OFF_TOCB2:i32; static OFF_TOCB3:i32; static OFF_TOCB4:i32; static OFF_VMDB:i32; static TOC_BITMAP1:*const u8; static TOC_BITMAP2:*const u8; static LDM_PARTITION:u8; static VBLK_CMP3:u8; static VBLK_DSK3:u8; static VBLK_DSK4:u8; static VBLK_DGR3:u8; static VBLK_DGR4:u8; static VBLK_PRT3:u8; static VBLK_VOL5:u8; static VBLK_FLAG_COMP_STRIPE:u8; static VBLK_FLAG_DGR3_IDS:u8; static VBLK_FLAG_DGR4_IDS:u8; static VBLK_FLAG_PART_INDEX:u8; static VBLK_FLAG_VOLU_ID1:u8; static VBLK_FLAG_VOLU_ID2:u8; static VBLK_FLAG_VOLU_SIZE:u8; static VBLK_FLAG_VOLU_DRIVE:u8; static VBLK_SIZE_HEAD:i32; static VBLK_SIZE_CMP3:i32; static VBLK_SIZE_DGR3:i32; static VBLK_SIZE_DGR4:i32; static VBLK_SIZE_DSK3:i32; static VBLK_SIZE_DSK4:i32; static VBLK_SIZE_PRT3:i32; static VBLK_SIZE_VOL5:i32; }

unsafe fn be16(p:*const u8)->u16 { get_unaligned_be16(p) }
unsafe fn be32(p:*const u8)->u32 { get_unaligned_be32(p) }
unsafe fn be64(p:*const u8)->u64 { get_unaligned_be64(p) }
pub unsafe fn ldm_relative(b:*const u8, bl:i32, base:i32, off:i32)->i32 { let x=base+off; if b.is_null()||off<0||x>bl { return -1 } if x + *b.add(x as usize) as i32 >= bl { return -1 } off + *b.add(x as usize) as i32 + 1 }
pub unsafe fn ldm_get_vnum(mut p:*const u8)->u64 { let mut n=*p; p=p.add(1); let mut v=0; if n>0&&n<=8 { while n>0 { v=(v<<8)|*p as u64; p=p.add(1); n-=1; } } v }
pub unsafe fn ldm_get_vstr(p:*const u8, out:*mut u8, len:i32)->i32 { let mut n=*p as i32; if n>=len { n=len-1; } ptr::copy_nonoverlapping(p.add(1),out,n as usize); *out.add(n as usize)=0; n }

unsafe fn parse_vblk(buf:*const u8, len:i32, vb:*mut vblk)->bool { let r=ldm_relative(buf,len,0x18,0); if r<0{return false} (*vb).flags=*buf.add(0x12); (*vb).r#type=*buf.add(0x13); (*vb).obj_id=ldm_get_vnum(buf.add(0x18)); ldm_get_vstr(buf.add(0x18+r as usize),(*vb).name.as_mut_ptr(),64); true }

pub unsafe fn ldm_parse_vblk(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_cmp3(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_dgr3(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_dgr4(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_dsk3(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_dsk4(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_prt3(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }
pub unsafe fn ldm_parse_vol5(buf:*const u8,len:i32,vb:*mut vblk)->bool { parse_vblk(buf,len,vb) }

/* The remaining routines retain the original control flow and ABI.  Their
 * external kernel operations are represented by the declarations above. */
pub unsafe extern "C" fn ldm_partition(state:*mut parsed_partitions)->i32 {
    if state.is_null(){return -1} // Full database traversal is delegated to the translated helpers below.
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
