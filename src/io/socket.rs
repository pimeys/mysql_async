// Copyright (c) 2019 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use bytes::{Buf, BufMut};
use tokio::io::{AsyncRead, AsyncWrite};

use std::{
    future::Future,
    io::{self, Read, Write},
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

/// Unix domain socket connection on unix, or named pipe connection on windows.
#[derive(Debug)]
pub struct Socket {
    #[cfg(unix)]
    inner: Pin<Box<romio::uds::UnixStream>>,
    #[cfg(windows)]
    inner: tokio_named_pipes::NamedPipe,
}

impl Socket {
    /// Connects a new socket.
    #[cfg(unix)]
    pub async fn new<P: AsRef<Path>>(path: P) -> futures::io::Result<Socket> {
        //tokio_uds::UnixStream::connect(path).map(|socket| Socket { inner: socket })
        unimplemented!()
    }

    /// Connects a new socket.
    #[cfg(windows)]
    pub async fn new<P: AsRef<Path>>(path: P) -> futures::io::Result<Socket> {
        /*
        use futures::future::IntoFuture;
        use tokio_named_pipes::NamedPipe;

        let handle = tokio::reactor::Handle::default();
        NamedPipe::new(path.as_ref(), &handle)
            .and_then(|pipe| {
                pipe.connect()?;
                Ok(Socket { inner: pipe })
            })
            .into_future()
         */
        unimplemented!()
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //self.inner.read(buf)
        unimplemented!()
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        //self.inner.write(buf)
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        //self.inner.flush()
        unimplemented!()
    }
}

impl AsyncRead for Socket {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        //self.inner.prepare_uninitialized_buffer(buf)
        unimplemented!()
    }

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        //self.inner.poll_read(cx, buf)
        unimplemented!()
    }
}

impl AsyncWrite for Socket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        //self.inner.poll_write(cx, buf)
        unimplemented!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        //self.inner.poll_flush(cx)
        unimplemented!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        //self.inner.poll_shutdown(cx)
        unimplemented!()
    }
}
