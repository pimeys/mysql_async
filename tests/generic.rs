// Copyright (c) 2019 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![feature(async_await)]
#![allow(unused_imports, unused_variables, unused_mut, dead_code)]

use futures::Future;
use mysql_async::error::Error;
use mysql_async::prelude::*;
use mysql_async::{MyFuture, Opts, Pool, QueryResult};

use std::env;
use std::io;

/// Same as `tokio::run`, but will panic if future panics and will return the result
/// of future execution.
fn run<F, T, U>(future: F) -> Result<T, U>
where
    F: Future<Output = Result<T, U>> + Send + 'static,
    T: Send + 'static,
    U: Send + 'static,
{
    /*
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(future);
    runtime.shutdown_on_idle().wait().unwrap();
    result
     */
    unimplemented!()
}

fn get_url() -> String {
    if let Ok(url) = env::var("DATABASE_URL") {
        let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
        if opts
            .get_db_name()
            .expect("a database name is required")
            .is_empty()
        {
            panic!("database name is empty");
        }
        url
    } else {
        "mysql://root:password@127.0.0.1:3307/mysql".into()
    }
}

pub async fn get_all_results<TupleType, T, P>(result: QueryResult<T, P>) -> Result<Vec<TupleType>, Error>
where
    TupleType: FromRow + Send + 'static,
    P: Protocol + Send + 'static,
    T: ConnectionLike + Sized + Send + 'static,
{
    //result.collect().map(|(_, data)| data)
    unimplemented!()
}

pub async fn get_single_result<TupleType, T, P>(result: QueryResult<T, P>) -> Result<TupleType, Error>
where
    TupleType: FromRow + Send + 'static,
    P: Protocol + Send + 'static,
    T: ConnectionLike + Sized + Send + 'static,
{
    /*
    get_all_results(result).and_then(|mut data| {
        if data.len() != 1 {
            Err(Error::from(io::Error::from(io::ErrorKind::InvalidData)))
        } else {
            Ok(data.remove(0))
        }
    })
     */
    unimplemented!()
}

#[test]
fn use_generic_code() {
    /*
    let pool = Pool::new(Opts::from_url(&*get_url()).unwrap());
    let fut = pool
        .get_conn()
        .and_then(move |conn| conn.query("SELECT 1, 2, 3"))
        .and_then(get_single_result::<(u8, u8, u8), _, _>)
        .and_then(|out| pool.disconnect().map(move |_| out));

    let result = run(fut).unwrap();
    assert_eq!(result, (1, 2, 3));
     */
    assert!(true)
}
