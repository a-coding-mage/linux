/* SPDX-License-Identifier: GPL-2.0-only */
/* Authors: Karl MacMillan <kmacmillan@tresys.com>
 *	    Frank Mayer <mayerf@tresys.com>
 *          Copyright (C) 2003 - 2004 Tresys Technology, LLC
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

pub type u16_t = u16;
pub type u32_t = u32;
pub type __le32 = u32;

pub const GFP_KERNEL: u32 = 0;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;

pub const SYM_BOOLS: usize = 3;

pub const COND_BOOL: u32 = 1;
pub const COND_NOT: u32 = 2;
pub const COND_OR: u32 = 3;
pub const COND_AND: u32 = 4;
pub const COND_XOR: u32 = 5;
pub const COND_EQ: u32 = 6;
pub const COND_NEQ: u32 = 7;
pub const COND_LAST: u32 = COND_NEQ;
pub const COND_EXPR_MAXDEPTH: usize = 10;

pub const AVTAB_ALLOWED: u16 = 0x0001;
pub const AVTAB_AUDITALLOW: u16 = 0x0002;
pub const AVTAB_AUDITDENY: u16 = 0x0004;
pub const AVTAB_TYPE: u16 = 0x0040;
pub const AVTAB_ENABLED: u16 = 0x8000;
pub const AVTAB_XPERMS: u16 = 0x0100;

#[repr(C)]
pub struct policydb {
    pub bool_val_to_struct: *mut *mut cond_bool_datum,
    pub cond_list: *mut cond_node,
    pub cond_list_len: u32,
    pub te_cond_avtab: avtab,
    pub te_avtab: avtab,
    pub p_bools: symtab,
    pub sym_val_to_name: *mut *mut *mut c_void,
}

#[repr(C)]
pub struct cond_expr {
    pub nodes: *mut cond_expr_node,
    pub len: u32,
}

#[repr(C)]
pub struct cond_expr_node {
    pub expr_type: u32,
    pub boolean: u32,
}

#[repr(C)]
pub struct cond_av_list {
    pub nodes: *mut *mut avtab_node,
    pub len: u32,
}

#[repr(C)]
pub struct cond_node {
    pub cur_state: c_int,
    pub expr: cond_expr,
    pub true_list: cond_av_list,
    pub false_list: cond_av_list,
}

#[repr(C)]
pub struct cond_bool_datum {
    pub value: u32,
    pub state: c_int,
}

#[repr(C)]
pub struct avtab {
    pub nel: u32,
}

#[repr(C)]
pub struct avtab_key {
    pub specified: u16,
}

#[repr(C)]
pub union avtab_datum_u {
    pub data: u32,
}

#[repr(C)]
pub struct avtab_datum {
    pub u: avtab_datum_u,
}

#[repr(C)]
pub struct avtab_node {
    pub key: avtab_key,
    pub datum: avtab_datum,
}

#[repr(C)]
pub struct symtab {
    pub table: hashtab,
    pub nprim: u32,
}

#[repr(C)]
pub struct hashtab {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashtab_node {
    pub key: *mut c_void,
    pub datum: *mut c_void,
}

#[repr(C)]
pub struct policy_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policy_data {
    pub fp: *mut policy_file,
}

#[repr(C)]
pub struct av_decision {
    pub allowed: u32,
    pub auditallow: u32,
    pub auditdeny: u32,
}

#[repr(C)]
pub struct extended_perms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct extended_perms_decision {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn kfree(ptr: *const c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: usize, flags: u32) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;

    fn avtab_init(h: *mut avtab);
    fn avtab_destroy(h: *mut avtab);
    fn avtab_hash_eval(h: *mut avtab, tag: *const c_char);
    fn avtab_search_node(h: *mut avtab, key: *const avtab_key) -> *mut avtab_node;
    fn avtab_search_node_next(node: *mut avtab_node, specified: c_int) -> *mut avtab_node;
    fn avtab_insert_nonunique(
        h: *mut avtab,
        key: *const avtab_key,
        datum: *const avtab_datum,
    ) -> *mut avtab_node;
    fn avtab_read_item(
        a: *mut avtab,
        fp: *mut policy_file,
        p: *mut policydb,
        insertf: unsafe extern "C" fn(
            *mut avtab,
            *const avtab_key,
            *const avtab_datum,
            *mut c_void,
        ) -> c_int,
        ptr: *mut c_void,
        expected: bool,
    ) -> c_int;
    fn avtab_alloc(h: *mut avtab, nrules: u32) -> c_int;
    fn avtab_write_item(p: *mut policydb, node: *mut avtab_node, fp: *mut policy_file) -> c_int;
    fn avtab_alloc_dup(new: *mut avtab, orig: *const avtab) -> c_int;

    fn next_entry(buf: *mut c_void, fp: *mut policy_file, bytes: usize) -> c_int;
    fn size_check(entry_size: usize, n: u32, fp: *mut policy_file) -> c_int;
    fn str_read(dest: *mut *mut c_char, flags: u32, fp: *mut policy_file, len: u32) -> c_int;
    fn symtab_insert(s: *mut symtab, key: *mut c_char, datum: *mut cond_bool_datum) -> c_int;
    fn put_entry(buf: *const c_void, bytes: usize, num: u32, fp: *mut policy_file) -> c_int;
    fn val_is_boolean(val: u32) -> bool;

    fn services_compute_xperms_decision(xpermd: *mut extended_perms_decision, node: *mut avtab_node);
    fn services_compute_xperms_drivers(xperms: *mut extended_perms, node: *mut avtab_node);

    fn hashtab_duplicate(
        new: *mut hashtab,
        orig: *const hashtab,
        copy: unsafe extern "C" fn(*mut hashtab_node, *const hashtab_node, *mut c_void) -> c_int,
        destroy: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int,
        args: *mut c_void,
    ) -> c_int;
    fn hashtab_map(
        h: *mut hashtab,
        apply: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int,
        args: *mut c_void,
    ) -> c_int;
    fn hashtab_destroy(h: *mut hashtab);
}

macro_rules! pr_err {
    ($s:expr) => {
        printk(concat!($s, "\0").as_ptr() as *const c_char)
    };
}

macro_rules! goto_err {
    ($rc:expr, $key:expr, $booldatum:expr) => {{
        pr_err!("SELinux: conditional: failed to read boolean\n");
        cond_destroy_bool($key as *mut c_void, $booldatum as *mut c_void, ptr::null_mut());
        return $rc;
    }};
}

unsafe fn le32_to_cpu(x: __le32) -> u32 {
    u32::from_le(x)
}

unsafe fn cpu_to_le32(x: u32) -> __le32 {
    x.to_le()
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_objs<T>(n: u32) -> *mut T {
    kzalloc(size_of::<T>().wrapping_mul(n as usize), GFP_KERNEL) as *mut T
}

/*
 * cond_evaluate_expr evaluates a conditional expr
 * in reverse polish notation. It returns true (1), false (0),
 * or undefined (-1). Undefined occurs when the expression
 * exceeds the stack depth of COND_EXPR_MAXDEPTH.
 */
unsafe fn cond_evaluate_expr(p: *mut policydb, expr: *mut cond_expr) -> c_int {
    let mut i: u32;
    let mut s = [0 as c_int; COND_EXPR_MAXDEPTH];
    let mut sp: c_int = -1;

    if (*expr).len == 0 {
        return -1;
    }

    i = 0;
    while i < (*expr).len {
        let node = (*expr).nodes.add(i as usize);

        match (*node).expr_type {
            COND_BOOL => {
                if sp == (COND_EXPR_MAXDEPTH as c_int - 1) {
                    return -1;
                }
                sp += 1;
                s[sp as usize] = (**(*p).bool_val_to_struct.add(((*node).boolean - 1) as usize)).state;
            }
            COND_NOT => {
                if sp < 0 {
                    return -1;
                }
                s[sp as usize] = (s[sp as usize] == 0) as c_int;
            }
            COND_OR => {
                if sp < 1 {
                    return -1;
                }
                sp -= 1;
                s[sp as usize] |= s[(sp + 1) as usize];
            }
            COND_AND => {
                if sp < 1 {
                    return -1;
                }
                sp -= 1;
                s[sp as usize] &= s[(sp + 1) as usize];
            }
            COND_XOR => {
                if sp < 1 {
                    return -1;
                }
                sp -= 1;
                s[sp as usize] ^= s[(sp + 1) as usize];
            }
            COND_EQ => {
                if sp < 1 {
                    return -1;
                }
                sp -= 1;
                s[sp as usize] = (s[sp as usize] == s[(sp + 1) as usize]) as c_int;
            }
            COND_NEQ => {
                if sp < 1 {
                    return -1;
                }
                sp -= 1;
                s[sp as usize] = (s[sp as usize] != s[(sp + 1) as usize]) as c_int;
            }
            _ => return -1,
        }
        i += 1;
    }
    s[0]
}

/*
 * evaluate_cond_node evaluates the conditional stored in
 * a struct cond_node and if the result is different than the
 * current state of the node it sets the rules in the true/false
 * list appropriately. If the result of the expression is undefined
 * all of the rules are disabled for safety.
 */
unsafe fn evaluate_cond_node(p: *mut policydb, node: *mut cond_node) {
    let mut avnode: *mut avtab_node;
    let new_state: c_int;
    let mut i: u32;

    new_state = cond_evaluate_expr(p, &mut (*node).expr);
    if new_state != (*node).cur_state {
        (*node).cur_state = new_state;
        if new_state == -1 {
            pr_err!("SELinux: expression result was undefined - disabling all rules.\n");
        }
        /* turn the rules on or off */
        i = 0;
        while i < (*node).true_list.len {
            avnode = *(*node).true_list.nodes.add(i as usize);
            if new_state <= 0 {
                (*avnode).key.specified &= !AVTAB_ENABLED;
            } else {
                (*avnode).key.specified |= AVTAB_ENABLED;
            }
            i += 1;
        }

        i = 0;
        while i < (*node).false_list.len {
            avnode = *(*node).false_list.nodes.add(i as usize);
            /* -1 or 1 */
            if new_state != 0 {
                (*avnode).key.specified &= !AVTAB_ENABLED;
            } else {
                (*avnode).key.specified |= AVTAB_ENABLED;
            }
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evaluate_cond_nodes(p: *mut policydb) {
    let mut i: u32 = 0;

    while i < (*p).cond_list_len {
        evaluate_cond_node(p, (*p).cond_list.add(i as usize));
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_policydb_init(p: *mut policydb) {
    (*p).bool_val_to_struct = ptr::null_mut();
    (*p).cond_list = ptr::null_mut();
    (*p).cond_list_len = 0;

    avtab_init(&mut (*p).te_cond_avtab);
}

unsafe fn cond_node_destroy(node: *mut cond_node) {
    kfree((*node).expr.nodes as *const c_void);
    /* the avtab_ptr_t nodes are destroyed by the avtab */
    kfree((*node).true_list.nodes as *const c_void);
    kfree((*node).false_list.nodes as *const c_void);
}

unsafe fn cond_list_destroy(p: *mut policydb) {
    let mut i: u32 = 0;

    while i < (*p).cond_list_len {
        cond_node_destroy((*p).cond_list.add(i as usize));
        i += 1;
    }
    kfree((*p).cond_list as *const c_void);
    (*p).cond_list = ptr::null_mut();
    (*p).cond_list_len = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_policydb_destroy(p: *mut policydb) {
    kfree((*p).bool_val_to_struct as *const c_void);
    avtab_destroy(&mut (*p).te_cond_avtab);
    cond_list_destroy(p);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_init_bool_indexes(p: *mut policydb) -> c_int {
    kfree((*p).bool_val_to_struct as *const c_void);
    (*p).bool_val_to_struct = kzalloc_objs::<*mut cond_bool_datum>((*p).p_bools.nprim);
    if (*p).bool_val_to_struct.is_null() {
        return -ENOMEM;
    }

    avtab_hash_eval(&mut (*p).te_cond_avtab, c"conditional_rules".as_ptr());

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_destroy_bool(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_index_bool(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p: *mut policydb;
    let booldatum: *mut cond_bool_datum;

    booldatum = datum as *mut cond_bool_datum;
    p = datap as *mut policydb;

    if (*booldatum).value == 0 || (*booldatum).value > (*p).p_bools.nprim {
        return -EINVAL;
    }

    *(*(*p).sym_val_to_name.add(SYM_BOOLS)).add(((*booldatum).value - 1) as usize) = key;
    *(*p).bool_val_to_struct.add(((*booldatum).value - 1) as usize) = booldatum;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_read_bool(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let booldatum: *mut cond_bool_datum;
    let mut buf = [0 as __le32; 3];
    let len: u32;
    let val: u32;
    let mut rc: c_int;

    booldatum = kzalloc_obj::<cond_bool_datum>();
    if booldatum.is_null() {
        return -ENOMEM;
    }

    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc != 0 {
        goto_err!(rc, key, booldatum);
    }

    (*booldatum).value = le32_to_cpu(buf[0]);
    val = le32_to_cpu(buf[1]);

    rc = -EINVAL;
    if !val_is_boolean(val) {
        goto_err!(rc, key, booldatum);
    }
    (*booldatum).state = val as c_int;

    len = le32_to_cpu(buf[2]);

    rc = str_read(&mut key, GFP_KERNEL, fp, len);
    if rc != 0 {
        goto_err!(rc, key, booldatum);
    }

    rc = symtab_insert(s, key, booldatum);
    if rc != 0 {
        goto_err!(rc, key, booldatum);
    }

    return 0;
}

#[repr(C)]
struct cond_insertf_data {
    p: *mut policydb,
    dst: *mut *mut avtab_node,
    other: *mut cond_av_list,
}

unsafe extern "C" fn cond_insertf(
    _a: *mut avtab,
    k: *const avtab_key,
    d: *const avtab_datum,
    ptr: *mut c_void,
) -> c_int {
    let data = ptr as *mut cond_insertf_data;
    let p = (*data).p;
    let other = (*data).other;
    let mut node_ptr: *mut avtab_node;
    let mut i: u32;
    let mut found: bool;

    /*
     * For type rules we have to make certain there aren't any
     * conflicting rules by searching the te_avtab and the
     * cond_te_avtab.
     */
    if ((*k).specified & AVTAB_TYPE) != 0 {
        if !avtab_search_node(&mut (*p).te_avtab, k).is_null() {
            pr_err!("SELinux: type rule already exists outside of a conditional.\n");
            return -EINVAL;
        }
        /*
         * If we are reading the false list other will be a pointer to
         * the true list. We can have duplicate entries if there is only
         * 1 other entry and it is in our true list.
         *
         * If we are reading the true list (other == NULL) there shouldn't
         * be any other entries.
         */
        if !other.is_null() {
            node_ptr = avtab_search_node(&mut (*p).te_cond_avtab, k);
            if !node_ptr.is_null() {
                if !avtab_search_node_next(node_ptr, (*k).specified as c_int).is_null() {
                    pr_err!("SELinux: too many conflicting type rules.\n");
                    return -EINVAL;
                }
                found = false;
                i = 0;
                while i < (*other).len {
                    if *(*other).nodes.add(i as usize) == node_ptr {
                        found = true;
                        break;
                    }
                    i += 1;
                }
                if !found {
                    pr_err!("SELinux: conflicting type rules.\n");
                    return -EINVAL;
                }
            }
        } else if !avtab_search_node(&mut (*p).te_cond_avtab, k).is_null() {
            pr_err!("SELinux: conflicting type rules when adding type rule for true.\n");
            return -EINVAL;
        }
    }

    node_ptr = avtab_insert_nonunique(&mut (*p).te_cond_avtab, k, d);
    if node_ptr.is_null() {
        pr_err!("SELinux: could not insert rule.\n");
        return -ENOMEM;
    }

    *(*data).dst = node_ptr;
    0
}

unsafe fn cond_read_av_list(
    p: *mut policydb,
    fp: *mut policy_file,
    list: *mut cond_av_list,
    other: *mut cond_av_list,
) -> c_int {
    let mut rc: c_int;
    let mut buf = [0 as __le32; 1];
    let mut i: u32;
    let len: u32;
    let mut data = cond_insertf_data {
        p: ptr::null_mut(),
        dst: ptr::null_mut(),
        other: ptr::null_mut(),
    };

    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32>());
    if rc != 0 {
        return rc;
    }

    len = le32_to_cpu(buf[0]);
    if len == 0 {
        return 0;
    }

    /* avtab_read_item() reads at least 96 bytes for any valid entry */
    rc = size_check(3 * size_of::<u32>(), len, fp);
    if rc != 0 {
        return rc;
    }

    (*list).nodes = kzalloc_objs::<*mut avtab_node>(len);
    if (*list).nodes.is_null() {
        return -ENOMEM;
    }

    data.p = p;
    data.other = other;
    i = 0;
    while i < len {
        data.dst = (*list).nodes.add(i as usize);
        rc = avtab_read_item(&mut (*p).te_cond_avtab, fp, p, cond_insertf, &mut data as *mut _ as *mut c_void, true);
        if rc != 0 {
            kfree((*list).nodes as *const c_void);
            (*list).nodes = ptr::null_mut();
            return rc;
        }
        i += 1;
    }

    (*list).len = len;
    0
}

unsafe fn expr_node_isvalid(p: *mut policydb, expr: *mut cond_expr_node) -> c_int {
    if (*expr).expr_type <= 0 || (*expr).expr_type > COND_LAST {
        pr_err!("SELinux: conditional expressions uses unknown operator.\n");
        return 0;
    }

    if (*expr).expr_type == COND_BOOL
        && ((*expr).boolean == 0 || (*expr).boolean > (*p).p_bools.nprim)
    {
        pr_err!("SELinux: conditional expressions uses unknown bool.\n");
        return 0;
    }
    1
}

unsafe fn cond_read_node(p: *mut policydb, node: *mut cond_node, fp: *mut policy_file) -> c_int {
    let mut buf = [0 as __le32; 2];
    let mut i: u32;
    let len: u32;
    let mut rc: c_int;

    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32>() * 2);
    if rc != 0 {
        return rc;
    }

    (*node).cur_state = le32_to_cpu(buf[0]) as c_int;

    /* expr */
    len = le32_to_cpu(buf[1]);

    /* we will read 64 bytes per node */
    rc = size_check(2 * size_of::<u32>(), len, fp);
    if rc != 0 {
        return rc;
    }

    (*node).expr.nodes = kzalloc_objs::<cond_expr_node>(len);
    if (*node).expr.nodes.is_null() {
        return -ENOMEM;
    }

    (*node).expr.len = len;

    i = 0;
    while i < len {
        let expr = (*node).expr.nodes.add(i as usize);

        rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32>() * 2);
        if rc != 0 {
            return rc;
        }

        (*expr).expr_type = le32_to_cpu(buf[0]);
        (*expr).boolean = le32_to_cpu(buf[1]);

        if expr_node_isvalid(p, expr) == 0 {
            return -EINVAL;
        }
        i += 1;
    }

    rc = cond_read_av_list(p, fp, &mut (*node).true_list, ptr::null_mut());
    if rc != 0 {
        return rc;
    }
    cond_read_av_list(p, fp, &mut (*node).false_list, &mut (*node).true_list)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_read_list(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf = [0 as __le32; 1];
    let mut i: u32;
    let len: u32;
    let mut rc: c_int;

    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc != 0 {
        return rc;
    }

    len = le32_to_cpu(buf[0]);

    /* cond_read_node() reads at least 128 bytes for any valid node */
    rc = size_check(4 * size_of::<u32>(), len, fp);
    if rc != 0 {
        return rc;
    }

    (*p).cond_list = kzalloc_objs::<cond_node>(len);
    if (*p).cond_list.is_null() {
        return -ENOMEM;
    }

    rc = avtab_alloc(&mut (*p).te_cond_avtab, (*p).te_avtab.nel);
    if rc != 0 {
        cond_list_destroy(p);
        return rc;
    }

    (*p).cond_list_len = len;

    i = 0;
    while i < len {
        rc = cond_read_node(p, (*p).cond_list.add(i as usize), fp);
        if rc != 0 {
            cond_list_destroy(p);
            return rc;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_write_bool(vkey: *mut c_void, datum: *mut c_void, ptr: *mut c_void) -> c_int {
    let key = vkey as *mut c_char;
    let booldatum = datum as *mut cond_bool_datum;
    let pd = ptr as *mut policy_data;
    let fp = (*pd).fp;
    let mut buf = [0 as __le32; 3];
    let len: u32;
    let mut rc: c_int;

    len = strlen(key) as u32;
    buf[0] = cpu_to_le32((*booldatum).value);
    buf[1] = cpu_to_le32((*booldatum).state as u32);
    buf[2] = cpu_to_le32(len);
    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 3, fp);
    if rc != 0 {
        return rc;
    }
    rc = put_entry(key as *const c_void, 1, len, fp);
    if rc != 0 {
        return rc;
    }
    0
}

/*
 * cond_write_cond_av_list doesn't write out the av_list nodes.
 * Instead it writes out the key/value pairs from the avtab. This
 * is necessary because there is no way to uniquely identifying rules
 * in the avtab so it is not possible to associate individual rules
 * in the avtab with a conditional without saving them as part of
 * the conditional. This means that the avtab with the conditional
 * rules will not be saved but will be rebuilt on policy load.
 */
unsafe fn cond_write_av_list(p: *mut policydb, list: *mut cond_av_list, fp: *mut policy_file) -> c_int {
    let mut buf = [0 as __le32; 1];
    let mut i: u32;
    let mut rc: c_int;

    buf[0] = cpu_to_le32((*list).len);
    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
    if rc != 0 {
        return rc;
    }

    i = 0;
    while i < (*list).len {
        rc = avtab_write_item(p, *(*list).nodes.add(i as usize), fp);
        if rc != 0 {
            return rc;
        }
        i += 1;
    }

    0
}

unsafe fn cond_write_node(p: *mut policydb, node: *mut cond_node, fp: *mut policy_file) -> c_int {
    let mut buf = [0 as __le32; 2];
    let mut rc: c_int;
    let mut i: u32;

    buf[0] = cpu_to_le32((*node).cur_state as u32);
    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
    if rc != 0 {
        return rc;
    }

    buf[0] = cpu_to_le32((*node).expr.len);
    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
    if rc != 0 {
        return rc;
    }

    i = 0;
    while i < (*node).expr.len {
        buf[0] = cpu_to_le32((*(*node).expr.nodes.add(i as usize)).expr_type);
        buf[1] = cpu_to_le32((*(*node).expr.nodes.add(i as usize)).boolean);
        rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 2, fp);
        if rc != 0 {
            return rc;
        }
        i += 1;
    }

    rc = cond_write_av_list(p, &mut (*node).true_list, fp);
    if rc != 0 {
        return rc;
    }
    rc = cond_write_av_list(p, &mut (*node).false_list, fp);
    if rc != 0 {
        return rc;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_write_list(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut i: u32;
    let mut buf = [0 as __le32; 1];
    let mut rc: c_int;

    buf[0] = cpu_to_le32((*p).cond_list_len);
    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
    if rc != 0 {
        return rc;
    }

    i = 0;
    while i < (*p).cond_list_len {
        rc = cond_write_node(p, (*p).cond_list.add(i as usize), fp);
        if rc != 0 {
            return rc;
        }
        i += 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_compute_xperms(
    ctab: *mut avtab,
    key: *mut avtab_key,
    xpermd: *mut extended_perms_decision,
) {
    let mut node: *mut avtab_node;

    if ctab.is_null() || key.is_null() || xpermd.is_null() {
        return;
    }

    node = avtab_search_node(ctab, key);
    while !node.is_null() {
        if ((*node).key.specified & AVTAB_ENABLED) != 0 {
            services_compute_xperms_decision(xpermd, node);
        }
        node = avtab_search_node_next(node, (*key).specified as c_int);
    }
}

/* Determine whether additional permissions are granted by the conditional
 * av table, and if so, add them to the result
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_compute_av(
    ctab: *mut avtab,
    key: *mut avtab_key,
    avd: *mut av_decision,
    xperms: *mut extended_perms,
) {
    let mut node: *mut avtab_node;

    if ctab.is_null() || key.is_null() || avd.is_null() {
        return;
    }

    node = avtab_search_node(ctab, key);
    while !node.is_null() {
        if (AVTAB_ALLOWED | AVTAB_ENABLED)
            == ((*node).key.specified & (AVTAB_ALLOWED | AVTAB_ENABLED))
        {
            (*avd).allowed |= (*node).datum.u.data;
        }
        if (AVTAB_AUDITDENY | AVTAB_ENABLED)
            == ((*node).key.specified & (AVTAB_AUDITDENY | AVTAB_ENABLED))
        {
            /* Since a '0' in an auditdeny mask represents a
             * permission we do NOT want to audit (dontaudit), we use
             * the '&' operand to ensure that all '0's in the mask
             * are retained (much unlike the allow and auditallow cases).
             */
            (*avd).auditdeny &= (*node).datum.u.data;
        }
        if (AVTAB_AUDITALLOW | AVTAB_ENABLED)
            == ((*node).key.specified & (AVTAB_AUDITALLOW | AVTAB_ENABLED))
        {
            (*avd).auditallow |= (*node).datum.u.data;
        }
        if !xperms.is_null()
            && ((*node).key.specified & AVTAB_ENABLED) != 0
            && ((*node).key.specified & AVTAB_XPERMS) != 0
        {
            services_compute_xperms_drivers(xperms, node);
        }
        node = avtab_search_node_next(node, (*key).specified as c_int);
    }
}

unsafe fn cond_dup_av_list(
    new: *mut cond_av_list,
    orig: *const cond_av_list,
    avtab: *mut avtab,
) -> c_int {
    let mut i: u32;

    memset(new as *mut c_void, 0, size_of::<cond_av_list>());

    (*new).nodes = kzalloc_objs::<*mut avtab_node>((*orig).len);
    if (*new).nodes.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*orig).len {
        *(*new).nodes.add(i as usize) = avtab_insert_nonunique(
            avtab,
            &(*(*(*orig).nodes.add(i as usize))).key,
            &(*(*(*orig).nodes.add(i as usize))).datum,
        );
        if (*(*new).nodes.add(i as usize)).is_null() {
            return -ENOMEM;
        }
        (*new).len += 1;
        i += 1;
    }

    0
}

unsafe fn duplicate_policydb_cond_list(newp: *mut policydb, origp: *const policydb) -> c_int {
    let mut rc: c_int;
    let mut i: u32;

    rc = avtab_alloc_dup(&mut (*newp).te_cond_avtab, &(*origp).te_cond_avtab);
    if rc != 0 {
        return rc;
    }

    (*newp).cond_list_len = 0;
    (*newp).cond_list = kzalloc_objs::<cond_node>((*origp).cond_list_len);
    if (*newp).cond_list.is_null() {
        avtab_destroy(&mut (*newp).te_cond_avtab);
        cond_list_destroy(newp);
        return -ENOMEM;
    }

    i = 0;
    while i < (*origp).cond_list_len {
        let newn = (*newp).cond_list.add(i as usize);
        let orign = (*origp).cond_list.add(i as usize);

        (*newp).cond_list_len += 1;

        (*newn).cur_state = (*orign).cur_state;
        (*newn).expr.nodes = kmemdup(
            (*orign).expr.nodes as *const c_void,
            ((*orign).expr.len as usize).wrapping_mul(size_of::<cond_expr_node>()),
            GFP_KERNEL,
        ) as *mut cond_expr_node;
        if (*newn).expr.nodes.is_null() {
            avtab_destroy(&mut (*newp).te_cond_avtab);
            cond_list_destroy(newp);
            return -ENOMEM;
        }

        (*newn).expr.len = (*orign).expr.len;

        rc = cond_dup_av_list(&mut (*newn).true_list, &(*orign).true_list, &mut (*newp).te_cond_avtab);
        if rc != 0 {
            avtab_destroy(&mut (*newp).te_cond_avtab);
            cond_list_destroy(newp);
            return -ENOMEM;
        }

        rc = cond_dup_av_list(&mut (*newn).false_list, &(*orign).false_list, &mut (*newp).te_cond_avtab);
        if rc != 0 {
            avtab_destroy(&mut (*newp).te_cond_avtab);
            cond_list_destroy(newp);
            return -ENOMEM;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn cond_bools_destroy(_key: *mut c_void, datum: *mut c_void, _args: *mut c_void) -> c_int {
    /* key was not copied so no need to free here */
    kfree(datum);
    0
}

unsafe extern "C" fn cond_bools_copy(new: *mut hashtab_node, orig: *const hashtab_node, _args: *mut c_void) -> c_int {
    let datum: *mut cond_bool_datum;

    datum = kmemdup((*orig).datum, size_of::<cond_bool_datum>(), GFP_KERNEL) as *mut cond_bool_datum;
    if datum.is_null() {
        return -ENOMEM;
    }

    (*new).key = (*orig).key; /* No need to copy, never modified */
    (*new).datum = datum as *mut c_void;
    0
}

unsafe extern "C" fn cond_bools_index(_key: *mut c_void, datum: *mut c_void, args: *mut c_void) -> c_int {
    let booldatum: *mut cond_bool_datum;
    let cond_bool_array: *mut *mut cond_bool_datum;

    booldatum = datum as *mut cond_bool_datum;
    cond_bool_array = args as *mut *mut cond_bool_datum;
    *cond_bool_array.add(((*booldatum).value - 1) as usize) = booldatum;

    0
}

unsafe fn duplicate_policydb_bools(newdb: *mut policydb, orig: *const policydb) -> c_int {
    let cond_bool_array: *mut *mut cond_bool_datum;
    let rc: c_int;

    cond_bool_array = kzalloc_objs::<*mut cond_bool_datum>((*orig).p_bools.nprim);
    if cond_bool_array.is_null() {
        return -ENOMEM;
    }

    rc = hashtab_duplicate(
        &mut (*newdb).p_bools.table,
        &(*orig).p_bools.table,
        cond_bools_copy,
        cond_bools_destroy,
        ptr::null_mut(),
    );
    if rc != 0 {
        kfree(cond_bool_array as *const c_void);
        return -ENOMEM;
    }

    hashtab_map(&mut (*newdb).p_bools.table, cond_bools_index, cond_bool_array as *mut c_void);
    (*newdb).bool_val_to_struct = cond_bool_array;

    (*newdb).p_bools.nprim = (*orig).p_bools.nprim;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_policydb_destroy_dup(p: *mut policydb) {
    hashtab_map(&mut (*p).p_bools.table, cond_bools_destroy, ptr::null_mut());
    hashtab_destroy(&mut (*p).p_bools.table);
    cond_policydb_destroy(p);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cond_policydb_dup(new: *mut policydb, orig: *const policydb) -> c_int {
    cond_policydb_init(new);

    if duplicate_policydb_bools(new, orig) != 0 {
        return -ENOMEM;
    }

    if duplicate_policydb_cond_list(new, orig) != 0 {
        cond_policydb_destroy_dup(new);
        return -ENOMEM;
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
