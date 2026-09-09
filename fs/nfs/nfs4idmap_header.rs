/*
 * fs/nfs/nfs4idmap.h
 *
 *  UID and GID to name mapping for clients.
 *
 *  Copyright (c) 2002 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Marius Aamodt Eriksen <marius@umich.edu>
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted provided that the following conditions
 *  are met:
 *
 *  1. Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *  2. Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *  3. Neither the name of the University nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 *  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 *  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 *  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 *  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 *  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 *  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Forward declarations corresponding to the C header's included kernel types.
// `kuid_t` and `kgid_t` are supplied by the Linux uid/gid dependency.
extern "C" {
    pub type nfs_client;
    pub type nfs_server;
    pub type nfs_fattr;
    pub type nfs4_string;

    pub fn nfs_idmap_init() -> ::core::ffi::c_int;
    pub fn nfs_idmap_quit();
    pub fn nfs_idmap_new(client: *mut nfs_client) -> ::core::ffi::c_int;
    pub fn nfs_idmap_delete(client: *mut nfs_client);

    pub fn nfs_fattr_init_names(
        fattr: *mut nfs_fattr,
        owner_name: *mut nfs4_string,
        group_name: *mut nfs4_string,
    );
    pub fn nfs_fattr_free_names(fattr: *mut nfs_fattr);
    pub fn nfs_fattr_map_and_free_names(server: *mut nfs_server, fattr: *mut nfs_fattr);

    pub fn nfs_map_name_to_uid(
        server: *const nfs_server,
        name: *const ::core::ffi::c_char,
        namelen: usize,
        uid: *mut kuid_t,
    ) -> ::core::ffi::c_int;
    pub fn nfs_map_group_to_gid(
        server: *const nfs_server,
        name: *const ::core::ffi::c_char,
        namelen: usize,
        gid: *mut kgid_t,
    ) -> ::core::ffi::c_int;
    pub fn nfs_map_uid_to_name(
        server: *const nfs_server,
        uid: kuid_t,
        name: *mut ::core::ffi::c_char,
        namelen: usize,
    ) -> ::core::ffi::c_int;
    pub fn nfs_map_gid_to_group(
        server: *const nfs_server,
        gid: kgid_t,
        name: *mut ::core::ffi::c_char,
        namelen: usize,
    ) -> ::core::ffi::c_int;

    pub fn nfs_map_string_to_numeric(
        name: *const ::core::ffi::c_char,
        namelen: usize,
        res: *mut u32,
    ) -> ::core::ffi::c_int;

    pub static mut nfs_idmap_cache_timeout: ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
