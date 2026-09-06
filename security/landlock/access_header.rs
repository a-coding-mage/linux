/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Access types and helpers
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 */

/*
 * Dependencies from the original header:
 * - <linux/bitops.h>
 * - <linux/build_bug.h>
 * - <linux/kernel.h>
 * - <uapi/linux/landlock.h>
 * - "limits.h"
 */

/*
 * All access rights that are denied by default whether they are handled or not
 * by a ruleset/layer.  This must be ORed with all domain->handled_masks[]
 * entries when we need to get the absolute handled access masks, see
 * landlock_upgrade_handled_access_masks().
 */
pub const _LANDLOCK_ACCESS_FS_INITIALLY_DENIED: u32 = LANDLOCK_ACCESS_FS_REFER;

pub const _LANDLOCK_ACCESS_FS_OPTIONAL: u32 =
    LANDLOCK_ACCESS_FS_TRUNCATE | LANDLOCK_ACCESS_FS_IOCTL_DEV;

pub type access_mask_t = u32;

const fn bits_per_type<T>() -> usize {
    core::mem::size_of::<T>() * 8
}

const fn hweight(mut value: usize) -> usize {
    let mut weight = 0usize;

    while value != 0 {
        weight += value & 1;
        value >>= 1;
    }

    weight
}

/* Makes sure all filesystem access rights can be stored. */
const _: () = assert!(bits_per_type::<access_mask_t>() >= LANDLOCK_NUM_ACCESS_FS as usize);
/* Makes sure all network access rights can be stored. */
const _: () = assert!(bits_per_type::<access_mask_t>() >= LANDLOCK_NUM_ACCESS_NET as usize);
/* Makes sure all scoped rights can be stored. */
const _: () = assert!(bits_per_type::<access_mask_t>() >= LANDLOCK_NUM_SCOPE as usize);
/* Makes sure for_each_set_bit() and for_each_clear_bit() calls are OK. */
const _: () = assert!(core::mem::size_of::<usize>() >= core::mem::size_of::<access_mask_t>());

/* Ruleset access masks. */
#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct access_masks {
    /*
     * Original C bitfields:
     * access_mask_t fs : LANDLOCK_NUM_ACCESS_FS;
     * access_mask_t net : LANDLOCK_NUM_ACCESS_NET;
     * access_mask_t scope : LANDLOCK_NUM_SCOPE;
     */
    pub all: u32,
}

impl access_masks {
    const FS_SHIFT: u32 = 0;
    const NET_SHIFT: u32 = LANDLOCK_NUM_ACCESS_FS as u32;
    const SCOPE_SHIFT: u32 = (LANDLOCK_NUM_ACCESS_FS + LANDLOCK_NUM_ACCESS_NET) as u32;

    const fn field_mask(bits: u32, shift: u32) -> u32 {
        if bits == 0 {
            0
        } else if bits >= 32 {
            u32::MAX << shift
        } else {
            ((1u32 << bits) - 1) << shift
        }
    }

    pub const fn fs(&self) -> access_mask_t {
        (self.all & Self::field_mask(LANDLOCK_NUM_ACCESS_FS as u32, Self::FS_SHIFT))
            >> Self::FS_SHIFT
    }

    pub const fn net(&self) -> access_mask_t {
        (self.all & Self::field_mask(LANDLOCK_NUM_ACCESS_NET as u32, Self::NET_SHIFT))
            >> Self::NET_SHIFT
    }

    pub const fn scope(&self) -> access_mask_t {
        (self.all & Self::field_mask(LANDLOCK_NUM_SCOPE as u32, Self::SCOPE_SHIFT))
            >> Self::SCOPE_SHIFT
    }

    pub fn set_fs(&mut self, fs: access_mask_t) {
        let mask = Self::field_mask(LANDLOCK_NUM_ACCESS_FS as u32, Self::FS_SHIFT);

        self.all = (self.all & !mask) | ((fs << Self::FS_SHIFT) & mask);
    }

    pub fn set_net(&mut self, net: access_mask_t) {
        let mask = Self::field_mask(LANDLOCK_NUM_ACCESS_NET as u32, Self::NET_SHIFT);

        self.all = (self.all & !mask) | ((net << Self::NET_SHIFT) & mask);
    }

    pub fn set_scope(&mut self, scope: access_mask_t) {
        let mask = Self::field_mask(LANDLOCK_NUM_SCOPE as u32, Self::SCOPE_SHIFT);

        self.all = (self.all & !mask) | ((scope << Self::SCOPE_SHIFT) & mask);
    }
}

#[repr(C)]
pub union access_masks_all {
    pub masks: access_masks,
    pub all: u32,
}

/* Makes sure all fields are covered. */
const _: () = assert!(core::mem::size_of::<access_masks>() == core::mem::size_of::<u32>());

/**
 * struct layer_mask - The access rights and rule flags for a layer.
 *
 * This has a bit for each access rights and rule flags.  During access checks,
 * it is used to represent the access rights for each layer which still need to
 * be fulfilled.  When all bits are 0, the access request is considered to be
 * fulfilled.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_mask {
    /**
     * @access: The unfulfilled access rights for this layer.
     */
    /*
     * Original C bitfield:
     * access_mask_t access : LANDLOCK_NUM_ACCESS_MAX;
     *
     * If CONFIG_SECURITY_LANDLOCK_LOG is enabled, the next bit is:
     * access_mask_t quiet : 1;
     */
    pub bits: access_mask_t,
}

impl layer_mask {
    const ACCESS_SHIFT: u32 = 0;
    const QUIET_SHIFT: u32 = LANDLOCK_NUM_ACCESS_MAX as u32;

    const fn field_mask(bits: u32, shift: u32) -> access_mask_t {
        if bits == 0 {
            0
        } else if bits >= 32 {
            u32::MAX << shift
        } else {
            ((1u32 << bits) - 1) << shift
        }
    }

    pub const fn access(&self) -> access_mask_t {
        (self.bits & Self::field_mask(LANDLOCK_NUM_ACCESS_MAX as u32, Self::ACCESS_SHIFT))
            >> Self::ACCESS_SHIFT
    }

    pub fn set_access(&mut self, access: access_mask_t) {
        let mask = Self::field_mask(LANDLOCK_NUM_ACCESS_MAX as u32, Self::ACCESS_SHIFT);

        self.bits = (self.bits & !mask) | ((access << Self::ACCESS_SHIFT) & mask);
    }

    /*
     * CONFIG_SECURITY_LANDLOCK_LOG:
     * @quiet: Whether we have encountered a rule with the quiet flag for
     * this layer.  Used to control logging.
     */
    pub const fn quiet(&self) -> access_mask_t {
        (self.bits & Self::field_mask(1, Self::QUIET_SHIFT)) >> Self::QUIET_SHIFT
    }

    pub fn set_quiet(&mut self, quiet: access_mask_t) {
        let mask = Self::field_mask(1, Self::QUIET_SHIFT);

        self.bits = (self.bits & !mask) | ((quiet << Self::QUIET_SHIFT) & mask);
    }
}

/*
 * Make sure that we don't increase the size of struct layer_mask when storing
 * rule flags.
 */
const _: () = assert!(core::mem::size_of::<layer_mask>() == core::mem::size_of::<access_mask_t>());

/**
 * struct layer_masks - An array of struct layer_mask, one per layer.
 */
#[repr(C)]
pub struct layer_masks {
    /**
     * @layers: The unfulfilled access rights for each layer.
     */
    pub layers: [layer_mask; LANDLOCK_MAX_NUM_LAYERS as usize],
}

/*
 * Tracks domains responsible of a denied access.  This avoids storing in each
 * object the full matrix of per-layer unfulfilled access rights, which is
 * required by update_request().
 *
 * Each nibble represents the layer index of the newest layer which denied a
 * certain access right.  For file system access rights, the upper four bits are
 * the index of the layer which denies LANDLOCK_ACCESS_FS_IOCTL_DEV and the
 * lower nibble represents LANDLOCK_ACCESS_FS_TRUNCATE.
 */
pub type deny_masks_t = u8;

/*
 * Makes sure all optional access rights can be tied to a layer index (cf.
 * get_deny_mask).
 */
const _: () = assert!(
    bits_per_type::<deny_masks_t>()
        >= (hweight((LANDLOCK_MAX_NUM_LAYERS - 1) as usize)
            * hweight(_LANDLOCK_ACCESS_FS_OPTIONAL as usize))
);

/* LANDLOCK_MAX_NUM_LAYERS must be a power of two (cf. deny_masks_t assert). */
const _: () = assert!(hweight(LANDLOCK_MAX_NUM_LAYERS as usize) == 1);

/* Upgrades with all initially denied by default access rights. */
pub fn landlock_upgrade_handled_access_masks(mut access_masks: access_masks) -> access_masks {
    /*
     * All access rights that are denied by default whether they are
     * explicitly handled or not.
     */
    if access_masks.fs() != 0 {
        access_masks.set_fs(access_masks.fs() | _LANDLOCK_ACCESS_FS_INITIALLY_DENIED);
    }

    access_masks
}

/* Checks the subset relation between access masks. */
pub const fn access_mask_subset(subset: access_mask_t, superset: access_mask_t) -> bool {
    (subset | superset) == superset
}

/* A bitmask that is large enough to hold set of optional accesses. */
pub type optional_access_t = u8;
const _: () =
    assert!(bits_per_type::<optional_access_t>() >= hweight(_LANDLOCK_ACCESS_FS_OPTIONAL as usize));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
