use std::io::{self, Write};
use std::ffi::CStr;
use std::ptr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
fn log(i: u32, base: u32) -> u32 {
    // Based on: Integer Logarithm, A. Jaffer 2008
    // https://people.csail.mit.edu/jaffer/III/ilog.pdf
    fn ilog(n: &mut u32, m: u32, b: u32, k: u32) -> u32 {
        if b > k {
            k
        } else {
            *n += m;
            let q = ilog(n, m + m, b * b, k / b);
            if b > q {
                q
            } else {
                *n += m;
                q / b
            }
        }
    }

    let mut n = 1;
    ilog(&mut n, 1, base, i/ base)
}

pub struct CountingWriter<'w, W>
where
    W: ?Sized,
{
    inner: &'w mut W,
    written: usize,
}

impl<'w, W> CountingWriter<'w, W>
where
    W: ?Sized,
    &'w mut W: Write,
{
    pub fn new(inner: &'w mut W) -> Self {
        Self { inner, written: 0 }
    }

    pub fn written(&self) -> usize {
        self.written
    }
}

impl<'w, W> io::Write for CountingWriter<'w, W>
where
    W: ?Sized,
    &'w mut W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.written += written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ));
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

fn log_3(msg: &str) {
    let written;
    let buf = &mut [0u8; 256];
    {
        let mut counting_writer = CountingWriter::new(&mut buf[..]);
        let filename = std::path::Path::new(file!()).file_name().unwrap().to_str().unwrap();
        writeln!(
            counting_writer,
            "{}:{}: {}",
            filename, line!(), msg).unwrap();
        written = counting_writer.written();
    }
    buf[written] = 0;
    let sl = &buf[..written+1];
    let cstring = CStr::from_bytes_with_nul(&sl).unwrap();
    // println!("{:?}", cstring);
    black_box(cstring);
}


#[no_mangle]
fn log_1(msg: &str) {
  let filename = std::path::Path::new(file!()).file_name().unwrap().to_str().unwrap();
  let cstr = ::std::ffi::CString::new(format!("{}:{}: {}\n", filename, line!(), msg)).unwrap(); 
  black_box(cstr);
}

#[no_mangle]
fn log_2(msg: &str) {
        let mut buf = [0 as u8; 256];

    let filename = std::path::Path::new(file!()).file_name().unwrap().to_str().unwrap();

    write!(&mut buf[..], "{}:{}: {}", filename, line!(), msg).unwrap(); 
    // Annoying to have to convert to f32: https://github.com/rust-lang/rust/issues/70887
    let l = filename.len() + ((line!() as f32).log10()  as usize) + msg.len() + 4;
    buf[l] = 0;
    let sl = &buf[..l+1];
    let cstring = CStr::from_bytes_with_nul(&sl).unwrap();
    black_box(cstring);
}

#[no_mangle]
fn log_4(msg: &str) {
        let mut buf = [0 as u8; 256];

    let filename = std::path::Path::new(file!()).file_name().unwrap().to_str().unwrap();

    write!(&mut buf[..], "{}:{}: {}", filename, line!(), msg).unwrap(); 
    // Annoying to have to convert to f32: https://github.com/rust-lang/rust/issues/70887
    let l = filename.len() + (log(black_box(line!()), 10) as usize) + msg.len() + 4;
    buf[l] = 0;
    let sl = &buf[..l+1];
    let cstring = CStr::from_bytes_with_nul(&sl).unwrap();
    black_box(cstring);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("log1", |b| b.iter(|| log_1(black_box("asdasd"))));
    c.bench_function("log2", |b| b.iter(|| log_2(black_box("asdasd"))));
    c.bench_function("log3", |b| b.iter(|| log_3(black_box("asdasd"))));
    c.bench_function("log4", |b| b.iter(|| log_4(black_box("asdasd"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
