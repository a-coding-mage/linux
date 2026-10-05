// SPDX-License-Identifier: GPL-2.0
// Typed native READ_ONCE/WRITE_ONCE leaves. Do not substitute Rust volatile
// access: configured architecture and instrumentation expansions belong to C.
// Pointer values remain typed; no integer round-trip or reference is introduced.
trait FairOnce: Sized {
    /// The pointer must be aligned, valid for this access, and synchronized as
    /// required by the corresponding original C READ_ONCE callsite.
    unsafe fn read_once(value: *const Self) -> Self;
    /// The pointer must be aligned, writable, and synchronized as required by
    /// the corresponding original C WRITE_ONCE callsite.
    unsafe fn write_once(value: *mut Self, new: Self);
}

macro_rules! fair_once_type {
    ($ty:ty, $read:ident, $write:ident) => {
        impl FairOnce for $ty {
            unsafe fn read_once(value: *const Self) -> Self {
                // SAFETY: The caller supplies the original access preconditions;
                // the leaf executes the configured READ_ONCE on this exact type.
                unsafe { b::$read(value) }
            }
            unsafe fn write_once(value: *mut Self, new: Self) {
                // SAFETY: The caller supplies the original access preconditions;
                // the leaf executes the configured WRITE_ONCE on this exact type.
                unsafe { b::$write(value, new) }
            }
        }
    };
}
fair_once_type!(c_int, rust_fair_read_once_int, rust_fair_write_once_int);
fair_once_type!(c_uint, rust_fair_read_once_uint, rust_fair_write_once_uint);
fair_once_type!(c_long, rust_fair_read_once_long, rust_fair_write_once_long);
fair_once_type!(c_ulong, rust_fair_read_once_ulong, rust_fair_write_once_ulong);
fair_once_type!(u64, rust_fair_read_once_u64, rust_fair_write_once_u64);
fair_once_type!(bool, rust_fair_read_once_bool, rust_fair_write_once_bool);
fair_once_type!(
    *mut b::task_struct,
    rust_fair_read_once_task,
    rust_fair_write_once_task
);
fair_once_type!(
    *mut b::sched_entity,
    rust_fair_read_once_entity,
    rust_fair_write_once_entity
);
