// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel headers are intentionally left external.

use crate::*;

static mut quota_mcgrps: [genl_multicast_group; 1] = [genl_multicast_group {
    name: b"events\0".as_ptr() as *const i8,
}];

/* Netlink family structure for quota */
static mut quota_genl_family: genl_family = genl_family {
    module: THIS_MODULE,
    hdrsize: 0,
    name: b"VFS_DQUOT\0".as_ptr() as *const i8,
    version: 1,
    maxattr: QUOTA_NL_A_MAX,
    mcgrps: unsafe { quota_mcgrps.as_ptr() },
    n_mcgrps: 1,
};

/**
 * quota_send_warning - Send warning to userspace about exceeded quota
 * @qid: The kernel internal quota identifier.
 * @dev: The device on which the fs is mounted (sb->s_dev)
 * @warntype: The type of the warning: QUOTA_NL_...
 *
 * This can be used by filesystems (including those which don't use
 * dquot) to send a message to userspace relating to quota limits.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn quota_send_warning(qid: kqid, dev: dev_t, warntype: i8) {
    static mut seq: atomic_t = atomic_t { counter: 0 };
    let mut skb: *mut sk_buff;
    let mut msg_head: *mut core::ffi::c_void;
    let mut ret: i32;
    let msg_size: usize = 4 * nla_total_size(core::mem::size_of::<u32>())
        + 2 * nla_total_size_64bit(core::mem::size_of::<u64>());

    /* We have to allocate using GFP_NOFS as we are called from a
     * filesystem performing write and thus further recursion into the
     * fs to free some data could cause deadlocks. */
    skb = genlmsg_new(msg_size, GFP_NOFS);
    if skb.is_null() {
        printk(KERN_ERR, b"VFS: Not enough memory to send quota warning.\n\0".as_ptr());
        return;
    }
    msg_head = genlmsg_put(
        skb,
        0,
        atomic_add_return(1, &mut seq),
        &quota_genl_family,
        0,
        QUOTA_NL_C_WARNING,
    );
    if msg_head.is_null() {
        printk(KERN_ERR, b"VFS: Cannot store netlink header in quota warning.\n\0".as_ptr());
        goto err_out;
    }
    ret = nla_put_u32(skb, QUOTA_NL_A_QTYPE, qid.type_);
    if ret != 0 { goto attr_err_out; }
    ret = nla_put_u64_64bit(
        skb,
        QUOTA_NL_A_EXCESS_ID,
        from_kqid_munged(&init_user_ns, qid),
        QUOTA_NL_A_PAD,
    );
    if ret != 0 { goto attr_err_out; }
    ret = nla_put_u32(skb, QUOTA_NL_A_WARNING, warntype as u32);
    if ret != 0 { goto attr_err_out; }
    ret = nla_put_u32(skb, QUOTA_NL_A_DEV_MAJOR, MAJOR(dev));
    if ret != 0 { goto attr_err_out; }
    ret = nla_put_u32(skb, QUOTA_NL_A_DEV_MINOR, MINOR(dev));
    if ret != 0 { goto attr_err_out; }
    ret = nla_put_u64_64bit(
        skb,
        QUOTA_NL_A_CAUSED_ID,
        from_kuid_munged(&init_user_ns, current_uid()),
        QUOTA_NL_A_PAD,
    );
    if ret != 0 { goto attr_err_out; }
    genlmsg_end(skb, msg_head);

    genlmsg_multicast(&quota_genl_family, skb, 0, 0, GFP_NOFS);
    return;
attr_err_out:
    printk(KERN_ERR, b"VFS: Not enough space to compose quota message!\n\0".as_ptr());
err_out:
    kfree_skb(skb);
}

#[used]
static quota_send_warning_export: unsafe extern "C" fn(kqid, dev_t, i8) = quota_send_warning;

unsafe extern "C" fn quota_init() -> i32 {
    if genl_register_family(&mut quota_genl_family) != 0 {
        printk(KERN_ERR, b"VFS: Failed to create quota netlink interface.\n\0".as_ptr());
    }
    0
}

// fs_initcall(quota_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
