#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_generate_replacement_record_impl(
    port_: MessagePort,
    mem_capacity: impl Wire2Api<usize> + UnwindSafe,
    total_instrument: impl Wire2Api<usize> + UnwindSafe,
    page_size: impl Wire2Api<usize> + UnwindSafe,
    algo_choice: impl Wire2Api<AlgoChoice> + UnwindSafe,
    gen_choice: impl Wire2Api<GenChoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ExecRecord, _>(
        WrapInfo {
            debug_name: "generate_replacement_record",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mem_capacity = mem_capacity.wire2api();
            let api_total_instrument = total_instrument.wire2api();
            let api_page_size = page_size.wire2api();
            let api_algo_choice = algo_choice.wire2api();
            let api_gen_choice = gen_choice.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(generate_replacement_record(
                    api_mem_capacity,
                    api_total_instrument,
                    api_page_size,
                    api_algo_choice,
                    api_gen_choice,
                ))
            }
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<AlgoChoice> for i32 {
    fn wire2api(self) -> AlgoChoice {
        match self {
            0 => AlgoChoice::FIFO,
            1 => AlgoChoice::LRU,
            _ => unreachable!("Invalid variant for AlgoChoice: {}", self),
        }
    }
}
impl Wire2Api<GenChoice> for i32 {
    fn wire2api(self) -> GenChoice {
        match self {
            0 => GenChoice::Random,
            1 => GenChoice::Sequential,
            _ => unreachable!("Invalid variant for GenChoice: {}", self),
        }
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for ExecRecord {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.records.into_into_dart().into_dart(),
            self.total_instrument.into_into_dart().into_dart(),
            self.total_faults.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ExecRecord {}
impl rust2dart::IntoIntoDart<ExecRecord> for ExecRecord {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for MemState {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.sequential.into_into_dart().into_dart(),
            self.instrument.into_into_dart().into_dart(),
            self.frame.into_into_dart().into_dart(),
            self.info.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for MemState {}
impl rust2dart::IntoIntoDart<MemState> for MemState {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use self::io::*;
