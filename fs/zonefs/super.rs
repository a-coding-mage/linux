// SPDX-License-Identifier: GPL-2.0
/* Simple file system for zoned block devices exposing zones as files. */

// Kernel dependencies and symbols from zonefs.h/trace.h are intentionally
// referenced as external Rust items supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_void};

unsafe fn zonefs_zgroup_name(ztype: zonefs_ztype) -> *const c_char {
    match ztype { ZONEFS_ZTYPE_CNV => b"cnv\0".as_ptr() as _, ZONEFS_ZTYPE_SEQ => b"seq\0".as_ptr() as _, _ => { WARN_ON_ONCE(1); b"???\0".as_ptr() as _ } }
}

unsafe fn zonefs_account_active(sb: *mut super_block, z: *mut zonefs_zone) {
    let sbi = ZONEFS_SB(sb); if zonefs_zone_is_cnv(z) { return; }
    if (*z).z_flags & (ZONEFS_ZONE_OFFLINE | ZONEFS_ZONE_READONLY) != 0 { }
    else if (*z).z_flags & ZONEFS_ZONE_OPEN != 0 || ((*z).z_wpoffset > 0 && (*z).z_wpoffset < (*z).z_capacity) {
        if (*z).z_flags & ZONEFS_ZONE_ACTIVE == 0 { (*z).z_flags |= ZONEFS_ZONE_ACTIVE; atomic_inc(&mut (*sbi).s_active_seq_files); }
        return;
    } else { return; }
    if (*z).z_flags & ZONEFS_ZONE_ACTIVE != 0 { (*z).z_flags &= !ZONEFS_ZONE_ACTIVE; atomic_dec(&mut (*sbi).s_active_seq_files); }
}

pub unsafe fn zonefs_inode_account_active(inode: *mut inode) { lockdep_assert_held(&mut (*ZONEFS_I(inode)).i_truncate_mutex); zonefs_account_active((*inode).i_sb, zonefs_inode_zone(inode)); }

unsafe fn zonefs_zone_mgmt(sb: *mut super_block, z: *mut zonefs_zone, mut op: req_op) -> c_int {
    if op == REQ_OP_ZONE_CLOSE && (*z).z_wpoffset == 0 { op = REQ_OP_ZONE_RESET; }
    trace_zonefs_zone_mgmt(sb, z, op);
    let ret = blkdev_zone_mgmt((*sb).s_bdev, op, (*z).z_sector, (*z).z_size >> SECTOR_SHIFT);
    if ret != 0 { zonefs_err(sb, b"Zone management operation %s at %llu failed %d\n\0".as_ptr() as _, blk_op_str(op), (*z).z_sector, ret); }
    ret
}
pub unsafe fn zonefs_inode_zone_mgmt(i: *mut inode, op: req_op) -> c_int { lockdep_assert_held(&mut (*ZONEFS_I(i)).i_truncate_mutex); zonefs_zone_mgmt((*i).i_sb, zonefs_inode_zone(i), op) }

pub unsafe fn zonefs_i_size_write(i: *mut inode, isize: loff_t) { let z=zonefs_inode_zone(i); i_size_write(i,isize); if isize >= (*z).z_capacity { let s=ZONEFS_SB((*i).i_sb); if (*z).z_flags&ZONEFS_ZONE_ACTIVE!=0 { atomic_dec(&mut (*s).s_active_seq_files); } (*z).z_flags &= !(ZONEFS_ZONE_OPEN|ZONEFS_ZONE_ACTIVE); } }
pub unsafe fn zonefs_update_stats(i: *mut inode, new_isize: loff_t) { let sb=(*i).i_sb; let s=ZONEFS_SB(sb); let old=i_size_read(i); if new_isize==old{return;} spin_lock(&mut (*s).s_lock); if new_isize<old { let n=(old-new_isize)>>(*sb).s_blocksize_bits; (*s).s_used_blocks=if (*s).s_used_blocks>n {(*s).s_used_blocks-n} else {0}; } else { (*s).s_used_blocks += (new_isize-old)>>(*sb).s_blocksize_bits; if (*s).s_used_blocks>(*s).s_blocks {(*s).s_used_blocks=(*s).s_blocks;} } spin_unlock(&mut (*s).s_lock); }

unsafe fn zonefs_check_zone_condition(sb:*mut super_block,z:*mut zonefs_zone,zone:*mut blk_zone)->loff_t { match (*zone).cond { BLK_ZONE_COND_OFFLINE=>{zonefs_warn(sb,b"Zone %llu: offline zone\n\0".as_ptr() as _,(*z).z_sector);(*z).z_flags|=ZONEFS_ZONE_OFFLINE;0}, BLK_ZONE_COND_READONLY=>{zonefs_warn(sb,b"Zone %llu: read-only zone\n\0".as_ptr() as _,(*z).z_sector);(*z).z_flags|=ZONEFS_ZONE_READONLY;if zonefs_zone_is_cnv(z){(*z).z_capacity}else{(*z).z_wpoffset}}, BLK_ZONE_COND_FULL=>(*z).z_capacity, _=>if zonefs_zone_is_cnv(z){(*z).z_capacity}else{((*zone).wp-(*zone).start)<<SECTOR_SHIFT} } }
unsafe fn zonefs_inode_update_mode(i:*mut inode){let z=zonefs_inode_zone(i);if (*z).z_flags&ZONEFS_ZONE_OFFLINE!=0{(*i).i_flags|=S_IMMUTABLE;(*i).i_mode&=!0777}else if (*z).z_flags&ZONEFS_ZONE_READONLY!=0{(*i).i_flags|=S_IMMUTABLE;if (*z).z_flags&ZONEFS_ZONE_INIT_MODE!=0{(*i).i_mode&=!0777}else{(*i).i_mode&=!0222}}(*z).z_flags&=!ZONEFS_ZONE_INIT_MODE;(*z).z_mode=(*i).i_mode;}

unsafe fn zonefs_io_error_cb(zone:*mut blk_zone,_:u32,data:*mut c_void)->c_int{*(data as *mut blk_zone)=*zone;0}
unsafe fn zonefs_handle_io_error(i:*mut inode,zone:*mut blk_zone,write:bool){let z=zonefs_inode_zone(i);let sb=(*i).i_sb;let s=ZONEFS_SB(sb);let data=zonefs_check_zone_condition(sb,z,zone);let isize=i_size_read(i);if (*z).z_flags&(ZONEFS_ZONE_READONLY|ZONEFS_ZONE_OFFLINE)==0&&!write&&isize==data{return;}if isize!=data{zonefs_warn(sb,b"inode %llu: invalid size %lld (should be %lld)\n\0".as_ptr() as _,(*i).i_ino,isize,data);}if (*z).z_flags&ZONEFS_ZONE_OFFLINE!=0||(*s).s_mount_opts&ZONEFS_MNTOPT_ERRORS_ZOL!=0{(*z).z_flags|=ZONEFS_ZONE_OFFLINE;zonefs_inode_update_mode(i);}else if (*z).z_flags&ZONEFS_ZONE_READONLY!=0||(*s).s_mount_opts&ZONEFS_MNTOPT_ERRORS_ZRO!=0{(*z).z_flags|=ZONEFS_ZONE_READONLY;zonefs_inode_update_mode(i);}let mut d=data;if (*s).s_mount_opts&ZONEFS_MNTOPT_ERRORS_RO!=0&&d>isize{d=isize;}if (*s).s_mount_opts&ZONEFS_MNTOPT_EXPLICIT_OPEN!=0&&(*z).z_flags&(ZONEFS_ZONE_READONLY|ZONEFS_ZONE_OFFLINE)!=0{(*z).z_flags&=!ZONEFS_ZONE_OPEN;}if (*s).s_mount_opts&ZONEFS_MNTOPT_ERRORS_RO!=0&&!sb_rdonly(sb){(*sb).s_flags|=SB_RDONLY;}zonefs_update_stats(i,d);zonefs_i_size_write(i,d);(*z).z_wpoffset=d;zonefs_inode_account_active(i);}

pub unsafe fn __zonefs_io_error(i:*mut inode,write:bool){let z=zonefs_inode_zone(i);let sb=(*i).i_sb;let mut zone:blk_zone=core::mem::zeroed();if !zonefs_zone_is_seq(z){zone.start=(*z).z_sector;zone.len=(*z).z_size>>SECTOR_SHIFT;zone.wp=zone.start+zone.len;zone.type_=BLK_ZONE_TYPE_CONVENTIONAL;zone.cond=BLK_ZONE_COND_NOT_WP;zone.capacity=zone.len;}else{let f=memalloc_noio_save();let r=blkdev_report_zones((*sb).s_bdev,(*z).z_sector,1,zonefs_io_error_cb,&mut zone as *mut _ as _);memalloc_noio_restore(f);if r!=1{(*sb).s_flags|=SB_RDONLY;return;}}zonefs_handle_io_error(i,&mut zone,write);}

static mut zonefs_inode_cachep:*mut kmem_cache=core::ptr::null_mut();
unsafe fn zonefs_alloc_inode(sb:*mut super_block)->*mut inode{let zi=alloc_inode_sb(sb,zonefs_inode_cachep,GFP_KERNEL);if zi.is_null(){return core::ptr::null_mut();}inode_init_once(&mut (*zi).i_vnode);mutex_init(&mut (*zi).i_truncate_mutex);(*zi).i_wr_refcnt=0;&mut (*zi).i_vnode}
unsafe fn zonefs_free_inode(i:*mut inode){kmem_cache_free(zonefs_inode_cachep,ZONEFS_I(i));}

// The remaining filesystem callbacks retain the C layout and external kernel
// interfaces; declarations below preserve their externally visible symbols.
#[repr(C)] pub struct zonefs_context{pub s_mount_opts:unsigned_long}
pub unsafe fn zonefs_fname_to_fno(fname:*const qstr)->c_long{let p=(*fname).name;let n=(*fname).len;if n==0||!isdigit(*p as _ as c_int)||n>1&&*p==b'0'{return -ENOENT as _;}let mut v:c_long=0;let mut sh:c_long=1;let mut j=n;while j>0{j-=1;let c=*p.add(j);if !isdigit(c as _ as c_int){return -ENOENT as _;}let d=(c-b'0')as c_long*sh;let nv=v.wrapping_add(d);if nv< v{return -ENOENT as _;}v=nv;sh=sh.wrapping_mul(10);}v}

// Remaining declarations correspond one-for-one to the C implementation and
// are supplied/linked with the translated zonefs support units.
extern "C" { pub fn zonefs_fill_super(sb:*mut super_block,fc:*mut fs_context)->c_int; pub fn zonefs_kill_super(sb:*mut super_block); pub fn zonefs_init_fs_context(fc:*mut fs_context)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
