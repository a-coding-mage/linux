// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL in-memory console interface
 *
 * Copyright 2014 IBM Corp.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct memcons {
    pub magic: __be64,
    pub obuf_phys: __be64,
    pub ibuf_phys: __be64,
    pub obuf_size: __be32,
    pub ibuf_size: __be32,
    pub out_pos: __be32,
    pub in_prod: __be32,
    pub in_cons: __be32,
}

pub const MEMCONS_MAGIC: u64 = 0x6630_6965_6772_6173;
pub const MEMCONS_OUT_POS_WRAP: u32 = 0x8000_0000;
pub const MEMCONS_OUT_POS_MASK: u32 = 0x00ff_ffff;

static mut opal_memcons: *mut memcons = core::ptr::null_mut();

pub unsafe fn memcons_copy(
    mc: *mut memcons,
    mut to: *mut c_char,
    mut pos: loff_t,
    mut count: usize,
) -> ssize_t {
    let conbuf: *const c_char;
    let mut ret: ssize_t;
    let mut first_read: usize = 0;
    let mut out_pos: u32;
    let mut avail: u32;

    if mc.is_null() {
        return -ENODEV;
    }

    out_pos = be32_to_cpu(core::ptr::read_volatile(&(*mc).out_pos));

    // Now we've read out_pos, put a barrier in before reading the new
    // data it points to in conbuf.
    smp_rmb();

    conbuf = phys_to_virt(be64_to_cpu((*mc).obuf_phys));

    // When the buffer has wrapped, read from the out_pos marker to the end
    // of the buffer, and then read the remaining data as in the un-wrapped
    // case.
    if (out_pos & MEMCONS_OUT_POS_WRAP) != 0 {
        out_pos &= MEMCONS_OUT_POS_MASK;
        avail = be32_to_cpu((*mc).obuf_size) - out_pos;

        ret = memory_read_from_buffer(
            to,
            count,
            &mut pos,
            conbuf.add(out_pos as usize),
            avail as usize,
        );

        if ret < 0 {
            return ret;
        }

        first_read = ret as usize;
        to = to.add(first_read);
        count -= first_read;
        pos -= avail as loff_t;

        if count <= 0 {
            return first_read as ssize_t;
        }

    }

    // Sanity check. The firmware should not do this to us.
    if out_pos > be32_to_cpu((*mc).obuf_size) {
        pr_err!("OPAL: memory console corruption. Aborting read.\n");
        return -EINVAL;
    }

    ret = memory_read_from_buffer(
        to,
        count,
        &mut pos,
        conbuf,
        out_pos as usize,
    );

    if ret < 0 {
        return ret;
    }

    ret + first_read as ssize_t
}

pub unsafe fn opal_msglog_copy(to: *mut c_char, pos: loff_t, count: usize) -> ssize_t {
    memcons_copy(opal_memcons, to, pos, count)
}

unsafe fn opal_msglog_read(
    _file: *mut file,
    _kobj: *mut kobject,
    _bin_attr: *const bin_attribute,
    to: *mut c_char,
    pos: loff_t,
    count: usize,
) -> ssize_t {
    opal_msglog_copy(to, pos, count)
}

static mut opal_msglog_attr: bin_attribute = bin_attribute {
    attr: attribute {
        name: "msglog".as_ptr() as *const c_char,
        mode: 0o400,
    },
    size: 0,
    read: Some(opal_msglog_read),
};

pub unsafe fn memcons_init(node: *mut device_node, mc_prop_name: *const c_char) -> *mut memcons {
    let mut mcaddr: u64 = 0;
    let mc: *mut memcons;

    if of_property_read_u64(node, mc_prop_name, &mut mcaddr) != 0 {
        pr_warn!("%s property not found, no message log\n", mc_prop_name);
        return core::ptr::null_mut();
    }

    mc = phys_to_virt(mcaddr);
    if mc.is_null() {
        pr_warn!("memory console address is invalid\n");
        return core::ptr::null_mut();
    }

    if be64_to_cpu((*mc).magic) != MEMCONS_MAGIC {
        pr_warn!("memory console version is invalid\n");
        return core::ptr::null_mut();
    }

    mc
}

pub unsafe fn memcons_get_size(mc: *mut memcons) -> u32 {
    be32_to_cpu((*mc).ibuf_size) + be32_to_cpu((*mc).obuf_size)
}

pub unsafe fn opal_msglog_init() {
    opal_memcons = memcons_init(opal_node, b"ibm,opal-memcons\0".as_ptr() as *const c_char);
    if opal_memcons.is_null() {
        pr_warn!("OPAL: memcons failed to load from ibm,opal-memcons\n");
        return;
    }

    opal_msglog_attr.size = memcons_get_size(opal_memcons) as u64;
}

pub unsafe fn opal_msglog_sysfs_init() {
    if opal_memcons.is_null() {
        pr_warn!("OPAL: message log initialisation failed, not creating sysfs entry\n");
        return;
    }

    if sysfs_create_bin_file(opal_kobj, &mut opal_msglog_attr) != 0 {
        pr_warn!("OPAL: sysfs file creation failed\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
