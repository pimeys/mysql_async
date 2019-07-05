// Copyright (c) 2016 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use futures::{
    future::{select_ok, SelectOk},
    ready,
};
use std::{
    future::Future,
    io,
    net::ToSocketAddrs,
    pin::Pin,
    task::{
        Context,
        Poll::{self, Ready},
    },
};
use tokio::net::TcpStream;
use tokio_codec::Framed;

use crate::{
    error::*,
    io::{packet_codec::PacketCodec, Stream},
};

enum Step {
    WaitForStream(Pin<Box<SelectOk<Pin<Box<dyn Future<Output = io::Result<TcpStream>>>>>>>),
    Fail(Error),
}

enum Out {
    WaitForStream(Pin<Box<TcpStream>>),
    Fail(Error),
}

/// Future that resolves to a `Stream` connected to a MySql server.
pub struct ConnectingTcpStream {
    step: Step,
}

impl ConnectingTcpStream {
    fn either_poll(&mut self, cx: &mut Context) -> Result<Poll<Out>> {
        /*
        match self.step {
            Step::WaitForStream(fut) => Ok(Ready(Out::WaitForStream(ready!(fut.poll(cx))))),
            Step::Fail(error) => Err(Out::Fail(error)),
        }
         */
        unimplemented!()
    }
}

pub fn new<S>(addr: S) -> ConnectingTcpStream
where
    S: ToSocketAddrs,
{
    /*
    match addr.to_socket_addrs() {
        Ok(addresses) => {
            let mut streams = Vec::new();

            for address in addresses {
                streams.push(TcpStream::connect(&address));
            }

            if !streams.is_empty() {
                ConnectingTcpStream {
                    step: Step::WaitForStream(select_ok(streams)),
                }
            } else {
                let err = io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "could not resolve to any address",
                );
                ConnectingTcpStream {
                    step: Step::Fail(failed(err.into())),
                }
            }
        }
        Err(err) => ConnectingTcpStream {
            step: Step::Fail(failed(err.into())),
        },
    }
     */
    unimplemented!()
}

impl Future for ConnectingTcpStream {
    type Output = Result<Stream>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        /*
        match try_ready!(self.either_poll()) {
            Out::WaitForStream((stream, _)) => Ok(Ready(Stream {
                closed: false,
                codec: Box::new(Framed::new(stream.into(), PacketCodec::new())).into(),
            })),
            Out::Fail(_) => unreachable!(),
        }
         */
        unimplemented!()
    }
}
