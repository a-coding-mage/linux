/* Avoid duplicate definitions due to system headers.
 *
 * C source intent:
 *   #ifdef __CONCAT
 *   #undef __CONCAT
 *   #endif
 *
 * Rust has no preprocessor macro namespace equivalent for this file-local
 * undef operation.
 */

/* Dependency intent from C source:
 *   #include "../../../../include/linux/idr.h"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
