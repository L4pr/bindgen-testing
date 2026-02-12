use crate::Calculator;
use std::ffi::{c_char, CStr, CString};

// --- 1. Constructor (Rust -> C) ---
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calc_create(name_ptr: *const c_char, start_val: i32) -> *mut Calculator {
    if name_ptr.is_null() { return std::ptr::null_mut(); }

    // Convert C String to Rust String
    let c_str = CStr::from_ptr(name_ptr);
    let owner = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Create the Rust struct
    let calc = Calculator::new(owner, start_val);

    // BOX IT: Move it to the heap and give C the pointer
    Box::into_raw(Box::new(calc))
}

// --- 2. Destructor (C -> Rust) ---
// C cannot free Rust memory. It must give the pointer back to us to kill it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calc_destroy(ptr: *mut Calculator) {
    if ptr.is_null() { return; }

    // Take the pointer back, turn it into a Box, and let it drop (free)
    let _ = Box::from_raw(ptr);
}

// --- 3. Methods (C -> Rust) ---
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calc_add(ptr: *mut Calculator, amount: i32) {
    // Convert raw pointer to a Rust reference
    let calc = match ptr.as_mut() {
        Some(c) => c,
        None => return,
    };

    calc.add(amount);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calc_get_summary(ptr: *const Calculator) -> *mut c_char {
    let calc = match ptr.as_ref() {
        Some(c) => c,
        None => return std::ptr::null_mut(),
    };

    // Get the Rust string
    let summary = calc.get_summary();

    // Convert Rust String -> C String
    // We must invoke CString::new to ensure it has a null terminator
    match CString::new(summary) {
        Ok(c_str) => c_str.into_raw(), // Give ownership to C
        Err(_) => std::ptr::null_mut(),
    }
}

// --- 4. String Destructor ---
// Because calc_get_summary returned a NEW string, C needs a way to free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calc_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    // Retake ownership and drop it
    let _ = CString::from_raw(s);
}