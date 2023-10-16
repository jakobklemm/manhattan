//! # Manhattan
//!
//! A basic prototype for an Actor Framework for Rust, somewhat inspired by OTP Erlang.

#![feature(async_fn_in_trait)]

use std::any::Any;

pub trait Handle<M> {
    async fn handle(message: M) -> u32;
}
