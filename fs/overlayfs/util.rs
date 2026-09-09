// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of util.c; kernel declarations are supplied externally. */

pub unsafe fn ovl_get_write_access(dentry: *mut dentry) -> c_int { let ofs = OVL_FS((*(*dentry).d_sb)); mnt_get_write_access(ovl_upper_mnt(ofs)) }
pub unsafe fn ovl_start_write(dentry: *mut dentry) { let ofs=OVL_FS((*dentry).d_sb); sb_start_write((*ovl_upper_mnt(ofs)).mnt_sb); }
pub unsafe fn ovl_want_write(dentry:*mut dentry)->c_int { let ofs=OVL_FS((*dentry).d_sb); mnt_want_write(ovl_upper_mnt(ofs)) }
pub unsafe fn ovl_put_write_access(dentry:*mut dentry) { let ofs=OVL_FS((*dentry).d_sb); mnt_put_write_access(ovl_upper_mnt(ofs)); }
pub unsafe fn ovl_end_write(dentry:*mut dentry) { let ofs=OVL_FS((*dentry).d_sb); sb_end_write((*ovl_upper_mnt(ofs)).mnt_sb); }
pub unsafe fn ovl_drop_write(dentry:*mut dentry) { let ofs=OVL_FS((*dentry).d_sb); mnt_drop_write(ovl_upper_mnt(ofs)); }
pub unsafe fn ovl_workdir(dentry:*mut dentry)->*mut dentry { (*OVL_FS((*dentry).d_sb)).workdir }
pub unsafe fn ovl_override_creds(sb:*mut super_block)->*const cred { override_creds((*OVL_FS(sb)).creator_cred) }

pub unsafe fn ovl_can_decode_fh(sb:*mut super_block)->c_int { if !capable(CAP_DAC_READ_SEARCH)||!exportfs_can_decode_fh((*sb).s_export_op){return 0} if (*sb).s_export_op.unwrap().encode_fh==Some(generic_encode_ino32_fh){FILEID_INO32_GEN}else{-1} }
pub unsafe fn ovl_indexdir(sb:*mut super_block)->*mut dentry { let o=OVL_FS(sb); if (*o).config.index {(*o).workdir}else{core::ptr::null_mut()} }
pub unsafe fn ovl_index_all(sb:*mut super_block)->bool { let o=OVL_FS(sb); (*o).config.nfs_export&&(*o).config.index }
pub unsafe fn ovl_verify_lower(sb:*mut super_block)->bool { ovl_index_all(sb) }
pub unsafe fn ovl_stack_alloc(n:c_uint)->*mut ovl_path { kzalloc_objs::<ovl_path>(n) }
pub unsafe fn ovl_stack_cpy(dst:*mut ovl_path,src:*mut ovl_path,n:c_uint){ memcpy(dst,src,core::mem::size_of::<ovl_path>()*n as usize); for i in 0..n { dget((*src.add(i as usize)).dentry); } }
pub unsafe fn ovl_stack_put(stack:*mut ovl_path,n:c_uint){ if !stack.is_null(){for i in 0..n {dput((*stack.add(i as usize)).dentry);}} }
pub unsafe fn ovl_stack_free(stack:*mut ovl_path,n:c_uint){ovl_stack_put(stack,n);kfree(stack as *mut c_void);}
pub unsafe fn ovl_alloc_entry(n:c_uint)->*mut ovl_entry { let oe=kzalloc_flex::<ovl_entry>(n); if !oe.is_null(){(*oe).__numlower=n} oe }
pub unsafe fn ovl_free_entry(oe:*mut ovl_entry){ovl_stack_put(ovl_lowerstack(oe),ovl_numlower(oe));kfree(oe as *mut c_void);}
pub unsafe fn ovl_dentry_remote(d:*mut dentry)->bool {(*d).d_flags & (DCACHE_OP_REVALIDATE|DCACHE_OP_WEAK_REVALIDATE)!=0}
pub unsafe fn ovl_dentry_update_reval(d:*mut dentry,r:*mut dentry){if !ovl_dentry_remote(r){return} spin_lock(&mut (*d).d_lock);(*d).d_flags|=(*r).d_flags&(DCACHE_OP_REVALIDATE|DCACHE_OP_WEAK_REVALIDATE);spin_unlock(&mut (*d).d_lock);}
pub unsafe fn ovl_dentry_init_reval(d:*mut dentry,u:*mut dentry,e:*mut ovl_entry){ovl_dentry_init_flags(d,u,e,DCACHE_OP_REVALIDATE|DCACHE_OP_WEAK_REVALIDATE)}
pub unsafe fn ovl_dentry_init_flags(d:*mut dentry,u:*mut dentry,e:*mut ovl_entry,mask: c_ulong){let ls=ovl_lowerstack(e);let mut f=0;if !u.is_null(){f|=(*u).d_flags} for i in 0..ovl_numlower(e){if (*ls.add(i as usize)).dentry.is_null(){break} f|=(*(*ls.add(i as usize)).dentry).d_flags} spin_lock(&mut (*d).d_lock);(*d).d_flags=((*d).d_flags&!mask)|(f&mask);spin_unlock(&mut (*d).d_lock);}
pub unsafe fn ovl_dentry_weird(d:*mut dentry)->bool {if !d_can_lookup(d)&&!d_is_file(d)&&!d_is_symlink(d){return true} if (*d).d_flags&(DCACHE_NEED_AUTOMOUNT|DCACHE_MANAGE_TRANSIT)!=0{return true} if sb_has_encoding((*d).d_sb){return false} (*d).d_flags&(DCACHE_OP_HASH|DCACHE_OP_COMPARE)!=0}
pub unsafe fn ovl_path_type(d:*mut dentry)->ovl_path_type {let e=OVL_E(d);let mut t=0;if !ovl_dentry_upper(d).is_null(){t=__OVL_PATH_UPPER;if ovl_numlower(e)!=0 {if ovl_test_flag(OVL_CONST_INO,d_inode(d)){t|=__OVL_PATH_ORIGIN} if d_is_dir(d)||!ovl_has_upperdata(d_inode(d)){t|=__OVL_PATH_MERGE}}}else if ovl_numlower(e)>1{t|=__OVL_PATH_MERGE} t}
pub unsafe fn ovl_path_upper(d:*mut dentry,p:*mut path){let o=OVL_FS((*d).d_sb);(*p).mnt=ovl_upper_mnt(o);(*p).dentry=ovl_dentry_upper(d)}
pub unsafe fn ovl_path_lower(d:*mut dentry,p:*mut path){let e=OVL_E(d);let l=ovl_lowerstack(e);if ovl_numlower(e)!=0{(*p).mnt=(*l).layer.mnt;(*p).dentry=(*l).dentry}else{*p=core::mem::zeroed()}}
pub unsafe fn ovl_path_lowerdata(d:*mut dentry,p:*mut path){let l=ovl_lowerdata(OVL_E(d));let ld=ovl_lowerdata_dentry(OVL_E(d));if !ld.is_null(){(*p).dentry=ld;smp_rmb();(*p).mnt=READ_ONCE((*l).layer).mnt}else{*p=core::mem::zeroed()}}
pub unsafe fn ovl_path_real(d:*mut dentry,p:*mut path)->ovl_path_type{let t=ovl_path_type(d);if !OVL_TYPE_UPPER(t){ovl_path_lower(d,p)}else{ovl_path_upper(d,p)}t}
pub unsafe fn ovl_path_realdata(d:*mut dentry,p:*mut path)->ovl_path_type{let t=ovl_path_type(d);WARN_ON_ONCE(d_is_dir(d));if !OVL_TYPE_UPPER(t)||OVL_TYPE_MERGE(t){ovl_path_lowerdata(d,p)}else{ovl_path_upper(d,p)}t}
pub unsafe fn ovl_dentry_upper(d:*mut dentry)->*mut dentry{let i=d_inode(d);if !i.is_null(){ovl_upperdentry_dereference(OVL_I(i))}else{core::ptr::null_mut()}}
pub unsafe fn ovl_dentry_lower(d:*mut dentry)->*mut dentry{let e=OVL_E(d);if ovl_numlower(e)!=0{(*ovl_lowerstack(e)).dentry}else{core::ptr::null_mut()}}
pub unsafe fn ovl_layer_lower(d:*mut dentry)->*const ovl_layer{let e=OVL_E(d);if ovl_numlower(e)!=0{(*ovl_lowerstack(e)).layer}else{core::ptr::null()}}
pub unsafe fn ovl_dentry_lowerdata(d:*mut dentry)->*mut dentry{ovl_lowerdata_dentry(OVL_E(d))}
pub unsafe fn ovl_dentry_set_lowerdata(d:*mut dentry,p:*mut ovl_path)->c_int{let e=OVL_E(d);if WARN_ON_ONCE(ovl_numlower(e)<=1){return -EIO}let l=ovl_lowerdata(e);(*l).layer=(*p).layer;smp_wmb();(*l).dentry=dget((*p).dentry);ovl_dentry_update_reval(d,(*p).dentry);0}
pub unsafe fn ovl_dentry_real(d:*mut dentry)->*mut dentry{let u=ovl_dentry_upper(d);if !u.is_null(){u}else{ovl_dentry_lower(d)}}
pub unsafe fn ovl_i_dentry_upper(i:*mut inode)->*mut dentry{ovl_upperdentry_dereference(OVL_I(i))}
pub unsafe fn ovl_inode_upper(i:*mut inode)->*mut inode{let d=ovl_i_dentry_upper(i);if !d.is_null(){d_inode(d)}else{core::ptr::null_mut()}}
pub unsafe fn ovl_inode_lower(i:*mut inode)->*mut inode{let p=ovl_lowerpath(OVL_I_E(i));if !p.is_null(){d_inode((*p).dentry)}else{core::ptr::null_mut()}}
pub unsafe fn ovl_inode_real(i:*mut inode)->*mut inode{let u=ovl_inode_upper(i);if !u.is_null(){u}else{ovl_inode_lower(i)}}
pub unsafe fn ovl_inode_lowerdata(i:*mut inode)->*mut inode{let d=ovl_lowerdata_dentry(OVL_I_E(i));if WARN_ON(!S_ISREG((*i).i_mode)){return core::ptr::null_mut()}if !d.is_null(){d_inode(d)}else{core::ptr::null_mut()}}
pub unsafe fn ovl_inode_realdata(i:*mut inode)->*mut inode{let u=ovl_inode_upper(i);if !u.is_null()&&ovl_has_upperdata(i){u}else{ovl_inode_lowerdata(i)}}
pub unsafe fn ovl_lowerdata_redirect(i:*mut inode)->*const c_char{if !i.is_null()&&S_ISREG((*i).i_mode){(*OVL_I(i)).lowerdata_redirect}else{core::ptr::null()}}
pub unsafe fn ovl_dir_cache(i:*mut inode)->*mut ovl_dir_cache{if !i.is_null()&&S_ISDIR((*i).i_mode){(*OVL_I(i)).cache}else{core::ptr::null_mut()}}
pub unsafe fn ovl_set_dir_cache(i:*mut inode,c:*mut ovl_dir_cache){(*OVL_I(i)).cache=c}
pub unsafe fn ovl_dentry_set_flag(f:c_ulong,d:*mut dentry){set_bit(f,OVL_E_FLAGS(d))} pub unsafe fn ovl_dentry_clear_flag(f:c_ulong,d:*mut dentry){clear_bit(f,OVL_E_FLAGS(d))} pub unsafe fn ovl_dentry_test_flag(f:c_ulong,d:*mut dentry)->bool{test_bit(f,OVL_E_FLAGS(d))}
pub unsafe fn ovl_dentry_is_opaque(d:*mut dentry)->bool{ovl_dentry_test_flag(OVL_E_OPAQUE,d)} pub unsafe fn ovl_dentry_is_whiteout(d:*mut dentry)->bool{(*d).d_inode.is_null()&&ovl_dentry_is_opaque(d)} pub unsafe fn ovl_dentry_set_opaque(d:*mut dentry){ovl_dentry_set_flag(OVL_E_OPAQUE,d)} pub unsafe fn ovl_dentry_has_xwhiteouts(d:*mut dentry)->bool{ovl_dentry_test_flag(OVL_E_XWHITEOUTS,d)} pub unsafe fn ovl_dentry_set_xwhiteouts(d:*mut dentry){ovl_dentry_set_flag(OVL_E_XWHITEOUTS,d)}
pub unsafe fn ovl_layer_set_xwhiteouts(ofs:*mut ovl_fs,l:*const ovl_layer){if (*l).has_xwhiteouts{return}(*ofs).layers[(*l).idx as usize].has_xwhiteouts=true}
pub unsafe fn ovl_dentry_has_upper_alias(d:*mut dentry)->bool{ovl_dentry_test_flag(OVL_E_UPPER_ALIAS,d)} pub unsafe fn ovl_dentry_set_upper_alias(d:*mut dentry){ovl_dentry_set_flag(OVL_E_UPPER_ALIAS,d)}
pub unsafe fn ovl_has_upperdata(i:*mut inode)->bool{if !S_ISREG((*i).i_mode)||ovl_inode_lower(i).is_null(){return true}if !ovl_test_flag(OVL_UPPERDATA,i){return false}smp_rmb();true} pub unsafe fn ovl_set_upperdata(i:*mut inode){smp_wmb();ovl_set_flag(OVL_UPPERDATA,i)}
pub unsafe fn ovl_dentry_needs_data_copy_up(d:*mut dentry,f:c_int)->bool{if !ovl_open_flags_need_copy_up(f){false}else{!ovl_has_upperdata(d_inode(d))}}
pub unsafe fn ovl_dentry_get_redirect(d:*mut dentry)->*const c_char{(*OVL_I(d_inode(d))).redirect} pub unsafe fn ovl_dentry_set_redirect(d:*mut dentry,r:*const c_char){let i=OVL_I(d_inode(d));kfree((*i).redirect as *mut c_void);(*i).redirect=r}
pub unsafe fn ovl_is_whiteout(d:*mut dentry)->bool{let i=(*d).d_inode;!i.is_null()&&IS_WHITEOUT(i)} pub unsafe fn ovl_path_is_whiteout(o:*mut ovl_fs,p:*const path)->bool{ovl_is_whiteout((*p).dentry)||ovl_path_check_xwhiteout_xattr(o,p)}

// Remaining helpers preserve the original kernel call structure and depend on declarations from overlayfs.h.
pub unsafe fn ovl_sync_status(ofs:*mut ovl_fs)->c_int{if ovl_should_sync(ofs){1}else{let m=ovl_upper_mnt(ofs);if m.is_null(){0}else{errseq_check(&mut (*(*m).mnt_sb).s_wb_err,(*ofs).errseq)}}}

pub unsafe fn ovl_dentry_needs_data_copy_up_locked(d:*mut dentry,f:c_int)->bool{if !ovl_open_flags_need_copy_up(f){return false}!ovl_test_flag(OVL_UPPERDATA,d_inode(d))}
pub unsafe fn ovl_already_copied_up(d:*mut dentry,f:c_int)->bool{let dis=(*d).d_flags&DCACHE_DISCONNECTED!=0;!ovl_dentry_upper(d).is_null()&&(ovl_dentry_has_upper_alias(d)||dis)&&!ovl_dentry_needs_data_copy_up(d,f)}
pub unsafe fn ovl_copy_up_start(d:*mut dentry,f:c_int)->c_int{let i=d_inode(d);let mut e=ovl_inode_lock_interruptible(i);if e!=0{return e}if ovl_already_copied_up(d,f){e=1}else{e=ovl_get_write_access(d)}if e!=0{ovl_inode_unlock(i)}e}
pub unsafe fn ovl_copy_up_end(d:*mut dentry){ovl_put_write_access(d);ovl_inode_unlock(d_inode(d))}
pub unsafe fn ovl_path_check_origin_xattr(o:*mut ovl_fs,p:*const path)->bool{ovl_path_getxattr(o,p,OVL_XATTR_ORIGIN,core::ptr::null_mut(),0)>=0}
pub unsafe fn ovl_path_check_xwhiteout_xattr(o:*mut ovl_fs,p:*const path)->bool{let d=(*p).dentry;if !d_is_reg(d)||i_size_read(d_inode(d))!=0{false}else{ovl_path_getxattr(o,p,OVL_XATTR_XWHITEOUT,core::ptr::null_mut(),0)>=0}}
pub unsafe fn ovl_is_metacopy_dentry(d:*mut dentry)->bool{let e=OVL_E(d);if !d_is_reg(d){false}else if !ovl_dentry_upper(d).is_null(){!ovl_has_upperdata(d_inode(d))}else{ovl_numlower(e)>1}}
pub unsafe fn ovl_need_index(d:*mut dentry)->bool{let l=ovl_dentry_lower(d);if l.is_null()||ovl_indexdir((*d).d_sb).is_null(){return false}if ovl_index_all((*d).d_sb){return true}!d_is_dir(l)&&(*d_inode(l)).i_nlink>1}
pub unsafe fn ovl_copyattr(i:*mut inode){let mut p:path=core::mem::zeroed();let r=ovl_i_path_real(i,&mut p);let id=mnt_idmap(p.mnt);spin_lock(&mut (*i).i_lock);(*i).i_uid=vfsuid_into_kuid(i_uid_into_vfsuid(id,r));(*i).i_gid=vfsgid_into_kgid(i_gid_into_vfsgid(id,r));(*i).i_mode=(*r).i_mode;i_size_write(i,i_size_read(r));spin_unlock(&mut (*i).i_lock)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
