/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard _TOOLS_LINUX_CONTAINER_OF_H omitted in Rust. */

/* Original C conditional: define container_of only if it is not already defined. */

/**
 * container_of - cast a member of a structure out to the containing structure
 * @ptr:	the pointer to the member.
 * @type:	the type of the container struct this is embedded in.
 * @member:	the name of the member within the struct.
 *
 */
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr = $ptr;
        (__mptr as *const u8)
            .wrapping_sub(::core::mem::offset_of!($type, $member))
            as *mut $type
    }};
}
