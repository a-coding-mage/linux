/* Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependencies supplied by vmlinux.h, net_shared.h, and the BPF headers.
// The SEC, BPF_CORE_READ, BPF_CORE_READ_STR_INTO, and PT_REGS_PARM1
// constructs below correspond to their C definitions.

#[repr(C)]
pub struct pt_regs {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

type u32 = core::ffi::c_uint;

unsafe extern "C" {
    fn bpf_core_read(dst: *mut core::ffi::c_void, size: usize, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_trace_printk(fmt: *const core::ffi::c_char, fmt_size: usize, ...) -> i64;
}

/* kprobe is NOT a stable ABI
 * kernel functions can be removed, renamed or completely change semantics.
 * Number of arguments and their positions can change, etc.
 * In such case this bpf+kprobe example will no longer be meaningful
 */
#[link_section = "kprobe.multi/__netif_receive_skb_core*"]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    /* attaches to kprobe __netif_receive_skb_core,
     * looks for packets on loopback device and prints them
     * (wildcard is used for avoiding symbol mismatch due to optimization)
     */
    let mut devname = [0i8; 16]; // IFNAMSIZ
    let mut dev: *mut net_device;
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut len: i32;

    bpf_core_read(
        (&mut skb as *mut *mut sk_buff).cast(),
        core::mem::size_of::<*mut sk_buff>(),
        (PT_REGS_PARM1(ctx)).cast(),
    );
    dev = BPF_CORE_READ!(skb, dev);
    len = BPF_CORE_READ!(skb, len);

    BPF_CORE_READ_STR_INTO!(&mut devname, dev, name);

    if devname[0] == b'l' as i8 && devname[1] == b'o' as i8 {
        let fmt = *b"skb %p len %d\n\0";
        /* using bpf_trace_printk() for DEBUG ONLY */
        bpf_trace_printk(
            fmt.as_ptr().cast(),
            core::mem::size_of_val(&fmt),
            skb,
            len,
        );
    }

    0
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "version"]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
