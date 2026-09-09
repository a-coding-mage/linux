// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of dlm/dir.c.  Types, constants, and helper
// functions referenced here are supplied by the surrounding DLM sources.

use core::{mem, ptr};

pub unsafe fn dlm_hash2nodeid(ls: *mut dlm_ls, hash: u32) -> i32 {
    if (*ls).ls_num_nodes == 1 {
        dlm_our_nodeid()
    } else {
        let node = ((hash >> 16) % (*ls).ls_total_weight) as usize;
        *(*ls).ls_node_array.add(node)
    }
}

pub unsafe fn dlm_dir_nodeid(r: *mut dlm_rsb) -> i32 { (*r).res_dir_nodeid }

pub unsafe fn dlm_recover_dir_nodeid(ls: *mut dlm_ls, root_list: *const list_head) {
    let mut p = (*root_list).next;
    while p != root_list as *mut list_head {
        let r = list_entry(p, mem::size_of::<dlm_rsb>(), res_root_list);
        (*r).res_dir_nodeid = dlm_hash2nodeid(ls, (*r).res_hash);
        p = (*p).next;
    }
}

pub unsafe fn dlm_recover_directory(ls: *mut dlm_ls, seq: u64) -> i32 {
    let mut error: i32 = -ENOMEM;
    let mut last_name: *mut i8 = ptr::null_mut();
    let mut count: u32 = 0;
    let mut count_bad: u32 = 0;
    let mut count_add: u32 = 0;
    log_rinfo(ls, "dlm_recover_directory");
    if dlm_no_directory(ls) { error = 0; dlm_set_recover_status(ls, DLM_RS_DIR); return error; }
    last_name = kmalloc(DLM_RESNAME_MAXLEN, GFP_NOFS) as *mut i8;
    if last_name.is_null() { return error; }
    let mut mp = (*ls).ls_nodes.next;
    while mp != &mut (*ls).ls_nodes as *mut list_head {
        let memb = list_entry(mp, mem::size_of::<dlm_member>(), list);
        if (*memb).nodeid != dlm_our_nodeid() {
            ptr::write_bytes(last_name as *mut u8, 0, DLM_RESNAME_MAXLEN);
            let mut last_len = 0i32;
            loop {
                if dlm_recovery_stopped(ls) { error = -EINTR; break; }
                error = dlm_rcom_names(ls, (*memb).nodeid, last_name, last_len, seq);
                if error != 0 { break; }
                cond_resched();
                let mut b = (*ls).ls_recover_buf.rc_buf;
                let mut left = le16_to_cpu((*ls).ls_recover_buf.rc_header.h_length) as i32 - mem::size_of::<dlm_rcom>() as i32;
                loop {
                    error = -EINVAL;
                    if left < 2 { break; }
                    let namelen = be16_to_cpu(ptr::read_unaligned(b as *const u16));
                    b = b.add(2); left -= 2;
                    if namelen == 0xffff { error = 0; break; }
                    if namelen == 0 || namelen as i32 > left || namelen as usize > DLM_RESNAME_MAXLEN { break; }
                    let mut nodeid = 0; let mut result = 0;
                    error = dlm_master_lookup(ls, (*memb).nodeid, b, namelen, DLM_LU_RECOVER_DIR, &mut nodeid, &mut result);
                    if error != 0 { log_error(ls, "recover_dir lookup %d", error); break; }
                    if result == DLM_LU_MATCH && nodeid != (*memb).nodeid {
                        count_bad += 1;
                        log_error(ls, "recover_dir lookup %d nodeid %d memb %d bad %u", result, nodeid, (*memb).nodeid, count_bad);
                        print_hex_dump_bytes("dlm_recover_dir ", DUMP_PREFIX_NONE, b, namelen as usize);
                    }
                    if result == DLM_LU_ADD { count_add += 1; }
                    last_len = namelen as i32;
                    ptr::copy_nonoverlapping(b, last_name as *mut u8, namelen as usize);
                    b = b.add(namelen as usize); left -= namelen as i32; count += 1;
                }
                if error != 0 || left <= 0 { break; }
            }
        }
        if error != 0 { break; }
        mp = (*mp).next;
    }
    if error == 0 { dlm_set_recover_status(ls, DLM_RS_DIR); log_rinfo(ls, "dlm_recover_directory %u in %u new", count, count_add); }
    kfree(last_name as *mut _); error
}

unsafe fn find_rsb_root(ls: *mut dlm_ls, name: *const i8, len: i32) -> *mut dlm_rsb {
    let mut r = ptr::null_mut();
    read_lock_bh(&mut (*ls).ls_rsbtbl_lock);
    let rv = dlm_search_rsb_tree(&mut (*ls).ls_rsbtbl, name, len, &mut r);
    read_unlock_bh(&mut (*ls).ls_rsbtbl_lock);
    if rv == 0 { return r; }
    let mut p = (*ls).ls_masters_list.next;
    while p != &mut (*ls).ls_masters_list as *mut list_head {
        r = list_entry(p, mem::size_of::<dlm_rsb>(), res_masters_list);
        if len == (*r).res_length && libc::memcmp(name as *const _, (*r).res_name as *const _, len as usize) == 0 { return r; }
        p = (*p).next;
    }
    ptr::null_mut()
}

#[repr(C)]
pub struct dlm_dir_dump { pub seq_init: u64, pub nodeid_init: u64, pub last: *mut list_head, pub sent_res: u32, pub sent_msg: u32, pub list: list_head }

unsafe fn drop_dir_ctx(ls: *mut dlm_ls, nodeid: i32) {
    let mut p = (*ls).ls_dir_dump_list.next;
    while p != &mut (*ls).ls_dir_dump_list as *mut list_head { let next = (*p).next; let dd = list_entry(p, mem::size_of::<dlm_dir_dump>(), list); if (*dd).nodeid_init as i32 == nodeid { list_del(&mut (*dd).list); kfree(dd as *mut _); } p = next; }
}

unsafe fn lookup_dir_dump(ls: *mut dlm_ls, nodeid: i32) -> *mut dlm_dir_dump { let mut p=(*ls).ls_dir_dump_list.next; while p != &mut (*ls).ls_dir_dump_list as *mut list_head { let d=list_entry(p,mem::size_of::<dlm_dir_dump>(),list); if (*d).nodeid_init as i32==nodeid{return d;} p=(*p).next;} ptr::null_mut() }

unsafe fn init_dir_dump(ls: *mut dlm_ls, nodeid: i32) -> *mut dlm_dir_dump {
    if !lookup_dir_dump(ls,nodeid).is_null() { drop_dir_ctx(ls,nodeid); }
    let dd = kzalloc(mem::size_of::<dlm_dir_dump>(), GFP_ATOMIC) as *mut dlm_dir_dump; if dd.is_null(){return dd;}
    (*dd).seq_init=(*ls).ls_recover_seq; (*dd).nodeid_init=nodeid as u64; list_add(&mut (*dd).list,&mut (*ls).ls_dir_dump_list); dd
}

pub unsafe fn dlm_copy_master_names(ls:*mut dlm_ls,inbuf:*const i8,inlen:i32,outbuf:*mut i8,outlen:i32,nodeid:i32) {
    let mut dd; let mut list; if inlen>1 { dd=lookup_dir_dump(ls,nodeid); if dd.is_null(){return;} let r=find_rsb_root(ls,inbuf,inlen); if r.is_null(){return;} list=(*(*r).res_masters_list.next).next; } else { dd=init_dir_dump(ls,nodeid); if dd.is_null(){return;} list=(*ls).ls_masters_list.next; (*dd).last=list; }
    let mut offset=0i32; while list != &mut (*ls).ls_masters_list as *mut list_head { let r=list_entry(list,mem::size_of::<dlm_rsb>(),res_masters_list); if dlm_dir_nodeid(r)!=nodeid {list=(*list).next;continue;} if offset+4+(*r).res_length>outlen { ptr::write_unaligned(outbuf.add(offset as usize) as *mut u16,cpu_to_be16(0)); return; } ptr::write_unaligned(outbuf.add(offset as usize) as *mut u16,cpu_to_be16((*r).res_length as u16)); offset+=2; ptr::copy_nonoverlapping((*r).res_name as *const u8,outbuf.add(offset as usize) as *mut u8,(*r).res_length as usize); offset+=(*r).res_length; (*dd).last=list; list=(*list).next; }
    if offset+2<=outlen { ptr::write_unaligned(outbuf.add(offset as usize) as *mut u16,cpu_to_be16(0xffff)); list_del_init(&mut (*dd).list); kfree(dd as *mut _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
