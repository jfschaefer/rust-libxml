//! # A wrapper for libxml2
//! This library provides an interface to a subset of the libxml API.
//! The idea is to extend it whenever more functionality is needed.
//! Providing a more or less complete wrapper would be too much work.
#![deny(missing_docs)]
extern crate libc;

/// Bindings to the C interface
mod c_signatures;
/// Global memory management
pub mod global;
/// Manipulations on the DOM representation
pub mod tree;
/// XML and HTML parsing
pub mod parser;
/// XPath module for global lookup in the DOM
pub mod xpath;
