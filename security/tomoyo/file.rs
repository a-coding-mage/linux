// SPDX-License-Identifier: GPL-2.0
/* Rust translation of security/tomoyo/file.c. */

/* Types, constants, macros, and external functions referenced here are
 * supplied by the surrounding TOMOYO translation. */

static TOMOYO_P2MAC: [u8; TOMOYO_MAX_PATH_OPERATION as usize] = {
    let mut a = [0; TOMOYO_MAX_PATH_OPERATION as usize];
    a[TOMOYO_TYPE_EXECUTE as usize] = TOMOYO_MAC_FILE_EXECUTE;
    a[TOMOYO_TYPE_READ as usize] = TOMOYO_MAC_FILE_OPEN;
    a[TOMOYO_TYPE_WRITE as usize] = TOMOYO_MAC_FILE_OPEN;
    a[TOMOYO_TYPE_APPEND as usize] = TOMOYO_MAC_FILE_OPEN;
    a[TOMOYO_TYPE_UNLINK as usize] = TOMOYO_MAC_FILE_UNLINK;
    a[TOMOYO_TYPE_GETATTR as usize] = TOMOYO_MAC_FILE_GETATTR;
    a[TOMOYO_TYPE_RMDIR as usize] = TOMOYO_MAC_FILE_RMDIR;
    a[TOMOYO_TYPE_TRUNCATE as usize] = TOMOYO_MAC_FILE_TRUNCATE;
    a[TOMOYO_TYPE_SYMLINK as usize] = TOMOYO_MAC_FILE_SYMLINK;
    a[TOMOYO_TYPE_CHROOT as usize] = TOMOYO_MAC_FILE_CHROOT;
    a[TOMOYO_TYPE_UMOUNT as usize] = TOMOYO_MAC_FILE_UMOUNT;
    a
};
pub static TOMOYO_PNNN2MAC: [u8; TOMOYO_MAX_MKDEV_OPERATION as usize] = {
    let mut a = [0; TOMOYO_MAX_MKDEV_OPERATION as usize];
    a[TOMOYO_TYPE_MKBLOCK as usize] = TOMOYO_MAC_FILE_MKBLOCK;
    a[TOMOYO_TYPE_MKCHAR as usize] = TOMOYO_MAC_FILE_MKCHAR;
    a
};
pub static TOMOYO_PP2MAC: [u8; TOMOYO_MAX_PATH2_OPERATION as usize] = {
    let mut a = [0; TOMOYO_MAX_PATH2_OPERATION as usize];
    a[TOMOYO_TYPE_LINK as usize] = TOMOYO_MAC_FILE_LINK;
    a[TOMOYO_TYPE_RENAME as usize] = TOMOYO_MAC_FILE_RENAME;
    a[TOMOYO_TYPE_PIVOT_ROOT as usize] = TOMOYO_MAC_FILE_PIVOT_ROOT;
    a
};
pub static TOMOYO_PN2MAC: [u8; TOMOYO_MAX_PATH_NUMBER_OPERATION as usize] = {
    let mut a = [0; TOMOYO_MAX_PATH_NUMBER_OPERATION as usize];
    a[TOMOYO_TYPE_CREATE as usize] = TOMOYO_MAC_FILE_CREATE;
    a[TOMOYO_TYPE_MKDIR as usize] = TOMOYO_MAC_FILE_MKDIR;
    a[TOMOYO_TYPE_MKFIFO as usize] = TOMOYO_MAC_FILE_MKFIFO;
    a[TOMOYO_TYPE_MKSOCK as usize] = TOMOYO_MAC_FILE_MKSOCK;
    a[TOMOYO_TYPE_IOCTL as usize] = TOMOYO_MAC_FILE_IOCTL;
    a[TOMOYO_TYPE_CHMOD as usize] = TOMOYO_MAC_FILE_CHMOD;
    a[TOMOYO_TYPE_CHOWN as usize] = TOMOYO_MAC_FILE_CHOWN;
    a[TOMOYO_TYPE_CHGRP as usize] = TOMOYO_MAC_FILE_CHGRP;
    a
};

pub unsafe fn tomoyo_put_name_union(ptr: *mut tomoyo_name_union) {
    tomoyo_put_group((*ptr).group); tomoyo_put_name((*ptr).filename);
}
pub unsafe fn tomoyo_compare_name_union(name: *const tomoyo_path_info, ptr: *const tomoyo_name_union) -> *const tomoyo_path_info {
    if !(*ptr).group.is_null() { return tomoyo_path_matches_group(name, (*ptr).group); }
    if tomoyo_path_matches_pattern(name, (*ptr).filename) { (*ptr).filename } else { core::ptr::null() }
}
pub unsafe fn tomoyo_put_number_union(ptr: *mut tomoyo_number_union) { tomoyo_put_group((*ptr).group); }
pub unsafe fn tomoyo_compare_number_union(value: c_ulong, ptr: *const tomoyo_number_union) -> bool {
    if !(*ptr).group.is_null() { tomoyo_number_matches_group(value, value, (*ptr).group) }
    else { value >= (*ptr).values[0] && value <= (*ptr).values[1] }
}

unsafe fn tomoyo_add_slash(buf: *mut tomoyo_path_info) {
    if (*buf).is_dir { return; }
    strcat((*buf).name as *mut i8, b"/\0".as_ptr() as *const i8);
    tomoyo_fill_path_info(buf);
}
unsafe fn tomoyo_get_realpath(buf: *mut tomoyo_path_info, path: *const path) -> bool {
    (*buf).name = tomoyo_realpath_from_path(path);
    if !(*buf).name.is_null() { tomoyo_fill_path_info(buf); true } else { false }
}

unsafe fn tomoyo_audit_path_log(r: *mut tomoyo_request_info) -> c_int {
    tomoyo_supervisor(r, b"file %s %s\n\0".as_ptr() as *const i8,
        tomoyo_path_keyword[(*r).param.path.operation as usize], (*r).param.path.filename.name)
}
unsafe fn tomoyo_audit_path2_log(r: *mut tomoyo_request_info) -> c_int {
    tomoyo_supervisor(r, b"file %s %s %s\n\0".as_ptr() as *const i8,
        tomoyo_mac_keywords[TOMOYO_PP2MAC[(*r).param.path2.operation as usize] as usize],
        (*r).param.path2.filename1.name, (*r).param.path2.filename2.name)
}
unsafe fn tomoyo_audit_mkdev_log(r: *mut tomoyo_request_info) -> c_int {
    tomoyo_supervisor(r, b"file %s %s 0%o %u %u\n\0".as_ptr() as *const i8,
        tomoyo_mac_keywords[TOMOYO_PNNN2MAC[(*r).param.mkdev.operation as usize] as usize],
        (*r).param.mkdev.filename.name, (*r).param.mkdev.mode,
        (*r).param.mkdev.major, (*r).param.mkdev.minor)
}
unsafe fn tomoyo_audit_path_number_log(r: *mut tomoyo_request_info) -> c_int {
    let t = (*r).param.path_number.operation;
    let radix = match t { TOMOYO_TYPE_CREATE|TOMOYO_TYPE_MKDIR|TOMOYO_TYPE_MKFIFO|TOMOYO_TYPE_MKSOCK|TOMOYO_TYPE_CHMOD => TOMOYO_VALUE_TYPE_OCTAL, TOMOYO_TYPE_IOCTL => TOMOYO_VALUE_TYPE_HEXADECIMAL, _ => TOMOYO_VALUE_TYPE_DECIMAL };
    let mut buffer = [0i8; 64];
    tomoyo_print_ulong(buffer.as_mut_ptr(), buffer.len(), (*r).param.path_number.number, radix);
    tomoyo_supervisor(r, b"file %s %s %s\n\0".as_ptr() as *const i8, tomoyo_mac_keywords[TOMOYO_PN2MAC[t as usize] as usize], (*r).param.path_number.filename.name, buffer.as_ptr())
}

unsafe fn tomoyo_check_path_acl(r: *mut tomoyo_request_info, ptr: *const tomoyo_acl_info) -> bool {
    let acl: *const tomoyo_path_acl = container_of!(ptr, tomoyo_path_acl, head);
    if (*acl).perm & (1u16 << (*r).param.path.operation) != 0 { (*r).param.path.matched_path = tomoyo_compare_name_union((*r).param.path.filename, &(*acl).name); !(*r).param.path.matched_path.is_null() } else { false }
}
unsafe fn tomoyo_check_path_number_acl(r: *mut tomoyo_request_info, ptr: *const tomoyo_acl_info) -> bool { let a=*container_of!(ptr,tomoyo_path_number_acl,head); a.perm & (1u8<<(*r).param.path_number.operation)!=0 && tomoyo_compare_number_union((*r).param.path_number.number,&a.number) && !tomoyo_compare_name_union((*r).param.path_number.filename,&a.name).is_null() }
unsafe fn tomoyo_check_path2_acl(r:*mut tomoyo_request_info,ptr:*const tomoyo_acl_info)->bool { let a=*container_of!(ptr,tomoyo_path2_acl,head); a.perm&(1u8<<(*r).param.path2.operation)!=0 && !tomoyo_compare_name_union((*r).param.path2.filename1,&a.name1).is_null() && !tomoyo_compare_name_union((*r).param.path2.filename2,&a.name2).is_null() }
unsafe fn tomoyo_check_mkdev_acl(r:*mut tomoyo_request_info,ptr:*const tomoyo_acl_info)->bool { let a=*container_of!(ptr,tomoyo_mkdev_acl,head); a.perm&(1u8<<(*r).param.mkdev.operation)!=0 && tomoyo_compare_number_union((*r).param.mkdev.mode,&a.mode) && tomoyo_compare_number_union((*r).param.mkdev.major,&a.major) && tomoyo_compare_number_union((*r).param.mkdev.minor,&a.minor) && !tomoyo_compare_name_union((*r).param.mkdev.filename,&a.name).is_null() }

unsafe fn merge_perm<T>(a:*mut u8,b:*const u8,del:bool)->bool { let mut p=core::ptr::read_volatile(a); let q=core::ptr::read(b); if del {p&=!q}else{p|=q}; core::ptr::write_volatile(a,p); p==0 }
unsafe fn tomoyo_same_path_acl(a:*const tomoyo_acl_info,b:*const tomoyo_acl_info)->bool { tomoyo_same_name_union(&(*container_of!(a,tomoyo_path_acl,head)).name,&(*container_of!(b,tomoyo_path_acl,head)).name) }
unsafe fn tomoyo_merge_path_acl(a:*mut tomoyo_acl_info,b:*mut tomoyo_acl_info,d:bool)->bool { merge_perm(&mut (*container_of!(a,tomoyo_path_acl,head)).perm as *mut _ as *mut u8,&(*container_of!(b,tomoyo_path_acl,head)).perm as *const _ as *const u8,d) }
unsafe fn tomoyo_same_mkdev_acl(a:*const tomoyo_acl_info,b:*const tomoyo_acl_info)->bool { let x=container_of!(a,tomoyo_mkdev_acl,head);let y=container_of!(b,tomoyo_mkdev_acl,head);tomoyo_same_name_union(&(*x).name,&(*y).name)&&tomoyo_same_number_union(&(*x).mode,&(*y).mode)&&tomoyo_same_number_union(&(*x).major,&(*y).major)&&tomoyo_same_number_union(&(*x).minor,&(*y).minor) }
unsafe fn tomoyo_merge_mkdev_acl(a:*mut tomoyo_acl_info,b:*mut tomoyo_acl_info,d:bool)->bool { merge_perm(&mut (*container_of!(a,tomoyo_mkdev_acl,head)).perm,&(*container_of!(b,tomoyo_mkdev_acl,head)).perm,d) }
unsafe fn tomoyo_same_path2_acl(a:*const tomoyo_acl_info,b:*const tomoyo_acl_info)->bool { let x=container_of!(a,tomoyo_path2_acl,head);let y=container_of!(b,tomoyo_path2_acl,head);tomoyo_same_name_union(&(*x).name1,&(*y).name1)&&tomoyo_same_name_union(&(*x).name2,&(*y).name2) }
unsafe fn tomoyo_merge_path2_acl(a:*mut tomoyo_acl_info,b:*mut tomoyo_acl_info,d:bool)->bool { merge_perm(&mut (*container_of!(a,tomoyo_path2_acl,head)).perm,&(*container_of!(b,tomoyo_path2_acl,head)).perm,d) }
unsafe fn tomoyo_same_path_number_acl(a:*const tomoyo_acl_info,b:*const tomoyo_acl_info)->bool { let x=container_of!(a,tomoyo_path_number_acl,head);let y=container_of!(b,tomoyo_path_number_acl,head);tomoyo_same_name_union(&(*x).name,&(*y).name)&&tomoyo_same_number_union(&(*x).number,&(*y).number) }
unsafe fn tomoyo_merge_path_number_acl(a:*mut tomoyo_acl_info,b:*mut tomoyo_acl_info,d:bool)->bool { merge_perm(&mut (*container_of!(a,tomoyo_path_number_acl,head)).perm,&(*container_of!(b,tomoyo_path_number_acl,head)).perm,d) }

unsafe fn tomoyo_update_path_acl(perm:u16,p:*mut tomoyo_acl_param)->c_int { let mut e=core::mem::zeroed::<tomoyo_path_acl>();e.head.type_=TOMOYO_TYPE_PATH_ACL;e.perm=perm;let err=if !tomoyo_parse_name_union(p,&mut e.name){-EINVAL}else{tomoyo_update_domain(&mut e.head,core::mem::size_of_val(&e),p,tomoyo_same_path_acl,tomoyo_merge_path_acl)};tomoyo_put_name_union(&mut e.name);err }
unsafe fn tomoyo_update_mkdev_acl(perm:u8,p:*mut tomoyo_acl_param)->c_int { let mut e=core::mem::zeroed::<tomoyo_mkdev_acl>();e.head.type_=TOMOYO_TYPE_MKDEV_ACL;e.perm=perm;let err=if !tomoyo_parse_name_union(p,&mut e.name)||!tomoyo_parse_number_union(p,&mut e.mode)||!tomoyo_parse_number_union(p,&mut e.major)||!tomoyo_parse_number_union(p,&mut e.minor){-EINVAL}else{tomoyo_update_domain(&mut e.head,core::mem::size_of_val(&e),p,tomoyo_same_mkdev_acl,tomoyo_merge_mkdev_acl)};tomoyo_put_name_union(&mut e.name);tomoyo_put_number_union(&mut e.mode);tomoyo_put_number_union(&mut e.major);tomoyo_put_number_union(&mut e.minor);err }
unsafe fn tomoyo_update_path2_acl(perm:u8,p:*mut tomoyo_acl_param)->c_int { let mut e=core::mem::zeroed::<tomoyo_path2_acl>();e.head.type_=TOMOYO_TYPE_PATH2_ACL;e.perm=perm;let err=if !tomoyo_parse_name_union(p,&mut e.name1)||!tomoyo_parse_name_union(p,&mut e.name2){-EINVAL}else{tomoyo_update_domain(&mut e.head,core::mem::size_of_val(&e),p,tomoyo_same_path2_acl,tomoyo_merge_path2_acl)};tomoyo_put_name_union(&mut e.name1);tomoyo_put_name_union(&mut e.name2);err }
unsafe fn tomoyo_update_path_number_acl(perm:u8,p:*mut tomoyo_acl_param)->c_int { let mut e=core::mem::zeroed::<tomoyo_path_number_acl>();e.head.type_=TOMOYO_TYPE_PATH_NUMBER_ACL;e.perm=perm;let err=if !tomoyo_parse_name_union(p,&mut e.name)||!tomoyo_parse_number_union(p,&mut e.number){-EINVAL}else{tomoyo_update_domain(&mut e.head,core::mem::size_of_val(&e),p,tomoyo_same_path_number_acl,tomoyo_merge_path_number_acl)};tomoyo_put_name_union(&mut e.name);tomoyo_put_number_union(&mut e.number);err }

unsafe fn tomoyo_path_permission(r:*mut tomoyo_request_info,op:u8,f:*const tomoyo_path_info)->c_int {(*r).type_=TOMOYO_P2MAC[op as usize];(*r).mode=tomoyo_get_mode((*r).domain.ns,(*r).profile,(*r).type_);if (*r).mode==TOMOYO_CONFIG_DISABLED{return 0}(*r).param_type=TOMOYO_TYPE_PATH_ACL;(*r).param.path.filename=f;(*r).param.path.operation=op;let mut e;loop{tomoyo_check_acl(r,tomoyo_check_path_acl);e=tomoyo_audit_path_log(r);if e!=TOMOYO_RETRY_REQUEST{break}}e}
pub unsafe fn tomoyo_execute_permission(r:*mut tomoyo_request_info,f:*const tomoyo_path_info)->c_int {(*r).type_=TOMOYO_MAC_FILE_EXECUTE;(*r).mode=tomoyo_get_mode((*r).domain.ns,(*r).profile,(*r).type_);(*r).param_type=TOMOYO_TYPE_PATH_ACL;(*r).param.path.filename=f;(*r).param.path.operation=TOMOYO_TYPE_EXECUTE;tomoyo_check_acl(r,tomoyo_check_path_acl);(*r).ee.transition=if !(*r).matched_acl.is_null()&&!(*(*r).matched_acl).cond.is_null(){(*(*r).matched_acl).cond.transit}else{core::ptr::null_mut()};if (*r).mode!=TOMOYO_CONFIG_DISABLED{tomoyo_audit_path_log(r)}else{0}}

unsafe fn tomoyo_same_mount_acl(a:*const tomoyo_acl_info,b:*const tomoyo_acl_info)->bool {let x=container_of!(a,tomoyo_mount_acl,head);let y=container_of!(b,tomoyo_mount_acl,head);tomoyo_same_name_union(&(*x).dev_name,&(*y).dev_name)&&tomoyo_same_name_union(&(*x).dir_name,&(*y).dir_name)&&tomoyo_same_name_union(&(*x).fs_type,&(*y).fs_type)&&tomoyo_same_number_union(&(*x).flags,&(*y).flags)}
unsafe fn tomoyo_update_mount_acl(p:*mut tomoyo_acl_param)->c_int {let mut e=core::mem::zeroed::<tomoyo_mount_acl>();e.head.type_=TOMOYO_TYPE_MOUNT_ACL;let err=if !tomoyo_parse_name_union(p,&mut e.dev_name)||!tomoyo_parse_name_union(p,&mut e.dir_name)||!tomoyo_parse_name_union(p,&mut e.fs_type)||!tomoyo_parse_number_union(p,&mut e.flags){-EINVAL}else{tomoyo_update_domain(&mut e.head,core::mem::size_of_val(&e),p,tomoyo_same_mount_acl,None)};tomoyo_put_name_union(&mut e.dev_name);tomoyo_put_name_union(&mut e.dir_name);tomoyo_put_name_union(&mut e.fs_type);tomoyo_put_number_union(&mut e.flags);err}

pub unsafe fn tomoyo_write_file(p:*mut tomoyo_acl_param)->c_int {let mut perm:u16=0;let op=tomoyo_read_token(p);let mut t:u8=0;while t<TOMOYO_MAX_PATH_OPERATION{if tomoyo_permstr(op,tomoyo_path_keyword[t as usize]){perm|=1<<t};t+=1}if perm!=0{return tomoyo_update_path_acl(perm,p)};t=0;while t<TOMOYO_MAX_PATH2_OPERATION{if tomoyo_permstr(op,tomoyo_mac_keywords[TOMOYO_PP2MAC[t as usize] as usize]){perm|=1<<t};t+=1}if perm!=0{return tomoyo_update_path2_acl(perm as u8,p)};t=0;while t<TOMOYO_MAX_PATH_NUMBER_OPERATION{if tomoyo_permstr(op,tomoyo_mac_keywords[TOMOYO_PN2MAC[t as usize] as usize]){perm|=1<<t};t+=1}if perm!=0{return tomoyo_update_path_number_acl(perm as u8,p)};t=0;while t<TOMOYO_MAX_MKDEV_OPERATION{if tomoyo_permstr(op,tomoyo_mac_keywords[TOMOYO_PNNN2MAC[t as usize] as usize]){perm|=1<<t};t+=1}if perm!=0{return tomoyo_update_mkdev_acl(perm as u8,p)};if tomoyo_permstr(op,tomoyo_mac_keywords[TOMOYO_MAC_FILE_MOUNT as usize]){tomoyo_update_mount_acl(p)}else{-EINVAL}}

pub unsafe fn tomoyo_path_number_perm(ty:u8,path:*const path,num:c_ulong)->c_int { let mut r=core::mem::zeroed::<tomoyo_request_info>();let mut b=core::mem::zeroed::<tomoyo_path_info>();let mut o=core::mem::zeroed::<tomoyo_obj_info>();o.path1.mnt=(*path).mnt;o.path1.dentry=(*path).dentry;let mut e=-ENOMEM;if tomoyo_init_request_info(&mut r,core::ptr::null_mut(),TOMOYO_PN2MAC[ty as usize])==TOMOYO_CONFIG_DISABLED{return 0};let idx=tomoyo_read_lock();if tomoyo_get_realpath(&mut b,path){r.obj=&mut o;r.param_type=TOMOYO_TYPE_PATH_NUMBER_ACL;r.param.path_number.operation=ty;r.param.path_number.filename=&b;r.param.path_number.number=num;if ty==TOMOYO_TYPE_MKDIR{tomoyo_add_slash(&mut b)};loop{tomoyo_check_acl(&mut r,tomoyo_check_path_number_acl);e=tomoyo_audit_path_number_log(&mut r);if e!=TOMOYO_RETRY_REQUEST{break}};kfree(b.name)};tomoyo_read_unlock(idx);if r.mode!=TOMOYO_CONFIG_ENFORCING{0}else{e}}
pub unsafe fn tomoyo_check_open_permission(d:*mut tomoyo_domain_info,p:*const path,flag:c_int)->c_int {let a=ACC_MODE(flag);let mut r=core::mem::zeroed::<tomoyo_request_info>();let mut b=core::mem::zeroed::<tomoyo_path_info>();let mut o=core::mem::zeroed::<tomoyo_obj_info>();o.path1.mnt=(*p).mnt;o.path1.dentry=(*p).dentry;let mut e=0;let idx=tomoyo_read_lock();if a!=0&&tomoyo_init_request_info(&mut r,d,TOMOYO_MAC_FILE_OPEN)!=TOMOYO_CONFIG_DISABLED{if tomoyo_get_realpath(&mut b,p){r.obj=&mut o;if a&MAY_READ!=0{e=tomoyo_path_permission(&mut r,TOMOYO_TYPE_READ,&b)};if e==0&&a&MAY_WRITE!=0{e=tomoyo_path_permission(&mut r,if flag&O_APPEND!=0{TOMOYO_TYPE_APPEND}else{TOMOYO_TYPE_WRITE},&b)};kfree(b.name)}else{e=-ENOMEM}};tomoyo_read_unlock(idx);if r.mode!=TOMOYO_CONFIG_ENFORCING{0}else{e}}
pub unsafe fn tomoyo_path_perm(op:u8,p:*const path,target:*const i8)->c_int {let mut r=core::mem::zeroed::<tomoyo_request_info>();let mut b=core::mem::zeroed::<tomoyo_path_info>();let mut o=core::mem::zeroed::<tomoyo_obj_info>();o.path1.mnt=(*p).mnt;o.path1.dentry=(*p).dentry;let mut e=-ENOMEM;if tomoyo_init_request_info(&mut r,core::ptr::null_mut(),TOMOYO_P2MAC[op as usize])==TOMOYO_CONFIG_DISABLED{return 0};let enforce=r.mode==TOMOYO_CONFIG_ENFORCING;let idx=tomoyo_read_lock();if tomoyo_get_realpath(&mut b,p){r.obj=&mut o;if op==TOMOYO_TYPE_RMDIR||op==TOMOYO_TYPE_CHROOT{tomoyo_add_slash(&mut b)};e=tomoyo_path_permission(&mut r,op,&b);kfree(b.name)};tomoyo_read_unlock(idx);if enforce{e}else{0}}
pub unsafe fn tomoyo_mkdev_perm(op:u8,p:*const path,mode:c_uint,mut dev:c_uint)->c_int {let mut r=core::mem::zeroed::<tomoyo_request_info>();let mut b=core::mem::zeroed::<tomoyo_path_info>();let mut o=core::mem::zeroed::<tomoyo_obj_info>();o.path1.mnt=(*p).mnt;o.path1.dentry=(*p).dentry;if tomoyo_init_request_info(&mut r,core::ptr::null_mut(),TOMOYO_PNNN2MAC[op as usize])==TOMOYO_CONFIG_DISABLED{return 0};let idx=tomoyo_read_lock();let mut e=-ENOMEM;if tomoyo_get_realpath(&mut b,p){r.obj=&mut o;dev=new_decode_dev(dev);r.param_type=TOMOYO_TYPE_MKDEV_ACL;r.param.mkdev.filename=&b;r.param.mkdev.operation=op;r.param.mkdev.mode=mode;r.param.mkdev.major=MAJOR(dev);r.param.mkdev.minor=MINOR(dev);tomoyo_check_acl(&mut r,tomoyo_check_mkdev_acl);e=tomoyo_audit_mkdev_log(&mut r);kfree(b.name)};tomoyo_read_unlock(idx);if r.mode==TOMOYO_CONFIG_ENFORCING{e}else{0}}
pub unsafe fn tomoyo_path2_perm(op:u8,p1:*const path,p2:*const path)->c_int {let mut r=core::mem::zeroed::<tomoyo_request_info>();let mut b1=core::mem::zeroed::<tomoyo_path_info>();let mut b2=core::mem::zeroed::<tomoyo_path_info>();let mut o=core::mem::zeroed::<tomoyo_obj_info>();o.path1.mnt=(*p1).mnt;o.path1.dentry=(*p1).dentry;o.path2.mnt=(*p2).mnt;o.path2.dentry=(*p2).dentry;if tomoyo_init_request_info(&mut r,core::ptr::null_mut(),TOMOYO_PP2MAC[op as usize])==TOMOYO_CONFIG_DISABLED{return 0};let idx=tomoyo_read_lock();let mut e=-ENOMEM;if tomoyo_get_realpath(&mut b1,p1)&&tomoyo_get_realpath(&mut b2,p2){r.obj=&mut o;r.param_type=TOMOYO_TYPE_PATH2_ACL;r.param.path2.operation=op;r.param.path2.filename1=&b1;r.param.path2.filename2=&b2;loop{tomoyo_check_acl(&mut r,tomoyo_check_path2_acl);e=tomoyo_audit_path2_log(&mut r);if e!=TOMOYO_RETRY_REQUEST{break}}};kfree(b1.name);kfree(b2.name);tomoyo_read_unlock(idx);if r.mode==TOMOYO_CONFIG_ENFORCING{e}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
