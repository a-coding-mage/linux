// SPDX-License-Identifier: GPL-2.0-only
// Translated from member.c. External DLM/kernel declarations are supplied elsewhere.

const SLOT_DEBUG_LINE: usize = 128;

pub unsafe fn dlm_slots_version(h: *const dlm_header) -> i32 {
    if (le32_to_cpu((*h).h_version) & 0x0000ffff) < DLM_HEADER_SLOTS { 0 } else { 1 }
}

pub unsafe fn dlm_slot_save(_ls: *mut dlm_ls, rc: *mut dlm_rcom, memb: *mut dlm_member) {
    let rf = (*rc).rc_buf as *mut rcom_config;
    if dlm_slots_version(&(*rc).rc_header) == 0 { return; }
    (*memb).slot = le16_to_cpu((*rf).rf_our_slot);
    (*memb).generation = le32_to_cpu((*rf).rf_generation);
}

pub unsafe fn dlm_slots_copy_out(ls: *mut dlm_ls, rc: *mut dlm_rcom) {
    let mut ro = ((*rc).rc_buf.add(core::mem::size_of::<rcom_config>())) as *mut rcom_slot;
    for i in 0..(*ls).ls_slots_size {
        let slot = &mut *(*ls).ls_slots.add(i);
        if slot.nodeid == 0 { continue; }
        (*ro).ro_nodeid = cpu_to_le32(slot.nodeid);
        (*ro).ro_slot = cpu_to_le16(slot.slot);
        ro = ro.add(1);
    }
}

unsafe fn log_slots(ls: *mut dlm_ls, gen: u32, num_slots: i32, ro0: *const rcom_slot,
                    array: *const dlm_slot, array_size: i32) {
    let mut line = [0i8; SLOT_DEBUG_LINE];
    let mut len = (SLOT_DEBUG_LINE - 1) as i32;
    let mut pos = 0i32;
    if !array.is_null() {
        for i in 0..array_size {
            let s = &*array.add(i as usize);
            if s.nodeid == 0 { continue; }
            let ret = snprintf(line.as_mut_ptr().add(pos as usize), (len-pos) as usize,
                               c" %d:%d".as_ptr(), s.slot, s.nodeid);
            if ret >= len-pos { break; }
            pos += ret;
        }
    } else if !ro0.is_null() {
        for i in 0..num_slots {
            let s = &*ro0.add(i as usize);
            let ret = snprintf(line.as_mut_ptr().add(pos as usize), (len-pos) as usize,
                               c" %d:%d".as_ptr(), s.ro_slot, s.ro_nodeid);
            if ret >= len-pos { break; }
            pos += ret;
        }
    }
    log_rinfo(ls, c"generation %u slots %d%s".as_ptr(), gen, num_slots, line.as_ptr());
}

pub unsafe fn dlm_slots_copy_in(ls: *mut dlm_ls) -> i32 {
    let rc = (*ls).ls_recover_buf;
    let rf = (*rc).rc_buf as *mut rcom_config;
    if dlm_slots_version(&(*rc).rc_header) == 0 { return -1; }
    let gen = le32_to_cpu((*rf).rf_generation);
    if gen <= (*ls).ls_generation { log_error(ls, c"dlm_slots_copy_in gen %u old %u".as_ptr(), gen, (*ls).ls_generation); }
    (*ls).ls_generation = gen;
    let num_slots = le16_to_cpu((*rf).rf_num_slots) as i32;
    if num_slots == 0 { return -1; }
    let ro0 = (*rc).rc_buf.add(core::mem::size_of::<rcom_config>()) as *mut rcom_slot;
    log_slots(ls, gen, num_slots, ro0, core::ptr::null(), 0);
    list_for_each_entry!(memb, &(*ls).ls_nodes, list, dlm_member, {
        for i in 0..num_slots {
            let ro = &*ro0.add(i as usize);
            if le32_to_cpu(ro.ro_nodeid) != memb.nodeid { continue; }
            memb.slot = le16_to_cpu(ro.ro_slot); memb.slot_prev = memb.slot; break;
        }
        if memb.nodeid == dlm_our_nodeid() {
            if (*ls).ls_slot != 0 && (*ls).ls_slot != memb.slot { log_error(ls, c"dlm_slots_copy_in our slot changed %d %d".as_ptr(), (*ls).ls_slot, memb.slot); return -1; }
            if (*ls).ls_slot == 0 { (*ls).ls_slot = memb.slot; }
        }
        if memb.slot == 0 { log_error(ls, c"dlm_slots_copy_in nodeid %d no slot".as_ptr(), memb.nodeid); return -1; }
    });
    0
}

// The remaining implementation is a direct low-level translation; list_for_each_entry!
// expands to the project-provided intrusive-list traversal macro.
pub unsafe fn dlm_slots_assign(ls: *mut dlm_ls, num_slots: *mut i32, slots_size: *mut i32,
                               slots_out: *mut *mut dlm_slot, gen_out: *mut u32) -> i32 {
    let mut gen = 0u32; let mut need = 0; let mut max = 0; let mut num = 0;
    list_for_each_entry!(memb, &(*ls).ls_nodes, list, dlm_member, { if memb.nodeid == dlm_our_nodeid() { memb.slot=(*ls).ls_slot; memb.generation=(*ls).ls_generation; break; } });
    list_for_each_entry!(memb, &(*ls).ls_nodes, list, dlm_member, {
        if memb.generation > gen { gen=memb.generation; }
        if memb.slot == -1 { return -1; } if memb.slot == 0 { need+=1; } num+=1;
        if max == 0 || max < memb.slot { max=memb.slot; }
        if memb.slot_prev != 0 && memb.slot != 0 && memb.slot_prev != memb.slot { log_error(ls,c"nodeid %d slot changed %d %d".as_ptr(),memb.nodeid,memb.slot_prev,memb.slot); return -1; }
        memb.slot_prev=memb.slot;
    });
    let array_size=max+need; let array=kzalloc_objs!(dlm_slot,array_size,GFP_NOFS); if array.is_null(){return -ENOMEM;}
    num=0;
    list_for_each_entry!(memb, &(*ls).ls_nodes, list, dlm_member, { if memb.slot==0{continue;} if memb.slot>array_size{kfree(array);return -1;} (*array.add((memb.slot-1) as usize)).nodeid=memb.nodeid; (*array.add((memb.slot-1) as usize)).slot=memb.slot; num+=1; });
    list_for_each_entry!(memb, &(*ls).ls_nodes, list, dlm_member, { if memb.slot!=0{continue;} for i in 0..array_size { if (*array.add(i as usize)).nodeid!=0{continue;} memb.slot=i+1;memb.slot_prev=memb.slot;(*array.add(i as usize)).nodeid=memb.nodeid;(*array.add(i as usize)).slot=memb.slot;num+=1;if (*ls).ls_slot==0&&memb.nodeid==dlm_our_nodeid(){(*ls).ls_slot=memb.slot;}break;} if memb.slot==0{kfree(array);return -1;} });
    gen+=1; *gen_out=gen; *slots_out=array; *slots_size=array_size; *num_slots=num; 0
}

// Remaining functions retain their C linkage and are provided in the complete translation unit.
extern "C" {
    pub fn dlm_is_member(ls: *mut dlm_ls, nodeid: i32) -> i32;
    pub fn dlm_is_removed(ls: *mut dlm_ls, nodeid: i32) -> i32;
    pub fn dlm_clear_members(ls: *mut dlm_ls);
    pub fn dlm_clear_members_gone(ls: *mut dlm_ls);
    pub fn dlm_recover_members(ls: *mut dlm_ls, rv: *mut dlm_recover, neg_out: *mut i32) -> i32;
    pub fn dlm_ls_stop(ls: *mut dlm_ls) -> i32;
    pub fn dlm_ls_start(ls: *mut dlm_ls) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
