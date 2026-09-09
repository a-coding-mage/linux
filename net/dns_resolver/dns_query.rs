// SPDX-License-Identifier: LGPL-2.1-or-later
/* Upcall routine, designed to work as a key type and working through
 * /sbin/request-key to contact userspace when handling DNS queries.
 *
 * See Documentation/networking/dns_resolver.rst
 *
 *   Copyright (c) 2007 Igor Mammedov
 *   Author(s): Igor Mammedov (niallain@gmail.com)
 *              Steve French (sfrench@us.ibm.com)
 *              Wang Lei (wang840925@gmail.com)
 *	 David Howells (dhowells@redhat.com)
 *
 *   The upcall wrapper used to make an arbitrary DNS query.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

/// dns_query - Query the DNS
/// @net: The network namespace to operate in.
/// @type_: Query type (or NULL for straight host->IP lookup)
/// @name: Name to look up
/// @namelen: Length of name
/// @options: Request options (or NULL if no options)
/// @_result: Where to place the returned data (or NULL)
/// @_expiry: Where to store the result expiry time (or NULL)
/// @invalidate: Always invalidate the key after use
///
/// The data will be returned in the pointer at *result, if provided, and the
/// caller is responsible for freeing it.
///
/// The description should be of the form "[<query_type>:]<domain_name>", and
/// the options need to be appropriate for the query type requested.  If no
/// query_type is given, then the query is a straight hostname to IP address
/// lookup.
///
/// The DNS resolution lookup is performed by upcalling to userspace by way of
/// requesting a key of type dns_resolver.
///
/// Returns the size of the result on success, -ve error code otherwise.
pub unsafe fn dns_query(
    net: *mut net,
    type_: *const std::ffi::c_char,
    name: *const std::ffi::c_char,
    namelen: usize,
    options: *const std::ffi::c_char,
    _result: *mut *mut std::ffi::c_char,
    _expiry: *mut time64_t,
    invalidate: bool,
) -> i32 {
    let mut upayload: *mut user_key_payload;
    let mut rkey: *mut key;
    let mut ret: i32;
    let mut len: i32;
    let desc: *mut std::ffi::c_char;

    kenter!("%s,%*.*s,%zu,%s", type_, namelen as i32, namelen as i32, name, namelen, options);

    if name.is_null() || namelen < 1 || namelen > 255 {
        return -EINVAL;
    }
    if !type_.is_null() && *type_ == 0 {
        return -EINVAL;
    }

    /* construct the query key description as "[<type>:]<name>" */
    if !type_.is_null() {
        desc = kasprintf!(GFP_KERNEL, "%s:%.*s", type_, namelen as i32, name);
    } else {
        desc = kmemdup_nul(name, namelen, GFP_KERNEL);
    }
    if desc.is_null() {
        return -ENOMEM;
    }

    let options = if options.is_null() { c"".as_ptr() } else { options };
    kdebug!("call request_key(,%s,%s)", desc, options);

    /* make the upcall, using special credentials to prevent the use of
     * add_key() to preinstall malicious redirections
     */
    scoped_with_creds!(dns_resolver_cache);
    rkey = request_key_net(&key_type_dns_resolver, desc, net, options);
    kfree(desc as *mut _);
    if IS_ERR(rkey) {
        ret = PTR_ERR(rkey);
        goto_out!(ret);
    }

    down_read(&(*rkey).sem);
    set_bit(KEY_FLAG_ROOT_CAN_INVAL, &mut (*rkey).flags);
    (*rkey).perm |= KEY_USR_VIEW;

    ret = key_validate(rkey);
    if ret < 0 {
        goto_put!(ret);
    }

    /* If the DNS server gave an error, return that to the caller */
    ret = PTR_ERR((*rkey).payload.data[dns_key_error]);
    if ret != 0 {
        goto_put!(ret);
    }

    upayload = user_key_payload_locked(rkey);
    len = (*upayload).datalen;

    if !_result.is_null() {
        ret = -ENOMEM;
        *_result = kmemdup_nul((*upayload).data, len as usize, GFP_KERNEL);
        if (*_result).is_null() {
            goto_put!(ret);
        }
    }

    if !_expiry.is_null() {
        *_expiry = (*rkey).expiry;
    }

    ret = len;
put:
    up_read(&(*rkey).sem);
    if invalidate {
        key_invalidate(rkey);
    }
    key_put(rkey);
out:
    kleave!(" = %d", ret);
    ret
}

// EXPORT_SYMBOL(dns_query);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
