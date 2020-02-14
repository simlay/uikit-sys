#![cfg(target_os = "ios")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/uikit.rs"));
