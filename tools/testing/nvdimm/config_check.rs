// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <linux/bug.h>

unsafe extern "C" {
    fn BUILD_BUG_ON(condition: bool);
    fn IS_MODULE(symbol: bool) -> bool;
    fn IS_ENABLED(symbol: bool) -> bool;

    static CONFIG_LIBNVDIMM: bool;
    static CONFIG_BLK_DEV_PMEM: bool;
    static CONFIG_ND_BTT: bool;
    static CONFIG_ND_PFN: bool;
    static CONFIG_ACPI_NFIT: bool;
    static CONFIG_DEV_DAX: bool;
    static CONFIG_DEV_DAX_PMEM: bool;
}

#[no_mangle]
pub unsafe extern "C" fn check() {
    /*
     * These kconfig symbols must be set to "m" for nfit_test to
     * load and operate.
     */
    unsafe {
        BUILD_BUG_ON(!IS_MODULE(CONFIG_LIBNVDIMM));
        BUILD_BUG_ON(!IS_MODULE(CONFIG_BLK_DEV_PMEM));
        BUILD_BUG_ON(!IS_MODULE(CONFIG_ND_BTT));
        BUILD_BUG_ON(!IS_MODULE(CONFIG_ND_PFN));
        if IS_ENABLED(CONFIG_ACPI_NFIT) {
            BUILD_BUG_ON(!IS_MODULE(CONFIG_ACPI_NFIT));
        }
        BUILD_BUG_ON(!IS_MODULE(CONFIG_DEV_DAX));
        BUILD_BUG_ON(!IS_MODULE(CONFIG_DEV_DAX_PMEM));
    }
}
