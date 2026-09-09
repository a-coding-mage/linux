// SPDX-License-Identifier: GPL-2.0-only

// Dependencies corresponding to the C includes:
// generated/compile.h, generated/utsrelease.h, linux/proc_ns.h,
// linux/refcount.h, linux/uts.h, linux/utsname.h

#[no_mangle]
pub static mut init_uts_ns: crate::uts_namespace = crate::uts_namespace {
    ns: crate::NS_COMMON_INIT!(init_uts_ns),
    name: crate::utsname {
        sysname: crate::UTS_SYSNAME,
        nodename: crate::UTS_NODENAME,
        release: crate::UTS_RELEASE,
        version: crate::UTS_VERSION,
        machine: crate::UTS_MACHINE,
        domainname: crate::UTS_DOMAINNAME,
    },
    user_ns: &raw const crate::init_user_ns,
};

/* FIXED STRINGS! Don't touch! */
#[no_mangle]
pub static linux_banner: &[u8] = concat!(
    "Linux version ",
    crate::UTS_RELEASE,
    " (",
    crate::LINUX_COMPILE_BY,
    "@",
    crate::LINUX_COMPILE_HOST,
    ") (",
    crate::LINUX_COMPILER,
    ") ",
    crate::UTS_VERSION,
    "\n",
).as_bytes();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
