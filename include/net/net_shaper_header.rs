/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Dependencies supplied by the Linux type and uAPI headers are referenced here. */

use core::ffi::{c_int, c_ulong};

pub struct net_device;
pub struct devlink;
pub struct netlink_ext_ack;
pub struct rcu_head;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_shaper_binding_type {
    NET_SHAPER_BINDING_TYPE_NETDEV,
    /* NET_SHAPER_BINDING_TYPE_DEVLINK_PORT */
}

#[repr(C)]
pub union net_shaper_binding__bindgen_ty_1 {
    pub netdev: *mut net_device,
    pub devlink: *mut devlink,
}

#[repr(C)]
pub struct net_shaper_binding {
    pub type_: net_shaper_binding_type,
    pub __bindgen_anon_1: net_shaper_binding__bindgen_ty_1,
}

#[repr(C)]
pub struct net_shaper_handle {
    pub scope: net_shaper_scope,
    pub id: u32,
}

/**
 * struct net_shaper - represents a shaping node on the NIC H/W
 * zeroed field are considered not set.
 * @parent: Unique identifier for the shaper parent, usually implied
 * @handle: Unique identifier for this shaper
 * @metric: Specify if the rate limits refers to PPS or BPS
 * @bw_min: Minimum guaranteed rate for this shaper
 * @bw_max: Maximum peak rate allowed for this shaper
 * @burst: Maximum burst for the peek rate of this shaper
 * @priority: Scheduling priority for this shaper
 * @weight: Scheduling weight for this shaper
 */
#[repr(C)]
pub struct net_shaper {
    pub parent: net_shaper_handle,
    pub handle: net_shaper_handle,
    pub metric: net_shaper_metric,
    pub bw_min: u64,
    pub bw_max: u64,
    pub burst: u64,
    pub priority: u32,
    pub weight: u32,

    /* private: */
    pub leaves: u32, /* accounted only for NODE scope */
    pub valid: bool,
    pub rcu: rcu_head,
}

/**
 * struct net_shaper_ops - Operations on device H/W shapers
 *
 * The operations applies to either net_device and devlink objects.
 * The initial shaping configuration at device initialization is empty:
 * does not constraint the rate in any way.
 * The network core keeps track of the applied user-configuration in
 * the net_device or devlink structure.
 * The operations are serialized via a per device lock.
 *
 * Device not supporting any kind of nesting should not provide the
 * @group operation.
 *
 * Each shaper is uniquely identified within the device with a 'handle'
 * comprising the shaper scope and a scope-specific id.
 *
 * Driver ops vs uAPI
 * ------------------
 * Members of the driver ops mirror the Netlink uAPI but driver calls do not
 * map 1:1 to user calls. Drivers need to be careful when assuming that calls
 * disallowed at the uAPI level will never be made at the driver level.
 * The shaper core performs automatic reparenting and cleanup, generating
 * additional calls. Notably:
 *
 * - @group calls in the driver facing API may have nodes as leaves (user is
 *   only allowed to construct groups with queues as leaves)
 * - @group calls may update leaf's parent if the parent is about
 *   to be removed (re-parenting nodes explicitly is not supported in the uAPI)
 *
 * Implicit creation
 * -----------------
 * Shapers are created implicitly, meaning that @set and @group operations
 * are called both for existing and new shapers. The driver has to infer
 * whether the operation is an update or a creation by tracking the handles.
 * Removal of shapers is explicit and done with a @delete call.
 *
 * The @set operation implicitly creates NET_SHAPER_SCOPE_NETDEV and
 * NET_SHAPER_SCOPE_QUEUE shapers.
 * The @group operation implicitly creates NET_SHAPER_SCOPE_NETDEV and
 * NET_SHAPER_SCOPE_NODE shapers (the group shaper itself), as well as
 * NET_SHAPER_SCOPE_QUEUE shapers (leaves).
 */
#[repr(C)]
pub struct net_shaper_ops {
    pub group: Option<unsafe extern "C" fn(
        binding: *mut net_shaper_binding,
        leaves_count: c_int,
        leaves: *const net_shaper,
        node: *const net_shaper,
        extack: *mut netlink_ext_ack,
    ) -> c_int>,

    pub set: Option<unsafe extern "C" fn(
        binding: *mut net_shaper_binding,
        shaper: *const net_shaper,
        extack: *mut netlink_ext_ack,
    ) -> c_int>,

    pub delete: Option<unsafe extern "C" fn(
        binding: *mut net_shaper_binding,
        handle: *const net_shaper_handle,
        extack: *mut netlink_ext_ack,
    ) -> c_int>,

    pub capabilities: Option<unsafe extern "C" fn(
        binding: *mut net_shaper_binding,
        scope: net_shaper_scope,
        cap: *mut c_ulong,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
