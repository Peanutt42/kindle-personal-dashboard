use crate::foo_impl;

#[unsafe(no_mangle)]
pub extern "C" fn kindle_personal_dashboard_core_foo() {
    foo_impl();
}
