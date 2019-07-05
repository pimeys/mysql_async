// Copyright (c) 2017 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use futures::{future::FutureObj, ready};
use std::{
    future::Future,
    mem,
    pin::Pin,
    task::{
        Context,
        Poll::{self, Ready},
    },
};

use crate::{
    connection_like::ConnectionLike,
    error::*,
    queryable::{query_result::QueryResult, Protocol},
    BoxFuture, Row,
};

pub struct Map<T, P, F, U> {
    fut: BoxFuture<(QueryResult<T, P>, Option<Row>)>,
    acc: Vec<U>,
    fun: F,
}

impl<T, P, F, U> Map<T, P, F, U>
where
    F: FnMut(Row) -> U,
    P: Protocol,
    P: Send + 'static,
    T: ConnectionLike + Sized + 'static,
{
    pub fn new(query_result: QueryResult<T, P>, fun: F) -> Map<T, P, F, U> {
        Map {
            fut: FutureObj::new(Box::new(query_result.get_row())),
            acc: Vec::new(),
            fun,
        }
    }
}

impl<T, P, F, U> Future for Map<T, P, F, U>
where
    F: FnMut(Row) -> U,
    P: Protocol,
    P: Send + 'static,
    T: ConnectionLike + Sized + 'static,
{
    type Output = Result<(QueryResult<T, P>, Vec<U>)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        /*
        loop {
            let (query_result, row_opt) = try_ready!(self.fut.poll());
            match row_opt {
                Some(row) => {
                    let val = (self.fun)(row);
                    self.acc.push(val);
                }
                None => {
                    return Ok(Ready((
                        query_result,
                        mem::replace(&mut self.acc, Vec::new()),
                    )));
                }
            }
            self.fut = Box::new(query_result.get_row());
        }
         */
        unimplemented!()
    }
}
