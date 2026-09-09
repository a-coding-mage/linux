// SPDX-License-Identifier: GPL-2.0
/*
 * tree.c: Basic device tree traversal/scanning for the Linux
 *         prom library.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut prom_lock: spinlock_t;
    static mut prom_nodeops: *mut prom_nodeops;
    static mut prom_root_node: phandle;
    static mut romvec: *mut linux_romvec;
    fn restore_current();
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn simple_strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

static mut promlib_buf: [c_char; 128] = [0; 128];

/* Internal version of prom_getchild that does not alter return values. */
unsafe fn __prom_getchild(node: phandle) -> phandle {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let cnode = ((*prom_nodeops).no_child)(node);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    cnode
}

/* Return the child of node 'node' or zero if no this node has no
 * direct descendent.
 */
#[no_mangle]
pub unsafe extern "C" fn prom_getchild(node: phandle) -> phandle {
    if node as s32 == -1 { return 0; }
    let cnode = __prom_getchild(node);
    if cnode == 0 || cnode as s32 == -1 { return 0; }
    cnode
}

/* Internal version of prom_getsibling that does not alter return values. */
unsafe fn __prom_getsibling(node: phandle) -> phandle {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let cnode = ((*prom_nodeops).no_nextnode)(node);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    cnode
}

/* Return the next sibling of node 'node' or zero if no more siblings
 * at this level of depth in the tree.
 */
#[no_mangle]
pub unsafe extern "C" fn prom_getsibling(node: phandle) -> phandle {
    if node as s32 == -1 { return 0; }
    let sibnode = __prom_getsibling(node);
    if sibnode == 0 || sibnode as s32 == -1 { return 0; }
    sibnode
}

#[no_mangle]
pub unsafe extern "C" fn prom_getproplen(node: phandle, prop: *const c_char) -> c_int {
    if node == 0 || prop.is_null() { return -1; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let ret = ((*prom_nodeops).no_proplen)(node, prop);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn prom_getproperty(node: phandle, prop: *const c_char, buffer: *mut c_char, bufsize: c_int) -> c_int {
    let plen = prom_getproplen(node, prop);
    if plen > bufsize || plen == 0 || plen == -1 { return -1; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let ret = ((*prom_nodeops).no_getprop)(node, prop, buffer);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn prom_getint(node: phandle, prop: *mut c_char) -> c_int {
    static mut intprop: c_int = 0;
    if prom_getproperty(node, prop, &mut intprop as *mut c_int as *mut c_char, core::mem::size_of::<c_int>() as c_int) != -1 { intprop } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn prom_getintdefault(node: phandle, property: *mut c_char, deflt: c_int) -> c_int {
    let retval = prom_getint(node, property);
    if retval == -1 { deflt } else { retval }
}

#[no_mangle]
pub unsafe extern "C" fn prom_getbool(node: phandle, prop: *mut c_char) -> c_int {
    if prom_getproplen(node, prop) == -1 { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn prom_getstring(node: phandle, prop: *mut c_char, user_buf: *mut c_char, ubuf_size: c_int) {
    if prom_getproperty(node, prop, user_buf, ubuf_size) == -1 { *user_buf = 0; }
}

#[no_mangle]
pub unsafe extern "C" fn prom_searchsiblings(mut node_start: phandle, nodename: *mut c_char) -> phandle {
    while node_start != 0 {
        let error = prom_getproperty(node_start, b"name\0".as_ptr() as *const c_char, promlib_buf.as_mut_ptr(), 128);
        if error != -1 && strcmp(nodename, promlib_buf.as_ptr()) == 0 { return node_start; }
        node_start = prom_getsibling(node_start);
    }
    0
}

unsafe fn __prom_nextprop(node: phandle, oprop: *mut c_char) -> *mut c_char {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let prop = ((*prom_nodeops).no_nextprop)(node, oprop);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    prop
}

#[no_mangle]
pub unsafe extern "C" fn prom_nextprop(node: phandle, oprop: *mut c_char, _buffer: *mut c_char) -> *mut c_char {
    if node == 0 || node as s32 == -1 { return b"\0".as_ptr() as *mut c_char; }
    __prom_nextprop(node, oprop)
}

#[no_mangle]
pub unsafe extern "C" fn prom_finddevice(mut name: *mut c_char) -> phandle {
    let mut nbuf = [0 as c_char; 128];
    let mut s = name;
    let mut node = prom_root_node;
    while *s != 0 {
        s = s.add(1);
        if *s == 0 { return node; }
        let mut d = nbuf.as_mut_ptr();
        while *s != 0 && *s != b'@' as c_char && *s != b'/' as c_char { *d = *s; d = d.add(1); s = s.add(1); }
        *d = 0;
        node = prom_searchsiblings(prom_getchild(node), nbuf.as_mut_ptr());
        if node == 0 { return 0; }
        if *s == b'@' as c_char {
            if isxdigit(*s.add(1) as c_int) != 0 && *s.add(2) == b',' as c_char {
                let which_io = simple_strtoul(s.add(1), core::ptr::null_mut(), 16) as c_uint;
                let phys_addr = simple_strtoul(s.add(3), &mut d, 16) as c_uint;
                if d != s.add(3) && (*d == 0 || *d == b'/' as c_char) && d <= s.add(11) {
                    let mut node2 = node;
                    let mut reg = [core::mem::MaybeUninit::<linux_prom_registers>::uninit(); PROMREG_MAX];
                    while node2 != 0 && node2 as s32 != -1 {
                        if prom_getproperty(node2, b"reg\0".as_ptr() as *const c_char, reg.as_mut_ptr() as *mut c_char, core::mem::size_of_val(&reg) as c_int) > 0 {
                            let r = &*reg[0].as_ptr();
                            if which_io == r.which_io && phys_addr == r.phys_addr { node = node2; break; }
                        }
                        node2 = prom_getsibling(node2);
                        if node2 == 0 || node2 as s32 == -1 { break; }
                        node2 = prom_searchsiblings(prom_getsibling(node2), nbuf.as_mut_ptr());
                    }
                }
            }
            while *s != 0 && *s != b'/' as c_char { s = s.add(1); }
        }
    }
    node
}

#[no_mangle]
pub unsafe extern "C" fn prom_setprop(node: phandle, pname: *const c_char, value: *mut c_char, size: c_int) -> c_int {
    if size == 0 || pname.is_null() || value.is_null() { return 0; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let ret = ((*prom_nodeops).no_setprop)(node, pname, value, size);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn prom_inst2pkg(inst: c_int) -> phandle {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    let node = ((*romvec).pv_v2devops.v2_inst2pkg)(inst);
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    if node as s32 == -1 { 0 } else { node }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
