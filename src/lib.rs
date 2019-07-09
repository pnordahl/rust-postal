// Knockway Inc. and its affiliates. All Rights Reserved

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

extern crate parking_lot;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use parking_lot::Mutex;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct ExpandAddressOptions {
    pub opts: libpostal_normalize_options_t,
    c_languages: Option<Vec<CString>>,
}
impl ExpandAddressOptions {
    pub fn new() -> Self {
        unsafe {
            ExpandAddressOptions {
                opts: libpostal_get_default_options(),
                c_languages: None,
            }
        }
    }
    pub fn set_languages(&mut self, langs: &[&str]) {
        let c_langs: Vec<CString> = langs.iter().map(|l| CString::new(*l).unwrap()).collect();
        self.opts.languages = c_langs.as_ptr() as *mut *mut c_char;
        self.opts.num_languages = c_langs.len();
        self.c_languages = Some(c_langs);
    }
}

pub struct Expansions<'a> {
    index: isize,
    array: *mut *mut c_char,
    array_length: isize,
    // this is used to deal with the lifetime of the Item in our Iterator implementation;
    // necessary to avoid the alloc
    phantom: PhantomData<&'a str>,
}
impl<'a> Expansions<'a> {
    pub fn new(array: *mut *mut c_char, length: usize) -> Self {
        Expansions {
            index: 0,
            array: array,
            array_length: length as isize,
            phantom: PhantomData,
        }
    }
}
impl<'a> Iterator for Expansions<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array_length {
            unsafe {
                let p = *self.array.offset(self.index);
                let cs = CStr::from_ptr(p as *const c_char);
                match cs.to_str() {
                    Ok(s) => {
                        self.index += 1;
                        Some(s)
                    }
                    Err(_) => {
                        // TODO: this means libpostal returned non-utf8, which is nasty but not
                        // something we can handle here
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}
impl<'a> Drop for Expansions<'a> {
    fn drop(&mut self) {
        unsafe {
            libpostal_expansion_array_destroy(self.array, self.array_length as usize);
        }
    }
}

pub struct InitOptions {
    pub expand_address: bool,
}

pub struct Context {
    setup_done: bool,
    expand_address_enabled: bool,
    mutex: Mutex<bool>,
}
impl Context {
    // TODO: best effort check that the user hasn't made more than one of these, probably just an atomic counter
    // of calls to ::new() would be fine, along with liberal warnings in the docs about not being threadsafe
    pub fn new() -> Context {
        Context {
            setup_done: false,
            expand_address_enabled: false,
            // this mutex is a sentinel for a global lock on all operations of this Context; the bool it
            // protects is unused.
            mutex: Mutex::new(false),
        }
    }
    pub fn init(&mut self, opts: InitOptions) -> Result<(), PostalError> {
        let _ = self.mutex.lock();
        unsafe {
            if !libpostal_setup() {
                return Err(PostalError::LibpostalSetup);
            }
        }
        self.setup_done = true;
        if opts.expand_address {
            unsafe {
                if !libpostal_setup_language_classifier() {
                    return Err(PostalError::LibpostalEnableExpansion);
                }
            }
            self.expand_address_enabled = true;
        }
        Ok(())
    }
    pub fn expand_address(
        &self,
        a: &str,
        opts: &mut ExpandAddressOptions,
    ) -> Result<Expansions, PostalError> {
        if self.setup_done && self.expand_address_enabled {
            let _ = self.mutex.lock();
            unsafe {
                match CString::new(a) {
                    Ok(c_string) => {
                        let addr = c_string.as_ptr() as *mut c_char;

                        let mut num_expansions: usize = 0;
                        let raw = libpostal_expand_address(addr, opts.opts, &mut num_expansions);
                        Ok(Expansions::new(raw, num_expansions))
                    }
                    Err(e) => Err(PostalError::BadCString(e)),
                }
            }
        } else {
            Err(PostalError::LibpostalNotReady)
        }
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        if self.setup_done {
            unsafe {
                libpostal_teardown();
            }
        }
        if self.expand_address_enabled {
            unsafe {
                libpostal_teardown_language_classifier();
            }
        }
    }
}

#[derive(Debug)]
pub enum PostalError {
    LibpostalSetup,
    LibpostalEnableExpansion,
    BadCString(std::ffi::NulError),
    LibpostalNotReady,
}
impl fmt::Display for PostalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostalError::LibpostalSetup => write!(f, "libpostal_setup failed"),
            PostalError::LibpostalEnableExpansion => {
                write!(f, "libpostal_setup_language_classifier failed")
            }
            PostalError::BadCString(ref err) => {
                write!(f, "failed to convert &str into c string, error: '{}'", err)
            }
            PostalError::LibpostalNotReady => write!(
                f,
                "libpostal is not ready, call init() with desired options"
            ),
        }
    }
}

impl error::Error for PostalError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PostalError::LibpostalSetup => None,
            PostalError::LibpostalEnableExpansion => None,
            PostalError::BadCString(ref err) => Some(err),
            PostalError::LibpostalNotReady => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_address() {
        let mut ctx = Context::new();
        ctx.init(InitOptions {
            expand_address: true,
        })
        .unwrap();
        let mut opts = ExpandAddressOptions::new();
        opts.set_languages(vec!["fr"].as_slice());
        let expansions = ctx
            .expand_address("Thirty W 26th St Fl #7", &mut opts)
            .unwrap();
        let expect = vec![
            "thirty w 26th saint fleuve numero 7",
            "thirty w 26 th saint fleuve numero 7",
        ];

        assert!(expansions.eq(expect));
    }

    #[test]
    fn test_expand_address_no_languages() {
        let mut ctx = Context::new();
        ctx.init(InitOptions {
            expand_address: true,
        })
        .unwrap();
        let mut opts = ExpandAddressOptions::new();
        let expansions = ctx
            .expand_address("1234 Cherry Ln, Podunk, TX", &mut opts)
            .unwrap();
        let expect = vec![
            "1234 cherry lane podunk texas",
            "1234 cherry lane podunk tx",
            "1234 cherry line podunk texas",
            "1234 cherry line podunk tx",
        ];

        assert!(expansions.eq(expect));
    }
}
