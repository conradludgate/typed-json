use std::fmt;
use std::io;

use serde::Serialize;

use crate::Array;
use crate::Expr;
use crate::ItemSer;
use crate::KeyValuePairSer;
use crate::Map;
use crate::Null;

struct WriterFormatter<'a, 'b: 'a> {
    inner: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Safety: the serializer below only emits valid utf8 when using
        // the default formatter.
        // > Serialization guarantees it only feeds valid UTF-8 sequences to the writer.
        // <https://docs.rs/serde_json/1.0.108/serde_json/fn.to_writer.html>
        // <https://docs.rs/serde_json/1.0.108/serde_json/fn.to_writer_pretty.html>
        let s = unsafe { std::str::from_utf8_unchecked(buf) };
        self.inner.write_str(s).map_err(io_error)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn io_error(_: fmt::Error) -> io::Error {
    // Error value does not matter because Display impl just maps it
    // back to fmt::Error.
    io::Error::new(io::ErrorKind::Other, "fmt error")
}

impl<T: KeyValuePairSer> fmt::Display for Map<T> {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use typed_json::json;
    /// #
    /// let json = json!({ "city": "London", "street": "10 Downing Street" });
    ///
    /// // Compact format:
    /// //
    /// // {"city":"London","street":"10 Downing Street"}
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
    ///
    /// // Pretty format:
    /// //
    /// // {
    /// //   "city": "London",
    /// //   "street": "10 Downing Street"
    /// // }
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

impl<T: ItemSer> fmt::Display for Array<T> {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use typed_json::json;
    /// #
    /// let json = json!(["London", "10 Downing Street" ]);
    ///
    /// // Compact format:
    /// //
    /// // ["London","10 Downing Street"]
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "[\"London\",\"10 Downing Street\"]");
    ///
    /// // Pretty format:
    /// //
    /// // [
    /// //   "London",
    /// //   "10 Downing Street"
    /// // ]
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "[\n  \"London\",\n  \"10 Downing Street\"\n]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

impl<S: Serialize> fmt::Display for Expr<S> {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use typed_json::json;
    /// #
    /// let json = json!(1);
    ///
    /// // Compact format:
    /// let compact = format!("{}", json);
    /// assert_eq!(compact, "1");
    ///
    /// // Pretty format:
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty, "1");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

impl fmt::Display for Null {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use typed_json::json;
    /// #
    /// let json = json!(null);
    ///
    /// // Compact format:
    /// let compact = format!("{}", json);
    /// assert_eq!(compact, "null");
    ///
    /// // Pretty format:
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty, "null");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}
