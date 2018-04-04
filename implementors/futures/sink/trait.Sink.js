(function() {var implementors = {};
implementors["futures"] = [];
implementors["tokio_core"] = ["impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_core/net/trait.UdpCodec.html\" title=\"trait tokio_core::net::UdpCodec\">UdpCodec</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_core/net/struct.UdpFramed.html\" title=\"struct tokio_core::net::UdpFramed\">UdpFramed</a>&lt;C&gt;",];
implementors["tokio_io"] = ["impl&lt;T, U&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/struct.Framed.html\" title=\"struct tokio_io::codec::Framed\">Framed</a>&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: <a class=\"trait\" href=\"tokio_io/codec/trait.Encoder.html\" title=\"trait tokio_io::codec::Encoder\">Encoder</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U::<a class=\"type\" href=\"tokio_io/codec/trait.Encoder.html#associatedtype.Error\" title=\"type tokio_io::codec::Encoder::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;,&nbsp;</span>","impl&lt;T, D&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/struct.FramedRead.html\" title=\"struct tokio_io::codec::FramedRead\">FramedRead</a>&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a>,&nbsp;</span>","impl&lt;T, E&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/struct.FramedWrite.html\" title=\"struct tokio_io::codec::FramedWrite\">FramedWrite</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"tokio_io/codec/trait.Encoder.html\" title=\"trait tokio_io::codec::Encoder\">Encoder</a>,&nbsp;</span>","impl&lt;T:&nbsp;<a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>, B:&nbsp;<a class=\"trait\" href=\"bytes/buf/into_buf/trait.IntoBuf.html\" title=\"trait bytes::buf::into_buf::IntoBuf\">IntoBuf</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/length_delimited/struct.Framed.html\" title=\"struct tokio_io::codec::length_delimited::Framed\">Framed</a>&lt;T, B&gt;","impl&lt;T:&nbsp;<a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/length_delimited/struct.FramedRead.html\" title=\"struct tokio_io::codec::length_delimited::FramedRead\">FramedRead</a>&lt;T&gt;","impl&lt;T:&nbsp;<a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>, B:&nbsp;<a class=\"trait\" href=\"bytes/buf/into_buf/trait.IntoBuf.html\" title=\"trait bytes::buf::into_buf::IntoBuf\">IntoBuf</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_io/codec/length_delimited/struct.FramedWrite.html\" title=\"struct tokio_io::codec::length_delimited::FramedWrite\">FramedWrite</a>&lt;T, B&gt;",];
implementors["tokio_udp"] = ["impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_io/codec/encoder/trait.Encoder.html\" title=\"trait tokio_io::codec::encoder::Encoder\">Encoder</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_udp/struct.UdpFramed.html\" title=\"struct tokio_udp::UdpFramed\">UdpFramed</a>&lt;C&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
