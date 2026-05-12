use crate::rust_foo_impl;

#[unsafe(no_mangle)]
pub extern "C" fn rust_foo() {
    rust_foo_impl();
}
