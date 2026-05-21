/*
 * this is somewhat of a hack: when cross-compiling the rust library
 * kindle_personal_dashboard_core for the Kindle PW2, the toolchain we use still
 * uses glibc version 2.12, which does not include `getauxval` which is
 * required by ring. however, the actual device uses a newer glibc version 2.20
 * which does include it. to solve this, we implement a stub implementation of
 * getauxval to prevent linker errors, but because of __attribute__((weak)), we
 * actually then use the real glibc implementation at runtime.
 */

#include <features.h>

#if defined(__GLIBC__) && __GLIBC__ <= 2 && __GLIBC_MINOR__ < 16

// this will never get called, since the Kindle PW2 actually has newer glibc
// version with getauxval support
extern "C" __attribute__((weak)) unsigned long getauxval(unsigned long) {
	return 0;
}

#endif
