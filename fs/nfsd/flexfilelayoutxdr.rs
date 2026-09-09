// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2016 Tom Haynes <loghyr@primarydata.com>
 */
// Dependencies supplied by the surrounding NFS/RPC implementation:
// linux/sunrpc/svc.h, linux/nfs4.h, nfsd.h, and flexfilelayoutxdr.h.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PNFS;

#[repr(C)]
struct ff_idmap {
    buf: [::std::os::raw::c_char; 11],
    len: ::std::os::raw::c_int,
}

pub unsafe fn nfsd4_ff_encode_layoutget(
    xdr: *mut xdr_stream,
    lgp: *const nfsd4_layoutget,
) -> __be32 {
    let fl: *const pnfs_ff_layout = (*lgp).lg_content as *const pnfs_ff_layout;
    let mut len: ::std::os::raw::c_int;
    let mut mirror_len: ::std::os::raw::c_int;
    let mut ds_len: ::std::os::raw::c_int;
    let fh_len: ::std::os::raw::c_int;
    let mut p: *mut __be32;

    /*
     * Unlike nfsd4_encode_user, we know these will
     * always be stringified.
     */
    let mut uid: ff_idmap = ::std::mem::zeroed();
    let mut gid: ff_idmap = ::std::mem::zeroed();

    fh_len = 4 + xdr_align_size((*fl).fh.size) as ::std::os::raw::c_int;

    uid.len = sprintf(uid.buf.as_mut_ptr(), b"%u\0".as_ptr() as *const _,
                      from_kuid(&init_user_ns, (*fl).uid));
    gid.len = sprintf(gid.buf.as_mut_ptr(), b"%u\0".as_ptr() as *const _,
                      from_kgid(&init_user_ns, (*fl).gid));

    /* data server entry: deviceid + efficiency + stateid + fh list +
     * user + group + flags + stats_collect_hint
     */
    ds_len = 16 + 4 + 4 + ::std::mem::size_of::<stateid_opaque_t>() as i32 + 4 + fh_len
        + 4 + xdr_align_size(uid.len) as i32
        + 4 + xdr_align_size(gid.len) as i32
        + 4 + 4;

    /* mirror: ds_count + ds */
    mirror_len = 4 + ds_len;

    /* stripe_unit + mirror_count + mirror */
    len = 12 + mirror_len;

    p = xdr_reserve_space(xdr, (::std::mem::size_of::<__be32>() as i32 + len) as usize);
    if p.is_null() {
        return nfserr_toosmall;
    }

    *p = cpu_to_be32(len as u32); p = p.add(1);
    p = xdr_encode_hyper(p, 0); /* stripe unit of 1 */

    *p = cpu_to_be32(1); p = p.add(1); /* single mirror */
    *p = cpu_to_be32(1); p = p.add(1); /* single data server */

    p = svcxdr_encode_deviceid4(p, &(*fl).deviceid);
    *p = cpu_to_be32(1); p = p.add(1); /* efficiency */

    *p = cpu_to_be32((*fl).stateid.si_generation); p = p.add(1);
    p = xdr_encode_opaque_fixed(p, &(*fl).stateid.si_opaque,
                                ::std::mem::size_of::<stateid_opaque_t>());

    *p = cpu_to_be32(1); p = p.add(1); /* single file handle */
    p = xdr_encode_opaque(p, (*fl).fh.data, (*fl).fh.size);
    p = xdr_encode_opaque(p, uid.buf.as_ptr(), uid.len as usize);
    p = xdr_encode_opaque(p, gid.buf.as_ptr(), gid.len as usize);

    *p = cpu_to_be32((*fl).flags); p = p.add(1);
    *p = cpu_to_be32(0); /* No stats collect hint */

    0
}

pub unsafe fn nfsd4_ff_encode_getdeviceinfo(
    xdr: *mut xdr_stream,
    gdp: *const nfsd4_getdeviceinfo,
) -> __be32 {
    let da: *mut pnfs_ff_device_addr = (*gdp).gd_device;
    let mut len: ::std::os::raw::c_int;
    let ver_len: ::std::os::raw::c_int;
    let addr_len: ::std::os::raw::c_int;
    let mut p: *mut __be32;

    /* See paragraph 5 of RFC 8881 S18.40.3. */
    if (*gdp).gd_maxcount == 0 {
        if xdr_stream_encode_u32(xdr, 0) != XDR_UNIT {
            return nfserr_resource;
        }
        return nfs_ok;
    }

    /* len + padding for two strings */
    addr_len = 8 + xdr_align_size((*da).netaddr.netid_len) as i32
        + xdr_align_size((*da).netaddr.addr_len) as i32;
    ver_len = 20;
    len = 4 + ver_len + 4 + addr_len;

    p = xdr_reserve_space(xdr, (len + ::std::mem::size_of::<__be32>() as i32) as usize);
    if p.is_null() {
        return nfserr_resource;
    }

    /* Fill in the overall length and number of volumes at the beginning
     * of the layout.
     */
    *p = cpu_to_be32(len as u32); p = p.add(1);
    *p = cpu_to_be32(1); p = p.add(1); /* 1 netaddr */
    p = xdr_encode_opaque(p, (*da).netaddr.netid, (*da).netaddr.netid_len);
    p = xdr_encode_opaque(p, (*da).netaddr.addr, (*da).netaddr.addr_len);

    *p = cpu_to_be32(1); p = p.add(1); /* 1 versions */
    *p = cpu_to_be32((*da).version); p = p.add(1);
    *p = cpu_to_be32((*da).minor_version); p = p.add(1);
    *p = cpu_to_be32((*da).rsize); p = p.add(1);
    *p = cpu_to_be32((*da).wsize); p = p.add(1);
    *p = cpu_to_be32((*da).tightly_coupled);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
