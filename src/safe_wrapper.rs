use std::ffi::CString;

mod raw {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { raw::add(a, b) }
}

pub fn subtract(a: i32, b: i32) -> i32 {
    unsafe { raw::subtract(a, b) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    unsafe { raw::multiply(a, b) }
}

pub fn system_limit() -> i32 {
    raw::SYSTEM_LIMIT as i32
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub label: String,
}

impl Point {
    pub fn distance(&self) -> f32 {
        let c_label = CString::new(self.label.clone()).unwrap();

        let raw_p = raw::DataPoint {
            x: self.x,
            y: self.y, 
            label: c_label.as_ptr(),
        };

        unsafe { raw::calculate_distance(raw_p) }
    }
}

pub struct SmartPoint {
    inner: *mut raw::DataPoint,
}

impl SmartPoint {
    pub fn new(x: f32, y: f32, label: &str) -> Self {
        let c_label = CString::new(label).unwrap();

        let ptr = unsafe {
            raw::create_heap_point(x, y, c_label.as_ptr())
        };

        SmartPoint { inner: ptr }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { (*self.inner).x }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { (*self.inner).y }
    }
}

impl Drop for SmartPoint {
    fn drop(&mut self) {
        println!("  [Rust] Automatically freeing C memory by dropping...");
        unsafe {
            raw::free_heap_point(self.inner);
        }
    }
}