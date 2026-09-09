// Dependency declarations are supplied by nitrox_common.h in the original C source.

unsafe extern "C" {
    fn nitrox_register_skciphers() -> core::ffi::c_int;
    fn nitrox_register_aeads() -> core::ffi::c_int;
    fn nitrox_unregister_skciphers();
    fn nitrox_unregister_aeads();
}

pub unsafe fn nitrox_crypto_register() -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    err = nitrox_register_skciphers();
    if err != 0 {
        return err;
    }

    err = nitrox_register_aeads();
    if err != 0 {
        nitrox_unregister_skciphers();
        return err;
    }

    0
}

pub unsafe fn nitrox_crypto_unregister() {
    nitrox_unregister_aeads();
    nitrox_unregister_skciphers();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
