// Copyright (c) 2016 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    conn::{pool::Pool, Conn},
    error::*,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// This future will take connection from a pool and resolve to `Conn`.
pub struct GetConn {
    pool: Pin<Box<Pool>>,
}

pub fn new(pool: &Pool) -> GetConn {
    GetConn {
        pool: Box::pin(pool.clone()),
    }
}

impl Future for GetConn {
    type Output = Result<Conn>;

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        //self.pool.poll()
        unimplemented!()
    }
}
