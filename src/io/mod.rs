// Copyright (c) 2016 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use futures::{future::ok, stream};
#[cfg(feature = "ssl")]
use futures::{future::Either::*, IntoFuture};
use mysql_common::packets::RawPacket;
#[cfg(feature = "ssl")]
use native_tls::{Certificate, Identity, TlsConnector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_codec::Framed;
#[cfg(feature = "ssl")]
use tokio_codec::FramedParts;
use tokio_io::{AsyncRead, AsyncWrite};

use std::{
    fmt,
    future::Future,
    io,
    net::ToSocketAddrs,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[cfg(feature = "ssl")]
use std::{fs::File, io::Read};

use crate::{
    error::*,
    io::{
        io_futures::{
            new_connecting_tcp_stream, new_write_packet, ConnectingTcpStream, WritePacket,
        },
        socket::Socket,
    },
    opts::SslOpts,
    MyFuture,
};

#[cfg(feature = "ssl")]
mod async_tls;
pub mod io_futures;
mod packet_codec;
mod socket;

#[derive(Debug)]
pub enum Endpoint {
    Plain(Pin<Box<TcpStream>>),
    #[cfg(feature = "ssl")]
    Secure(Pin<Box<self::async_tls::TlsStream<TcpStream>>>),
    Socket(Pin<Box<Socket>>),
}

impl Endpoint {
    #[cfg(feature = "ssl")]
    pub fn is_secure(&self) -> bool {
        if let Endpoint::Secure(_) = *self {
            true
        } else {
            false
        }
    }

    #[cfg(not(feature = "ssl"))]
    pub fn is_secure(&self) -> bool {
        false
    }

    pub fn set_keepalive_ms(&self, ms: Option<u32>) -> Result<()> {
        let ms = ms.map(|val| Duration::from_millis(u64::from(val)));
        match *self {
            Endpoint::Plain(ref stream) => stream.set_keepalive(ms)?,
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.get_ref().get_ref().set_keepalive(ms)?,
            Endpoint::Socket(_) => (/* inapplicable */),
        }
        Ok(())
    }

    pub fn set_tcp_nodelay(&self, val: bool) -> Result<()> {
        match *self {
            Endpoint::Plain(ref stream) => stream.set_nodelay(val)?,
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.get_ref().get_ref().set_nodelay(val)?,
            Endpoint::Socket(_) => (/* inapplicable */),
        }
        Ok(())
    }

    #[cfg(feature = "ssl")]
    pub fn make_secure(self, domain: String, ssl_opts: SslOpts) -> impl MyFuture<Self> {
        /*
        if let Endpoint::Socket(_) = self {
            // inapplicable
            return A(ok(self));
        }

        let fut = (|| {
            let mut builder = TlsConnector::builder();
            match ssl_opts.root_cert_path() {
                Some(root_cert_path) => {
                    let mut root_cert_der = vec![];
                    let mut root_cert_file = File::open(root_cert_path)?;
                    root_cert_file.read_to_end(&mut root_cert_der)?;
                    let root_cert = Certificate::from_der(&*root_cert_der).map_err(Error::from)?;
                    builder.add_root_certificate(root_cert);
                }
                None => (),
            }
            if let Some(pkcs12_path) = ssl_opts.pkcs12_path() {
                let der = std::fs::read(pkcs12_path)?;
                let identity = Identity::from_pkcs12(&*der, ssl_opts.password().unwrap_or(""))
                    .map_err(Error::from)?;
                builder.identity(identity);
            }
            builder.danger_accept_invalid_hostnames(ssl_opts.skip_domain_validation());
            builder.danger_accept_invalid_certs(ssl_opts.accept_invalid_certs());
            builder.build().map_err(Error::from)
        })()
        .into_future()
        .and_then(move |tls_connector| match self {
            Endpoint::Plain(stream) => {
                self::async_tls::connect_async(&tls_connector, &*domain, stream)
                    .map_err(Error::from)
            }
            Endpoint::Secure(_) | Endpoint::Socket(_) => unreachable!(),
        })
        .map(|tls_stream| Endpoint::Secure(tls_stream));

        B(fut)
         *
         */
        unimplemented!()
    }
}

impl From<TcpStream> for Endpoint {
    fn from(stream: TcpStream) -> Self {
        Endpoint::Plain(Box::pin(stream))
    }
}

impl From<Socket> for Endpoint {
    fn from(socket: Socket) -> Self {
        Endpoint::Socket(Box::pin(socket))
    }
}

#[cfg(feature = "ssl")]
impl From<self::async_tls::TlsStream<TcpStream>> for Endpoint {
    fn from(stream: self::async_tls::TlsStream<TcpStream>) -> Self {
        Endpoint::Secure(Box::pin(stream))
    }
}

impl io::Read for Endpoint {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        /*
        match *self {
            Endpoint::Plain(ref mut stream) => stream.read(buf),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref mut stream) => stream.read(buf),
            Endpoint::Socket(ref mut stream) => stream.read(buf),
        }
         */
        unimplemented!()
    }
}

impl io::Write for Endpoint {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        /*
        match *self {
            Endpoint::Plain(ref mut stream) => stream.write(buf),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref mut stream) => stream.write(buf),
            Endpoint::Socket(ref mut stream) => stream.write(buf),
        }
         */
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        /*
        match *self {
            Endpoint::Plain(ref mut stream) => stream.flush(),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref mut stream) => stream.flush(),
            Endpoint::Socket(ref mut stream) => stream.flush(),
        }
         */
        unimplemented!()
    }
}

impl AsyncRead for Endpoint {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        /*
        match *self {
            Endpoint::Plain(ref stream) => stream.prepare_uninitialized_buffer(buf),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.prepare_uninitialized_buffer(buf),
            Endpoint::Socket(ref stream) => stream.prepare_uninitialized_buffer(buf),
        }
         */
        unimplemented!()
    }

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        /*
        match *self {
            Endpoint::Plain(ref stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.poll_read(cx, buf),
            Endpoint::Socket(ref stream) => stream.poll_read(cx, buf),
        }
         */
        unimplemented!()
    }
}

impl AsyncWrite for Endpoint {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<futures::io::Result<usize>> {
        /*
        match *self {
            Endpoint::Plain(ref stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.poll_write(cx, buf),
            Endpoint::Socket(ref stream) => stream.poll_write(cx, buf),
        }
         */
        unimplemented!()
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<std::result::Result<(), io::Error>> {
        /*
        match *self {
            Endpoint::Plain(ref stream) => stream.poll_flush(cx),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.poll_flush(cx),
            Endpoint::Socket(ref stream) => stream.poll_flush(cx),
        }
         */
        unimplemented!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<futures::io::Result<()>> {
        /*
        match *self {
            Endpoint::Plain(ref stream) => stream.poll_flush(cx),
            #[cfg(feature = "ssl")]
            Endpoint::Secure(ref stream) => stream.poll_flush(cx),
            Endpoint::Socket(ref stream) => stream.poll_flush(cx),
        }
         */
        unimplemented!()
    }
}

/// Stream connected to MySql server.
pub struct Stream {
    closed: bool,
    codec: Option<Box<Framed<Endpoint, packet_codec::PacketCodec>>>,
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stream (endpoint={:?})",
            self.codec.as_ref().unwrap().get_ref()
        )
    }
}

impl Stream {
    fn new<T: Into<Endpoint>>(endpoint: T) -> Self {
        let endpoint = endpoint.into();

        Self {
            closed: false,
            codec: Box::new(Framed::new(endpoint, packet_codec::PacketCodec::new())).into(),
        }
    }

    pub fn connect_tcp<S>(addr: S) -> ConnectingTcpStream
    where
        S: ToSocketAddrs,
    {
        new_connecting_tcp_stream(addr)
    }

    pub async fn connect_socket<P: AsRef<Path>>(path: P) -> Result<Stream> {
        //Socket::new(path).map(Stream::new).map_err(Error::from)
        unimplemented!()
    }

    pub fn write_packet(self, data: Vec<u8>, seq_id: u8) -> WritePacket {
        new_write_packet(self, data, seq_id)
    }

    pub fn set_keepalive_ms(&self, ms: Option<u32>) -> Result<()> {
        self.codec.as_ref().unwrap().get_ref().set_keepalive_ms(ms)
    }

    pub fn set_tcp_nodelay(&self, val: bool) -> Result<()> {
        self.codec.as_ref().unwrap().get_ref().set_tcp_nodelay(val)
    }

    #[cfg(not(feature = "ssl"))]
    #[allow(unused)]
    pub async fn make_secure(self, domain: String, ssl_opts: SslOpts) -> Result<Self> {
        //ok(panic!("Ssl connection requires `ssl` feature"))
        unimplemented!()
    }

    #[cfg(feature = "ssl")]
    pub async fn make_secure(mut self, domain: String, ssl_opts: SslOpts) -> Result<Self> {
        unimplemented!()
        /*
        let codec = self.codec.take().unwrap();
        let FramedParts { io, codec, .. } = codec.into_parts();
        io.make_secure(domain, ssl_opts).map(move |endpoint| {
            let codec = Framed::new(endpoint, codec);
            self.codec = Some(Box::new(codec));
            self
        })
         */
    }

    pub fn is_secure(&self) -> bool {
        self.codec.as_ref().unwrap().get_ref().is_secure()
    }
}

impl stream::Stream for Stream {
    type Item = Result<(RawPacket, u8)>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unimplemented!()
        /*
        if !self.closed {
            self.codec.as_mut().unwrap().poll().map_err(Error::from)
        } else {
            Ok(Async::Ready(None))
        }
         */
    }
}
