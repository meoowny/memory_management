use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_generate_replacement_record(
    port_: i64,
    mem_capacity: usize,
    total_instrument: usize,
    choice: i32,
    page_size: usize,
) {
    wire_generate_replacement_record_impl(port_, mem_capacity, total_instrument, choice, page_size)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
