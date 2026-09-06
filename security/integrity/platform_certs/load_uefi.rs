// SPDX-License-Identifier: GPL-2.0

// Translated from integrity/platform_certs/load_uefi.c.
// Kernel headers provide the concrete definitions for these external types,
// constants, macros, and functions in the original C translation unit.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type bool_ = bool;
type efi_status_t = c_ulong;
type efi_char16_t = u16;

#[repr(C)]
pub struct efi_guid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dmi_system_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct efi_mokvar_table_entry {
    pub data: *mut c_void,
    pub data_size: c_ulong,
}

#[repr(C)]
pub struct efi {
    pub get_variable: unsafe extern "C" fn(
        name: *mut efi_char16_t,
        guid: *mut efi_guid_t,
        attr: *mut c_void,
        size: *mut c_ulong,
        data: *mut c_void,
    ) -> efi_status_t,
}

extern "C" {
    static efi: efi;

    static EFI_SHIM_LOCK_GUID: efi_guid_t;
    static EFI_IMAGE_SECURITY_DATABASE_GUID: efi_guid_t;

    static EFI_SUCCESS: efi_status_t;
    static EFI_NOT_FOUND: efi_status_t;
    static EFI_BUFFER_TOO_SMALL: efi_status_t;
    static EFI_RT_SUPPORTED_GET_VARIABLE: c_ulong;
    static GFP_KERNEL: c_ulong;

    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn efi_rt_services_supported(mask: c_ulong) -> bool_;
    fn efi_mokvar_entry_find(name: *const c_char) -> *mut efi_mokvar_table_entry;
    fn parse_efi_signature_list(
        source: *const c_char,
        data: *mut c_void,
        size: c_ulong,
        handler: unsafe extern "C" fn(),
    ) -> c_int;
    fn get_handler_for_mok();
    fn get_handler_for_db();
    fn get_handler_for_dbx();
    fn arch_get_secureboot() -> bool_;
    fn kmalloc(size: c_ulong, flags: c_ulong) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

macro_rules! pr_err {
    ($($arg:tt)*) => {
        /* external printk-style macro */
    };
}

macro_rules! pr_debug {
    ($($arg:tt)*) => {
        /* external printk-style macro */
    };
}

macro_rules! pr_info {
    ($($arg:tt)*) => {
        /* external printk-style macro */
    };
}

const fn wstr<const N: usize>(s: [u16; N]) -> [u16; N] {
    s
}

/*
 * On T2 Macs reading the db and dbx efi variables to load UEFI Secure Boot
 * certificates causes occurrence of a page fault in Apple's firmware and
 * a crash disabling EFI runtime services. The following quirk skips reading
 * these variables.
 *
 * C initializes each element with UEFI_QUIRK_SKIP_CERT(vendor, product).
 * The exact dmi_system_id layout and macro expansion are supplied by headers.
 */
static uefi_skip_cert: [dmi_system_id; 17] = unsafe { core::mem::zeroed() };
/*
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro15,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro15,2")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro15,3")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro15,4")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro16,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro16,2")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro16,3")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookPro16,4")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookAir8,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookAir8,2")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacBookAir9,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "Macmini8,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "MacPro7,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "iMac20,1")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "iMac20,2")
 * UEFI_QUIRK_SKIP_CERT("Apple Inc.", "iMacPro1,1")
 * { }
 */

/*
 * Look to see if a UEFI variable called MokIgnoreDB exists and return true if
 * it does.
 *
 * This UEFI variable is set by the shim if a user tells the shim to not use
 * the certs/hashes in the UEFI db variable for verification purposes.  If it
 * is set, we should ignore the db variable also and the true return indicates
 * this.
 */
unsafe fn uefi_check_ignore_db() -> bool_ {
    let mut status: efi_status_t;
    let mut db: c_uint = 0;
    let mut size: c_ulong = core::mem::size_of_val(&db) as c_ulong;
    let mut guid: efi_guid_t = core::ptr::read(&EFI_SHIM_LOCK_GUID);
    let mut name = wstr([b'M' as u16, b'o' as u16, b'k' as u16, b'I' as u16, b'g' as u16, b'n' as u16, b'o' as u16, b'r' as u16, b'e' as u16, b'D' as u16, b'B' as u16, 0]);

    status = (efi.get_variable)(
        name.as_mut_ptr(),
        &mut guid,
        core::ptr::null_mut(),
        &mut size,
        &mut db as *mut _ as *mut c_void,
    );
    status == EFI_SUCCESS
}

type c_uint = u32;

/*
 * Get a certificate list blob from the named EFI variable.
 */
unsafe fn get_cert_list(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    size: *mut c_ulong,
    status: *mut efi_status_t,
) -> *mut c_void {
    let mut lsize: c_ulong = 4;
    let mut tmpdb: [c_ulong; 4] = [0; 4];
    let mut db: *mut c_void;

    *status = (efi.get_variable)(
        name,
        guid,
        core::ptr::null_mut(),
        &mut lsize,
        tmpdb.as_mut_ptr() as *mut c_void,
    );
    if *status == EFI_NOT_FOUND {
        return core::ptr::null_mut();
    }

    if *status != EFI_BUFFER_TOO_SMALL {
        pr_err!("Couldn't get size: 0x%lx\n", *status);
        return core::ptr::null_mut();
    }

    db = kmalloc(lsize, GFP_KERNEL);
    if db.is_null() {
        return core::ptr::null_mut();
    }

    *status = (efi.get_variable)(name, guid, core::ptr::null_mut(), &mut lsize, db);
    if *status != EFI_SUCCESS {
        kfree(db);
        pr_err!("Error reading db var: 0x%lx\n", *status);
        return core::ptr::null_mut();
    }

    *size = lsize;
    db
}

/*
 * load_moklist_certs() - Load MokList certs
 *
 * Load the certs contained in the UEFI MokListRT database into the
 * platform trusted keyring.
 *
 * This routine checks the EFI MOK config table first. If and only if
 * that fails, this routine uses the MokListRT ordinary UEFI variable.
 *
 * Return:	Status
 */
unsafe fn load_moklist_certs() -> c_int {
    let mut mokvar_entry: *mut efi_mokvar_table_entry;
    let mut mok_var: efi_guid_t = core::ptr::read(&EFI_SHIM_LOCK_GUID);
    let mut mok: *mut c_void;
    let mut moksize: c_ulong = 0;
    let mut status: efi_status_t = 0;
    let mut rc: c_int;
    let mut moklistrt = wstr([b'M' as u16, b'o' as u16, b'k' as u16, b'L' as u16, b'i' as u16, b's' as u16, b't' as u16, b'R' as u16, b'T' as u16, 0]);

    /* First try to load certs from the EFI MOKvar config table.
     * It's not an error if the MOKvar config table doesn't exist
     * or the MokListRT entry is not found in it.
     */
    mokvar_entry = efi_mokvar_entry_find(c"MokListRT".as_ptr());
    if !mokvar_entry.is_null() {
        rc = parse_efi_signature_list(
            c"UEFI:MokListRT (MOKvar table)".as_ptr(),
            (*mokvar_entry).data,
            (*mokvar_entry).data_size,
            get_handler_for_mok,
        );
        /* All done if that worked. */
        if rc == 0 {
            return rc;
        }

        pr_err!(
            "Couldn't parse MokListRT signatures from EFI MOKvar config table: %d\n",
            rc
        );
    }

    /* Get MokListRT. It might not exist, so it isn't an error
     * if we can't get it.
     */
    mok = get_cert_list(moklistrt.as_mut_ptr(), &mut mok_var, &mut moksize, &mut status);
    if !mok.is_null() {
        rc = parse_efi_signature_list(
            c"UEFI:MokListRT".as_ptr(),
            mok,
            moksize,
            get_handler_for_mok,
        );
        kfree(mok);
        if rc != 0 {
            pr_err!("Couldn't parse MokListRT signatures: %d\n", rc);
        }
        return rc;
    }
    if status == EFI_NOT_FOUND {
        pr_debug!("MokListRT variable wasn't found\n");
    } else {
        pr_info!("Couldn't get UEFI MokListRT\n");
    }
    0
}

/*
 * load_uefi_certs() - Load certs from UEFI sources
 *
 * Load the certs contained in the UEFI databases into the platform trusted
 * keyring and the UEFI blacklisted X.509 cert SHA256 hashes into the blacklist
 * keyring.
 */
unsafe fn load_uefi_certs() -> c_int {
    let mut secure_var: efi_guid_t = core::ptr::read(&EFI_IMAGE_SECURITY_DATABASE_GUID);
    let mut mok_var: efi_guid_t = core::ptr::read(&EFI_SHIM_LOCK_GUID);
    let mut db: *mut c_void = core::ptr::null_mut();
    let mut dbx: *mut c_void = core::ptr::null_mut();
    let mut mokx: *mut c_void = core::ptr::null_mut();
    let mut dbsize: c_ulong = 0;
    let mut dbxsize: c_ulong = 0;
    let mut mokxsize: c_ulong = 0;
    let mut status: efi_status_t = 0;
    let mut rc: c_int = 0;
    let mut dmi_id: *const dmi_system_id;
    let mut db_name = wstr([b'd' as u16, b'b' as u16, 0]);
    let mut dbx_name = wstr([b'd' as u16, b'b' as u16, b'x' as u16, 0]);
    let mut mokx_name = wstr([b'M' as u16, b'o' as u16, b'k' as u16, b'L' as u16, b'i' as u16, b's' as u16, b't' as u16, b'X' as u16, b'R' as u16, b'T' as u16, 0]);

    dmi_id = dmi_first_match(uefi_skip_cert.as_ptr());
    if !dmi_id.is_null() {
        pr_err!("Reading UEFI Secure Boot Certs is not supported on T2 Macs.\n");
        return false as c_int;
    }

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) {
        return false as c_int;
    }

    /* Get db and dbx.  They might not exist, so it isn't an error
     * if we can't get them.
     */
    if !uefi_check_ignore_db() {
        db = get_cert_list(db_name.as_mut_ptr(), &mut secure_var, &mut dbsize, &mut status);
        if db.is_null() {
            if status == EFI_NOT_FOUND {
                pr_debug!("MODSIGN: db variable wasn't found\n");
            } else {
                pr_err!("MODSIGN: Couldn't get UEFI db list\n");
            }
        } else {
            rc = parse_efi_signature_list(c"UEFI:db".as_ptr(), db, dbsize, get_handler_for_db);
            if rc != 0 {
                pr_err!("Couldn't parse db signatures: %d\n", rc);
            }
            kfree(db);
        }
    }

    dbx = get_cert_list(dbx_name.as_mut_ptr(), &mut secure_var, &mut dbxsize, &mut status);
    if dbx.is_null() {
        if status == EFI_NOT_FOUND {
            pr_debug!("dbx variable wasn't found\n");
        } else {
            pr_info!("Couldn't get UEFI dbx list\n");
        }
    } else {
        rc = parse_efi_signature_list(c"UEFI:dbx".as_ptr(), dbx, dbxsize, get_handler_for_dbx);
        if rc != 0 {
            pr_err!("Couldn't parse dbx signatures: %d\n", rc);
        }
        kfree(dbx);
    }

    /* the MOK/MOKx can not be trusted when secure boot is disabled */
    if !arch_get_secureboot() {
        return 0;
    }

    mokx = get_cert_list(mokx_name.as_mut_ptr(), &mut mok_var, &mut mokxsize, &mut status);
    if mokx.is_null() {
        if status == EFI_NOT_FOUND {
            pr_debug!("mokx variable wasn't found\n");
        } else {
            pr_info!("Couldn't get mokx list\n");
        }
    } else {
        rc = parse_efi_signature_list(
            c"UEFI:MokListXRT".as_ptr(),
            mokx,
            mokxsize,
            get_handler_for_dbx,
        );
        if rc != 0 {
            pr_err!("Couldn't parse mokx signatures %d\n", rc);
        }
        kfree(mokx);
    }

    /* Load the MokListRT certs */
    rc = load_moklist_certs();

    rc
}

// late_initcall(load_uefi_certs);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
