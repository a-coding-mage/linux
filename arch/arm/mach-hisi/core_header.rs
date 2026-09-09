/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/reboot.h> is preserved by the declarations
// below; required shared types are supplied by the surrounding translation.

unsafe extern "C" {
    pub fn hi3xxx_set_cpu_jump(cpu: ::core::ffi::c_int, jump_addr: *mut ::core::ffi::c_void);
    pub fn hi3xxx_get_cpu_jump(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn secondary_startup();

    pub fn hi3xxx_cpu_die(cpu: ::core::ffi::c_uint);
    pub fn hi3xxx_cpu_kill(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn hi3xxx_set_cpu(cpu: ::core::ffi::c_int, enable: bool);

    pub fn hix5hd2_set_cpu(cpu: ::core::ffi::c_int, enable: bool);
    pub fn hix5hd2_cpu_die(cpu: ::core::ffi::c_uint);

    pub fn hip01_set_cpu(cpu: ::core::ffi::c_int, enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
