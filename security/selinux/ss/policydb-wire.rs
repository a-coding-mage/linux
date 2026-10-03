// SPDX-License-Identifier: GPL-2.0-only
// Included by policydb.rs: all C ABI types come from configured bindgen output.
// Translation of policydb.c, range_read() through policydb_write().

macro_rules! wire_try {
    ($operation:expr) => {{
        let wire_rc = $operation;
        if wire_rc != 0 { return wire_rc; }
    }};
}

// Keep each C next_entry/put_entry batch atomic with respect to fp->len/data.
unsafe fn wire_get(fp: *mut policy_file, words: &mut [u32]) -> c_int {
    let rc = next_entry(words.as_mut_ptr().cast(), fp, words.len() * mem::size_of::<u32>());
    if rc != 0 { return rc; }
    for word in words { *word = u32::from_le(*word); }
    0
}

unsafe fn wire_put_count<const N: usize>(fp: *mut policy_file, mut words: [u32; N], n: usize) -> c_int {
    for word in &mut words { *word = word.to_le(); }
    put_entry(words.as_ptr().cast(), mem::size_of::<u32>(), n, fp)
}

unsafe fn wire_put<const N: usize>(fp: *mut policy_file, words: [u32; N]) -> c_int {
    wire_put_count(fp, words, N)
}

unsafe fn range_read(p: *mut policydb, fp: *mut policy_file) -> c_int {
    if (*p).policyvers < POLICYDB_VERSION_MLS { return 0; }
    let mut buf = [0u32; 2];
    wire_try!(wire_get(fp, &mut buf[..1]));
    let nel = buf[0];
    wire_try!(size_check(3 * mem::size_of::<u32>(), nel as usize, fp));
    wire_try!(hashtab_init(&mut (*p).range_tr, nel));
    let mut rt: *mut range_trans = ptr::null_mut();
    let mut r: *mut mls_range = ptr::null_mut();
    let rc = (|| -> c_int {
        for _ in 0..nel {
            rt = zalloc_obj();
            if rt.is_null() { return -ENOMEM; }
            wire_try!(wire_get(fp, &mut buf));
            (*rt).source_type = buf[0];
            (*rt).target_type = buf[1];
            if (*p).policyvers >= POLICYDB_VERSION_RANGETRANS {
                wire_try!(wire_get(fp, &mut buf[..1]));
                if buf[0] > u16::MAX as u32 { return -EINVAL; }
                (*rt).target_class = buf[0] as u16;
            } else { (*rt).target_class = (*p).process_class; }
            if !policydb_type_isvalid(p, (*rt).source_type)
                || !policydb_type_isvalid(p, (*rt).target_type)
                || !policydb_class_isvalid(p, (*rt).target_class) { return -EINVAL; }
            r = zalloc_obj();
            if r.is_null() { return -ENOMEM; }
            wire_try!(mls_read_range_helper(r, fp));
            if !mls_range_isvalid(p, r) {
                pr_warn(b"SELinux:  rangetrans:  invalid range\n\0".as_ptr().cast());
                return -EINVAL;
            }
            wire_try!(hashtab_insert(&mut (*p).range_tr, rt.cast(), r.cast(), rangetr_key_params));
            rt = ptr::null_mut();
            r = ptr::null_mut();
        }
        hash_eval(&mut (*p).range_tr, b"rangetr\0".as_ptr().cast(), ptr::null());
        0
    })();
    // Match C: partially read range bitmaps are not separately destroyed here.
    kfree(rt.cast());
    kfree(r.cast());
    if rc != 0 { pr_err(b"SELinux:  invalid range\n\0".as_ptr().cast()); }
    rc
}

unsafe fn filename_trans_read_helper_compat(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf = [0u32; 4];
    wire_try!(wire_get(fp, &mut buf[..1]));
    let mut name: *mut c_char = ptr::null_mut();
    wire_try!(str_read(&mut name, GFP_KERNEL, fp, buf[0]));
    let mut ft: *mut filename_trans_key = ptr::null_mut();
    let mut datum: *mut filename_trans_datum = ptr::null_mut();
    // `out` frees datum in the C reader, while the two final bitmap updates
    // return directly and leave the inserted object under policydb ownership.
    let mut direct_return = false;
    let rc = (|| -> c_int {
        wire_try!(wire_get(fp, &mut buf));
        let stype = buf[0];
        if !policydb_type_isvalid(p, stype) { return -EINVAL; }
        let mut key: filename_trans_key = mem::zeroed();
        key.ttype = buf[1];
        if !policydb_type_isvalid(p, key.ttype) { return -EINVAL; }
        let val = buf[2];
        if val > u16::MAX as u32 || !policydb_class_isvalid(p, val as u16) { return -EINVAL; }
        key.tclass = val as u16;
        key.name = name;
        let otype = buf[3];
        if !policydb_simpletype_isvalid(p, otype) { return -EINVAL; }
        let mut last: *mut filename_trans_datum = ptr::null_mut();
        datum = policydb_filenametr_search(p, &mut key);
        while !datum.is_null() {
            if ebitmap_get_bit(&(*datum).stypes, stype.wrapping_sub(1)) != 0 {
                // Legacy conflicting/duplicate rules are ignored.
                datum = ptr::null_mut();
                return 0;
            }
            if (*datum).otype == otype { break; }
            last = datum;
            datum = (*datum).next;
        }
        if datum.is_null() {
            datum = malloc_obj();
            if datum.is_null() { return -ENOMEM; }
            ebitmap_init(ptr::addr_of_mut!((*datum).stypes));
            (*datum).otype = otype;
            (*datum).next = ptr::null_mut();
            if !last.is_null() { (*last).next = datum; }
            else {
                ft = kmemdup((&key as *const filename_trans_key).cast(), mem::size_of::<filename_trans_key>(), GFP_KERNEL).cast();
                if ft.is_null() { return -ENOMEM; }
                wire_try!(hashtab_insert(&mut (*p).filename_trans, ft.cast(), datum.cast(), filenametr_key_params));
                name = ptr::null_mut();
                let rc = ebitmap_set_bit(&mut (*p).filename_trans_ttypes, key.ttype, 1);
                if rc != 0 { direct_return = true; return rc; }
            }
        }
        kfree(name.cast());
        name = ptr::null_mut();
        direct_return = true;
        ebitmap_set_bit(&mut (*datum).stypes, stype.wrapping_sub(1), 1)
    })();
    if direct_return { return rc; }
    kfree(ft.cast());
    kfree(name.cast());
    kfree(datum.cast());
    if rc != 0 { pr_err(b"SELinux:  invalid compat filename transition\n\0".as_ptr().cast()); }
    rc
}

unsafe fn filename_trans_read_helper(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf = [0u32; 3];
    wire_try!(wire_get(fp, &mut buf[..1]));
    let mut name: *mut c_char = ptr::null_mut();
    wire_try!(str_read(&mut name, GFP_KERNEL, fp, buf[0]));
    let mut ft: *mut filename_trans_key = ptr::null_mut();
    let mut first: *mut filename_trans_datum = ptr::null_mut();
    let mut inserted = false;
    let rc = (|| -> c_int {
        wire_try!(wire_get(fp, &mut buf));
        let ttype = buf[0];
        if !policydb_type_isvalid(p, ttype) { return -EINVAL; }
        let val = buf[1];
        if val > u16::MAX as u32 || !policydb_class_isvalid(p, val as u16) { return -EINVAL; }
        let tclass = val as u16;
        let ndatum = buf[2];
        if ndatum == 0 {
            pr_err(b"SELinux:  Filename transition key with no datum\n\0".as_ptr().cast());
            return -ENOENT;
        }
        let mut dst: *mut *mut filename_trans_datum = &mut first;
        for _ in 0..ndatum {
            let datum: *mut filename_trans_datum = malloc_obj();
            if datum.is_null() { return -ENOMEM; }
            (*datum).next = ptr::null_mut();
            *dst = datum;
            // ebitmap_read initializes stypes, including on failure.
            wire_try!(ebitmap_read(ptr::addr_of_mut!((*datum).stypes), fp));
            wire_try!(wire_get(fp, &mut buf[..1]));
            (*datum).otype = buf[0];
            if !policydb_simpletype_isvalid(p, (*datum).otype) { return -EINVAL; }
            dst = &mut (*datum).next;
        }
        ft = malloc_obj();
        if ft.is_null() { return -ENOMEM; }
        (*ft).ttype = ttype;
        (*ft).tclass = tclass;
        (*ft).name = name;
        let rc = hashtab_insert(&mut (*p).filename_trans, ft.cast(), first.cast(), filenametr_key_params);
        if rc == -EEXIST { pr_err(b"SELinux:  Duplicate filename transition key\n\0".as_ptr().cast()); }
        if rc != 0 { return rc; }
        inserted = true;
        ebitmap_set_bit(&mut (*p).filename_trans_ttypes, ttype, 1)
    })();
    if inserted { return rc; }
    kfree(ft.cast());
    kfree(name.cast());
    while !first.is_null() {
        let datum = first;
        first = (*first).next;
        ebitmap_destroy(&mut (*datum).stypes);
        kfree(datum.cast());
    }
    if rc != 0 { pr_err(b"SELinux:  invalid filename transition\n\0".as_ptr().cast()); }
    rc
}

unsafe fn filename_trans_read(p: *mut policydb, fp: *mut policy_file) -> c_int {
    if (*p).policyvers < POLICYDB_VERSION_FILENAME_TRANS { return 0; }
    let mut buf = [0u32; 1];
    wire_try!(wire_get(fp, &mut buf));
    let nel = buf[0];
    if (*p).policyvers < POLICYDB_VERSION_COMP_FTRANS {
        (*p).compat_filename_trans_count = nel;
        wire_try!(hashtab_init(&mut (*p).filename_trans, 1 << 11));
        for _ in 0..nel { wire_try!(filename_trans_read_helper_compat(p, fp)); }
    } else {
        wire_try!(hashtab_init(&mut (*p).filename_trans, nel));
        for _ in 0..nel { wire_try!(filename_trans_read_helper(p, fp)); }
    }
    hash_eval(&mut (*p).filename_trans, b"filenametr\0".as_ptr().cast(), ptr::null());
    0
}

unsafe fn genfs_read(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf = [0u32; 1];
    wire_try!(wire_get(fp, &mut buf));
    let nel = buf[0];
    let mut newc: *mut ocontext = ptr::null_mut();
    let mut newgenfs: *mut genfs = ptr::null_mut();
    let rc = (|| -> c_int {
        for _ in 0..nel {
            wire_try!(wire_get(fp, &mut buf));
            let len = buf[0];
            newgenfs = zalloc_obj();
            if newgenfs.is_null() { return -ENOMEM; }
            wire_try!(str_read(&mut (*newgenfs).fstype, GFP_KERNEL, fp, len));
            let mut prev: *mut genfs = ptr::null_mut();
            let mut cur = (*p).genfs;
            while !cur.is_null() {
                let cmp = strcmp((*newgenfs).fstype, (*cur).fstype);
                if cmp == 0 {
                    pr_err(b"SELinux:  dup genfs fstype %s\n\0".as_ptr().cast(), (*newgenfs).fstype);
                    return -EINVAL;
                }
                if cmp < 0 { break; }
                prev = cur;
                cur = (*cur).next;
            }
            (*newgenfs).next = cur;
            if !prev.is_null() { (*prev).next = newgenfs; } else { (*p).genfs = newgenfs; }
            cur = newgenfs;
            newgenfs = ptr::null_mut();
            wire_try!(wire_get(fp, &mut buf));
            let nel2 = buf[0];
            for _ in 0..nel2 {
                wire_try!(wire_get(fp, &mut buf));
                let len = buf[0];
                newc = zalloc_obj();
                if newc.is_null() { return -ENOMEM; }
                wire_try!(str_read(&mut (*newc).u.name, GFP_KERNEL, fp, len));
                wire_try!(wire_get(fp, &mut buf));
                let val = buf[0];
                if val > u16::MAX as u32 || (val != 0 && !policydb_class_isvalid(p, val as u16)) { return -EINVAL; }
                (*newc).v.sclass = val as u16;
                wire_try!(context_read_and_validate(&mut (*newc).context[0], p, fp));
                let mut l: *mut ocontext = ptr::null_mut();
                let mut c = (*cur).head;
                while !c.is_null() {
                    if strcmp((*newc).u.name, (*c).u.name) == 0
                        && ((*c).v.sclass == 0 || (*newc).v.sclass == 0 || (*newc).v.sclass == (*c).v.sclass) {
                        pr_err(b"SELinux:  dup genfs entry (%s,%s)\n\0".as_ptr().cast(), (*cur).fstype, (*c).u.name);
                        return -EINVAL;
                    }
                    // C assigns strlen into u32 before comparing.
                    let len = strlen((*newc).u.name) as u32;
                    let len2 = strlen((*c).u.name) as u32;
                    if len > len2 { break; }
                    l = c;
                    c = (*c).next;
                }
                (*newc).next = c;
                if !l.is_null() { (*l).next = newc; } else { (*cur).head = newc; }
                newc = ptr::null_mut();
            }
        }
        0
    })();
    if !newgenfs.is_null() { kfree((*newgenfs).fstype.cast()); kfree(newgenfs.cast()); }
    ocontext_destroy(newc, OCON_FSUSE);
    if rc != 0 { pr_err(b"SELinux:  invalid genfs\n\0".as_ptr().cast()); }
    rc
}

unsafe fn ocontext_read(p: *mut policydb, info: *const policydb_compat_info, fp: *mut policy_file) -> c_int {
    let rc = (|| -> c_int {
        let mut buf = [0u32; 3];
        for i in 0..(*info).ocon_num {
            wire_try!(wire_get(fp, &mut buf[..1]));
            let nel = buf[0];
            let mut l: *mut ocontext = ptr::null_mut();
            for _ in 0..nel {
                let c: *mut ocontext = zalloc_obj();
                if c.is_null() { return -ENOMEM; }
                if !l.is_null() { (*l).next = c; } else { (*p).ocontexts[i as usize] = c; }
                l = c;
                match i {
                    OCON_ISID => {
                        wire_try!(wire_get(fp, &mut buf[..1]));
                        (*c).sid[0] = buf[0];
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_FS | OCON_NETIF => {
                        wire_try!(wire_get(fp, &mut buf[..1]));
                        wire_try!(str_read(&mut (*c).u.name, GFP_KERNEL, fp, buf[0]));
                        if i == OCON_FS { pr_warn(b"SELinux:  void and deprecated fs ocon %s\n\0".as_ptr().cast(), (*c).u.name); }
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                        wire_try!(context_read_and_validate(&mut (*c).context[1], p, fp));
                    }
                    OCON_PORT => {
                        wire_try!(wire_get(fp, &mut buf));
                        if buf[0] > u8::MAX as u32 { return -EINVAL; }
                        (*c).u.port.protocol = buf[0] as u8;
                        if buf[1] > u16::MAX as u32 { return -EINVAL; }
                        (*c).u.port.low_port = buf[1] as u16;
                        if buf[2] > u16::MAX as u32 { return -EINVAL; }
                        (*c).u.port.high_port = buf[2] as u16;
                        if (*c).u.port.low_port == 0 || (*c).u.port.low_port > (*c).u.port.high_port { return -EINVAL; }
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_NODE => {
                        let mut nodebuf = [0u32; 2];
                        wire_try!(next_entry(nodebuf.as_mut_ptr().cast(), fp, 2 * mem::size_of::<u32>()));
                        (*c).u.node.addr = nodebuf[0];
                        (*c).u.node.mask = nodebuf[1];
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_FSUSE => {
                        wire_try!(wire_get(fp, &mut buf[..2]));
                        (*c).v.behavior = buf[0];
                        if (*c).v.behavior == SECURITY_FS_USE_MNTPOINT || (*c).v.behavior > SECURITY_FS_USE_MAX { return -EINVAL; }
                        wire_try!(str_read(&mut (*c).u.name, GFP_KERNEL, fp, buf[1]));
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_NODE6 => {
                        let mut nodebuf = [0u32; 8];
                        wire_try!(next_entry(nodebuf.as_mut_ptr().cast(), fp, 8 * mem::size_of::<u32>()));
                        for k in 0..4 { (*c).u.node6.addr[k] = nodebuf[k]; }
                        for k in 0..4 { (*c).u.node6.mask[k] = nodebuf[k + 4]; }
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_IBPKEY => {
                        let mut prefix = 0u64;
                        wire_try!(next_entry((&mut prefix as *mut u64).cast(), fp, mem::size_of::<u64>()));
                        (*c).u.ibpkey.subnet_prefix = u64::from_be(prefix);
                        wire_try!(wire_get(fp, &mut buf[..2]));
                        if buf[0] > u16::MAX as u32 || buf[1] > u16::MAX as u32 { return -EINVAL; }
                        (*c).u.ibpkey.low_pkey = buf[0] as u16;
                        (*c).u.ibpkey.high_pkey = buf[1] as u16;
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    OCON_IBENDPORT => {
                        wire_try!(wire_get(fp, &mut buf[..2]));
                        wire_try!(str_read(&mut (*c).u.ibendport.dev_name, GFP_KERNEL, fp, buf[0]));
                        if buf[1] > u8::MAX as u32 || buf[1] == 0 { return -EINVAL; }
                        (*c).u.ibendport.port = buf[1] as u8;
                        wire_try!(context_read_and_validate(&mut (*c).context[0], p, fp));
                    }
                    _ => {}
                }
            }
        }
        0
    })();
    if rc != 0 { pr_err(b"SELinux:  invalid ocon\n\0".as_ptr().cast()); }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn policydb_read(p: *mut policydb, fp: *mut policy_file) -> c_int {
    policydb_init(p);
    let mut rtk: *mut role_trans_key = ptr::null_mut();
    let mut rtd: *mut role_trans_datum = ptr::null_mut();
    // These three early symbol-table failures target C's `out`, not `bad`.
    let mut destroy_on_error = true;
    let rc = (|| -> c_int {
        let mut buf = [0u32; 4];
        wire_try!(wire_get(fp, &mut buf[..2]));
        if buf[0] != POLICYDB_MAGIC {
            pr_err(b"SELinux:  policydb magic number 0x%x does not match expected magic number 0x%x\n\0".as_ptr().cast(), buf[0], POLICYDB_MAGIC);
            return -EINVAL;
        }
        let len = buf[1];
        if len as usize != POLICYDB_STRING.len() - 1 {
            pr_err(b"SELinux:  policydb string length %d does not match expected length %zu\n\0".as_ptr().cast(), len, POLICYDB_STRING.len() - 1);
            return -EINVAL;
        }
        let mut policydb_str: *mut c_char = ptr::null_mut();
        let rc = str_read(&mut policydb_str, GFP_KERNEL, fp, len);
        if rc != 0 {
            if rc == -ENOMEM { pr_err(b"SELinux:  unable to allocate memory for policydb string of length %d\n\0".as_ptr().cast(), len); }
            else { pr_err(b"SELinux:  truncated policydb string identifier\n\0".as_ptr().cast()); }
            return rc;
        }
        if strcmp(policydb_str, POLICYDB_STRING.as_ptr().cast()) != 0 {
            pr_err(b"SELinux:  policydb string %s does not match my string %s\n\0".as_ptr().cast(), policydb_str, POLICYDB_STRING.as_ptr().cast::<c_char>());
            kfree(policydb_str.cast());
            return -EINVAL;
        }
        kfree(policydb_str.cast());
        wire_try!(wire_get(fp, &mut buf));
        (*p).policyvers = buf[0];
        if (*p).policyvers < POLICYDB_VERSION_MIN || (*p).policyvers > POLICYDB_VERSION_MAX {
            pr_err(b"SELinux:  policydb version %d does not match my version range %d-%d\n\0".as_ptr().cast(), buf[0], POLICYDB_VERSION_MIN, POLICYDB_VERSION_MAX);
            return -EINVAL;
        }
        if buf[1] & POLICYDB_CONFIG_MLS != 0 {
            (*p).mls_enabled = 1;
            if (*p).policyvers < POLICYDB_VERSION_MLS {
                pr_err(b"SELinux: security policydb version %d (MLS) not backwards compatible\n\0".as_ptr().cast(), (*p).policyvers);
                return -EINVAL;
            }
        }
        (*p).set_reject_unknown((buf[1] & REJECT_UNKNOWN != 0) as u32);
        (*p).set_allow_unknown((buf[1] & ALLOW_UNKNOWN != 0) as u32);
        if (*p).policyvers >= POLICYDB_VERSION_POLCAP { wire_try!(ebitmap_read(&mut (*p).policycaps, fp)); }
        if (*p).policyvers >= POLICYDB_VERSION_PERMISSIVE { wire_try!(ebitmap_read(&mut (*p).permissive_map, fp)); }
        if (*p).policyvers >= POLICYDB_VERSION_NEVERAUDIT { wire_try!(ebitmap_read(&mut (*p).neveraudit_map, fp)); }
        let info = policydb_lookup_compat((*p).policyvers);
        if info.is_null() {
            pr_err(b"SELinux:  unable to find policy compat info for version %d\n\0".as_ptr().cast(), (*p).policyvers);
            return -EINVAL;
        }
        if buf[2] != (*info).sym_num || buf[3] != (*info).ocon_num {
            pr_err(b"SELinux:  policydb table sizes (%d,%d) do not match mine (%d,%d)\n\0".as_ptr().cast(), buf[2], buf[3], (*info).sym_num, (*info).ocon_num);
            return -EINVAL;
        }
        for i in 0..(*info).sym_num {
            wire_try!(wire_get(fp, &mut buf[..2]));
            let nprim = buf[0];
            let nel = buf[1];
            let rc = size_check(4 * mem::size_of::<u32>(), nel as usize, fp);
            if rc != 0 { destroy_on_error = false; return rc; }
            let rc = symtab_init(&mut (*p).symtab[i as usize], nel);
            if rc != 0 { destroy_on_error = false; return rc; }
            if i == SYM_ROLES {
                let rc = roles_init(p);
                if rc != 0 { destroy_on_error = false; return rc; }
            }
            for _ in 0..nel { wire_try!(read_f[i as usize].unwrap()(p, ptr::addr_of_mut!((*p).symtab[i as usize]), fp)); }
            (*p).symtab[i as usize].nprim = nprim;
        }
        wire_try!(policydb_index(p));
        (*p).process_class = string_to_security_class(p, b"process\0".as_ptr().cast());
        if (*p).process_class == 0 {
            pr_err(b"SELinux: process class is required, not defined in policy\n\0".as_ptr().cast());
            return -EINVAL;
        }
        wire_try!(avtab_read(ptr::addr_of_mut!((*p).te_avtab), fp, p));
        avtab_hash_eval(&mut (*p).te_avtab, b"rules\0".as_ptr().cast());
        if (*p).policyvers >= POLICYDB_VERSION_BOOL { wire_try!(cond_read_list(p, fp)); }
        wire_try!(wire_get(fp, &mut buf[..1]));
        let nel = buf[0];
        wire_try!(size_check(3 * mem::size_of::<u32>(), nel as usize, fp));
        wire_try!(hashtab_init(&mut (*p).role_tr, nel));
        for _ in 0..nel {
            rtk = malloc_obj();
            if rtk.is_null() { return -ENOMEM; }
            rtd = malloc_obj();
            if rtd.is_null() { return -ENOMEM; }
            wire_try!(wire_get(fp, &mut buf[..3]));
            (*rtk).role = buf[0];
            (*rtk).type_ = buf[1];
            (*rtd).new_role = buf[2];
            if (*p).policyvers >= POLICYDB_VERSION_ROLETRANS {
                wire_try!(wire_get(fp, &mut buf[..1]));
                if buf[0] > u16::MAX as u32 { return -EINVAL; }
                (*rtk).tclass = buf[0] as u16;
            } else { (*rtk).tclass = (*p).process_class; }
            if !policydb_role_isvalid(p, (*rtk).role) || !policydb_type_isvalid(p, (*rtk).type_)
                || !policydb_class_isvalid(p, (*rtk).tclass) || !policydb_role_isvalid(p, (*rtd).new_role) { return -EINVAL; }
            wire_try!(hashtab_insert(&mut (*p).role_tr, rtk.cast(), rtd.cast(), roletr_key_params));
            rtk = ptr::null_mut();
            rtd = ptr::null_mut();
        }
        hash_eval(&mut (*p).role_tr, b"roletr\0".as_ptr().cast(), ptr::null());
        wire_try!(wire_get(fp, &mut buf[..1]));
        let nel = buf[0];
        let mut lra: *mut role_allow = ptr::null_mut();
        for _ in 0..nel {
            let ra: *mut role_allow = zalloc_obj();
            if ra.is_null() { return -ENOMEM; }
            if !lra.is_null() { (*lra).next = ra; } else { (*p).role_allow = ra; }
            wire_try!(wire_get(fp, &mut buf[..2]));
            (*ra).role = buf[0];
            (*ra).new_role = buf[1];
            if !policydb_role_isvalid(p, (*ra).role) || !policydb_role_isvalid(p, (*ra).new_role) { return -EINVAL; }
            lra = ra;
        }
        wire_try!(filename_trans_read(p, fp));
        let perm = string_to_av_perm(p, (*p).process_class, b"transition\0".as_ptr().cast());
        if perm == 0 {
            pr_err(b"SELinux: process transition permission is required, not defined in policy\n\0".as_ptr().cast());
            return -EINVAL;
        }
        (*p).process_trans_perms = perm;
        let perm = string_to_av_perm(p, (*p).process_class, b"dyntransition\0".as_ptr().cast());
        if perm == 0 {
            pr_err(b"SELinux: process dyntransition permission is required, not defined in policy\n\0".as_ptr().cast());
            return -EINVAL;
        }
        (*p).process_trans_perms |= perm;
        wire_try!(ocontext_read(p, info, fp));
        wire_try!(genfs_read(p, fp));
        wire_try!(range_read(p, fp));
        let ntypes = (*p).symtab[SYM_TYPES as usize].nprim;
        (*p).type_attr_map_array = kvcalloc(ntypes);
        if (*p).type_attr_map_array.is_null() { return -ENOMEM; }
        for i in 0..ntypes { ebitmap_init((*p).type_attr_map_array.add(i as usize)); }
        for i in 0..ntypes {
            let e = (*p).type_attr_map_array.add(i as usize);
            if (*p).policyvers >= POLICYDB_VERSION_AVTAB { wire_try!(ebitmap_read(e, fp)); }
            if ebitmap_get_highest_set_bit(e) >= ntypes { return -EINVAL; }
            wire_try!(ebitmap_set_bit(e, i, 1));
        }
        wire_try!(policydb_bounds_sanity_check(p));
        0
    })();
    if rc != 0 && destroy_on_error {
        kfree(rtk.cast());
        kfree(rtd.cast());
        policydb_destroy(p);
    }
    rc
}

unsafe fn mls_write_level(l: *mut mls_level, fp: *mut policy_file) -> c_int {
    wire_try!(wire_put(fp, [(*l).sens]));
    wire_try!(ebitmap_write(&(*l).cat, fp));
    0
}

unsafe fn mls_write_range_helper(r: *mut mls_range, fp: *mut policy_file) -> c_int {
    let eq = mls_level_eq(&(*r).level[1], &(*r).level[0]) != 0;
    let items = if eq { 2 } else { 3 };
    wire_try!(wire_put_count(fp, [(items - 1) as u32, (*r).level[0].sens, (*r).level[1].sens], items));
    wire_try!(ebitmap_write(&(*r).level[0].cat, fp));
    if !eq { wire_try!(ebitmap_write(&(*r).level[1].cat, fp)); }
    0
}

unsafe extern "C" fn sens_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<level_datum>();
    let fp = (*data.cast::<policy_data>()).fp;
    let len = strlen(key);
    wire_try!(wire_put(fp, [len as u32, (*d).isalias as u32]));
    wire_try!(put_entry(key.cast(), 1, len, fp));
    wire_try!(mls_write_level(&mut (*d).level, fp));
    0
}

unsafe extern "C" fn cat_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<cat_datum>();
    let fp = (*data.cast::<policy_data>()).fp;
    let len = strlen(key);
    wire_try!(wire_put(fp, [len as u32, (*d).value, (*d).isalias as u32]));
    put_entry(key.cast(), 1, len, fp)
}

unsafe extern "C" fn role_trans_write_one(key: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let rtk = key.cast::<role_trans_key>();
    let rtd = datum.cast::<role_trans_datum>();
    let pd = data.cast::<policy_data>();
    let fp = (*pd).fp;
    wire_try!(wire_put(fp, [(*rtk).role, (*rtk).type_, (*rtd).new_role]));
    if (*(*pd).p).policyvers >= POLICYDB_VERSION_ROLETRANS {
        wire_try!(wire_put(fp, [(*rtk).tclass as u32]));
    }
    0
}

unsafe fn role_trans_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut pd = policy_data { p, fp };
    wire_try!(wire_put(fp, [(*p).role_tr.nel]));
    hashtab_map(ptr::addr_of_mut!((*p).role_tr), Some(role_trans_write_one), (&mut pd as *mut policy_data).cast())
}

unsafe fn role_allow_write(r: *mut role_allow, fp: *mut policy_file) -> c_int {
    let mut nel = 0usize;
    let mut ra = r;
    while !ra.is_null() { nel = nel.wrapping_add(1); ra = (*ra).next; }
    wire_try!(wire_put(fp, [nel as u32]));
    ra = r;
    while !ra.is_null() {
        wire_try!(wire_put(fp, [(*ra).role, (*ra).new_role]));
        ra = (*ra).next;
    }
    0
}

unsafe fn context_write(_p: *mut policydb, c: *mut context, fp: *mut policy_file) -> c_int {
    wire_try!(wire_put(fp, [(*c).user, (*c).role, (*c).type_]));
    wire_try!(mls_write_range_helper(&mut (*c).range, fp));
    0
}

unsafe extern "C" fn perm_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<perm_datum>();
    let fp = data.cast::<policy_file>();
    let len = strlen(key);
    wire_try!(wire_put(fp, [len as u32, (*d).value]));
    put_entry(key.cast(), 1, len, fp)
}

unsafe extern "C" fn common_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<common_datum>();
    let fp = (*data.cast::<policy_data>()).fp;
    let len = strlen(key);
    wire_try!(wire_put(fp, [len as u32, (*d).value, (*d).permissions.nprim, (*d).permissions.table.nel]));
    wire_try!(put_entry(key.cast(), 1, len, fp));
    wire_try!(hashtab_map(&mut (*d).permissions.table, Some(perm_write), fp.cast()));
    0
}

unsafe fn type_set_write(t: *mut type_set, fp: *mut policy_file) -> c_int {
    // This writer intentionally maps every subordinate error to EINVAL.
    if ebitmap_write(&(*t).types, fp) != 0 { return -EINVAL; }
    if ebitmap_write(&(*t).negset, fp) != 0 { return -EINVAL; }
    if wire_put(fp, [(*t).flags]) != 0 { return -EINVAL; }
    0
}

unsafe fn write_cons_helper(p: *mut policydb, node: *mut constraint_node, fp: *mut policy_file) -> c_int {
    let mut c = node;
    while !c.is_null() {
        let mut nel = 0u32;
        let mut e = (*c).expr;
        while !e.is_null() { nel = nel.wrapping_add(1); e = (*e).next; }
        wire_try!(wire_put(fp, [(*c).permissions, nel]));
        e = (*c).expr;
        while !e.is_null() {
            wire_try!(wire_put(fp, [(*e).expr_type, (*e).attr, (*e).op]));
            if (*e).expr_type == CEXPR_NAMES {
                wire_try!(ebitmap_write(&(*e).names, fp));
                if (*p).policyvers >= POLICYDB_VERSION_CONSTRAINT_NAMES {
                    wire_try!(type_set_write((*e).type_names, fp));
                }
            }
            e = (*e).next;
        }
        c = (*c).next;
    }
    0
}

unsafe extern "C" fn class_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<class_datum>();
    let pd = data.cast::<policy_data>();
    let p = (*pd).p;
    let fp = (*pd).fp;
    let len = strlen(key);
    let len2 = if (*d).comkey.is_null() { 0 } else { strlen((*d).comkey) };
    let mut ncons = 0u32;
    let mut c = (*d).constraints;
    while !c.is_null() { ncons = ncons.wrapping_add(1); c = (*c).next; }
    wire_try!(wire_put(fp, [len as u32, len2 as u32, (*d).value as u32,
        (*d).permissions.nprim, (*d).permissions.table.nel, ncons]));
    wire_try!(put_entry(key.cast(), 1, len, fp));
    if !(*d).comkey.is_null() { wire_try!(put_entry((*d).comkey.cast(), 1, len2, fp)); }
    wire_try!(hashtab_map(&mut (*d).permissions.table, Some(perm_write), fp.cast()));
    wire_try!(write_cons_helper(p, (*d).constraints, fp));
    ncons = 0;
    c = (*d).validatetrans;
    while !c.is_null() { ncons = ncons.wrapping_add(1); c = (*c).next; }
    wire_try!(wire_put(fp, [ncons]));
    wire_try!(write_cons_helper(p, (*d).validatetrans, fp));
    if (*p).policyvers >= POLICYDB_VERSION_NEW_OBJECT_DEFAULTS {
        wire_try!(wire_put(fp, [(*d).default_user as u32, (*d).default_role as u32, (*d).default_range as u32]));
    }
    if (*p).policyvers >= POLICYDB_VERSION_DEFAULT_TYPE { wire_try!(wire_put(fp, [(*d).default_type as u32])); }
    0
}

unsafe extern "C" fn role_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<role_datum>();
    let pd = data.cast::<policy_data>();
    let fp = (*pd).fp;
    let len = strlen(key);
    let items = if (*(*pd).p).policyvers >= POLICYDB_VERSION_BOUNDARY { 3 } else { 2 };
    wire_try!(wire_put_count(fp, [len as u32, (*d).value, (*d).bounds], items));
    wire_try!(put_entry(key.cast(), 1, len, fp));
    wire_try!(ebitmap_write(&(*d).dominates, fp));
    wire_try!(ebitmap_write(&(*d).types, fp));
    0
}

unsafe extern "C" fn type_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<type_datum>();
    let pd = data.cast::<policy_data>();
    let fp = (*pd).fp;
    let len = strlen(key);
    let mut buf = [len as u32, (*d).value, 0, 0];
    let items;
    if (*(*pd).p).policyvers >= POLICYDB_VERSION_BOUNDARY {
        let mut properties = 0u32;
        if (*d).primary != 0 { properties |= TYPEDATUM_PROPERTY_PRIMARY; }
        if (*d).attribute != 0 { properties |= TYPEDATUM_PROPERTY_ATTRIBUTE; }
        buf[2] = properties;
        buf[3] = (*d).bounds;
        items = 4;
    } else {
        buf[2] = (*d).primary as u32;
        items = 3;
    }
    wire_try!(wire_put_count(fp, buf, items));
    put_entry(key.cast(), 1, len, fp)
}

unsafe extern "C" fn user_write(vkey: *mut c_void, datum: *mut c_void, data: *mut c_void) -> c_int {
    let key = vkey.cast::<c_char>();
    let d = datum.cast::<user_datum>();
    let pd = data.cast::<policy_data>();
    let fp = (*pd).fp;
    let len = strlen(key);
    let items = if (*(*pd).p).policyvers >= POLICYDB_VERSION_BOUNDARY { 3 } else { 2 };
    wire_try!(wire_put_count(fp, [len as u32, (*d).value, (*d).bounds], items));
    wire_try!(put_entry(key.cast(), 1, len, fp));
    wire_try!(ebitmap_write(&(*d).roles, fp));
    wire_try!(mls_write_range_helper(&mut (*d).range, fp));
    wire_try!(mls_write_level(&mut (*d).dfltlevel, fp));
    0
}

static write_f: [unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int; SYM_NUM as usize] = [
    common_write, class_write, role_write, type_write, user_write,
    cond_write_bool, sens_write, cat_write,
];

unsafe fn ocontext_write(p: *mut policydb, info: *const policydb_compat_info, fp: *mut policy_file) -> c_int {
    for i in 0..(*info).ocon_num {
        let mut nel = 0usize;
        let mut c = (*p).ocontexts[i as usize];
        while !c.is_null() { nel = nel.wrapping_add(1); c = (*c).next; }
        wire_try!(wire_put(fp, [nel as u32]));
        c = (*p).ocontexts[i as usize];
        while !c.is_null() {
            match i {
                OCON_ISID => {
                    wire_try!(wire_put(fp, [(*c).sid[0]]));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_FS | OCON_NETIF => {
                    let len = strlen((*c).u.name);
                    wire_try!(wire_put(fp, [len as u32]));
                    wire_try!(put_entry((*c).u.name.cast(), 1, len, fp));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                    wire_try!(context_write(p, &mut (*c).context[1], fp));
                }
                OCON_PORT => {
                    wire_try!(wire_put(fp, [(*c).u.port.protocol as u32, (*c).u.port.low_port as u32, (*c).u.port.high_port as u32]));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_NODE => {
                    let nodebuf = [(*c).u.node.addr, (*c).u.node.mask];
                    wire_try!(put_entry(nodebuf.as_ptr().cast(), mem::size_of::<u32>(), 2, fp));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_FSUSE => {
                    let len = strlen((*c).u.name);
                    wire_try!(wire_put(fp, [(*c).v.behavior, len as u32]));
                    wire_try!(put_entry((*c).u.name.cast(), 1, len, fp));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_NODE6 => {
                    let mut nodebuf = [0u32; 8];
                    for j in 0..4 { nodebuf[j] = (*c).u.node6.addr[j]; }
                    for j in 0..4 { nodebuf[j + 4] = (*c).u.node6.mask[j]; }
                    wire_try!(put_entry(nodebuf.as_ptr().cast(), mem::size_of::<u32>(), 8, fp));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_IBPKEY => {
                    let prefix = (*c).u.ibpkey.subnet_prefix.to_be();
                    wire_try!(put_entry((&prefix as *const u64).cast(), mem::size_of::<u64>(), 1, fp));
                    wire_try!(wire_put(fp, [(*c).u.ibpkey.low_pkey as u32, (*c).u.ibpkey.high_pkey as u32]));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                OCON_IBENDPORT => {
                    let len = strlen((*c).u.ibendport.dev_name);
                    wire_try!(wire_put(fp, [len as u32, (*c).u.ibendport.port as u32]));
                    wire_try!(put_entry((*c).u.ibendport.dev_name.cast(), 1, len, fp));
                    wire_try!(context_write(p, &mut (*c).context[0], fp));
                }
                _ => {}
            }
            c = (*c).next;
        }
    }
    0
}

unsafe fn genfs_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut len = 0usize;
    let mut g = (*p).genfs;
    while !g.is_null() { len = len.wrapping_add(1); g = (*g).next; }
    wire_try!(wire_put(fp, [len as u32]));
    g = (*p).genfs;
    while !g.is_null() {
        len = strlen((*g).fstype);
        wire_try!(wire_put(fp, [len as u32]));
        wire_try!(put_entry((*g).fstype.cast(), 1, len, fp));
        len = 0;
        let mut c = (*g).head;
        while !c.is_null() { len = len.wrapping_add(1); c = (*c).next; }
        wire_try!(wire_put(fp, [len as u32]));
        c = (*g).head;
        while !c.is_null() {
            len = strlen((*c).u.name);
            wire_try!(wire_put(fp, [len as u32]));
            wire_try!(put_entry((*c).u.name.cast(), 1, len, fp));
            wire_try!(wire_put(fp, [(*c).v.sclass as u32]));
            wire_try!(context_write(p, &mut (*c).context[0], fp));
            c = (*c).next;
        }
        g = (*g).next;
    }
    0
}

unsafe extern "C" fn range_write_helper(key: *mut c_void, data: *mut c_void, arg: *mut c_void) -> c_int {
    let rt = key.cast::<range_trans>();
    let r = data.cast::<mls_range>();
    let pd = arg.cast::<policy_data>();
    let fp = (*pd).fp;
    wire_try!(wire_put(fp, [(*rt).source_type, (*rt).target_type]));
    if (*(*pd).p).policyvers >= POLICYDB_VERSION_RANGETRANS { wire_try!(wire_put(fp, [(*rt).target_class as u32])); }
    wire_try!(mls_write_range_helper(r, fp));
    0
}

unsafe fn range_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut pd = policy_data { p, fp };
    wire_try!(wire_put(fp, [(*p).range_tr.nel]));
    wire_try!(hashtab_map(ptr::addr_of_mut!((*p).range_tr), Some(range_write_helper), (&mut pd as *mut policy_data).cast()));
    0
}

unsafe extern "C" fn filename_write_helper_compat(key: *mut c_void, data: *mut c_void, arg: *mut c_void) -> c_int {
    let ft = key.cast::<filename_trans_key>();
    let mut datum = data.cast::<filename_trans_datum>();
    let fp = arg.cast::<policy_file>();
    let len = strlen((*ft).name) as u32;
    loop {
        let e = &(*datum).stypes as *const ebitmap;
        let mut node: *mut ebitmap_node = ptr::null_mut();
        let mut bit = ebitmap_start_positive(e, &mut node);
        while bit < (*e).highbit {
            wire_try!(wire_put(fp, [len]));
            wire_try!(put_entry((*ft).name.cast(), mem::size_of::<c_char>(), len as usize, fp));
            wire_try!(wire_put(fp, [bit.wrapping_add(1), (*ft).ttype, (*ft).tclass as u32, (*datum).otype]));
            bit = ebitmap_next_positive(e, &mut node, bit);
        }
        datum = (*datum).next;
        if datum.is_null() { break; }
    }
    0
}

unsafe extern "C" fn filename_write_helper(key: *mut c_void, data: *mut c_void, arg: *mut c_void) -> c_int {
    let ft = key.cast::<filename_trans_key>();
    let fp = arg.cast::<policy_file>();
    let len = strlen((*ft).name) as u32;
    wire_try!(wire_put(fp, [len]));
    wire_try!(put_entry((*ft).name.cast(), mem::size_of::<c_char>(), len as usize, fp));
    let mut ndatum = 0u32;
    let mut datum = data.cast::<filename_trans_datum>();
    loop {
        ndatum = ndatum.wrapping_add(1);
        datum = (*datum).next;
        if datum.is_null() { break; }
    }
    wire_try!(wire_put(fp, [(*ft).ttype, (*ft).tclass as u32, ndatum]));
    datum = data.cast::<filename_trans_datum>();
    loop {
        wire_try!(ebitmap_write(&(*datum).stypes, fp));
        wire_try!(wire_put(fp, [(*datum).otype]));
        datum = (*datum).next;
        if datum.is_null() { break; }
    }
    0
}

unsafe fn filename_trans_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    if (*p).policyvers < POLICYDB_VERSION_FILENAME_TRANS { return 0; }
    if (*p).policyvers < POLICYDB_VERSION_COMP_FTRANS {
        wire_try!(wire_put(fp, [(*p).compat_filename_trans_count]));
        hashtab_map(&mut (*p).filename_trans, Some(filename_write_helper_compat), fp.cast())
    } else {
        wire_try!(wire_put(fp, [(*p).filename_trans.nel]));
        hashtab_map(&mut (*p).filename_trans, Some(filename_write_helper), fp.cast())
    }
}

#[no_mangle]
pub unsafe extern "C" fn policydb_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    if (*p).policyvers < POLICYDB_VERSION_AVTAB {
        pr_err(b"SELinux: refusing to write policy version %d.  Because it is less than version %d\n\0".as_ptr().cast(), (*p).policyvers, POLICYDB_VERSION_AVTAB);
        return -EINVAL;
    }
    let mut config = 0u32;
    if (*p).mls_enabled != 0 { config |= POLICYDB_CONFIG_MLS; }
    if (*p).reject_unknown() != 0 { config |= REJECT_UNKNOWN; }
    if (*p).allow_unknown() != 0 { config |= ALLOW_UNKNOWN; }
    let len = POLICYDB_STRING.len() - 1;
    wire_try!(wire_put(fp, [POLICYDB_MAGIC, len as u32]));
    wire_try!(put_entry(POLICYDB_STRING.as_ptr().cast(), 1, len, fp));
    let info = policydb_lookup_compat((*p).policyvers);
    if info.is_null() {
        pr_err(b"SELinux: compatibility lookup failed for policy version %d\n\0".as_ptr().cast(), (*p).policyvers);
        return -EINVAL;
    }
    wire_try!(wire_put(fp, [(*p).policyvers, config, (*info).sym_num, (*info).ocon_num]));
    if (*p).policyvers >= POLICYDB_VERSION_POLCAP { wire_try!(ebitmap_write(&(*p).policycaps, fp)); }
    if (*p).policyvers >= POLICYDB_VERSION_PERMISSIVE { wire_try!(ebitmap_write(&(*p).permissive_map, fp)); }
    if (*p).policyvers >= POLICYDB_VERSION_NEVERAUDIT { wire_try!(ebitmap_write(&(*p).neveraudit_map, fp)); }
    for i in 0..(*info).sym_num {
        let mut pd = policy_data { p, fp };
        let s = ptr::addr_of_mut!((*p).symtab[i as usize]);
        wire_try!(wire_put(fp, [(*s).nprim, (*s).table.nel]));
        wire_try!(hashtab_map(ptr::addr_of_mut!((*s).table), Some(write_f[i as usize]), (&mut pd as *mut policy_data).cast()));
    }
    wire_try!(avtab_write(p, ptr::addr_of_mut!((*p).te_avtab), fp));
    wire_try!(cond_write_list(p, fp));
    wire_try!(role_trans_write(p, fp));
    wire_try!(role_allow_write((*p).role_allow, fp));
    wire_try!(filename_trans_write(p, fp));
    wire_try!(ocontext_write(p, info, fp));
    wire_try!(genfs_write(p, fp));
    wire_try!(range_write(p, fp));
    for i in 0..(*p).symtab[SYM_TYPES as usize].nprim {
        wire_try!(ebitmap_write((*p).type_attr_map_array.add(i as usize), fp));
    }
    0
}
