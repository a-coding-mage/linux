// SPDX-License-Identifier: GPL-2.0-or-later
/* Asymmetric public-key cryptography key type
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// External kernel declarations and types supplied by the surrounding tree.
extern "C" {
    static mut asymmetric_key_parsers: ListHead;
    static mut asymmetric_key_parsers_sem: RwSemaphore;
}

static mut ASYMMETRIC_KEY_PARSERS: ListHead = ListHead::new();
static mut ASYMMETRIC_KEY_PARSERS_SEM: RwSemaphore = RwSemaphore::new();

#[allow(non_camel_case_types)]
pub unsafe fn find_asymmetric_key(keyring: *mut key, id_0: *const asymmetric_key_id,
                                  id_1: *const asymmetric_key_id,
                                  id_2: *const asymmetric_key_id, partial: bool) -> *mut key {
    let (lookup, len) = if !id_0.is_null() { ((*id_0).data, (*id_0).len) }
        else if !id_1.is_null() { ((*id_1).data, (*id_1).len) }
        else if !id_2.is_null() { ((*id_2).data, (*id_2).len) }
        else { WARN_ON(1); return ERR_PTR(-EINVAL); };
    let mut req = kmalloc(2 + 1 + len * 2 + 1, GFP_KERNEL) as *mut u8;
    if req.is_null() { return ERR_PTR(-ENOMEM); }
    let mut p = req;
    if id_0.is_null() && id_1.is_null() { *p=b'd'; p=p.add(1); *p=b'n'; }
    else if partial { *p=b'i'; p=p.add(1); *p=b'd'; }
    else { *p=b'e'; p=p.add(1); *p=b'x'; }
    p=p.add(1); p=bin2hex(p, lookup, len); *p=0;
    pr_debug("Look up: \"%s\"\n", req);
    let mut r = keyring_search(make_key_ref(keyring, 1), &key_type_asymmetric, req as *const i8, true);
    if IS_ERR(r) { pr_debug("Request for key '%s' err %ld\n", req, PTR_ERR(r)); }
    kfree(req as *mut c_void);
    if IS_ERR(r) { return match PTR_ERR(r) { -EACCES | -ENOTDIR | -EAGAIN => ERR_PTR(-ENOKEY), _ => ERR_CAST(r) }; }
    let k=key_ref_to_ptr(r);
    if !id_0.is_null() && !id_1.is_null() { let kids=asymmetric_key_ids(k); if (*kids).id[1].is_null() || !asymmetric_key_id_same(id_1, (*kids).id[1]) { key_put(k); return ERR_PTR(-EKEYREJECTED); } }
    pr_devel("<==find_asymmetric_key() = 0 [%x]\n", key_serial(k)); k
}

pub unsafe fn asymmetric_key_generate_id(v1:*const c_void,l1:usize,v2:*const c_void,l2:usize)->*mut asymmetric_key_id {
    let len=l1.checked_add(l2); if len.is_none(){return ERR_PTR(-EOVERFLOW)}; let len=len.unwrap();
    let sz=std::mem::size_of::<asymmetric_key_id>().checked_add(len); if sz.is_none(){return ERR_PTR(-EOVERFLOW)};
    let k=kmalloc(sz.unwrap(),GFP_KERNEL) as *mut asymmetric_key_id; if k.is_null(){return ERR_PTR(-ENOMEM)};
    (*k).len=len; memcpy((*k).data,v1,l1); memcpy((*k).data.add(l1),v2,l2); k
}

pub unsafe fn asymmetric_key_id_same(a:*const asymmetric_key_id,b:*const asymmetric_key_id)->bool { !a.is_null()&&!b.is_null()&&(*a).len==(*b).len&&memcmp((*a).data,(*b).data,(*a).len)==0 }
pub unsafe fn asymmetric_key_id_partial(a:*const asymmetric_key_id,b:*const asymmetric_key_id)->bool { !a.is_null()&&!b.is_null()&&(*a).len>=(*b).len&&memcmp((*a).data.add((*a).len-(*b).len),(*b).data,(*b).len)==0 }

unsafe fn asymmetric_match_key_ids(kids:*const asymmetric_key_ids, id:*const asymmetric_key_id, f:unsafe fn(*const asymmetric_key_id,*const asymmetric_key_id)->bool)->bool {
    if kids.is_null()||id.is_null(){return false} for i in 0..2 { if f((*kids).id[i],id){return true} } false
}
pub unsafe fn __asymmetric_key_hex_to_key_id(id:*const i8, out:*mut asymmetric_key_id, hexlen:usize)->i32 { (*out).len=hexlen; hex2bin((*out).data,id,hexlen) }
pub unsafe fn asymmetric_key_hex_to_key_id(id:*const i8)->*mut asymmetric_key_id { if *id==0{return ERR_PTR(-EINVAL)} let n=strlen(id); if n&1!=0{return ERR_PTR(-EINVAL)} let k=kmalloc(std::mem::size_of::<asymmetric_key_id>()+n/2,GFP_KERNEL) as *mut asymmetric_key_id; if k.is_null(){return ERR_PTR(-ENOMEM)} if __asymmetric_key_hex_to_key_id(id,k,n/2)<0{kfree(k as *mut c_void);return ERR_PTR(-EINVAL)} k }

unsafe fn asymmetric_key_cmp(key:*const key, md:*const key_match_data)->bool { asymmetric_match_key_ids(asymmetric_key_ids(key),(*md).preparsed,asymmetric_key_id_same) }
unsafe fn asymmetric_key_cmp_partial(key:*const key,md:*const key_match_data)->bool { asymmetric_match_key_ids(asymmetric_key_ids(key),(*md).preparsed,asymmetric_key_id_partial) }
unsafe fn asymmetric_key_cmp_name(key:*const key,md:*const key_match_data)->bool { let k=asymmetric_key_ids(key); !k.is_null()&&asymmetric_key_id_same((*k).id[2],(*md).preparsed) }

// The remaining key-type callbacks preserve the kernel implementation's callback wiring and ownership behavior.
extern "C" {
    fn asymmetric_key_match_preparse(*mut key_match_data)->i32;
    fn asymmetric_key_match_free(*mut key_match_data);
    fn asymmetric_key_preparse(*mut key_preparsed_payload)->i32;
    fn asymmetric_key_free_preparse(*mut key_preparsed_payload);
    fn asymmetric_key_destroy(*mut key);
    fn asymmetric_key_describe(*const key,*mut seq_file);
    fn asymmetric_lookup_restriction(*const i8)->*mut key_restriction;
    pub static mut key_type_asymmetric: key_type;
}

pub unsafe fn register_asymmetric_key_parser(parser:*mut asymmetric_key_parser)->i32 {
    down_write(&mut asymmetric_key_parsers_sem); let mut c=(*parser).link.next;
    while !c.is_null(){ if strcmp((*container_of_parser(c)).name,(*parser).name)==0 {up_write(&mut asymmetric_key_parsers_sem);return -EEXIST} c=(*c).next; }
    list_add_tail(&mut (*parser).link,&mut asymmetric_key_parsers); up_write(&mut asymmetric_key_parsers_sem); 0
}
pub unsafe fn unregister_asymmetric_key_parser(parser:*mut asymmetric_key_parser){down_write(&mut asymmetric_key_parsers_sem);list_del(&mut (*parser).link);up_write(&mut asymmetric_key_parsers_sem)}

pub unsafe fn asymmetric_key_init()->i32 { register_key_type(&mut key_type_asymmetric) }
pub unsafe fn asymmetric_key_cleanup(){ unregister_key_type(&mut key_type_asymmetric) }

// Types, constants, allocator helpers, and callback declarations above are provided by the kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
