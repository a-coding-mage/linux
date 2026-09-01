// SPDX-License-Identifier: GPL-2.0
// C dependencies: <sys/capability.h>, <linux/capability.h>

type cap_flag_value_t = ::std::os::raw::c_int;
type cap_t = *mut ::std::os::raw::c_void;

const CAP_SYS_ADMIN: ::std::os::raw::c_int = 21;
const CAP_EFFECTIVE: ::std::os::raw::c_int = 0;

unsafe extern "C" {
    fn cap_get_proc() -> cap_t;
    fn cap_get_flag(
        cap_p: cap_t,
        cap: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        value_p: *mut cap_flag_value_t,
    ) -> ::std::os::raw::c_int;
    fn cap_free(obj_d: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}

fn main() -> ::std::process::ExitCode {
    let mut val: cap_flag_value_t = 0;
    let caps: cap_t = unsafe { cap_get_proc() };

    if caps.is_null() {
        return ::std::process::ExitCode::from(1);
    }

    if unsafe { cap_get_flag(caps, CAP_SYS_ADMIN, CAP_EFFECTIVE, &mut val) } != 0 {
        return ::std::process::ExitCode::from(1);
    }

    if unsafe { cap_free(caps) } != 0 {
        return ::std::process::ExitCode::from(1);
    }

    ::std::process::ExitCode::from(0)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
