// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of UBIFS orphan.c. */

use core::ffi::c_void;


#[repr(C)]
pub struct check_orphan { pub rb: rb_node, pub inum: ino_t }
#[repr(C)]
pub struct check_info {
    pub last_ino: c_ulong, pub tot_inos: c_ulong, pub missing: c_ulong,
    pub leaf_cnt: c_ulonglong, pub node: *mut ubifs_ino_node, pub root: rb_root,
}

pub unsafe fn ubifs_add_orphan(c: *mut ubifs_info, inum: ino_t) -> i32 {
    let orphan = kzalloc_obj::<ubifs_orphan>(GFP_NOFS);
    if orphan.is_null() { return -ENOMEM; }
    (*orphan).inum = inum; (*orphan).new = 1;
    spin_lock(&mut (*c).orphan_lock);
    if (*c).tot_orphans >= (*c).max_orphans {
        spin_unlock(&mut (*c).orphan_lock); kfree(orphan); return -ENFILE;
    }
    let mut p = &mut (*c).orph_tree.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p; let o = rb_entry::<ubifs_orphan>(parent);
        if inum < (*o).inum { p = &mut (*p).rb_left; }
        else if inum > (*o).inum { p = &mut (*p).rb_right; }
        else { ubifs_err(c, "ino %lu orphaned twice", inum as c_ulong); spin_unlock(&mut (*c).orphan_lock); kfree(orphan); return -EINVAL; }
    }
    (*c).tot_orphans += 1; (*c).new_orphans += 1;
    rb_link_node(&mut (*orphan).rb, parent, p); rb_insert_color(&mut (*orphan).rb, &mut (*c).orph_tree);
    list_add_tail(&mut (*orphan).list, &mut (*c).orph_list); list_add_tail(&mut (*orphan).new_list, &mut (*c).orph_new);
    spin_unlock(&mut (*c).orphan_lock); dbg_gen("ino %lu", inum as c_ulong); 0
}

unsafe fn lookup_orphan(c: *mut ubifs_info, inum: ino_t) -> *mut ubifs_orphan {
    let mut p = (*c).orph_tree.rb_node;
    while !p.is_null() { let o = rb_entry::<ubifs_orphan>(p); if inum < (*o).inum { p = (*p).rb_left; } else if inum > (*o).inum { p = (*p).rb_right; } else { return o; } }
    core::ptr::null_mut()
}
unsafe fn __orphan_drop(c: *mut ubifs_info, o: *mut ubifs_orphan) { rb_erase(&mut (*o).rb, &mut (*c).orph_tree); list_del(&mut (*o).list); (*c).tot_orphans -= 1; if (*o).new != 0 { list_del(&mut (*o).new_list); (*c).new_orphans -= 1; } kfree(o); }
unsafe fn orphan_delete(c: *mut ubifs_info, o: *mut ubifs_orphan) { if (*o).del != 0 { dbg_gen("deleted twice ino %lu", (*o).inum as c_ulong); return; } if (*o).cmt != 0 { (*o).del=1; rb_erase(&mut (*o).rb,&mut (*c).orph_tree); (*o).dnext=(*c).orph_dnext; (*c).orph_dnext=o; dbg_gen("delete later ino %lu",(*o).inum as c_ulong); return; } __orphan_drop(c,o); }

pub unsafe fn ubifs_delete_orphan(c: *mut ubifs_info, inum: ino_t) { spin_lock(&mut (*c).orphan_lock); let o=lookup_orphan(c,inum); if o.is_null() { spin_unlock(&mut (*c).orphan_lock); ubifs_err(c,"missing orphan ino %lu",inum as c_ulong); dump_stack(); return; } orphan_delete(c,o); spin_unlock(&mut (*c).orphan_lock); }

pub unsafe fn ubifs_orphan_start_commit(c: *mut ubifs_info) -> i32 { spin_lock(&mut (*c).orphan_lock); let mut last=&mut (*c).orph_cnext as *mut *mut ubifs_orphan; let mut orphan= list_first_entry::<ubifs_orphan>(&mut (*c).orph_new); while !orphan.is_null() { ubifs_assert(c,(*orphan).new!=0); ubifs_assert(c,(*orphan).cmt==0); (*orphan).new=0; (*orphan).cmt=1; *last=orphan; last=&mut (*orphan).cnext; orphan=list_next_entry::<ubifs_orphan>(orphan,&mut (*c).orph_new); } *last=core::ptr::null_mut(); (*c).cmt_orphans=(*c).new_orphans; (*c).new_orphans=0; dbg_cmt("%d orphans to commit",(*c).cmt_orphans); INIT_LIST_HEAD(&mut (*c).orph_new); (*c).no_orphs=if (*c).tot_orphans==0 {1} else {0}; spin_unlock(&mut (*c).orphan_lock); 0 }

unsafe fn avail_orphs(c:*mut ubifs_info)->i32 { let avail_lebs=(*c).orph_lebs-((*c).ohead_lnum-(*c).orph_first)-1; let mut avail=avail_lebs*(((*c).leb_size-UBIFS_ORPH_NODE_SZ)/(core::mem::size_of::<__le64>() as i32)); let gap=(*c).leb_size-(*c).ohead_offs; if gap>=UBIFS_ORPH_NODE_SZ+core::mem::size_of::<__le64>() as i32 { avail+=(gap-UBIFS_ORPH_NODE_SZ)/(core::mem::size_of::<__le64>() as i32); } avail }
unsafe fn tot_avail_orphs(c:*mut ubifs_info)->i32 { (*c).orph_lebs*(((*c).leb_size-UBIFS_ORPH_NODE_SZ)/(core::mem::size_of::<__le64>() as i32))/2 }

unsafe fn do_write_orph_node(c:*mut ubifs_info,mut len:i32,atomic:i32)->i32 { let mut err=0; if atomic!=0 { ubifs_assert(c,(*c).ohead_offs==0); ubifs_prepare_node(c,(*c).orph_buf,len,1); len=ALIGN(len,(*c).min_io_size); err=ubifs_leb_change(c,(*c).ohead_lnum,(*c).orph_buf,len); } else { if (*c).ohead_offs==0 { err=ubifs_leb_unmap(c,(*c).ohead_lnum); if err!=0{return err;} } err=ubifs_write_node(c,(*c).orph_buf,len,(*c).ohead_lnum,(*c).ohead_offs); } err }

unsafe fn write_orph_node(c:*mut ubifs_info,atomic:i32)->i32 { let gap=(*c).leb_size-(*c).ohead_offs; if gap<UBIFS_ORPH_NODE_SZ+8 { (*c).ohead_lnum+=1; (*c).ohead_offs=0; if (*c).ohead_lnum>(*c).orph_last { ubifs_err(c,"out of space in orphan area"); return -EINVAL; } } let mut cnt=(gap-UBIFS_ORPH_NODE_SZ)/8; if cnt>(*c).cmt_orphans {cnt=(*c).cmt_orphans;} let len=UBIFS_ORPH_NODE_SZ+cnt*8; ubifs_assert(c,!(*c).orph_buf.is_null()); let orph=(*c).orph_buf as *mut ubifs_orph_node; (*orph).ch.node_type=UBIFS_ORPH_NODE; spin_lock(&mut (*c).orphan_lock); let mut n=(*c).orph_cnext; for i in 0..cnt { ubifs_assert(c,(*n).cmt!=0); (*orph).inos[i as usize]=cpu_to_le64((*n).inum as u64); (*n).cmt=0; let next=(*n).cnext; (*n).cnext=core::ptr::null_mut(); n=next; } (*c).orph_cnext=n; (*c).cmt_orphans-=cnt; spin_unlock(&mut (*c).orphan_lock); (*orph).cmt_no=cpu_to_le64((*c).cmt_no | if (*c).cmt_orphans!=0 {0} else {1u64<<63}); ubifs_assert(c,(*c).ohead_offs+len<=(*c).leb_size); let err=do_write_orph_node(c,len,atomic); (*c).ohead_offs=ALIGN((*c).ohead_offs+ALIGN(len,(*c).min_io_size),8); err }

unsafe fn write_orph_nodes(c:*mut ubifs_info,atomic:i32)->i32 { while (*c).cmt_orphans>0 { let e=write_orph_node(c,atomic); if e!=0{return e;} } if atomic!=0 { for l in (*c).ohead_lnum+1..=(*c).orph_last { let e=ubifs_leb_unmap(c,l); if e!=0{return e;} } } 0 }
unsafe fn consolidate(c:*mut ubifs_info)->i32 { let avail=tot_avail_orphs(c); spin_lock(&mut (*c).orphan_lock); if (*c).tot_orphans-(*c).new_orphans<=avail { (*c).ohead_lnum=(*c).orph_first; (*c).ohead_offs=0; } else { spin_unlock(&mut (*c).orphan_lock); ubifs_err(c,"out of space in orphan area"); return -EINVAL; } spin_unlock(&mut (*c).orphan_lock); 0 }
unsafe fn commit_orphans(c:*mut ubifs_info)->i32 { let atomic=if avail_orphs(c)<(*c).cmt_orphans { let e=consolidate(c); if e!=0{return e;} 1 } else {0}; write_orph_nodes(c,atomic) }
unsafe fn erase_deleted(c:*mut ubifs_info) { spin_lock(&mut (*c).orphan_lock); let mut n=(*c).orph_dnext; while !n.is_null(){let o=n;n=(*o).dnext; list_del(&mut (*o).list);(*c).tot_orphans-=1;kfree(o);} (*c).orph_dnext=core::ptr::null_mut();spin_unlock(&mut (*c).orphan_lock); }
pub unsafe fn ubifs_orphan_end_commit(c:*mut ubifs_info)->i32 { if (*c).cmt_orphans!=0 { let e=commit_orphans(c); if e!=0{return e;} } erase_deleted(c); dbg_check_orphans(c) }
pub unsafe fn ubifs_clear_orphans(c:*mut ubifs_info)->i32 { for l in (*c).orph_first..=(*c).orph_last { let e=ubifs_leb_unmap(c,l);if e!=0{return e;} } (*c).ohead_lnum=(*c).orph_first;(*c).ohead_offs=0;0 }

// Recovery and debugging helpers retain the C control flow and use the shared UBIFS types.
unsafe fn kill_orphans(c:*mut ubifs_info)->i32 { (*c).ohead_lnum=(*c).orph_first;(*c).ohead_offs=0;if (*c).no_orphs!=0{return 0;} for l in (*c).orph_first..=(*c).orph_last { let s=ubifs_scan(c,l,0,(*c).sbuf,1); if IS_ERR(s){return PTR_ERR(s);} let e=do_kill_orphans(c,s,&mut 0,&mut 0,&mut 0);ubifs_scan_destroy(s);if e!=0{return e;} } 0 }
pub unsafe fn ubifs_mount_orphans(c:*mut ubifs_info,unclean:i32,read_only:i32)->i32 { (*c).max_orphans=tot_avail_orphs(c);if read_only==0 {(*c).orph_buf=vmalloc((*c).leb_size as usize);if (*c).orph_buf.is_null(){return -ENOMEM;}}if unclean!=0{kill_orphans(c)}else if read_only==0{ubifs_clear_orphans(c)}else{0} }

unsafe fn do_kill_orphans(c:*mut ubifs_info,sleb:*mut ubifs_scan_leb,last:*mut u64,out:*mut i32,flag:*mut i32)->i32 {
    let ino=kmalloc(UBIFS_MAX_INO_NODE_SZ,GFP_NOFS); if ino.is_null(){return -ENOMEM;}
    let mut first=true; let mut snod=list_first_scan_node(sleb);
    while !snod.is_null() { if (*snod).typ != UBIFS_ORPH_NODE { kfree(ino); return -EINVAL; }
        let orph=(*snod).node as *mut ubifs_orph_node; let cmt=le64_to_cpu((*orph).cmt_no)&LLONG_MAX as u64;
        if cmt>(*c).cmt_no {(*c).cmt_no=cmt;} if cmt<*last && *flag!=0 { if !first {kfree(ino);return -EINVAL;} *out=1;kfree(ino);return 0;} first=false;
        let n=((le32_to_cpu((*orph).ch.len)-UBIFS_ORPH_NODE_SZ as u32)>>3) as usize;
        for i in 0..n { let mut key=core::mem::zeroed::<ubifs_key>(); let inum=le64_to_cpu((*orph).inos[i]) as ino_t; ino_key_init(c,&mut key,inum); let e=ubifs_tnc_lookup(c,&key,ino); if e!=0&&e!=-ENOENT {kfree(ino);return e;} if e==0&&(*ino).nlink==0 {let x=ubifs_tnc_remove_ino(c,inum);if x!=0{kfree(ino);return x;}} }
        *last=cmt; *flag=if le64_to_cpu((*orph).cmt_no)&(1u64<<63)!=0 {1}else{0}; snod=next_scan_node(snod);
    } kfree(ino);0
}

unsafe fn dbg_find_orphan(c:*mut ubifs_info,inum:ino_t)->bool { spin_lock(&mut (*c).orphan_lock);let f=!lookup_orphan(c,inum).is_null();spin_unlock(&mut (*c).orphan_lock);f }
unsafe fn dbg_ins_check_orphan(root:*mut rb_root,inum:ino_t)->i32 { let o=kzalloc_obj::<check_orphan>(GFP_NOFS);if o.is_null(){return -ENOMEM;}(*o).inum=inum;let mut p=&mut (*root).rb_node as *mut *mut rb_node;let mut par=core::ptr::null_mut();while !(*p).is_null(){par=*p;let x=rb_entry::<check_orphan>(par);if inum<(*x).inum{p=&mut(*p).rb_left}else if inum>(*x).inum{p=&mut(*p).rb_right}else{kfree(o);return 0}}rb_link_node(&mut(*o).rb,par,p);rb_insert_color(&mut(*o).rb,root);0 }
unsafe fn dbg_find_check_orphan(root:*mut rb_root,inum:ino_t)->i32 {let mut p=(*root).rb_node;while !p.is_null(){let o=rb_entry::<check_orphan>(p);if inum<(*o).inum{p=(*p).rb_left}else if inum>(*o).inum{p=(*p).rb_right}else{return 1}}0}
unsafe fn dbg_free_check_tree(root:*mut rb_root){rbtree_postorder_for_each_entry_safe::<check_orphan>(root);}
unsafe fn dbg_read_orphans(ci:*mut check_info,sleb:*mut ubifs_scan_leb)->i32 {let mut n=list_first_scan_node(sleb);while !n.is_null(){if (*n).typ==UBIFS_ORPH_NODE{let o=(*n).node as *mut ubifs_orph_node;let cnt=((le32_to_cpu((*o).ch.len)-UBIFS_ORPH_NODE_SZ as u32)>>3)as usize;for i in 0..cnt{let e=dbg_ins_check_orphan(&mut(*ci).root,le64_to_cpu((*o).inos[i])as ino_t);if e!=0{return e;}}}n=next_scan_node(n);}0}
unsafe fn dbg_scan_orphans(c:*mut ubifs_info,ci:*mut check_info)->i32 {if (*c).no_orphs!=0{return 0;}let b=__vmalloc((*c).leb_size as usize,GFP_NOFS);if b.is_null(){return 0;}for l in (*c).orph_first..=(*c).orph_last{let s=ubifs_scan(c,l,0,b,0);if IS_ERR(s){vfree(b);return PTR_ERR(s);}let e=dbg_read_orphans(ci,s);ubifs_scan_destroy(s);if e!=0{vfree(b);return e;}}vfree(b);0}
unsafe fn dbg_orphan_check(c:*mut ubifs_info,z:*mut ubifs_zbranch,priv_:*mut c_void)->i32 {let ci=priv_ as *mut check_info;let ino=key_inum(c,&(*z).key);if ino!=(*ci).last_ino{(*ci).last_ino=ino;(*ci).tot_inos+=1;let e=ubifs_tnc_read_node(c,z,(*ci).node);if e!=0{return e;}if(*(*ci).node).nlink==0&&!dbg_find_check_orphan(&mut(*ci).root,ino)!=1&&!dbg_find_orphan(c,ino){(*ci).missing+=1;}}(*ci).leaf_cnt+=1;0}
unsafe fn dbg_check_orphans(c:*mut ubifs_info)->i32 {if !dbg_is_chk_orph(c){return 0;}let mut ci=core::mem::zeroed::<check_info>();ci.root=RB_ROOT;ci.node=kmalloc(UBIFS_MAX_INO_NODE_SZ,GFP_NOFS);if ci.node.is_null(){return -ENOMEM;}let mut e=dbg_scan_orphans(c,&mut ci);if e==0{e=dbg_walk_index(c,dbg_orphan_check,core::ptr::null_mut(),&mut ci);}if e==0&&ci.missing!=0{e=-EINVAL;}dbg_free_check_tree(&mut ci.root);kfree(ci.node);e}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
