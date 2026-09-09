// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/hfsplus/xattr.c. Kernel-provided types, constants,
 * helpers, and callbacks are intentionally referenced as external dependencies. */

extern "C" {
    static hfsplus_xattr_osx_handler: xattr_handler;
    static hfsplus_xattr_user_handler: xattr_handler;
    static hfsplus_xattr_trusted_handler: xattr_handler;
    static hfsplus_xattr_security_handler: xattr_handler;
}

#[repr(C)] pub struct xattr_handler { pub prefix: *const c_char, pub get: Option<unsafe extern "C" fn(*const xattr_handler,*mut dentry,*mut inode,*const c_char,*mut c_void,usize)->isize>, pub set: Option<unsafe extern "C" fn(*const xattr_handler,*mut mnt_idmap,*mut dentry,*mut inode,*const c_char,*const c_void,usize,i32)->i32> }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mapping: *mut address_space, pub i_ino: u64, pub i_size: i64 }
#[repr(C)] pub struct super_block { pub s_blocksize: u32 }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct page;
#[repr(C)] pub struct hfs_bnode_desc { pub next: u32, pub type_: u8, pub num_recs: u16 }
#[repr(C)] pub struct hfs_btree_header_rec { pub node_size:u16, pub node_count:u32, pub free_nodes:u32, pub clump_size:u32, pub attributes:u32, pub max_key_len:u16, pub leaf_tail:u32 }
#[repr(C)] pub struct hfs_find_data { pub bnode:*mut c_void, pub entryoffset:u16, pub keyoffset:u16, pub tree:*mut c_void, pub key:*mut hfsplus_attr_key }
#[repr(C)] pub struct hfsplus_attr_key { pub cnid:u32, pub attr: attr_key_data }
#[repr(C)] pub struct attr_key_data { pub key_name: unicode_string }
#[repr(C)] pub struct unicode_string { pub unicode:[u16;255] }
#[repr(C)] pub struct hfsplus_cat_entry { pub type_:u16, pub folder: cat_folder, pub file: cat_file }
#[repr(C)] pub struct cat_folder { pub info:[u8;32], pub flags:u16, pub user_info:[u8;32] }
#[repr(C)] pub struct cat_file { pub info:[u8;16], pub flags:u16, pub user_info:[u8;16] }
#[repr(C)] pub struct hfsplus_attr_entry { pub inline_data: inline_data }
#[repr(C)] pub struct inline_data { pub length:u16, pub raw_bytes:[u8;4096] }

use core::ffi::{c_char,c_void};

extern "C" { fn hfsplus_removexattr(inode:*mut inode,name:*const c_char)->i32; }

#[no_mangle] pub static hfsplus_xattr_handlers:[*const xattr_handler;5]=[
    unsafe { &hfsplus_xattr_osx_handler }, unsafe { &hfsplus_xattr_user_handler },
    unsafe { &hfsplus_xattr_trusted_handler }, unsafe { &hfsplus_xattr_security_handler }, core::ptr::null()];

unsafe fn strcmp_xattr_finder_info(name:*const c_char)->i32 { if !name.is_null(){ strncmp(name,HFSPLUS_XATTR_FINDER_INFO_NAME,HFSPLUS_XATTR_FINDER_INFO_NAME.len()) } else {-1} }
unsafe fn strcmp_xattr_acl(name:*const c_char)->i32 { if !name.is_null(){ strncmp(name,HFSPLUS_XATTR_ACL_NAME,HFSPLUS_XATTR_ACL_NAME.len()) } else {-1} }
unsafe fn is_known_namespace(name:*const c_char)->bool { strncmp(name,XATTR_SYSTEM_PREFIX,XATTR_SYSTEM_PREFIX_LEN)!=0 && strncmp(name,XATTR_USER_PREFIX,XATTR_USER_PREFIX_LEN)!=0 && strncmp(name,XATTR_SECURITY_PREFIX,XATTR_SECURITY_PREFIX_LEN)!=0 && strncmp(name,XATTR_TRUSTED_PREFIX,XATTR_TRUSTED_PREFIX_LEN)!=0 }

unsafe fn hfsplus_init_header_node(attr_file:*mut inode,clump_size:u32,buf:*mut c_char,node_size:u16)->u32 {
    let mut rec_offsets=buf.add(node_size as usize) as *mut u16; let desc=buf as *mut hfs_bnode_desc; (*desc).type_=HFS_NODE_HEADER; (*desc).num_recs=cpu_to_be16(HFSPLUS_BTREE_HDR_NODE_RECS_COUNT); let mut offset=core::mem::size_of::<hfs_bnode_desc>() as u16; rec_offsets=rec_offsets.sub(1); *rec_offsets=cpu_to_be16(offset);
    let head=buf.add(offset as usize) as *mut hfs_btree_header_rec; (*head).node_size=cpu_to_be16(node_size); let mut tmp=i_size_read(attr_file) as u64; tmp/=node_size as u64; (*head).node_count=cpu_to_be32(tmp as u32); (*head).free_nodes=cpu_to_be32(be32_to_cpu((*head).node_count)-1); (*head).clump_size=cpu_to_be32(clump_size); (*head).attributes|=cpu_to_be32(HFS_TREE_BIGKEYS|HFS_TREE_VARIDXKEYS); (*head).max_key_len=cpu_to_be16(HFSPLUS_ATTR_KEYLEN-core::mem::size_of::<u16>() as u16); offset+=core::mem::size_of::<hfs_btree_header_rec>() as u16; rec_offsets=rec_offsets.sub(1); *rec_offsets=cpu_to_be16(offset); offset+=HFSPLUS_BTREE_HDR_USER_BYTES; rec_offsets=rec_offsets.sub(1); *rec_offsets=cpu_to_be16(offset);
    let bits=8*((node_size as usize)-offset as usize-4*core::mem::size_of::<u16>()) as u32; let mut maps=0; if be32_to_cpu((*head).node_count)>bits { let map_bits=8*((node_size as usize)-core::mem::size_of::<hfs_bnode_desc>()-2*core::mem::size_of::<u16>()-2) as u32; (*desc).next=cpu_to_be32(be32_to_cpu((*head).leaf_tail)+1); maps=(be32_to_cpu((*head).node_count)-bits+map_bits-1)/map_bits; (*head).free_nodes=cpu_to_be32(be32_to_cpu((*head).free_nodes)-maps); }
    let mut bmp=buf.add(offset as usize) as *mut u8; let mut used=be32_to_cpu((*head).node_count)-be32_to_cpu((*head).free_nodes); let used_bytes=used/8; if used_bytes!=0 { core::ptr::write_bytes(bmp,0xff,used_bytes as usize); bmp=bmp.add(used_bytes as usize); used%=8; } *bmp=!(0xffu8>>used); offset+=(bits/8) as u16; rec_offsets=rec_offsets.sub(1); *rec_offsets=cpu_to_be16(offset); maps
}

unsafe fn hfsplus_init_map_node(buf:*mut u8,node_size:u16,next_node:u32){ core::ptr::write_bytes(buf,0,node_size as usize); let d=buf as *mut hfs_bnode_desc; (*d).type_=HFS_NODE_MAP; (*d).num_recs=cpu_to_be16(1); (*d).next=cpu_to_be32(next_node); let r=(buf.add(node_size as usize)) as *mut u16; let mut o=core::mem::size_of::<hfs_bnode_desc>() as u16; *r.sub(1)=cpu_to_be16(o); o=node_size-o; o-=HFSPLUS_BTREE_MAP_NODE_RECS_COUNT as u16*core::mem::size_of::<u16>() as u16; o-=HFSPLUS_BTREE_MAP_NODE_RESERVED_BYTES; *r.sub(2)=cpu_to_be16(o); }

unsafe fn is_xattr_operation_supported(i:*mut inode)->bool { !HFSPLUS_IS_RSRC(i) }

pub unsafe fn __hfsplus_setxattr(inode:*mut inode,name:*const c_char,value:*const c_void,size:usize,flags:i32)->i32 { if !is_xattr_operation_supported(inode){return -EOPNOTSUPP;} if value.is_null(){return hfsplus_removexattr(inode,name);} let mut fd=hfs_find_data::default(); let mut err=hfs_find_init((*(*inode).i_sb).cat_tree,&mut fd); if err!=0{return err;} err=hfsplus_find_cat((*inode).i_sb,(*inode).i_ino,&mut fd); if err!=0{hfs_find_exit(&mut fd);return err;} if strcmp_xattr_finder_info(name)==0 { hfs_find_exit(&mut fd); return -EOPNOTSUPP; } if (*(*inode).i_sb).attr_tree.is_null(){err=hfsplus_create_attributes_file((*inode).i_sb);if err!=0{hfs_find_exit(&mut fd);return err;}} if hfsplus_attr_exists(inode,name){if flags&XATTR_CREATE!=0{err=-EOPNOTSUPP;}else{err=hfsplus_replace_attr(inode,name,value,size);}}else if flags&XATTR_REPLACE!=0{err=-EOPNOTSUPP;}else{err=hfsplus_create_attr(inode,name,value,size);} hfs_find_exit(&mut fd);err }

pub unsafe fn hfsplus_setxattr(inode:*mut inode,name:*const c_char,value:*const c_void,size:usize,flags:i32,prefix:*const c_char,prefixlen:usize)->i32 { let n=kmalloc(NLS_MAX_CHARSET_SIZE*HFSPLUS_ATTR_MAX_STRLEN+1,GFP_KERNEL) as *mut c_char;if n.is_null(){return -ENOMEM;} strcpy(n,prefix);strcpy(n.add(prefixlen),name);let r=__hfsplus_setxattr(inode,n,value,size,flags);kfree(n as *mut c_void);r }

pub unsafe fn __hfsplus_getxattr(inode:*mut inode,name:*const c_char,value:*mut c_void,size:usize)->isize { if !is_xattr_operation_supported(inode){return -EOPNOTSUPP as isize;} if strcmp_xattr_finder_info(name)==0{return hfsplus_getxattr_finder_info(inode,value,size);} if (*(*inode).i_sb).attr_tree.is_null(){return -EOPNOTSUPP as isize;} let e=hfsplus_alloc_attr_entry();if e.is_null(){return -ENOMEM as isize;}let mut fd=hfs_find_data::default();let mut r=hfs_find_init((*(*inode).i_sb).attr_tree,&mut fd);if r!=0{hfsplus_destroy_attr_entry(e);return r as isize;}r=hfsplus_find_attr((*inode).i_sb,(*inode).i_ino,name,&mut fd);if r!=0{hfs_find_exit(&mut fd);hfsplus_destroy_attr_entry(e);return if r==-ENOENT||r==-ENODATA{-ENODATA}else{r} as isize;}let l=hfs_bnode_read_u16(fd.bnode,fd.entryoffset+ATTR_INLINE_LENGTH_OFFSET);if l>HFSPLUS_MAX_INLINE_DATA_SIZE{r=-EIO;}else if size<l as usize{r=if size==0{l as i32}else{-ERANGE};}else{hfs_bnode_read(fd.bnode,e as *mut c_void,fd.entryoffset,ATTR_INLINE_RAW_OFFSET+l as usize);memcpy(value,(*e).inline_data.raw_bytes.as_ptr() as *const c_void,l as usize);r=l as i32;}hfs_find_exit(&mut fd);hfsplus_destroy_attr_entry(e);r as isize }

pub unsafe fn hfsplus_getxattr(inode:*mut inode,name:*const c_char,value:*mut c_void,size:usize,prefix:*const c_char,prefixlen:usize)->isize {let n=kmalloc(NLS_MAX_CHARSET_SIZE*HFSPLUS_ATTR_MAX_STRLEN+1,GFP_KERNEL) as *mut c_char;if n.is_null(){return -ENOMEM as isize;}strcpy(n,prefix);strcpy(n.add(prefixlen),name);let r=__hfsplus_getxattr(inode,n,value,size);kfree(n as *mut c_void);r}

pub unsafe fn hfsplus_listxattr(_d:*mut dentry,_b:*mut c_char,_s:usize)->isize { -EOPNOTSUPP as isize }

unsafe extern "C" fn hfsplus_osx_getxattr(_: *const xattr_handler,_:*mut dentry,i:*mut inode,n:*const c_char,b:*mut c_void,s:usize)->isize {if is_known_namespace(n){return -EOPNOTSUPP as isize;}__hfsplus_getxattr(i,n,b,s)}
unsafe extern "C" fn hfsplus_osx_setxattr(_: *const xattr_handler,_:*mut mnt_idmap,_:*mut dentry,i:*mut inode,n:*const c_char,b:*const c_void,s:usize,f:i32)->i32 {if is_known_namespace(n){return -EOPNOTSUPP;}__hfsplus_setxattr(i,n,b,s,f)}

#[no_mangle] pub static hfsplus_xattr_osx_handler:xattr_handler=xattr_handler{prefix:XATTR_MAC_OSX_PREFIX,get:Some(hfsplus_osx_getxattr),set:Some(hfsplus_osx_setxattr)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
