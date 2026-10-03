// SPDX-License-Identifier: GPL-2.0-only
// Ownership, lookup and indexing from retained policydb.c.

#[allow(non_camel_case_types)]
struct policydb_compat_info {
    version: c_uint,
    sym_num: c_uint,
    ocon_num: c_uint,
}

static policydb_compat: [policydb_compat_info; 21] = {
    [
        policydb_compat_info { version: POLICYDB_VERSION_BASE, sym_num: SYM_NUM as c_uint - 3, ocon_num: OCON_NUM as c_uint - 3 },
        policydb_compat_info { version: POLICYDB_VERSION_BOOL, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 3 },
        policydb_compat_info { version: POLICYDB_VERSION_IPV6, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_NLCLASS, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_MLS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_AVTAB, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_RANGETRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_POLCAP, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_PERMISSIVE, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_BOUNDARY, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_FILENAME_TRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_ROLETRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_NEW_OBJECT_DEFAULTS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_DEFAULT_TYPE, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_CONSTRAINT_NAMES, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_XPERMS_IOCTL, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_INFINIBAND, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_GLBLUB, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_COMP_FTRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_COND_XPERMS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_NEVERAUDIT, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
    ]
};

unsafe fn policydb_lookup_compat(version: c_uint) -> *const policydb_compat_info {
    let mut i = 0usize;
    while i < policydb_compat.len() {
        if policydb_compat[i].version == version {
            return &policydb_compat[i];
        }
        i += 1;
    }
    ptr::null()
}

unsafe extern "C" fn perm_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe extern "C" fn common_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let comdatum = datum as *mut common_datum;
        hashtab_map(&mut (*comdatum).permissions.table, Some(perm_destroy), ptr::null_mut());
        hashtab_destroy(&mut (*comdatum).permissions.table);
    }
    kfree(datum);
    0
}

unsafe fn constraint_expr_destroy(expr: *mut constraint_expr) {
    if !expr.is_null() {
        ebitmap_destroy(&mut (*expr).names);
        if !(*expr).type_names.is_null() {
            ebitmap_destroy(&mut (*(*expr).type_names).types);
            ebitmap_destroy(&mut (*(*expr).type_names).negset);
            kfree((*expr).type_names as *mut c_void);
        }
        kfree(expr as *mut c_void);
    }
}

unsafe extern "C" fn cls_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let cladatum = datum as *mut class_datum;
        hashtab_map(&mut (*cladatum).permissions.table, Some(perm_destroy), ptr::null_mut());
        hashtab_destroy(&mut (*cladatum).permissions.table);
        let mut constraint = (*cladatum).constraints;
        while !constraint.is_null() {
            let mut e = (*constraint).expr;
            while !e.is_null() {
                let etmp = e;
                e = (*e).next;
                constraint_expr_destroy(etmp);
            }
            let ctemp = constraint;
            constraint = (*constraint).next;
            kfree(ctemp as *mut c_void);
        }
        constraint = (*cladatum).validatetrans;
        while !constraint.is_null() {
            let mut e = (*constraint).expr;
            while !e.is_null() {
                let etmp = e;
                e = (*e).next;
                constraint_expr_destroy(etmp);
            }
            let ctemp = constraint;
            constraint = (*constraint).next;
            kfree(ctemp as *mut c_void);
        }
        kfree((*cladatum).comkey as *mut c_void);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn role_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let role = datum as *mut role_datum;
        ebitmap_destroy(&mut (*role).dominates);
        ebitmap_destroy(&mut (*role).types);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn type_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe extern "C" fn user_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let usrdatum = datum as *mut user_datum;
        ebitmap_destroy(&mut (*usrdatum).roles);
        ebitmap_destroy(&mut (*usrdatum).range.level[0].cat);
        ebitmap_destroy(&mut (*usrdatum).range.level[1].cat);
        ebitmap_destroy(&mut (*usrdatum).dfltlevel.cat);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn sens_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let levdatum = datum as *mut level_datum;
        ebitmap_destroy(&mut (*levdatum).level.cat);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn cat_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

static destroy_f: [Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>; SYM_NUM as usize] = [
    Some(common_destroy), Some(cls_destroy), Some(role_destroy), Some(type_destroy),
    Some(user_destroy), Some(cond_destroy_bool), Some(sens_destroy), Some(cat_destroy),
];

unsafe extern "C" fn filenametr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    let ft = key as *mut filename_trans_key;
    let mut d = datum as *mut filename_trans_datum;
    kfree((*ft).name as *mut c_void);
    kfree(key);
    loop {
        ebitmap_destroy(&mut (*d).stypes);
        let next = (*d).next;
        kfree(d as *mut c_void);
        d = next;
        if d.is_null() { break; }
    }
    cond_resched();
    0
}

unsafe extern "C" fn range_tr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    let rt = datum as *mut mls_range;
    kfree(key);
    ebitmap_destroy(&mut (*rt).level[0].cat);
    ebitmap_destroy(&mut (*rt).level[1].cat);
    kfree(datum);
    cond_resched();
    0
}

unsafe extern "C" fn role_tr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe fn ocontext_destroy(c: *mut ocontext, i: c_uint) {
    if c.is_null() { return; }
    context_destroy(&mut (*c).context[0]);
    context_destroy(&mut (*c).context[1]);
    if i == OCON_ISID as c_uint || i == OCON_FS as c_uint || i == OCON_NETIF as c_uint || i == OCON_FSUSE as c_uint {
        kfree((*c).u.name as *mut c_void);
    }
    kfree(c as *mut c_void);
}

unsafe fn roles_init(p: *mut policydb) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let role = zalloc_obj::<role_datum>();
    if role.is_null() { return -ENOMEM; }
    let mut rc = -EINVAL;
    (*p).symtab[SYM_ROLES as usize].nprim = (*p).symtab[SYM_ROLES as usize].nprim.wrapping_add(1);
    (*role).value = (*p).symtab[SYM_ROLES as usize].nprim;
    if (*role).value != OBJECT_R_VAL { goto_out_free(key, role as *mut c_void, rc) } else {
        rc = -ENOMEM;
        key = kstrdup(OBJECT_R.as_ptr().cast(), GFP_KERNEL);
        if key.is_null() { goto_out_free(key, role as *mut c_void, rc) } else {
            rc = symtab_insert(&mut (*p).symtab[SYM_ROLES as usize], key, role as *mut c_void);
            if rc != 0 { goto_out_free(key, role as *mut c_void, rc) } else { 0 }
        }
    }
}

unsafe fn goto_out_free(key: *mut c_char, datum: *mut c_void, rc: c_int) -> c_int {
    kfree(key as *mut c_void);
    kfree(datum);
    rc
}

unsafe extern "C" fn filenametr_hash(k: *const c_void) -> u32 {
    let ft = k as *const filename_trans_key;
    let salt = ((*ft).ttype ^ (*ft).tclass as u32) as c_ulong;
    full_name_hash(salt as *mut c_void, (*ft).name, strlen((*ft).name) as u32)
}

unsafe extern "C" fn filenametr_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let ft1 = k1 as *const filename_trans_key;
    let ft2 = k2 as *const filename_trans_key;
    let mut v = cmp_int((*ft1).ttype, (*ft2).ttype);
    if v != 0 { return v; }
    v = cmp_int((*ft1).tclass as u32, (*ft2).tclass as u32);
    if v != 0 { return v; }
    strcmp((*ft1).name, (*ft2).name)
}

static filenametr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(filenametr_hash),
    cmp: Some(filenametr_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_filenametr_search(p: *mut policydb, key: *mut filename_trans_key) -> *mut filename_trans_datum {
    hashtab_search(&mut (*p).filename_trans, key as *const c_void, filenametr_key_params) as *mut filename_trans_datum
}

unsafe extern "C" fn rangetr_hash(k: *const c_void) -> u32 {
    let key = k as *const range_trans;
    (*key).source_type.wrapping_add((*key).target_type << 3).wrapping_add(((*key).target_class as u32) << 5)
}

unsafe extern "C" fn rangetr_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let key1 = k1 as *const range_trans;
    let key2 = k2 as *const range_trans;
    let mut v = cmp_int((*key1).source_type, (*key2).source_type);
    if v != 0 { return v; }
    v = cmp_int((*key1).target_type, (*key2).target_type);
    if v != 0 { return v; }
    cmp_int((*key1).target_class as u32, (*key2).target_class as u32)
}

static rangetr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(rangetr_hash),
    cmp: Some(rangetr_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_rangetr_search(p: *mut policydb, key: *mut range_trans) -> *mut mls_range {
    hashtab_search(&mut (*p).range_tr, key as *const c_void, rangetr_key_params) as *mut mls_range
}

unsafe extern "C" fn role_trans_hash(k: *const c_void) -> u32 {
    let key = k as *const role_trans_key;
    jhash_3words((*key).role, (*key).type_, ((*key).tclass as u32) << 16 | (*key).tclass as u32, 0)
}

unsafe extern "C" fn role_trans_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let key1 = k1 as *const role_trans_key;
    let key2 = k2 as *const role_trans_key;
    let mut v = cmp_int((*key1).role, (*key2).role);
    if v != 0 { return v; }
    v = cmp_int((*key1).type_, (*key2).type_);
    if v != 0 { return v; }
    cmp_int((*key1).tclass as u32, (*key2).tclass as u32)
}

static roletr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(role_trans_hash),
    cmp: Some(role_trans_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_roletr_search(p: *mut policydb, key: *mut role_trans_key) -> *mut role_trans_datum {
    hashtab_search(&mut (*p).role_tr, key as *const c_void, roletr_key_params) as *mut role_trans_datum
}

unsafe fn policydb_init(p: *mut policydb) {
    memset(p as *mut c_void, 0, size_of::<policydb>());
    avtab_init(&mut (*p).te_avtab);
    cond_policydb_init(p);
    ebitmap_init(&mut (*p).filename_trans_ttypes);
    ebitmap_init(&mut (*p).policycaps);
    ebitmap_init(&mut (*p).permissive_map);
    ebitmap_init(&mut (*p).neveraudit_map);
}

unsafe extern "C" fn common_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let comdatum = datum as *mut common_datum;
    let p = datap as *mut policydb;
    if (*comdatum).value == 0 || (*comdatum).value > (*p).symtab[SYM_COMMONS as usize].nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_COMMONS as usize].add((*comdatum).value as usize - 1) = key as *mut c_char;
    0
}

unsafe extern "C" fn class_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let cladatum = datum as *mut class_datum;
    let p = datap as *mut policydb;
    if (*cladatum).value == 0 || (*cladatum).value as u32 > (*p).symtab[SYM_CLASSES as usize].nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_CLASSES as usize].add((*cladatum).value as usize - 1) = key as *mut c_char;
    *(*p).class_val_to_struct.add((*cladatum).value as usize - 1) = cladatum;
    0
}

unsafe extern "C" fn role_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let role = datum as *mut role_datum;
    let p = datap as *mut policydb;
    if (*role).value == 0 || (*role).value > (*p).symtab[SYM_ROLES as usize].nprim || (*role).bounds > (*p).symtab[SYM_ROLES as usize].nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_ROLES as usize].add((*role).value as usize - 1) = key as *mut c_char;
    *(*p).role_val_to_struct.add((*role).value as usize - 1) = role;
    0
}

unsafe extern "C" fn type_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let typdatum = datum as *mut type_datum;
    let p = datap as *mut policydb;
    if (*typdatum).value == 0 || (*typdatum).value > (*p).symtab[SYM_TYPES as usize].nprim || (*typdatum).bounds > (*p).symtab[SYM_TYPES as usize].nprim {
        pr_err(b"SELinux: type %s had value %u bounds %u nprim %u\n\0".as_ptr() as *const c_char, key as *mut c_char, (*typdatum).value, (*typdatum).bounds, (*p).symtab[SYM_TYPES as usize].nprim);
        return -EINVAL;
    }
    if (*typdatum).primary != 0 {
        *(*p).sym_val_to_name[SYM_TYPES as usize].add((*typdatum).value as usize - 1) = key as *mut c_char;
        *(*p).type_val_to_struct.add((*typdatum).value as usize - 1) = typdatum;
    }
    0
}

unsafe extern "C" fn user_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let usrdatum = datum as *mut user_datum;
    let p = datap as *mut policydb;
    if (*usrdatum).value == 0 || (*usrdatum).value > (*p).symtab[SYM_USERS as usize].nprim || (*usrdatum).bounds > (*p).symtab[SYM_USERS as usize].nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_USERS as usize].add((*usrdatum).value as usize - 1) = key as *mut c_char;
    *(*p).user_val_to_struct.add((*usrdatum).value as usize - 1) = usrdatum;
    0
}

unsafe extern "C" fn sens_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let levdatum = datum as *mut level_datum;
    let p = datap as *mut policydb;
    if (*levdatum).level.sens == 0 || (*levdatum).level.sens > (*p).symtab[SYM_LEVELS as usize].nprim { return -EINVAL; }
    if (*levdatum).isalias == 0 {
        *(*p).sym_val_to_name[SYM_LEVELS as usize].add((*levdatum).level.sens as usize - 1) = key as *mut c_char;
    }
    0
}

unsafe extern "C" fn cat_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let catdatum = datum as *mut cat_datum;
    let p = datap as *mut policydb;
    if (*catdatum).value == 0 || (*catdatum).value > (*p).symtab[SYM_CATS as usize].nprim { return -EINVAL; }
    if (*catdatum).isalias == 0 {
        *(*p).sym_val_to_name[SYM_CATS as usize].add((*catdatum).value as usize - 1) = key as *mut c_char;
    }
    0
}

unsafe extern "C" fn sens_cat_index_check(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p = datap as *mut policydb;
    let levdatum = datum as *mut level_datum;
    let mut node: *mut ebitmap_node = ptr::null_mut();
    let mut bit = ebitmap_start_positive(&(*levdatum).level.cat, &mut node);
    while bit < (*levdatum).level.cat.highbit {
        if bit >= (*p).symtab[SYM_CATS as usize].nprim || sym_name(p, SYM_CATS, bit).is_null() {
            pr_err(b"SELinux: sensitivity %s allows undefined category %u\n\0".as_ptr() as *const c_char, key as *const c_char, bit + 1);
            return -EINVAL;
        }
        bit = ebitmap_next_positive(&(*levdatum).level.cat, &mut node, bit);
    }
    0
}

static index_f: [Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>; SYM_NUM as usize] = [
    Some(common_index), Some(class_index), Some(role_index), Some(type_index),
    Some(user_index), Some(cond_index_bool), Some(sens_index), Some(cat_index),
];

unsafe fn hash_eval(h: *mut hashtab, hash_name: *const c_char, hash_details: *const c_char) {
    if LUPOS_POLICYDB_DEBUG == 0 { return; }
    let mut info: hashtab_info = mem::zeroed();
    hashtab_stat(h, &mut info);
    pr_debug(b"SELinux: %s%s%s:  %d entries and %d/%d buckets used, longest chain length %d, sum of chain length^2 %llu\n\0".as_ptr() as *const c_char,
        hash_name, if hash_details.is_null() { b"\0".as_ptr() } else { b"@\0".as_ptr() } as *const c_char,
        if hash_details.is_null() { b"\0".as_ptr() as *const c_char } else { hash_details },
        (*h).nel, info.slots_used, (*h).size, info.max_chain_len, info.chain2_len_sum);
}

unsafe fn symtab_hash_eval(s: *mut symtab) {
    if LUPOS_POLICYDB_DEBUG == 0 { return; }
    let names: [&[u8]; SYM_NUM as usize] = [
        b"common prefixes\0", b"classes\0", b"roles\0", b"types\0",
        b"users\0", b"bools\0", b"levels\0", b"categories\0",
    ];
    for (i, name) in names.iter().enumerate() {
        hash_eval(&mut (*s.add(i)).table, name.as_ptr().cast(), ptr::null());
    }
}

unsafe fn policydb_index(p: *mut policydb) -> c_int {
    let mut i: usize;
    let mut rc: c_int;
    let mut v: u32;
    if (*p).mls_enabled != 0 {
        pr_debug(b"SELinux:  %d users, %d roles, %d types, %d bools, %d sens, %d cats\n\0".as_ptr().cast(),
            (*p).symtab[SYM_USERS as usize].nprim, (*p).symtab[SYM_ROLES as usize].nprim,
            (*p).symtab[SYM_TYPES as usize].nprim, (*p).symtab[SYM_BOOLS as usize].nprim,
            (*p).symtab[SYM_LEVELS as usize].nprim, (*p).symtab[SYM_CATS as usize].nprim);
    } else {
        pr_debug(b"SELinux:  %d users, %d roles, %d types, %d bools\n\0".as_ptr().cast(),
            (*p).symtab[SYM_USERS as usize].nprim, (*p).symtab[SYM_ROLES as usize].nprim,
            (*p).symtab[SYM_TYPES as usize].nprim, (*p).symtab[SYM_BOOLS as usize].nprim);
    }
    pr_debug(b"SELinux:  %d classes, %d rules\n\0".as_ptr().cast(),
        (*p).symtab[SYM_CLASSES as usize].nprim, (*p).te_avtab.nel);
    symtab_hash_eval((*p).symtab.as_mut_ptr());
    (*p).class_val_to_struct = kzalloc_objs::<*mut class_datum>((*p).symtab[SYM_CLASSES as usize].nprim);
    if (*p).class_val_to_struct.is_null() { return -ENOMEM; }
    (*p).role_val_to_struct = kzalloc_objs::<*mut role_datum>((*p).symtab[SYM_ROLES as usize].nprim);
    if (*p).role_val_to_struct.is_null() { return -ENOMEM; }
    (*p).user_val_to_struct = kzalloc_objs::<*mut user_datum>((*p).symtab[SYM_USERS as usize].nprim);
    if (*p).user_val_to_struct.is_null() { return -ENOMEM; }
    (*p).type_val_to_struct = kvzalloc_objs::<*mut type_datum>((*p).symtab[SYM_TYPES as usize].nprim);
    if (*p).type_val_to_struct.is_null() { return -ENOMEM; }
    rc = cond_init_bool_indexes(p);
    if rc != 0 { return rc; }
    i = 0;
    while i < SYM_NUM as usize {
        (*p).sym_val_to_name[i] = kvcalloc::<*mut c_char>((*p).symtab[i].nprim);
        if (*p).sym_val_to_name[i].is_null() { return -ENOMEM; }
        rc = hashtab_map(ptr::addr_of_mut!((*p).symtab[i].table), index_f[i], p as *mut c_void);
        if rc != 0 { return rc; }
        i += 1;
    }
    v = 0;
    while v < (*p).symtab[SYM_BOOLS as usize].nprim {
        if (*(*p).bool_val_to_struct.add(v as usize)).is_null() {
            pr_err(b"SELinux:  boolean %u is declared but not defined\n\0".as_ptr().cast(), v + 1);
            return -EINVAL;
        }
        v += 1;
    }
    if (*p).mls_enabled != 0 {
        rc = hashtab_map(ptr::addr_of_mut!((*p).symtab[SYM_LEVELS as usize].table), Some(sens_cat_index_check), p as *mut c_void);
        if rc != 0 { return rc; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn policydb_destroy(p: *mut policydb) {
    let mut i = 0usize;
    while i < SYM_NUM as usize {
        cond_resched();
        hashtab_map(&mut (*p).symtab[i].table, destroy_f[i], ptr::null_mut());
        hashtab_destroy(&mut (*p).symtab[i].table);
        i += 1;
    }
    i = 0;
    while i < SYM_NUM as usize {
        kvfree((*p).sym_val_to_name[i] as *mut c_void);
        i += 1;
    }
    kfree((*p).class_val_to_struct as *mut c_void);
    kfree((*p).role_val_to_struct as *mut c_void);
    kfree((*p).user_val_to_struct as *mut c_void);
    kvfree((*p).type_val_to_struct as *mut c_void);
    avtab_destroy(&mut (*p).te_avtab);
    i = 0;
    while i < OCON_NUM as usize {
        cond_resched();
        let mut c = (*p).ocontexts[i];
        while !c.is_null() {
            let ctmp = c;
            c = (*c).next;
            ocontext_destroy(ctmp, i as c_uint);
        }
        (*p).ocontexts[i] = ptr::null_mut();
        i += 1;
    }
    let mut g = (*p).genfs;
    while !g.is_null() {
        cond_resched();
        kfree((*g).fstype as *mut c_void);
        let mut c = (*g).head;
        while !c.is_null() {
            let ctmp = c;
            c = (*c).next;
            ocontext_destroy(ctmp, OCON_FSUSE as c_uint);
        }
        let gtmp = g;
        g = (*g).next;
        kfree(gtmp as *mut c_void);
    }
    (*p).genfs = ptr::null_mut();
    cond_policydb_destroy(p);
    hashtab_map(&mut (*p).role_tr, Some(role_tr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).role_tr);
    let mut ra = (*p).role_allow;
    let mut lra: *mut role_allow = ptr::null_mut();
    while !ra.is_null() {
        cond_resched();
        kfree(lra as *mut c_void);
        lra = ra;
        ra = (*ra).next;
    }
    kfree(lra as *mut c_void);
    hashtab_map(&mut (*p).filename_trans, Some(filenametr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).filename_trans);
    hashtab_map(&mut (*p).range_tr, Some(range_tr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).range_tr);
    if !(*p).type_attr_map_array.is_null() {
        i = 0;
        while i < (*p).symtab[SYM_TYPES as usize].nprim as usize {
            ebitmap_destroy((*p).type_attr_map_array.add(i));
            i += 1;
        }
        kvfree((*p).type_attr_map_array as *mut c_void);
    }
    ebitmap_destroy(&mut (*p).filename_trans_ttypes);
    ebitmap_destroy(&mut (*p).policycaps);
    ebitmap_destroy(&mut (*p).permissive_map);
    ebitmap_destroy(&mut (*p).neveraudit_map);
}

#[no_mangle]
pub unsafe extern "C" fn policydb_load_isids(p: *mut policydb, s: *mut sidtab) -> c_int {
    let mut rc = sidtab_init(s);
    if rc != 0 {
        pr_err(b"SELinux:  out of memory on SID table init\n\0".as_ptr() as *const c_char);
        return rc;
    }
    let isid_init = ebitmap_get_bit(&(*p).policycaps, POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT) != 0;
    let mut c = (*p).ocontexts[OCON_ISID as usize];
    while !c.is_null() {
        let sid = (*c).sid[0];
        let name = security_get_initial_sid_context(sid);
        if sid == SECSID_NULL {
            pr_err(b"SELinux:  SID 0 was assigned a context.\n\0".as_ptr() as *const c_char);
            sidtab_destroy(s);
            return -EINVAL;
        }
        if name.is_null() {
            c = (*c).next;
            continue;
        }
        if sid == SECINITSID_INIT && !isid_init {
            c = (*c).next;
            continue;
        }
        rc = sidtab_set_initial(s, sid, &mut (*c).context[0]);
        if rc != 0 {
            pr_err(b"SELinux:  unable to load initial SID %s.\n\0".as_ptr() as *const c_char, name);
            sidtab_destroy(s);
            return rc;
        }
        if sid == SECINITSID_KERNEL && !isid_init {
            rc = sidtab_set_initial(s, SECINITSID_INIT, &mut (*c).context[0]);
            if rc != 0 {
                pr_err(b"SELinux:  unable to load initial SID %s.\n\0".as_ptr() as *const c_char, name);
                sidtab_destroy(s);
                return rc;
            }
        }
        c = (*c).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn policydb_class_isvalid(p: *const policydb, class: u16) -> bool {
    if class == 0 || class as u32 > (*p).symtab[SYM_CLASSES as usize].nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_CLASSES as usize].add(class as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_user_isvalid(p: *const policydb, user: u32) -> bool {
    if user == 0 || user > (*p).symtab[SYM_USERS as usize].nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_USERS as usize].add(user as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_role_isvalid(p: *const policydb, role: u32) -> bool {
    if role == 0 || role > (*p).symtab[SYM_ROLES as usize].nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_ROLES as usize].add(role as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_type_isvalid(p: *const policydb, type_: u32) -> bool {
    if type_ == 0 || type_ > (*p).symtab[SYM_TYPES as usize].nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_TYPES as usize].add(type_ as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_simpletype_isvalid(p: *const policydb, type_: u32) -> bool {
    if type_ == 0 || type_ > (*p).symtab[SYM_TYPES as usize].nprim { return false; }
    let datum = *(*p).type_val_to_struct.add(type_ as usize - 1);
    if datum.is_null() { return false; }
    if (*datum).attribute != 0 { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn policydb_context_isvalid(p: *const policydb, c: *const context) -> bool {
    if (*c).role == 0 || (*c).role > (*p).symtab[SYM_ROLES as usize].nprim { return false; }
    if (*c).user == 0 || (*c).user > (*p).symtab[SYM_USERS as usize].nprim { return false; }
    if (*c).type_ == 0 || (*c).type_ > (*p).symtab[SYM_TYPES as usize].nprim { return false; }
    if (*c).role != OBJECT_R_VAL {
        let role = *(*p).role_val_to_struct.add((*c).role as usize - 1);
        if role.is_null() || ebitmap_get_bit(&(*role).types, (*c).type_ - 1) == 0 { return false; }
        let usrdatum = *(*p).user_val_to_struct.add((*c).user as usize - 1);
        if usrdatum.is_null() { return false; }
        if ebitmap_get_bit(&(*usrdatum).roles, (*c).role - 1) == 0 { return false; }
    }
    if !mls_context_isvalid(p, c) { return false; }
    true
}
