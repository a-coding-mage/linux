// SPDX-License-Identifier: GPL-2.0+
// EFI signature/key/certificate list parser
//
// Copyright (C) 2012, 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)

use std::ffi::c_void;
use std::ptr;

// External types from linux/efi.h

#[repr(C)]
pub struct efi_guid_t;

#[repr(C)]
pub struct efi_signature_list_t {
    pub signature_type: efi_guid_t,
    pub signature_list_size: u32,
    pub signature_header_size: u32,
    pub signature_size: u32,
}

#[repr(C)]
pub struct efi_signature_data_t {
    pub signature_data: [u8; 0],
}

pub type efi_element_handler_t = Option<unsafe extern "C" fn(*const i8, *const c_void, usize)>;

// Equivalent to -EBADMSG error code
const EBADMSG: i32 = -22;

/// Parse an EFI signature list for certificates
/// source: The source of the key
/// data: The data blob to parse
/// size: The size of the data blob
/// get_handler_for_guid: Get the handler func for the sig type (or NULL)
///
/// Parse an EFI signature list looking for elements of interest. A list is
/// made up of a series of sublists, where all the elements in a sublist are of
/// the same type, but sublists can be of different types.
///
/// For each sublist encountered, the get_handler_for_guid function is called
/// with the type specifier GUID and returns either a pointer to a function to
/// handle elements of that type or NULL if the type is not of interest.
///
/// If the sublist is of interest, each element is passed to the handler
/// function in turn.
///
/// Error EBADMSG is returned if the list doesn't parse correctly and 0 is
/// returned if the list was parsed correctly. No error can be returned from
/// the get_handler_for_guid function or the element handler function it
/// returns.
#[allow(dead_code)]
pub unsafe fn parse_efi_signature_list(
    source: *const i8,
    data: *const c_void,
    size: usize,
    get_handler_for_guid: unsafe fn(*const efi_guid_t) -> efi_element_handler_t,
) -> i32 {
    let mut handler: efi_element_handler_t;
    let mut offs: u32 = 0;
    let mut data = data as *const u8;
    let mut size = size;

    // pr_devel("-->%s(,%zu)\n", __func__, size);

    while size > 0 {
        let mut elem: *const efi_signature_data_t;
        let mut list: efi_signature_list_t = std::mem::zeroed();
        let lsize: usize;
        let esize: usize;
        let hsize: usize;
        let elsize: usize;

        if size < std::mem::size_of::<efi_signature_list_t>() {
            return EBADMSG;
        }

        ptr::copy_nonoverlapping(
            data as *const efi_signature_list_t,
            &mut list as *mut efi_signature_list_t,
            1,
        );
        // pr_devel("LIST[%04x] guid=%pUl ls=%x hs=%x ss=%x\n",
        //          offs,
        //          &list.signature_type, list.signature_list_size,
        //          list.signature_header_size, list.signature_size);

        lsize = list.signature_list_size as usize;
        hsize = list.signature_header_size as usize;
        esize = list.signature_size as usize;
        elsize = lsize - std::mem::size_of::<efi_signature_list_t>() - hsize;

        if lsize > size {
            // pr_devel("<--%s() = -EBADMSG [overrun @%x]\n",
            //          __func__, offs);
            return EBADMSG;
        }

        if lsize < std::mem::size_of::<efi_signature_list_t>()
            || lsize - std::mem::size_of::<efi_signature_list_t>() < hsize
            || esize < std::mem::size_of::<efi_signature_data_t>()
            || elsize < esize
            || elsize % esize != 0
        {
            // pr_devel("- bad size combo @%x\n", offs);
            return EBADMSG;
        }

        handler = get_handler_for_guid(&list.signature_type);
        if handler.is_none() {
            data = data.add(lsize);
            size -= lsize;
            offs = offs.wrapping_add(lsize as u32);
            continue;
        }

        data = data.add(std::mem::size_of::<efi_signature_list_t>() + hsize);
        size -= std::mem::size_of::<efi_signature_list_t>() + hsize;
        offs = offs.wrapping_add((std::mem::size_of::<efi_signature_list_t>() + hsize) as u32);

        let mut elsize = elsize;
        while elsize > 0 {
            elem = data as *const efi_signature_data_t;

            // pr_devel("ELEM[%04x]\n", offs);
            if let Some(handler_fn) = handler {
                handler_fn(
                    source,
                    &(*elem).signature_data as *const _ as *const c_void,
                    esize - std::mem::size_of::<efi_signature_data_t>(),
                );
            }

            data = data.add(esize);
            size -= esize;
            offs = offs.wrapping_add(esize as u32);
            elsize -= esize;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
