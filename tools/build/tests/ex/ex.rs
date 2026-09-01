// SPDX-License-Identifier: GPL-2.0

unsafe extern "C" {
    fn a() -> ::std::os::raw::c_int;
    fn b() -> ::std::os::raw::c_int;
    fn c() -> ::std::os::raw::c_int;
    fn d() -> ::std::os::raw::c_int;
    fn e() -> ::std::os::raw::c_int;
    fn f() -> ::std::os::raw::c_int;
    fn inc() -> ::std::os::raw::c_int;
}

pub unsafe extern "C" fn main() -> ::std::os::raw::c_int {
    unsafe {
        a();
        b();
        c();
        d();
        e();
        f();
        inc();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
