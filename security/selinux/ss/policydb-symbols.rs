// SPDX-License-Identifier: GPL-2.0-only
// Source reconstruction of policydb.c:1080-2107. Included by policydb.rs.
// Uses the configured C ABI and the parent's header-inline/macro bridges.
// Error returns, partial-object ownership and validation follow retained C.

unsafe fn mls_read_range_helper(r: *mut mls_range, fp: *mut policy_file) -> c_int {
    let mut buf = [0u32; 2];
    let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>());
    if rc != 0 { return rc; }

    let items = le32_to_cpu(buf[0]);
    if items > buf.len() as u32 {
        pr_err(b"SELinux: mls:  range overflow\n\0".as_ptr().cast());
        return -EINVAL;
    }
    rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>() * items as usize);
    if rc != 0 {
        pr_err(b"SELinux: mls:  truncated range\n\0".as_ptr().cast());
        return rc;
    }

    (*r).level[0].sens = le32_to_cpu(buf[0]);
    (*r).level[1].sens = if items > 1 { le32_to_cpu(buf[1]) } else { (*r).level[0].sens };
    rc = ebitmap_read(ptr::addr_of_mut!((*r).level[0].cat), fp);
    if rc != 0 {
        pr_err(b"SELinux: mls:  error reading low categories\n\0".as_ptr().cast());
        return rc;
    }
    if items > 1 {
        rc = ebitmap_read(ptr::addr_of_mut!((*r).level[1].cat), fp);
        if rc != 0 {
            pr_err(b"SELinux: mls:  error reading high categories\n\0".as_ptr().cast());
            ebitmap_destroy(ptr::addr_of_mut!((*r).level[0].cat));
            return rc;
        }
    } else {
        rc = ebitmap_cpy(ptr::addr_of_mut!((*r).level[1].cat), ptr::addr_of!((*r).level[0].cat));
        if rc != 0 {
            pr_err(b"SELinux: mls:  out of memory\n\0".as_ptr().cast());
            ebitmap_destroy(ptr::addr_of_mut!((*r).level[0].cat));
            return rc;
        }
    }
    0
}

unsafe fn context_read_and_validate(c: *mut context, p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf = [0u32; 3];
    let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
    if rc != 0 {
        pr_err(b"SELinux: context truncated\n\0".as_ptr().cast());
        return rc;
    }
    (*c).user = le32_to_cpu(buf[0]);
    (*c).role = le32_to_cpu(buf[1]);
    (*c).type_ = le32_to_cpu(buf[2]);
    if (*p).policyvers >= POLICYDB_VERSION_MLS {
        rc = mls_read_range_helper(ptr::addr_of_mut!((*c).range), fp);
        if rc != 0 {
            pr_err(b"SELinux: error reading MLS range of context\n\0".as_ptr().cast());
            return rc;
        }
    }
    if !policydb_context_isvalid(p, c) {
        pr_err(b"SELinux:  invalid security context\n\0".as_ptr().cast());
        context_destroy(c);
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn str_read(strp: *mut *mut c_char, flags: gfp_t, fp: *mut policy_file, len: u32) -> c_int {
    if len == 0 || len == u32::MAX { return -EINVAL; }
    if size_check(mem::size_of::<c_char>(), len as usize, fp) != 0 { return -EINVAL; }
    let string = kmalloc((len + 1) as usize, flags | __GFP_NOWARN).cast::<c_char>();
    if string.is_null() { return -ENOMEM; }
    let rc = next_entry(string.cast(), fp, len as usize);
    if rc != 0 {
        kfree(string.cast());
        return rc;
    }
    *string.add(len as usize) = 0;
    *strp = string;
    0
}

// A permission's one-based value occupies one of the SEL_VEC_MAX AV bits.
fn perm_claimed_mask(nprim: u32) -> u32 {
    if nprim != 0 { u32::MAX >> (SEL_VEC_MAX - nprim) } else { 0 }
}

unsafe fn perm_read(_p: *mut policydb, s: *mut symtab, fp: *mut policy_file, claimed: *mut u32) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let perdatum = zalloc_obj::<perm_datum>();
    if perdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let mut buf = [0u32; 2];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*perdatum).value = le32_to_cpu(buf[1]);
        if (*perdatum).value < 1 || (*perdatum).value > SEL_VEC_MAX { return -EINVAL; }
        if (*perdatum).value > (*s).nprim { return -EINVAL; }
        let bit = 1u32 << ((*perdatum).value - 1);
        if *claimed & bit != 0 { return -EINVAL; }
        *claimed |= bit;
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        symtab_insert(s, key, perdatum.cast())
    })();
    if rc != 0 { perm_destroy(key.cast(), perdatum.cast(), ptr::null_mut()); }
    rc
}

unsafe extern "C" fn common_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let comdatum = zalloc_obj::<common_datum>();
    if comdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let mut buf = [0u32; 4];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*comdatum).value = le32_to_cpu(buf[1]);
        let nel = le32_to_cpu(buf[3]);
        if nel > SEL_VEC_MAX { return -EINVAL; }
        rc = size_check(2 * mem::size_of::<u32>(), nel as usize, fp);
        if rc != 0 { return rc; }
        rc = symtab_init(ptr::addr_of_mut!((*comdatum).permissions), nel);
        if rc != 0 { return rc; }
        (*comdatum).permissions.nprim = le32_to_cpu(buf[2]);
        if (*comdatum).permissions.nprim > SEL_VEC_MAX { return -EINVAL; }
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        let mut claimed = 0u32;
        for _ in 0..nel {
            rc = perm_read(p, ptr::addr_of_mut!((*comdatum).permissions), fp, ptr::addr_of_mut!(claimed));
            if rc != 0 { return rc; }
        }
        if claimed != perm_claimed_mask((*comdatum).permissions.nprim) {
            pr_err(b"SELinux:  common %s does not define every permission it declares\n\0".as_ptr().cast(), key);
            return -EINVAL;
        }
        hash_eval(ptr::addr_of_mut!((*comdatum).permissions.table), b"common_permissions\0".as_ptr().cast(), key);
        symtab_insert(s, key, comdatum.cast())
    })();
    if rc != 0 { common_destroy(key.cast(), comdatum.cast(), ptr::null_mut()); }
    rc
}

unsafe fn type_set_init(t: *mut type_set) {
    ebitmap_init(ptr::addr_of_mut!((*t).types));
    ebitmap_init(ptr::addr_of_mut!((*t).negset));
}

unsafe fn type_set_read(t: *mut type_set, fp: *mut policy_file) -> c_int {
    if ebitmap_read(ptr::addr_of_mut!((*t).types), fp) != 0 { return -EINVAL; }
    if ebitmap_read(ptr::addr_of_mut!((*t).negset), fp) != 0 { return -EINVAL; }
    let mut buf = [0u32; 1];
    let rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>());
    if rc < 0 { return -EINVAL; }
    (*t).flags = le32_to_cpu(buf[0]);
    0
}

unsafe fn read_cons_helper(p: *mut policydb, nodep: *mut *mut constraint_node, ncons: u32, allowxtarget: c_int, fp: *mut policy_file) -> c_int {
    let mut lc: *mut constraint_node = ptr::null_mut();
    for _ in 0..ncons {
        let c = zalloc_obj::<constraint_node>();
        if c.is_null() { return -ENOMEM; }
        // Link each allocation immediately: the owning class unwinds partial lists.
        if !lc.is_null() { (*lc).next = c; } else { *nodep = c; }
        let mut buf = [0u32; 3];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, 2 * mem::size_of::<u32>());
        if rc != 0 { return rc; }
        (*c).permissions = le32_to_cpu(buf[0]);
        let nexpr = le32_to_cpu(buf[1]);
        let mut le: *mut constraint_expr = ptr::null_mut();
        let mut depth: c_int = -1;
        for _ in 0..nexpr {
            let e = zalloc_obj::<constraint_expr>();
            if e.is_null() { return -ENOMEM; }
            if !le.is_null() { (*le).next = e; } else { (*c).expr = e; }
            rc = next_entry(buf.as_mut_ptr().cast(), fp, 3 * mem::size_of::<u32>());
            if rc != 0 { return rc; }
            (*e).expr_type = le32_to_cpu(buf[0]);
            (*e).attr = le32_to_cpu(buf[1]);
            (*e).op = le32_to_cpu(buf[2]);
            match (*e).expr_type {
                CEXPR_NOT => {
                    if depth < 0 { return -EINVAL; }
                }
                CEXPR_AND | CEXPR_OR => {
                    if depth < 1 { return -EINVAL; }
                    depth -= 1;
                }
                CEXPR_ATTR => {
                    if depth == CEXPR_MAXDEPTH as c_int - 1 { return -EINVAL; }
                    depth += 1;
                    match (*e).attr {
                        CEXPR_USER | CEXPR_TYPE => {
                            if (*e).op != CEXPR_EQ && (*e).op != CEXPR_NEQ { return -EINVAL; }
                        }
                        CEXPR_ROLE | CEXPR_L1L2 | CEXPR_L1H2 | CEXPR_H1L2 | CEXPR_H1H2 | CEXPR_L1H1 | CEXPR_L2H2 => {
                            if (*e).op < CEXPR_EQ || (*e).op > CEXPR_INCOMP { return -EINVAL; }
                        }
                        _ => return -EINVAL,
                    }
                }
                CEXPR_NAMES => {
                    if allowxtarget == 0 && (*e).attr & CEXPR_XTARGET != 0 { return -EINVAL; }
                    if depth == CEXPR_MAXDEPTH as c_int - 1 { return -EINVAL; }
                    depth += 1;
                    match (*e).attr & !(CEXPR_TARGET | CEXPR_XTARGET) {
                        CEXPR_USER | CEXPR_ROLE | CEXPR_TYPE => (),
                        _ => return -EINVAL,
                    }
                    if (*e).attr & (CEXPR_TARGET | CEXPR_XTARGET) == (CEXPR_TARGET | CEXPR_XTARGET) { return -EINVAL; }
                    if (*e).op != CEXPR_EQ && (*e).op != CEXPR_NEQ { return -EINVAL; }
                    rc = ebitmap_read(ptr::addr_of_mut!((*e).names), fp);
                    if rc != 0 { return rc; }
                    if (*p).policyvers >= POLICYDB_VERSION_CONSTRAINT_NAMES {
                        (*e).type_names = zalloc_obj::<type_set>();
                        if (*e).type_names.is_null() { return -ENOMEM; }
                        type_set_init((*e).type_names);
                        rc = type_set_read((*e).type_names, fp);
                        if rc != 0 { return rc; }
                    }
                }
                _ => return -EINVAL,
            }
            le = e;
        }
        if depth != 0 { return -EINVAL; }
        lc = c;
    }
    0
}

unsafe extern "C" fn class_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let cladatum = zalloc_obj::<class_datum>();
    if cladatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let mut buf = [0u32; 6];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, 6 * mem::size_of::<u32>());
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        let len2 = le32_to_cpu(buf[1]);
        let nel = le32_to_cpu(buf[4]);
        if nel > SEL_VEC_MAX { return -EINVAL; }
        let val = le32_to_cpu(buf[2]);
        if val > u16::MAX as u32 { return -EINVAL; }
        (*cladatum).value = val as u16;
        rc = size_check(2 * mem::size_of::<u32>(), nel as usize, fp);
        if rc != 0 { return rc; }
        rc = symtab_init(ptr::addr_of_mut!((*cladatum).permissions), nel);
        if rc != 0 { return rc; }
        (*cladatum).permissions.nprim = le32_to_cpu(buf[3]);
        if (*cladatum).permissions.nprim > SEL_VEC_MAX { return -EINVAL; }
        let mut ncons = le32_to_cpu(buf[5]);
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        if len2 != 0 {
            rc = str_read(ptr::addr_of_mut!((*cladatum).comkey), GFP_KERNEL, fp, len2);
            if rc != 0 { return rc; }
            (*cladatum).comdatum = symtab_search(ptr::addr_of!((*p).symtab[SYM_COMMONS as usize]), (*cladatum).comkey).cast();
            if (*cladatum).comdatum.is_null() {
                pr_err(b"SELinux:  unknown common %s\n\0".as_ptr().cast(), (*cladatum).comkey);
                return -EINVAL;
            }
            if (*cladatum).permissions.nprim < (*(*cladatum).comdatum).permissions.nprim {
                pr_err(b"SELinux:  class %s has fewer permissions than common %s\n\0".as_ptr().cast(), key, (*cladatum).comkey);
                return -EINVAL;
            }
        }
        let mut claimed = 0u32;
        for _ in 0..nel {
            rc = perm_read(p, ptr::addr_of_mut!((*cladatum).permissions), fp, ptr::addr_of_mut!(claimed));
            if rc != 0 { return rc; }
        }
        let inherited = if (*cladatum).comdatum.is_null() { 0 } else { (*(*cladatum).comdatum).permissions.nprim };
        if claimed != (perm_claimed_mask((*cladatum).permissions.nprim) & !perm_claimed_mask(inherited)) {
            pr_err(b"SELinux:  class %s does not define every permission it declares\n\0".as_ptr().cast(), key);
            return -EINVAL;
        }
        hash_eval(ptr::addr_of_mut!((*cladatum).permissions.table), b"class_permissions\0".as_ptr().cast(), key);
        rc = read_cons_helper(p, ptr::addr_of_mut!((*cladatum).constraints), ncons, 0, fp);
        if rc != 0 { return rc; }
        if (*p).policyvers >= POLICYDB_VERSION_VALIDATETRANS {
            rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>());
            if rc != 0 { return rc; }
            ncons = le32_to_cpu(buf[0]);
            rc = read_cons_helper(p, ptr::addr_of_mut!((*cladatum).validatetrans), ncons, 1, fp);
            if rc != 0 { return rc; }
        }
        if (*p).policyvers >= POLICYDB_VERSION_NEW_OBJECT_DEFAULTS {
            rc = next_entry(buf.as_mut_ptr().cast(), fp, 3 * mem::size_of::<u32>());
            if rc != 0 { return rc; }
            let val = le32_to_cpu(buf[0]);
            match val {
                0 | DEFAULT_SOURCE | DEFAULT_TARGET => (*cladatum).default_user = val as c_char,
                _ => return -EINVAL,
            }
            let val = le32_to_cpu(buf[1]);
            match val {
                0 | DEFAULT_SOURCE | DEFAULT_TARGET => (*cladatum).default_role = val as c_char,
                _ => return -EINVAL,
            }
            let val = le32_to_cpu(buf[2]);
            match val {
                0 | DEFAULT_SOURCE_LOW | DEFAULT_SOURCE_HIGH | DEFAULT_SOURCE_LOW_HIGH |
                    DEFAULT_TARGET_LOW | DEFAULT_TARGET_HIGH | DEFAULT_TARGET_LOW_HIGH | DEFAULT_GLBLUB => {
                    (*cladatum).default_range = val as c_char;
                }
                _ => return -EINVAL,
            }
        }
        if (*p).policyvers >= POLICYDB_VERSION_DEFAULT_TYPE {
            rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>());
            if rc != 0 { return rc; }
            let val = le32_to_cpu(buf[0]);
            match val {
                0 | DEFAULT_TARGET | DEFAULT_SOURCE => (*cladatum).default_type = val as c_char,
                _ => return -EINVAL,
            }
        }
        symtab_insert(s, key, cladatum.cast())
    })();
    if rc != 0 {
        cls_destroy(key.cast(), cladatum.cast(), ptr::null_mut());
        pr_err(b"SELinux:  invalid class\n\0".as_ptr().cast());
    }
    rc
}

unsafe extern "C" fn role_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let role = zalloc_obj::<role_datum>();
    if role.is_null() { return -ENOMEM; }
    let mut inserted = false;
    let rc = (|| -> c_int {
        let to_read = if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY { 3 } else { 2 };
        let mut buf = [0u32; 3];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>() * to_read);
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*role).value = le32_to_cpu(buf[1]);
        if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY { (*role).bounds = le32_to_cpu(buf[2]); }
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        rc = ebitmap_read(ptr::addr_of_mut!((*role).dominates), fp);
        if rc != 0 { return rc; }
        rc = ebitmap_read(ptr::addr_of_mut!((*role).types), fp);
        if rc != 0 { return rc; }
        if strcmp(key, OBJECT_R.as_ptr().cast()) == 0 {
            if (*role).value != OBJECT_R_VAL {
                pr_err(b"SELinux: Role %s has wrong value %d\n\0".as_ptr().cast(), OBJECT_R.as_ptr().cast::<c_char>(), (*role).value);
                return -EINVAL;
            }
            // object_r already exists: the successfully read duplicate is destroyed.
            return 0;
        }
        rc = symtab_insert(s, key, role.cast());
        if rc == 0 { inserted = true; }
        rc
    })();
    if !inserted { role_destroy(key.cast(), role.cast(), ptr::null_mut()); }
    rc
}

unsafe extern "C" fn type_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let typdatum = zalloc_obj::<type_datum>();
    if typdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let to_read = if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY { 4 } else { 3 };
        let mut buf = [0u32; 4];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>() * to_read);
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*typdatum).value = le32_to_cpu(buf[1]);
        if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY {
            let prop = le32_to_cpu(buf[2]);
            if prop & TYPEDATUM_PROPERTY_PRIMARY != 0 { (*typdatum).primary = 1; }
            if prop & TYPEDATUM_PROPERTY_ATTRIBUTE != 0 { (*typdatum).attribute = 1; }
            (*typdatum).bounds = le32_to_cpu(buf[3]);
        } else {
            // The retained ABI narrows the old-format primary word to unsigned char.
            (*typdatum).primary = le32_to_cpu(buf[2]) as u8;
        }
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        symtab_insert(s, key, typdatum.cast())
    })();
    if rc != 0 { type_destroy(key.cast(), typdatum.cast(), ptr::null_mut()); }
    rc
}

unsafe fn mls_read_level(lp: *mut mls_level, fp: *mut policy_file) -> c_int {
    ptr::write_bytes(lp, 0, 1);
    let mut buf = [0u32; 1];
    let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
    if rc != 0 {
        pr_err(b"SELinux: mls: truncated level\n\0".as_ptr().cast());
        return rc;
    }
    (*lp).sens = le32_to_cpu(buf[0]);
    rc = ebitmap_read(ptr::addr_of_mut!((*lp).cat), fp);
    if rc != 0 {
        pr_err(b"SELinux: mls:  error reading level categories\n\0".as_ptr().cast());
        return rc;
    }
    0
}

unsafe extern "C" fn user_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let usrdatum = zalloc_obj::<user_datum>();
    if usrdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let to_read = if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY { 3 } else { 2 };
        let mut buf = [0u32; 3];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of::<u32>() * to_read);
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*usrdatum).value = le32_to_cpu(buf[1]);
        if (*p).policyvers >= POLICYDB_VERSION_BOUNDARY { (*usrdatum).bounds = le32_to_cpu(buf[2]); }
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        rc = ebitmap_read(ptr::addr_of_mut!((*usrdatum).roles), fp);
        if rc != 0 { return rc; }
        if (*p).policyvers >= POLICYDB_VERSION_MLS {
            rc = mls_read_range_helper(ptr::addr_of_mut!((*usrdatum).range), fp);
            if rc != 0 { return rc; }
            rc = mls_read_level(ptr::addr_of_mut!((*usrdatum).dfltlevel), fp);
            if rc != 0 { return rc; }
        }
        symtab_insert(s, key, usrdatum.cast())
    })();
    if rc != 0 { user_destroy(key.cast(), usrdatum.cast(), ptr::null_mut()); }
    rc
}

unsafe extern "C" fn sens_read(_p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let levdatum = zalloc_obj::<level_datum>();
    if levdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let mut buf = [0u32; 2];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        let val = le32_to_cpu(buf[1]);
        if !val_is_boolean(val) { return -EINVAL; }
        (*levdatum).isalias = val as u8;
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        rc = mls_read_level(ptr::addr_of_mut!((*levdatum).level), fp);
        if rc != 0 { return rc; }
        symtab_insert(s, key, levdatum.cast())
    })();
    if rc != 0 {
        sens_destroy(key.cast(), levdatum.cast(), ptr::null_mut());
        pr_err(b"SELinux:  invalid sensitivity\n\0".as_ptr().cast());
    }
    rc
}

unsafe extern "C" fn cat_read(_p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let catdatum = zalloc_obj::<cat_datum>();
    if catdatum.is_null() { return -ENOMEM; }
    let rc = (|| -> c_int {
        let mut buf = [0u32; 3];
        let mut rc = next_entry(buf.as_mut_ptr().cast(), fp, mem::size_of_val(&buf));
        if rc != 0 { return rc; }
        let len = le32_to_cpu(buf[0]);
        (*catdatum).value = le32_to_cpu(buf[1]);
        let val = le32_to_cpu(buf[2]);
        if !val_is_boolean(val) { return -EINVAL; }
        (*catdatum).isalias = val as u8;
        rc = str_read(ptr::addr_of_mut!(key), GFP_KERNEL, fp, len);
        if rc != 0 { return rc; }
        symtab_insert(s, key, catdatum.cast())
    })();
    if rc != 0 {
        cat_destroy(key.cast(), catdatum.cast(), ptr::null_mut());
        pr_err(b"SELinux:  invalid category\n\0".as_ptr().cast());
    }
    rc
}

static read_f: [Option<unsafe extern "C" fn(*mut policydb, *mut symtab, *mut policy_file) -> c_int>; SYM_NUM as usize] = [
    Some(common_read),
    Some(class_read),
    Some(role_read),
    Some(type_read),
    Some(user_read),
    Some(cond_read_bool),
    Some(sens_read),
    Some(cat_read),
];

unsafe extern "C" fn user_bounds_sanity_check(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p = datap.cast::<policydb>();
    let user = datum.cast::<user_datum>();
    let mut upper = user;
    let mut depth: c_int = 0;
    while (*upper).bounds != 0 {
        depth += 1;
        if depth == POLICYDB_BOUNDS_MAXDEPTH as c_int {
            pr_err(b"SELinux: user %s: too deep or looped boundary\n\0".as_ptr().cast(), key.cast::<c_char>());
            return -EINVAL;
        }
        if !policydb_user_isvalid(p, (*upper).bounds) {
            pr_err(b"SELinux: user %s: invalid boundary id %d\n\0".as_ptr().cast(), key.cast::<c_char>(), (*upper).bounds);
            return -EINVAL;
        }
        upper = *(*p).user_val_to_struct.add(((*upper).bounds - 1) as usize);
        let roles = ptr::addr_of!((*user).roles);
        let mut node: *mut ebitmap_node = ptr::null_mut();
        let mut bit = ebitmap_start_positive(roles, ptr::addr_of_mut!(node));
        while bit < (*roles).highbit {
            if ebitmap_get_bit(ptr::addr_of!((*upper).roles), bit) == 0 {
                pr_err(b"SELinux: boundary violated policy: user=%s role=%s bounds=%s\n\0".as_ptr().cast(),
                    sym_name(p, SYM_USERS, (*user).value.wrapping_sub(1)),
                    sym_name(p, SYM_ROLES, bit),
                    sym_name(p, SYM_USERS, (*upper).value.wrapping_sub(1)));
                return -EINVAL;
            }
            bit = ebitmap_next_positive(roles, ptr::addr_of_mut!(node), bit);
        }
    }
    0
}

unsafe extern "C" fn role_bounds_sanity_check(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p = datap.cast::<policydb>();
    let role = datum.cast::<role_datum>();
    let mut upper = role;
    let mut depth: c_int = 0;
    while (*upper).bounds != 0 {
        depth += 1;
        if depth == POLICYDB_BOUNDS_MAXDEPTH as c_int {
            pr_err(b"SELinux: role %s: too deep or looped bounds\n\0".as_ptr().cast(), key.cast::<c_char>());
            return -EINVAL;
        }
        if !policydb_role_isvalid(p, (*upper).bounds) {
            pr_err(b"SELinux: role %s: invalid boundary id %d\n\0".as_ptr().cast(), key.cast::<c_char>(), (*upper).bounds);
            return -EINVAL;
        }
        upper = *(*p).role_val_to_struct.add(((*upper).bounds - 1) as usize);
        let types = ptr::addr_of!((*role).types);
        let mut node: *mut ebitmap_node = ptr::null_mut();
        let mut bit = ebitmap_start_positive(types, ptr::addr_of_mut!(node));
        while bit < (*types).highbit {
            if ebitmap_get_bit(ptr::addr_of!((*upper).types), bit) == 0 {
                pr_err(b"SELinux: boundary violated policy: role=%s type=%s bounds=%s\n\0".as_ptr().cast(),
                    sym_name(p, SYM_ROLES, (*role).value.wrapping_sub(1)),
                    sym_name(p, SYM_TYPES, bit),
                    sym_name(p, SYM_ROLES, (*upper).value.wrapping_sub(1)));
                return -EINVAL;
            }
            bit = ebitmap_next_positive(types, ptr::addr_of_mut!(node), bit);
        }
    }
    0
}

unsafe extern "C" fn type_bounds_sanity_check(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p = datap.cast::<policydb>();
    let mut upper = datum.cast::<type_datum>();
    let mut depth: c_int = 0;
    while (*upper).bounds != 0 {
        depth += 1;
        if depth == POLICYDB_BOUNDS_MAXDEPTH as c_int {
            pr_err(b"SELinux: type %s: too deep or looped boundary\n\0".as_ptr().cast(), key.cast::<c_char>());
            return -EINVAL;
        }
        if !policydb_type_isvalid(p, (*upper).bounds) {
            pr_err(b"SELinux: type %s: invalid boundary id %d\n\0".as_ptr().cast(), key.cast::<c_char>(), (*upper).bounds);
            return -EINVAL;
        }
        upper = *(*p).type_val_to_struct.add(((*upper).bounds - 1) as usize);
        if (*upper).attribute != 0 {
            pr_err(b"SELinux: type %s: bounded by attribute %s\n\0".as_ptr().cast(),
                key.cast::<c_char>(), sym_name(p, SYM_TYPES, (*upper).value.wrapping_sub(1)));
            return -EINVAL;
        }
    }
    0
}

unsafe fn policydb_bounds_sanity_check(p: *mut policydb) -> c_int {
    if (*p).policyvers < POLICYDB_VERSION_BOUNDARY { return 0; }
    let mut rc = hashtab_map(ptr::addr_of_mut!((*p).symtab[SYM_USERS as usize].table), Some(user_bounds_sanity_check), p.cast());
    if rc != 0 { return rc; }
    rc = hashtab_map(ptr::addr_of_mut!((*p).symtab[SYM_ROLES as usize].table), Some(role_bounds_sanity_check), p.cast());
    if rc != 0 { return rc; }
    rc = hashtab_map(ptr::addr_of_mut!((*p).symtab[SYM_TYPES as usize].table), Some(type_bounds_sanity_check), p.cast());
    if rc != 0 { return rc; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn string_to_security_class(p: *mut policydb, name: *const c_char) -> u16 {
    let cladatum = symtab_search(ptr::addr_of!((*p).symtab[SYM_CLASSES as usize]), name).cast::<class_datum>();
    if cladatum.is_null() { return 0; }
    (*cladatum).value
}

#[no_mangle]
pub unsafe extern "C" fn string_to_av_perm(p: *mut policydb, tclass: u16, name: *const c_char) -> u32 {
    if tclass == 0 || tclass as u32 > (*p).symtab[SYM_CLASSES as usize].nprim { return 0; }
    let cladatum = *(*p).class_val_to_struct.add((tclass - 1) as usize);
    let comdatum = (*cladatum).comdatum;
    let mut perdatum: *mut perm_datum = ptr::null_mut();
    if !comdatum.is_null() {
        perdatum = symtab_search(ptr::addr_of!((*comdatum).permissions), name).cast();
    }
    if perdatum.is_null() {
        perdatum = symtab_search(ptr::addr_of!((*cladatum).permissions), name).cast();
    }
    if perdatum.is_null() { return 0; }
    1u32 << ((*perdatum).value - 1)
}
