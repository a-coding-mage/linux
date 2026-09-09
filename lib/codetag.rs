// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel translation environment.

#[repr(C)]
pub struct CodetagType {
    pub link: list_head,
    pub count: c_uint,
    pub mod_idr: idr,
    pub mod_lock: rw_semaphore,
    pub desc: codetag_type_desc,
    pub next_mod_seq: c_ulong,
    pub content_id: c_ulong,
}

#[repr(C)]
pub struct CodetagRange {
    pub start: *mut codetag,
    pub stop: *mut codetag,
}

#[repr(C)]
pub struct CodetagModule {
    pub mod_: *mut module,
    pub range: CodetagRange,
    pub mod_seq: c_ulong,
}

static mut CODETAG_LOCK: mutex = DEFINE_MUTEX!();
static mut CODETAG_TYPES: list_head = LIST_HEAD!();

pub unsafe fn codetag_lock_module_list(cttype: *mut CodetagType) {
    down_read(&mut (*cttype).mod_lock);
}

pub unsafe fn codetag_trylock_module_list(cttype: *mut CodetagType) -> bool {
    down_read_trylock(&mut (*cttype).mod_lock) != 0
}

pub unsafe fn codetag_unlock_module_list(cttype: *mut CodetagType) {
    up_read(&mut (*cttype).mod_lock);
}

pub unsafe fn codetag_get_content_id(cttype: *mut CodetagType) -> c_ulong {
    lockdep_assert_held(&(*cttype).mod_lock);
    (*cttype).content_id
}

pub unsafe fn codetag_get_count(cttype: *mut CodetagType) -> c_uint {
    lockdep_assert_held(&(*cttype).mod_lock);
    (*cttype).count
}

pub unsafe fn codetag_get_ct_iter(cttype: *mut CodetagType) -> codetag_iterator {
    codetag_iterator {
        cttype,
        cmod: core::ptr::null_mut(),
        mod_id: 0,
        ct: core::ptr::null_mut(),
        mod_seq: 0,
    }
}

unsafe fn get_first_module_ct(cmod: *mut CodetagModule) -> *mut codetag {
    if (*cmod).range.start < (*cmod).range.stop { (*cmod).range.start } else { core::ptr::null_mut() }
}

unsafe fn get_next_module_ct(iter: *mut codetag_iterator) -> *mut codetag {
    let res = ((*iter).ct as *mut u8).add((*(*iter).cttype).desc.tag_size as usize) as *mut codetag;
    if res < (*(*iter).cmod).range.stop { res } else { core::ptr::null_mut() }
}

pub unsafe fn codetag_next_ct(iter: *mut codetag_iterator) -> *mut codetag {
    let cttype = (*iter).cttype;
    let mut cmod: *mut CodetagModule;
    let mut ct: *mut codetag;
    lockdep_assert_held(&(*cttype).mod_lock);
    if idr_is_empty(&(*cttype).mod_idr) { return core::ptr::null_mut(); }
    ct = core::ptr::null_mut();
    loop {
        cmod = idr_find(&mut (*cttype).mod_idr, (*iter).mod_id);
        if cmod.is_null() { cmod = idr_get_next_ul(&mut (*cttype).mod_idr, &mut (*iter).mod_id); }
        if cmod.is_null() { break; }
        if (*iter).cmod.is_null() || (*iter).mod_seq != (*cmod).mod_seq {
            (*iter).cmod = cmod;
            (*iter).mod_seq = (*cmod).mod_seq;
            ct = get_first_module_ct(cmod);
        } else { ct = get_next_module_ct(iter); }
        if !ct.is_null() { break; }
        (*iter).mod_id += 1;
    }
    (*iter).ct = ct;
    ct
}

pub unsafe fn codetag_to_text(out: *mut seq_buf, ct: *mut codetag) {
    if !(*ct).modname.is_null() {
        seq_buf_printf(out, "%s:%u [%s] func:%s", (*ct).filename, (*ct).lineno, (*ct).modname, (*ct).function);
    } else {
        seq_buf_printf(out, "%s:%u func:%s", (*ct).filename, (*ct).lineno, (*ct).function);
    }
}

unsafe fn range_size(cttype: *const CodetagType, range: *const CodetagRange) -> usize {
    ((*range).stop as *const u8 as usize - (*range).start as *const u8 as usize) / (*cttype).desc.tag_size as usize
}

unsafe fn get_symbol(mod_: *mut module, prefix: *const c_char, name: *const c_char) -> *mut core::ffi::c_void {
    let mut sb = DECLARE_SEQ_BUF!(KSYM_NAME_LEN);
    seq_buf_printf(&mut sb, "%s%s", prefix, name);
    if seq_buf_has_overflowed(&sb) { return core::ptr::null_mut(); }
    let buf = seq_buf_str(&sb);
    preempt_disable();
    let ret = if !mod_.is_null() { find_kallsyms_symbol_value(mod_, buf) as *mut _ } else { kallsyms_lookup_name(buf) as *mut _ };
    preempt_enable();
    ret
}

unsafe fn get_section_range(mod_: *mut module, section: *const c_char) -> CodetagRange {
    CodetagRange { start: get_symbol(mod_, CODETAG_SECTION_START_PREFIX, section) as *mut codetag, stop: get_symbol(mod_, CODETAG_SECTION_STOP_PREFIX, section) as *mut codetag }
}

unsafe fn get_mod_name(_mod: *mut module) -> *const c_char { c"(built-in)".as_ptr() }

unsafe fn codetag_module_init(cttype: *mut CodetagType, mod_: *mut module) -> c_int {
    let range = get_section_range(mod_, (*cttype).desc.section);
    if range.start.is_null() || range.stop.is_null() { pr_warn!("Failed to load code tags of type %s from the module %s\n", (*cttype).desc.section, get_mod_name(mod_)); return -EINVAL; }
    if range.start == range.stop { return 0; }
    BUG_ON!(range.start > range.stop);
    let cmod = kmalloc_obj::<CodetagModule>();
    if cmod.is_null() { return -ENOMEM; }
    (*cmod).mod_ = mod_; (*cmod).range = range;
    down_write(&mut (*cttype).mod_lock);
    (*cmod).mod_seq = (*cttype).next_mod_seq.wrapping_add(1); (*cttype).next_mod_seq = (*cmod).mod_seq; (*cttype).content_id = (*cttype).content_id.wrapping_add(1);
    let mod_id = idr_alloc(&mut (*cttype).mod_idr, cmod, 0, 0, GFP_KERNEL);
    let err = if mod_id >= 0 { (*cttype).count += range_size(cttype, &range) as c_uint; 0 } else { mod_id };
    up_write(&mut (*cttype).mod_lock);
    if err < 0 { kfree(cmod); return err; } 0
}

// CONFIG_MODULES-only interfaces and implementation.
pub unsafe fn codetag_needs_module_section(mod_: *mut module, name: *const c_char, size: c_ulong) -> bool {
    let prefix = c".codetag.".as_ptr();
    if strncmp(name, prefix, strlen(prefix)) != 0 { return false; }
    let type_name = name.add(strlen(prefix));
    let mut ret = false;
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        if strcmp(type_name, (*cttype).desc.section) == 0 {
            if !(*cttype).desc.needs_section_mem.is_some() { break; }
            down_write(&mut (*cttype).mod_lock);
            ret = ((*cttype).desc.needs_section_mem.unwrap())(mod_, size);
            up_write(&mut (*cttype).mod_lock);
            break;
        }
    });
    mutex_unlock(&mut CODETAG_LOCK); ret
}

pub unsafe fn codetag_alloc_module_section(mod_: *mut module, name: *const c_char, size: c_ulong, prepend: c_uint, align: c_ulong) -> *mut core::ffi::c_void {
    let type_name = name.add(strlen(c".codetag.".as_ptr()));
    let mut ret = ERR_PTR(-EINVAL);
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        if strcmp(type_name, (*cttype).desc.section) == 0 {
            WARN_ON!(!(*cttype).desc.alloc_section_mem.is_some());
            if !(*cttype).desc.alloc_section_mem.is_some() { break; }
            down_write(&mut (*cttype).mod_lock);
            ret = ((*cttype).desc.alloc_section_mem.unwrap())(mod_, size, prepend, align);
            up_write(&mut (*cttype).mod_lock); break;
        }
    });
    mutex_unlock(&mut CODETAG_LOCK); ret
}

pub unsafe fn codetag_free_module_sections(mod_: *mut module) {
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        if !(*cttype).desc.free_section_mem.is_some() { continue; }
        down_write(&mut (*cttype).mod_lock);
        ((*cttype).desc.free_section_mem.unwrap())(mod_, false);
        up_write(&mut (*cttype).mod_lock);
    });
    mutex_unlock(&mut CODETAG_LOCK);
}

pub unsafe fn codetag_module_replaced(mod_: *mut module, new_mod: *mut module) {
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        if !(*cttype).desc.module_replaced.is_some() { continue; }
        down_write(&mut (*cttype).mod_lock);
        ((*cttype).desc.module_replaced.unwrap())(mod_, new_mod);
        up_write(&mut (*cttype).mod_lock);
    });
    mutex_unlock(&mut CODETAG_LOCK);
}

pub unsafe fn codetag_load_module(mod_: *mut module) -> c_int {
    if mod_.is_null() { return 0; }
    let mut ret = 0;
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        ret = codetag_module_init(cttype, mod_); if ret != 0 { break; }
    });
    mutex_unlock(&mut CODETAG_LOCK); ret
}

pub unsafe fn codetag_unload_module(mod_: *mut module) {
    if mod_.is_null() { return; }
    kvfree_rcu_barrier();
    mutex_lock(&mut CODETAG_LOCK);
    list_for_each_entry!(cttype in CODETAG_TYPES, link, {
        let mut found: *mut CodetagModule = core::ptr::null_mut();
        let mut cmod: *mut CodetagModule;
        let mut tmp: c_ulong = 0; let mut mod_id: c_ulong = 0;
        down_write(&mut (*cttype).mod_lock);
        idr_for_each_entry_ul!(&mut (*cttype).mod_idr, cmod, tmp, mod_id, {
            if !(*cmod).mod_.is_null() && (*cmod).mod_ == mod_ { found = cmod; break; }
        });
        if !found.is_null() {
            if (*cttype).desc.module_unload.is_some() { ((*cttype).desc.module_unload.unwrap())((*found).mod_, (*found).range.start, (*found).range.stop); }
            (*cttype).count -= range_size(cttype, &(*found).range) as c_uint;
            idr_remove(&mut (*cttype).mod_idr, mod_id); kfree(found); (*cttype).content_id = (*cttype).content_id.wrapping_add(1);
        }
        up_write(&mut (*cttype).mod_lock);
        if !found.is_null() && (*cttype).desc.free_section_mem.is_some() { ((*cttype).desc.free_section_mem.unwrap())(mod_, true); }
    });
    mutex_unlock(&mut CODETAG_LOCK);
}

pub unsafe fn codetag_register_type(desc: *const codetag_type_desc) -> *mut CodetagType {
    BUG_ON!((*desc).tag_size <= 0);
    let cttype = kzalloc_obj::<CodetagType>();
    if cttype.is_null() { return ERR_PTR(-ENOMEM); }
    (*cttype).desc = *desc; idr_init(&mut (*cttype).mod_idr); init_rwsem(&mut (*cttype).mod_lock);
    let err = codetag_module_init(cttype, core::ptr::null_mut());
    if err != 0 { kfree(cttype); return ERR_PTR(err); }
    mutex_lock(&mut CODETAG_LOCK); list_add_tail(&mut (*cttype).link, &mut CODETAG_TYPES); mutex_unlock(&mut CODETAG_LOCK); cttype
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
