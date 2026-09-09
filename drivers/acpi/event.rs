// SPDX-License-Identifier: GPL-2.0
/*
 * event.c - exporting ACPI events via procfs
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 */

// C includes and kernel-provided symbols are supplied by other translation units.

/* ACPI notifier chain */
static mut acpi_chain_head: BlockingNotifierHead = BLOCKING_NOTIFIER_HEAD!();

pub unsafe fn acpi_notifier_call_chain(
    device_class: *const core::ffi::c_char,
    bus_id: *const core::ffi::c_char,
    r#type: u32,
    data: u32,
) -> i32 {
    let mut event: acpi_bus_event = core::mem::zeroed();

    strscpy(event.device_class.as_mut_ptr(), device_class);
    strscpy(event.bus_id.as_mut_ptr(), bus_id);
    event.r#type = r#type;
    event.data = data;
    (blocking_notifier_call_chain(
        &raw mut acpi_chain_head,
        0,
        &mut event as *mut _ as *mut core::ffi::c_void,
    ) == NOTIFY_BAD) as i32 * -EINVAL
}

pub unsafe fn register_acpi_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_register(&raw mut acpi_chain_head, nb)
}

pub unsafe fn unregister_acpi_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_unregister(&raw mut acpi_chain_head, nb)
}

#[cfg(CONFIG_NET)]
static mut acpi_event_seqnum: u32 = 0;

#[cfg(CONFIG_NET)]
#[repr(C)]
struct acpi_genl_event {
    device_class: acpi_device_class,
    bus_id: [core::ffi::c_char; 15],
    r#type: u32,
    data: u32,
}

#[cfg(CONFIG_NET)]
enum {
    ACPI_GENL_ATTR_UNSPEC,
    ACPI_GENL_ATTR_EVENT,
    __ACPI_GENL_ATTR_MAX,
}

#[cfg(CONFIG_NET)]
const ACPI_GENL_ATTR_MAX: i32 = __ACPI_GENL_ATTR_MAX - 1;

#[cfg(CONFIG_NET)]
enum {
    ACPI_GENL_CMD_UNSPEC,
    ACPI_GENL_CMD_EVENT,
    __ACPI_GENL_CMD_MAX,
}

#[cfg(CONFIG_NET)]
const ACPI_GENL_CMD_MAX: i32 = __ACPI_GENL_CMD_MAX - 1;

#[cfg(CONFIG_NET)]
const ACPI_GENL_FAMILY_NAME: &[u8] = b"acpi_event\0";
#[cfg(CONFIG_NET)]
const ACPI_GENL_VERSION: u8 = 0x01;
#[cfg(CONFIG_NET)]
const ACPI_GENL_MCAST_GROUP_NAME: &[u8] = b"acpi_mc_group\0";

#[cfg(CONFIG_NET)]
static mut acpi_event_mcgrps: [genl_multicast_group; 1] = [genl_multicast_group {
    name: ACPI_GENL_MCAST_GROUP_NAME.as_ptr() as *const core::ffi::c_char,
}];

#[cfg(CONFIG_NET)]
static mut acpi_event_genl_family: genl_family = genl_family {
    module: THIS_MODULE,
    name: ACPI_GENL_FAMILY_NAME.as_ptr() as *const core::ffi::c_char,
    version: ACPI_GENL_VERSION,
    maxattr: ACPI_GENL_ATTR_MAX,
    mcgrps: unsafe { acpi_event_mcgrps.as_ptr() },
    n_mcgrps: 1,
};

#[cfg(CONFIG_NET)]
pub unsafe fn acpi_bus_generate_netlink_event(
    device_class: *const core::ffi::c_char,
    bus_id: *const core::ffi::c_char,
    r#type: u8,
    data: i32,
) -> i32 {
    let size = nla_total_size(core::mem::size_of::<acpi_genl_event>()) + nla_total_size(0);
    let skb = genlmsg_new(size, GFP_ATOMIC);
    if skb.is_null() { return -ENOMEM; }

    let msg_header = genlmsg_put(
        skb, 0, acpi_event_seqnum, &raw mut acpi_event_genl_family, 0,
        ACPI_GENL_CMD_EVENT,
    );
    acpi_event_seqnum = acpi_event_seqnum.wrapping_add(1);
    if msg_header.is_null() { nlmsg_free(skb); return -ENOMEM; }

    let attr = nla_reserve(skb, ACPI_GENL_ATTR_EVENT, core::mem::size_of::<acpi_genl_event>());
    if attr.is_null() { nlmsg_free(skb); return -EINVAL; }

    let event = nla_data(attr) as *mut acpi_genl_event;
    core::ptr::write_bytes(event, 0, 1);
    strscpy((*event).device_class.as_mut_ptr(), device_class, core::mem::size_of_val(&(*event).device_class));
    strscpy((*event).bus_id.as_mut_ptr(), bus_id, core::mem::size_of_val(&(*event).bus_id));
    (*event).r#type = r#type as u32;
    (*event).data = data as u32;

    genlmsg_end(skb, msg_header);
    genlmsg_multicast(&raw mut acpi_event_genl_family, skb, 0, 0, GFP_ATOMIC);
    0
}

#[cfg(CONFIG_NET)]
unsafe fn acpi_event_genetlink_init() -> i32 {
    genl_register_family(&raw mut acpi_event_genl_family)
}

#[cfg(not(CONFIG_NET))]
pub unsafe fn acpi_bus_generate_netlink_event(
    _device_class: *const core::ffi::c_char,
    _bus_id: *const core::ffi::c_char,
    _type: u8,
    _data: i32,
) -> i32 { 0 }

#[cfg(not(CONFIG_NET))]
unsafe fn acpi_event_genetlink_init() -> i32 { -ENODEV }

unsafe fn acpi_event_init() -> i32 {
    let mut error: i32;
    if acpi_disabled { return 0; }

    error = acpi_event_genetlink_init();
    if error != 0 {
        pr_warn!("Failed to create genetlink family for ACPI event\n");
    }
    0
}

fs_initcall!(acpi_event_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
