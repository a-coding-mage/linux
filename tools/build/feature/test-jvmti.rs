// SPDX-License-Identifier: GPL-2.0
// Requires JVMTI type definitions corresponding to <jvmti.h>.

pub fn main() -> i32 {
    let _jvm: core::mem::MaybeUninit<JavaVM> = core::mem::MaybeUninit::uninit();
    let _cb: core::mem::MaybeUninit<jvmtiEventCallbacks> = core::mem::MaybeUninit::uninit();
    let _caps: core::mem::MaybeUninit<jvmtiCapabilities> = core::mem::MaybeUninit::uninit();
    let _format: core::mem::MaybeUninit<jvmtiJlocationFormat> = core::mem::MaybeUninit::uninit();
    let _jvmti: core::mem::MaybeUninit<jvmtiEnv> = core::mem::MaybeUninit::uninit();

    0
}
