// SPDX-License-Identifier: GPL-2.0
/* Encryption policy functions for per-file encryption support. */

// C dependencies and kernel-provided symbols are intentionally external.

pub unsafe fn fscrypt_policies_equal(policy1: *const union_fscrypt_policy, policy2: *const union_fscrypt_policy) -> bool {
    if (*policy1).version != (*policy2).version { return false; }
    !memcmp(policy1 as *const _, policy2 as *const _, fscrypt_policy_size(policy1))
}

pub unsafe fn fscrypt_policy_to_key_spec(policy: *const union_fscrypt_policy, key_spec: *mut fscrypt_key_specifier) -> i32 {
    match (*policy).version {
        FSCRYPT_POLICY_V1 => { (*key_spec).type_ = FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR; memcpy((*key_spec).u.descriptor.as_mut_ptr() as *mut _, (*policy).v1.master_key_descriptor.as_ptr() as *const _, FSCRYPT_KEY_DESCRIPTOR_SIZE); 0 }
        FSCRYPT_POLICY_V2 => { (*key_spec).type_ = FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER; memcpy((*key_spec).u.identifier.as_mut_ptr() as *mut _, (*policy).v2.master_key_identifier.as_ptr() as *const _, FSCRYPT_KEY_IDENTIFIER_SIZE); 0 }
        _ => { WARN_ON_ONCE(1); -EINVAL }
    }
}

pub unsafe fn fscrypt_get_dummy_policy(sb: *mut super_block) -> *const union_fscrypt_policy {
    if (*(*sb).s_cop).get_dummy_policy.is_none() { return core::ptr::null(); }
    ((*(*sb).s_cop).get_dummy_policy.unwrap())(sb)
}

unsafe fn fscrypt_valid_enc_modes_v1(contents_mode: u32, filenames_mode: u32) -> bool {
    (contents_mode == FSCRYPT_MODE_AES_256_XTS && filenames_mode == FSCRYPT_MODE_AES_256_CTS) ||
    (contents_mode == FSCRYPT_MODE_AES_128_CBC && filenames_mode == FSCRYPT_MODE_AES_128_CTS) ||
    (contents_mode == FSCRYPT_MODE_ADIANTUM && filenames_mode == FSCRYPT_MODE_ADIANTUM)
}
unsafe fn fscrypt_valid_enc_modes_v2(contents_mode: u32, filenames_mode: u32) -> bool {
    (contents_mode == FSCRYPT_MODE_AES_256_XTS && filenames_mode == FSCRYPT_MODE_AES_256_HCTR2) ||
    (contents_mode == FSCRYPT_MODE_SM4_XTS && filenames_mode == FSCRYPT_MODE_SM4_CTS) ||
    fscrypt_valid_enc_modes_v1(contents_mode, filenames_mode)
}

unsafe fn supported_direct_key_modes(inode: *const inode, contents_mode: u32, filenames_mode: u32) -> bool {
    if contents_mode != filenames_mode { fscrypt_warn(inode, c"Direct key flag not allowed with different contents and filenames modes"); return false; }
    let mode = &fscrypt_modes[contents_mode as usize];
    if mode.ivsize < offsetofend::<union_fscrypt_iv>("nonce") { fscrypt_warn(inode, c"Direct key flag not allowed with %s", mode.friendly_name); return false; }
    true
}

unsafe fn supported_iv_ino_lblk_policy(policy: *const fscrypt_policy_v2, inode: *const inode) -> bool {
    let type_ = if (*policy).flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 != 0 { c"IV_INO_LBLK_64" } else { c"IV_INO_LBLK_32" };
    let sb = (*inode).i_sb;
    if (*policy).contents_encryption_mode != FSCRYPT_MODE_AES_256_XTS { fscrypt_warn(inode, c"Can't use %s policy with contents mode other than AES-256-XTS", type_); return false; }
    if (*(*sb).s_cop).has_stable_inodes.is_none() || !((*(*sb).s_cop).has_stable_inodes.unwrap())(sb) { fscrypt_warn(inode, c"Can't use %s policy on filesystem '%s' because it doesn't have stable inode numbers", type_, (*sb).s_id); return false; }
    if !(*(*sb).s_cop).has_32bit_inodes { fscrypt_warn(inode, c"Can't use %s policy on filesystem '%s' because its inode numbers are too long", type_, (*sb).s_id); return false; }
    if fscrypt_max_file_dun_bits(sb, fscrypt_policy_v2_du_bits(policy, inode)) > 32 { fscrypt_warn(inode, c"Can't use %s policy on filesystem '%s' because its maximum file size is too large", type_, (*sb).s_id); return false; }
    if (*policy).flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32 != 0 && (*sb).s_blocksize != PAGE_SIZE { fscrypt_warn(inode, c"Can't use %s policy on filesystem '%s' with block size != PAGE_SIZE", type_, (*sb).s_id); return false; }
    true
}

unsafe fn fscrypt_supported_v1_policy(policy: *const fscrypt_policy_v1, inode: *const inode) -> bool {
    if !fscrypt_valid_enc_modes_v1((*policy).contents_encryption_mode, (*policy).filenames_encryption_mode) { fscrypt_warn(inode, c"Unsupported encryption modes (contents %d, filenames %d)", (*policy).contents_encryption_mode, (*policy).filenames_encryption_mode); return false; }
    if (*policy).flags & !(FSCRYPT_POLICY_FLAGS_PAD_MASK | FSCRYPT_POLICY_FLAG_DIRECT_KEY) != 0 { fscrypt_warn(inode, c"Unsupported encryption flags (0x%02x)", (*policy).flags); return false; }
    if (*policy).flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0 && !supported_direct_key_modes(inode, (*policy).contents_encryption_mode, (*policy).filenames_encryption_mode) { return false; }
    if IS_CASEFOLDED(inode) { fscrypt_warn(inode, c"v1 policies can't be used on casefolded directories"); return false; }
    true
}

unsafe fn fscrypt_supported_v2_policy(policy: *const fscrypt_policy_v2, inode: *const inode) -> bool {
    if !fscrypt_valid_enc_modes_v2((*policy).contents_encryption_mode, (*policy).filenames_encryption_mode) { fscrypt_warn(inode, c"Unsupported encryption modes (contents %d, filenames %d)", (*policy).contents_encryption_mode, (*policy).filenames_encryption_mode); return false; }
    if (*policy).flags & !(FSCRYPT_POLICY_FLAGS_PAD_MASK | FSCRYPT_POLICY_FLAG_DIRECT_KEY | FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 | FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32) != 0 { fscrypt_warn(inode, c"Unsupported encryption flags (0x%02x)", (*policy).flags); return false; }
    let count = ((*policy).flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0) as i32 + ((*policy).flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 != 0) as i32 + ((*policy).flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32 != 0) as i32;
    if count > 1 { fscrypt_warn(inode, c"Mutually exclusive encryption flags (0x%02x)", (*policy).flags); return false; }
    if (*policy).log2_data_unit_size != 0 {
        if !(*(*inode).i_sb).s_cop.supports_subblock_data_units { fscrypt_warn(inode, c"Filesystem does not support configuring crypto data unit size"); return false; }
        if (*policy).log2_data_unit_size > (*inode).i_blkbits || (*policy).log2_data_unit_size < SECTOR_SHIFT { fscrypt_warn(inode, c"Unsupported log2_data_unit_size in encryption policy: %d", (*policy).log2_data_unit_size); return false; }
        if (*policy).log2_data_unit_size != (*inode).i_blkbits && (*policy).flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32 != 0 { fscrypt_warn(inode, c"Sub-block data units not yet supported with IV_INO_LBLK_32"); return false; }
    }
    if (*policy).flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0 && !supported_direct_key_modes(inode, (*policy).contents_encryption_mode, (*policy).filenames_encryption_mode) { return false; }
    if (*policy).flags & (FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 | FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32) != 0 && !supported_iv_ino_lblk_policy(policy, inode) { return false; }
    if memchr_inv((*policy).__reserved.as_ptr() as *const _, 0, core::mem::size_of_val(&(*policy).__reserved)) { fscrypt_warn(inode, c"Reserved bits set in encryption policy"); return false; }
    true
}

pub unsafe fn fscrypt_supported_policy(policy_u: *const union_fscrypt_policy, inode: *const inode) -> bool { match (*policy_u).version { FSCRYPT_POLICY_V1 => fscrypt_supported_v1_policy(&(*policy_u).v1, inode), FSCRYPT_POLICY_V2 => fscrypt_supported_v2_policy(&(*policy_u).v2, inode), _ => false } }

unsafe fn fscrypt_new_context(ctx_u: *mut union_fscrypt_context, policy_u: *const union_fscrypt_policy, nonce: *const u8) -> i32 {
    memset(ctx_u as *mut _, 0, core::mem::size_of::<union_fscrypt_context>());
    match (*policy_u).version {
        FSCRYPT_POLICY_V1 => { let p=&(*policy_u).v1; let c=&mut (*ctx_u).v1; c.version=FSCRYPT_CONTEXT_V1; c.contents_encryption_mode=p.contents_encryption_mode; c.filenames_encryption_mode=p.filenames_encryption_mode; c.flags=p.flags; memcpy(c.master_key_descriptor.as_mut_ptr() as *mut _, p.master_key_descriptor.as_ptr() as *const _, core::mem::size_of_val(&c.master_key_descriptor)); memcpy(c.nonce.as_mut_ptr() as *mut _, nonce as *const _, FSCRYPT_FILE_NONCE_SIZE); core::mem::size_of_val(c) as i32 }
        FSCRYPT_POLICY_V2 => { let p=&(*policy_u).v2; let c=&mut (*ctx_u).v2; c.version=FSCRYPT_CONTEXT_V2; c.contents_encryption_mode=p.contents_encryption_mode; c.filenames_encryption_mode=p.filenames_encryption_mode; c.flags=p.flags; c.log2_data_unit_size=p.log2_data_unit_size; memcpy(c.master_key_identifier.as_mut_ptr() as *mut _, p.master_key_identifier.as_ptr() as *const _, core::mem::size_of_val(&c.master_key_identifier)); memcpy(c.nonce.as_mut_ptr() as *mut _, nonce as *const _, FSCRYPT_FILE_NONCE_SIZE); core::mem::size_of_val(c) as i32 }
        _ => { BUG(); 0 }
    }
}

pub unsafe fn fscrypt_policy_from_context(policy_u: *mut union_fscrypt_policy, ctx_u: *const union_fscrypt_context, ctx_size: i32) -> i32 {
    memset(policy_u as *mut _, 0, core::mem::size_of::<union_fscrypt_policy>());
    if !fscrypt_context_is_valid(ctx_u, ctx_size) { return -EINVAL; }
    match (*ctx_u).version {
        FSCRYPT_CONTEXT_V1 => { let c=&(*ctx_u).v1; let p=&mut (*policy_u).v1; p.version=FSCRYPT_POLICY_V1; p.contents_encryption_mode=c.contents_encryption_mode; p.filenames_encryption_mode=c.filenames_encryption_mode; p.flags=c.flags; memcpy(p.master_key_descriptor.as_mut_ptr() as *mut _, c.master_key_descriptor.as_ptr() as *const _, core::mem::size_of_val(&p.master_key_descriptor)); 0 }
        FSCRYPT_CONTEXT_V2 => { let c=&(*ctx_u).v2; let p=&mut (*policy_u).v2; p.version=FSCRYPT_POLICY_V2; p.contents_encryption_mode=c.contents_encryption_mode; p.filenames_encryption_mode=c.filenames_encryption_mode; p.flags=c.flags; p.log2_data_unit_size=c.log2_data_unit_size; memcpy(p.__reserved.as_mut_ptr() as *mut _, c.__reserved.as_ptr() as *const _, core::mem::size_of_val(&p.__reserved)); memcpy(p.master_key_identifier.as_mut_ptr() as *mut _, c.master_key_identifier.as_ptr() as *const _, core::mem::size_of_val(&p.master_key_identifier)); 0 }
        _ => -EINVAL
    }
}

unsafe fn fscrypt_get_policy(inode: *mut inode, policy: *mut union_fscrypt_policy) -> i32 { let ci=fscrypt_get_inode_info(inode); if !ci.is_null() { *policy=(*ci).ci_policy; return 0; } if !IS_ENCRYPTED(inode) { return -ENODATA; } let mut ctx=core::mem::zeroed::<union_fscrypt_context>(); let ret=((*(*(*inode).i_sb).s_cop).get_context.unwrap())(inode,&mut ctx,core::mem::size_of::<union_fscrypt_context>()); if ret<0 { return if ret==-ERANGE {-EINVAL} else {ret}; } fscrypt_policy_from_context(policy,&ctx,ret) }

unsafe fn set_encryption_policy(inode: *mut inode, policy: *const union_fscrypt_policy) -> i32 { let mut nonce=[0u8;FSCRYPT_FILE_NONCE_SIZE]; let mut ctx=core::mem::zeroed::<union_fscrypt_context>(); if !fscrypt_supported_policy(policy,inode) { return -EINVAL; } match (*policy).version { FSCRYPT_POLICY_V1 => pr_warn_once(c"%s (pid %d) is setting deprecated v1 encryption policy; recommend upgrading to v2.\n", current.comm,current.pid), FSCRYPT_POLICY_V2 => { let err=fscrypt_verify_key_added((*inode).i_sb,(*policy).v2.master_key_identifier.as_ptr()); if err!=0{return err;} if (*policy).v2.flags&FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32!=0 { pr_warn_once(c"%s (pid %d) is setting an IV_INO_LBLK_32 encryption policy.  This should only be used if there are certain hardware limitations.\n",current.comm,current.pid); } }, _=>{WARN_ON_ONCE(1);return -EINVAL;} } get_random_bytes(nonce.as_mut_ptr(),FSCRYPT_FILE_NONCE_SIZE); let ctxsize=fscrypt_new_context(&mut ctx,policy,nonce.as_ptr()); ((*(*(*inode).i_sb).s_cop).set_context.unwrap())(inode,&ctx,ctxsize,core::ptr::null_mut()) }

pub unsafe fn fscrypt_ioctl_set_policy(filp:*mut file,arg:*const core::ffi::c_void)->i32 { let mut policy=core::mem::zeroed::<union_fscrypt_policy>(); let mut existing=core::mem::zeroed::<union_fscrypt_policy>(); let inode=file_inode(filp); let mut size:i32; let mut ret:i32; if get_user(&mut policy.version as *mut _,arg as *const u8)!=0{return -EFAULT;} size=fscrypt_policy_size(&policy); if size<=0{return -EINVAL;} if copy_from_user((&mut policy as *mut _ as *mut u8).add(1),arg as *const u8, (size-1) as usize)!=0{return -EFAULT;} if !inode_owner_or_capable(file_mnt_idmap(filp),inode){return -EACCES;} ret=mnt_want_write_file(filp); if ret!=0{return ret;} inode_lock(inode); ret=fscrypt_get_policy(inode,&mut existing); if ret==-ENODATA { if !S_ISDIR((*inode).i_mode){ret=-ENOTDIR;} else if IS_DEADDIR(inode){ret=-ENOENT;} else if !((*(*(*inode).i_sb).s_cop).empty_dir.unwrap())(inode){ret=-ENOTEMPTY;} else {ret=set_encryption_policy(inode,&policy);} } else if ret==-EINVAL || ret==0 && !fscrypt_policies_equal(&policy,&existing){ret=-EEXIST;} inode_unlock(inode); mnt_drop_write_file(filp); ret }

pub unsafe fn fscrypt_ioctl_get_policy(filp:*mut file,arg:*mut core::ffi::c_void)->i32 { let mut p=core::mem::zeroed::<union_fscrypt_policy>(); let e=fscrypt_get_policy(file_inode(filp),&mut p); if e!=0{return e;} if p.version!=FSCRYPT_POLICY_V1{return -EINVAL;} if copy_to_user(arg,&p,core::mem::size_of_val(&p.v1))!=0{-EFAULT}else{0} }
pub unsafe fn fscrypt_ioctl_get_policy_ex(filp:*mut file,uarg:*mut core::ffi::c_void)->i32 { let mut arg=core::mem::zeroed::<fscrypt_get_policy_ex_arg>(); let p=&mut arg.policy as *mut _ as *mut union_fscrypt_policy; let e=fscrypt_get_policy(file_inode(filp),p); if e!=0{return e;} let sz=fscrypt_policy_size(p); if copy_from_user(&mut arg.policy_size as *mut _,uarg,core::mem::size_of_val(&arg.policy_size))!=0{return -EFAULT;} if sz as usize>arg.policy_size{return -EOVERFLOW;} arg.policy_size=sz as _; if copy_to_user(uarg,&arg,core::mem::size_of_val(&arg.policy_size)+sz as usize)!=0{-EFAULT}else{0} }
pub unsafe fn fscrypt_ioctl_get_nonce(filp:*mut file,arg:*mut core::ffi::c_void)->i32 { let inode=file_inode(filp); let mut ctx=core::mem::zeroed::<union_fscrypt_context>(); let ret=((*(*(*inode).i_sb).s_cop).get_context.unwrap())(inode,&mut ctx,core::mem::size_of_val(&ctx)); if ret<0{return ret;} if !fscrypt_context_is_valid(&ctx,ret){return -EINVAL;} if copy_to_user(arg,fscrypt_context_nonce(&ctx),FSCRYPT_FILE_NONCE_SIZE)!=0{-EFAULT}else{0} }

pub unsafe fn fscrypt_has_permitted_context(parent:*mut inode,child:*mut inode)->i32 { if !S_ISREG((*child).i_mode)&&!S_ISDIR((*child).i_mode)&&!S_ISLNK((*child).i_mode){return 1;} if !IS_ENCRYPTED(parent){return 1;} if !IS_ENCRYPTED(child){return 0;} if fscrypt_get_encryption_info(parent,true)!=0||fscrypt_get_encryption_info(child,true)!=0{return 0;} let mut pp=core::mem::zeroed();let mut cp=core::mem::zeroed();let e1=fscrypt_get_policy(parent,&mut pp);let e2=fscrypt_get_policy(child,&mut cp);if e1==-EINVAL&&e2==-EINVAL{return 1;}if e1!=0||e2!=0{return 0;}fscrypt_policies_equal(&pp,&cp) as i32 }
pub unsafe fn fscrypt_policy_to_inherit(dir:*mut inode)->*const union_fscrypt_policy { if IS_ENCRYPTED(dir){let e=fscrypt_require_key(dir);if e!=0{return ERR_PTR(e);}return &fscrypt_get_inode_info_raw(dir).as_ref().unwrap().ci_policy;}fscrypt_get_dummy_policy((*dir).i_sb) }
pub unsafe fn fscrypt_context_for_new_inode(ctx:*mut core::ffi::c_void,inode:*mut inode)->i32 { let ci=fscrypt_get_inode_info_raw(inode);if WARN_ON_ONCE(ci.is_null()){return -ENOKEY;}fscrypt_new_context(ctx as *mut _,&(*ci).ci_policy,(*ci).ci_nonce.as_ptr()) }
pub unsafe fn fscrypt_set_context(inode:*mut inode,fs_data:*mut core::ffi::c_void)->i32 { let mut ctx=core::mem::zeroed();let sz=fscrypt_context_for_new_inode(&mut ctx,inode);if sz<0{return sz;}let ci=fscrypt_get_inode_info_raw(inode);if (*ci).ci_policy.version==FSCRYPT_POLICY_V2&&(*ci).ci_policy.v2.flags&FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32!=0{fscrypt_hash_inode_number(ci,(*ci).ci_master_key);}((*(*(*inode).i_sb).s_cop).set_context.unwrap())(inode,&ctx,sz,fs_data) }

pub unsafe fn fscrypt_parse_test_dummy_encryption(param:*const fs_parameter,dummy:*mut fscrypt_dummy_policy)->i32 { let arg=if (*param).type_==fs_value_is_string&&!(*param).string.is_null()&&*(*param).string!=0{(*param).string}else{c"v2"};let mut policy=kzalloc_obj::<union_fscrypt_policy>();if policy.is_null(){return -ENOMEM;}if strcmp(arg,c"v1")==0{(*policy).version=FSCRYPT_POLICY_V1;(*policy).v1.contents_encryption_mode=FSCRYPT_MODE_AES_256_XTS;(*policy).v1.filenames_encryption_mode=FSCRYPT_MODE_AES_256_CTS;memset((*policy).v1.master_key_descriptor.as_mut_ptr() as *mut _,0x42,FSCRYPT_KEY_DESCRIPTOR_SIZE);}else if strcmp(arg,c"v2")==0{(*policy).version=FSCRYPT_POLICY_V2;(*policy).v2.contents_encryption_mode=FSCRYPT_MODE_AES_256_XTS;(*policy).v2.filenames_encryption_mode=FSCRYPT_MODE_AES_256_CTS;fscrypt_get_test_dummy_key_identifier((*policy).v2.master_key_identifier.as_mut_ptr());}else{ kfree(policy as *mut _);return -EINVAL;}if !(*dummy).policy.is_null(){let e=if fscrypt_policies_equal(policy,(*dummy).policy){0}else{-EEXIST};kfree(policy as *mut _);return e;}(*dummy).policy=policy;0 }
pub unsafe fn fscrypt_dummy_policies_equal(p1:*const fscrypt_dummy_policy,p2:*const fscrypt_dummy_policy)->bool { if (*p1).policy.is_null()&&(*p2).policy.is_null(){return true;}if (*p1).policy.is_null()||(*p2).policy.is_null(){return false;}fscrypt_policies_equal((*p1).policy,(*p2).policy) }
pub unsafe fn fscrypt_show_test_dummy_encryption(seq:*mut seq_file,sep:core::ffi::c_char,sb:*mut super_block){let p=fscrypt_get_dummy_policy(sb);if p.is_null(){return;}let mut v=(*p).version;if v==FSCRYPT_POLICY_V1{v=1;}seq_printf(seq,c"%ctest_dummy_encryption=v%d",sep,v);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
