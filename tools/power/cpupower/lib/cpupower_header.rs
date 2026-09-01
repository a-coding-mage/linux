// SPDX-License-Identifier: GPL-2.0

pub const CPULIST_BUFFER: usize = 5;

#[repr(C)]
pub struct cpupower_topology {
    /* Amount of CPU cores, packages and threads per core in the system */
    pub cores: ::std::os::raw::c_uint,
    pub pkgs: ::std::os::raw::c_uint,
    pub threads: ::std::os::raw::c_uint, /* per core */

    /* Array gets mallocated with cores entries, holding per core info */
    pub core_info: *mut cpuid_core_info,
}

#[repr(C)]
pub struct cpuid_core_info {
    pub pkg: ::std::os::raw::c_int,
    pub core: ::std::os::raw::c_int,
    pub cpu: ::std::os::raw::c_int,
    pub core_cpu_list: [::std::os::raw::c_char; CPULIST_BUFFER],

    /* flags */
    /* C bitfield: unsigned int is_online:1; */
    pub is_online: ::std::os::raw::c_uint,
}

unsafe extern "C" {
    pub fn get_cpu_topology(cpu_top: *mut cpupower_topology) -> ::std::os::raw::c_int;
    pub fn cpu_topology_release(cpu_top: cpupower_topology);
    pub fn cpupower_is_cpu_online(cpu: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
