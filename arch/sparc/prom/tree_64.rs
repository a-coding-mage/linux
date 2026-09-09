// SPDX-License-Identifier: GPL-2.0
/*
 * tree.c: Basic device tree traversal/scanning for the Linux
 *         prom library.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

type Phandle = u32;

extern "C" {
    static prom_peer_name: *const core::ffi::c_char;
    static prom_getprop_name: *const core::ffi::c_char;
    static ldom_domaining_enabled: bool;

    fn p1275_cmd_direct(args: *mut usize);
    fn ldom_set_var(name: *const core::ffi::c_char, value: *mut core::ffi::c_char);
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char);
}

unsafe fn prom_node_to_node(type_: *const core::ffi::c_char, node: Phandle) -> Phandle {
    let mut args: [usize; 5] = [0; 5];

    args[0] = type_ as usize;
    args[1] = 1;
    args[2] = 1;
    args[3] = node as u32 as usize;
    args[4] = (-1isize) as usize;

    p1275_cmd_direct(args.as_mut_ptr());

    args[4] as Phandle
}

#[inline]
pub unsafe fn __prom_getchild(node: Phandle) -> Phandle {
    prom_node_to_node(b"child\0".as_ptr() as *const core::ffi::c_char, node)
}

pub unsafe fn prom_getchild(node: Phandle) -> Phandle {
    let cnode: Phandle;

    if node as i32 == -1 {
        return 0;
    }
    cnode = __prom_getchild(node);
    if cnode as i32 == -1 {
        return 0;
    }
    cnode
}

#[inline]
pub unsafe fn prom_getparent(node: Phandle) -> Phandle {
    let cnode: Phandle;

    if node as i32 == -1 {
        return 0;
    }
    cnode = prom_node_to_node(b"parent\0".as_ptr() as *const core::ffi::c_char, node);
    if cnode as i32 == -1 {
        return 0;
    }
    cnode
}

#[inline]
pub unsafe fn __prom_getsibling(node: Phandle) -> Phandle {
    prom_node_to_node(prom_peer_name, node)
}

pub unsafe fn prom_getsibling(node: Phandle) -> Phandle {
    let sibnode: Phandle;

    if node as i32 == -1 {
        return 0;
    }
    sibnode = __prom_getsibling(node);
    if sibnode as i32 == -1 {
        return 0;
    }

    sibnode
}

pub unsafe fn prom_getproplen(node: Phandle, prop: *const core::ffi::c_char) -> i32 {
    let mut args: [usize; 6] = [0; 6];

    if node == 0 || prop.is_null() {
        return -1;
    }

    args[0] = b"getproplen\0".as_ptr() as usize;
    args[1] = 2;
    args[2] = 1;
    args[3] = node as u32 as usize;
    args[4] = prop as usize;
    args[5] = (-1isize) as usize;

    p1275_cmd_direct(args.as_mut_ptr());

    args[5] as i32
}

pub unsafe fn prom_getproperty(node: Phandle, prop: *const core::ffi::c_char,
                               buffer: *mut core::ffi::c_char, bufsize: i32) -> i32 {
    let mut args: [usize; 8] = [0; 8];
    let plen = prom_getproplen(node, prop);
    if plen > bufsize || plen == 0 || plen == -1 {
        return -1;
    }

    args[0] = prom_getprop_name as usize;
    args[1] = 4;
    args[2] = 1;
    args[3] = node as u32 as usize;
    args[4] = prop as usize;
    args[5] = buffer as usize;
    args[6] = bufsize as usize;
    args[7] = (-1isize) as usize;

    p1275_cmd_direct(args.as_mut_ptr());
    args[7] as i32
}

pub unsafe fn prom_getint(node: Phandle, prop: *const core::ffi::c_char) -> i32 {
    let mut intprop: i32 = 0;
    if prom_getproperty(node, prop, &mut intprop as *mut i32 as *mut core::ffi::c_char,
                        core::mem::size_of::<i32>() as i32) != -1 {
        return intprop;
    }
    -1
}

pub unsafe fn prom_getintdefault(node: Phandle, property: *const core::ffi::c_char, deflt: i32) -> i32 {
    let retval = prom_getint(node, property);
    if retval == -1 { deflt } else { retval }
}

pub unsafe fn prom_getbool(node: Phandle, prop: *const core::ffi::c_char) -> i32 {
    if prom_getproplen(node, prop) == -1 { 0 } else { 1 }
}

pub unsafe fn prom_getstring(node: Phandle, prop: *const core::ffi::c_char,
                             user_buf: *mut core::ffi::c_char, ubuf_size: i32) {
    if prom_getproperty(node, prop, user_buf, ubuf_size) == -1 {
        *user_buf = 0;
    }
}

pub unsafe fn prom_nodematch(node: Phandle, name: *const core::ffi::c_char) -> i32 {
    let mut namebuf = [0i8; 128];
    prom_getproperty(node, b"name\0".as_ptr() as *const i8, namebuf.as_mut_ptr(), 128);
    if strcmp(namebuf.as_ptr(), name) == 0 { 1 } else { 0 }
}

pub unsafe fn prom_searchsiblings(node_start: Phandle, nodename: *const core::ffi::c_char) -> Phandle {
    let mut thisnode = node_start;
    let mut promlib_buf = [0i8; 128];
    while thisnode != 0 {
        let error = prom_getproperty(thisnode, b"name\0".as_ptr() as *const i8,
                                     promlib_buf.as_mut_ptr(), 128);
        if error != -1 && strcmp(nodename, promlib_buf.as_ptr()) == 0 { return thisnode; }
        thisnode = prom_getsibling(thisnode);
    }
    0
}

static prom_nextprop_name: &[u8] = b"nextprop\0";

pub unsafe fn prom_firstprop(node: Phandle, buffer: *mut core::ffi::c_char) -> *mut core::ffi::c_char {
    *buffer = 0;
    if node as i32 == -1 { return buffer; }
    let mut args: [usize; 7] = [0; 7];
    args[0] = prom_nextprop_name.as_ptr() as usize;
    args[1] = 3; args[2] = 1; args[3] = node as u32 as usize; args[4] = 0;
    args[5] = buffer as usize; args[6] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    buffer
}

pub unsafe fn prom_nextprop(node: Phandle, oprop: *const core::ffi::c_char,
                            buffer: *mut core::ffi::c_char) -> *mut core::ffi::c_char {
    let mut buf = [0i8; 32];
    if node as i32 == -1 { *buffer = 0; return buffer; }
    let mut prop = oprop;
    if oprop == buffer {
        strscpy(buf.as_mut_ptr(), oprop);
        prop = buf.as_ptr();
    }
    let mut args: [usize; 7] = [0; 7];
    args[0] = prom_nextprop_name.as_ptr() as usize;
    args[1] = 3; args[2] = 1; args[3] = node as u32 as usize; args[4] = prop as usize;
    args[5] = buffer as usize; args[6] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    buffer
}

pub unsafe fn prom_finddevice(name: *const core::ffi::c_char) -> Phandle {
    if name.is_null() { return 0; }
    let mut args: [usize; 5] = [0; 5];
    args[0] = b"finddevice\0".as_ptr() as usize;
    args[1] = 1; args[2] = 1; args[3] = name as usize; args[4] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    args[4] as i32 as Phandle
}

pub unsafe fn prom_node_has_property(node: Phandle, prop: *const core::ffi::c_char) -> i32 {
    let mut buf = [0i8; 32];
    *buf.as_mut_ptr() = 0;
    loop {
        prom_nextprop(node, buf.as_ptr(), buf.as_mut_ptr());
        if strcmp(buf.as_ptr(), prop) == 0 { return 1; }
        if *buf.as_ptr() == 0 { break; }
    }
    0
}

pub unsafe fn prom_setprop(node: Phandle, pname: *const core::ffi::c_char,
                           value: *mut core::ffi::c_char, size: i32) -> i32 {
    if size == 0 || pname.is_null() || value.is_null() { return 0; }
    // CONFIG_SUN_LDOMS: preserve the conditional build-time behavior.
    let mut args: [usize; 8] = [0; 8];
    args[0] = b"setprop\0".as_ptr() as usize;
    args[1] = 4; args[2] = 1; args[3] = node as u32 as usize; args[4] = pname as usize;
    args[5] = value as usize; args[6] = size as usize; args[7] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    args[7] as i32
}

#[inline]
pub unsafe fn prom_inst2pkg(inst: i32) -> Phandle {
    let mut args: [usize; 5] = [0; 5];
    args[0] = b"instance-to-package\0".as_ptr() as usize;
    args[1] = 1; args[2] = 1; args[3] = inst as u32 as usize; args[4] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    let node = args[4] as i32 as Phandle;
    if node as i32 == -1 { 0 } else { node }
}

pub unsafe fn prom_ihandle2path(handle: i32, buffer: *mut core::ffi::c_char, bufsize: i32) -> i32 {
    let mut args: [usize; 7] = [0; 7];
    args[0] = b"instance-to-path\0".as_ptr() as usize;
    args[1] = 3; args[2] = 1; args[3] = handle as u32 as usize;
    args[4] = buffer as usize; args[5] = bufsize as usize; args[6] = (-1isize) as usize;
    p1275_cmd_direct(args.as_mut_ptr());
    args[6] as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
