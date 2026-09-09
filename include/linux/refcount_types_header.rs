/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: `atomic_t` is supplied by the corresponding Linux types module.

/**
 * `refcount_t` - variant of `atomic_t` specialized for reference counts
 *
 * The counter saturates at `REFCOUNT_SATURATED` and will not move once
 * there. This avoids wrapping the counter and causing 'spurious'
 * use-after-free bugs.
 */
#[repr(C)]
pub struct refcount_t {
    pub refs: atomic_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
