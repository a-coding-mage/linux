/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/linux/stddef.h> are supplied by
// another translated header.

pub const NULL: *const core::ffi::c_void = core::ptr::null();

pub const r#false: i32 = 0;
pub const r#true: i32 = 1;

/// Return the byte offset of a field within a type.
#[macro_export]
macro_rules! offsetof {
	($ty:ty, $member:tt) => {
		core::mem::offset_of!($ty, $member)
	};
}

/// Report the size of a struct field in bytes.
#[macro_export]
macro_rules! sizeof_field {
	($ty:ty, $member:tt) => {
		core::mem::size_of_val(unsafe {
			&(*core::ptr::null::<$ty>()).$member
		})
	};
}

/// Report the offset of a struct field within the struct, including its size.
#[macro_export]
macro_rules! offsetofend {
	($ty:ty, $member:tt) => {
		$crate::offsetof!($ty, $member) + $crate::sizeof_field!($ty, $member)
	};
}

/// Wrap a set of declarations in a mirrored struct.
#[macro_export]
macro_rules! struct_group {
	($name:ident, $($members:tt)*) => {
		$crate::__struct_group!(, $name, , $($members)*);
	};
}

/// Create a struct_group with trailing attributes.
#[macro_export]
macro_rules! struct_group_attr {
	($name:ident, $attrs:tt, $($members:tt)*) => {
		$crate::__struct_group!(, $name, $attrs, $($members)*);
	};
}

/// Create a struct_group with a reusable tag.
#[macro_export]
macro_rules! struct_group_tagged {
	($tag:ident, $name:ident, $($members:tt)*) => {
		$crate::__struct_group!($tag, $name, , $($members)*);
	};
}

/// Declare a flexible array usable in a union.
#[macro_export]
macro_rules! DECLARE_FLEX_ARRAY {
	($ty:ty, $name:ident) => {
		$crate::__DECLARE_FLEX_ARRAY!($ty, $name);
	};
}

/// Overlap a flexible-array member with trailing members.
#[macro_export]
macro_rules! __TRAILING_OVERLAP {
	($ty:ty, $name:ident, $fam:ident, $attrs:tt, $($members:tt)*) => {
		$crate::__trailing_overlap!($ty, $name, $fam, $attrs, $($members)*);
	};
}

/// Overlap a flexible-array member with trailing members.
#[macro_export]
macro_rules! TRAILING_OVERLAP {
	($ty:ty, $name:ident, $fam:ident, $($members:tt)*) => {
		$crate::__TRAILING_OVERLAP!($ty, $name, $fam, , $($members)*);
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
