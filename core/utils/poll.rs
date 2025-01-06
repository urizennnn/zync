use core::panic;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::task::noop_waker_ref;
use log::log;

pub fn poll_future<F: futures::Future + Unpin>(mut func: F) -> F::Output
where
    F::Output: std::fmt::Debug,
{
    let waker = noop_waker_ref();
    let mut ctx = Context::from_waker(waker);
    let mut pinned_future = Pin::new(&mut func);
    loop {
        match pinned_future.as_mut().poll(&mut ctx) {
            Poll::Ready(ret_value) => {
                log::info!("{:?}", ret_value);
                return ret_value;
            }
            Poll::Pending => {
                // Future is not ready yet; continue polling
            }
        }
    }
}
