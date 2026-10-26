use log::kv::{self, Key, Value, VisitSource};
use log::{Level, LevelFilter, Metadata, Record};

use std::env;
use std::io::{self, Write};

const MAX_DIAGNOSTICS: usize = 8;

struct Logger {
    filter: LevelFilter,
}

pub fn init() {
    let filter = match env::var("RUST_LOG") {
        Ok(s) => s.parse().unwrap_or(LevelFilter::Info),
        Err(_) => LevelFilter::Info,
    };

    let logger = Box::new(Logger { filter });

    let _ = log::set_boxed_logger(logger);
    log::set_max_level(filter);
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.write_log(record);
        }
    }

    fn flush(&self) {}
}

impl Logger {
    fn write_log(&self, record: &Record) -> io::Result<()> {
        let level = format_level(record.level());
        let message = record.args();
        let file = record.file().unwrap_or("<unknown>");

        let mut visitor = DiagnosticVisitor::new();
        let _ = record.key_values().visit(&mut visitor);

        let stderr = io::stderr();
        let mut out = stderr.lock();

        match record.line() {
            Some(line) => writeln!(
                out,
                "{}: {} \x1b[2m({}:{})\x1b[0m",
                level, message, file, line
            )?,
            None => writeln!(out, "{}: {} \x1b[2m({}:?)\x1b[0m", level, message, file)?,
        }

        for (key, val) in visitor.data[..visitor.num_data].iter().flatten() {
            let mut wrt = PrefixedWriter::new(&mut out, "  \x1b[1;34m|\x1b[0m ");
            if key.as_str() == "data" {
                writeln!(wrt, "{:#}", val)?;
            } else {
                writeln!(wrt, "{}: {:#}", key.as_str(), val)?;
            }
        }

        for note in visitor.notes[..visitor.num_notes].iter().flatten() {
            let mut wrt =
                PrefixedWriter::new(&mut out, "  \x1b[1;34m=\x1b[0m \x1b[1;37mnote\x1b[0m: ");
            writeln!(wrt, "{}", note)?;
        }

        Ok(())
    }
}

const fn format_level(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[1;31merror\x1b[0m",
        Level::Warn => "\x1b[1;33mwarning\x1b[0m",
        Level::Info => "\x1b[1;32minfo\x1b[0m",
        Level::Debug => "\x1b[36mdebug\x1b[0m",
        Level::Trace => "\x1b[37mtrace\x1b[0m",
    }
}

struct DiagnosticVisitor<'kv> {
    notes: [Option<Value<'kv>>; MAX_DIAGNOSTICS],
    num_notes: usize,
    data: [Option<(Key<'kv>, Value<'kv>)>; MAX_DIAGNOSTICS],
    num_data: usize,
}

impl DiagnosticVisitor<'_> {
    fn new() -> Self {
        Self {
            notes: [const { None }; MAX_DIAGNOSTICS],
            num_notes: 0,
            data: [const { None }; MAX_DIAGNOSTICS],
            num_data: 0,
        }
    }
}

impl<'kv> VisitSource<'kv> for DiagnosticVisitor<'kv> {
    fn visit_pair(&mut self, key: Key<'kv>, value: Value<'kv>) -> Result<(), kv::Error> {
        if key.as_str() == "note" && self.num_notes < MAX_DIAGNOSTICS {
            self.notes[self.num_notes] = Some(value);
            self.num_notes += 1;
        } else if self.num_data < MAX_DIAGNOSTICS {
            self.data[self.num_data] = Some((key, value));
            self.num_data += 1;
        }

        Ok(())
    }
}

struct PrefixedWriter<W: Write> {
    writer: W,
    prefix: &'static [u8],
    at_line_start: bool,
}

impl<W: Write> PrefixedWriter<W> {
    fn new(writer: W, prefix: &'static str) -> Self {
        Self {
            writer,
            prefix: prefix.as_bytes(),
            at_line_start: true,
        }
    }
}

impl<W: Write> Write for PrefixedWriter<W> {
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let total = buf.len();
        while !buf.is_empty() {
            if self.at_line_start {
                self.writer.write_all(self.prefix)?;
                self.at_line_start = false;
            }
            if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                self.writer.write_all(&buf[..=pos])?;
                self.at_line_start = true;
                buf = &buf[pos + 1..];
            } else {
                self.writer.write_all(buf)?;
                break;
            }
        }
        Ok(total)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
