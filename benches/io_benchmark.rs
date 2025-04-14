use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use criterion::{Criterion, criterion_group, criterion_main};
use libc::{c_uint, c_void};
use libz_sys::*;
use std::ffi::CString;
use std::io;

const BUFFER_SIZE: usize = 128 * 1024;
const READ_BUFFER_SIZE: usize = 8 * 1024;
const TEST_FILE: &str = "M_abscessus_HiSeq.fq";

fn read_with_stdlib(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut buf = [0u8; READ_BUFFER_SIZE];
    let mut total = 0;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        total += n;
    }

    Ok(total)
}

fn read_with_gzread(path: &str) -> io::Result<usize> {
    let c_path = CString::new(path).unwrap();
    unsafe {
        let file: *mut gzFile_s = gzopen(c_path.as_ptr(), b"rb\0".as_ptr() as *const _);
        if file.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "gzopen failed"));
        }

        let mut buf = [0u8; READ_BUFFER_SIZE];
        let mut total = 0;

        loop {
            let n = gzread(file, buf.as_mut_ptr() as *mut c_void, buf.len() as c_uint);
            if n == 0 {
                break;
            } else if n < 0 {
                gzclose(file);
                return Err(io::Error::new(io::ErrorKind::Other, "gzread error"));
            }

            total += n as usize;
        }

        gzclose(file);
        Ok(total)
    }
}

struct GzReader {
    inner: *mut gzFile_s,
}

impl GzReader {
    fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let c_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            let gz = gzopen(c_path.as_ptr(), b"rb\0".as_ptr() as *const _);
            if gz.is_null() {
                Err(io::Error::new(io::ErrorKind::Other, "gzopen failed"))
            } else {
                Ok(Self { inner: gz })
            }
        }
    }
}

impl Read for GzReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = unsafe {
            gzread(
                self.inner,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as c_uint,
            )
        };
        if n < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "gzread error"));
        }
        Ok(n as usize)
    }
}

impl Drop for GzReader {
    fn drop(&mut self) {
        unsafe {
            gzclose(self.inner);
        }
    }
}

fn read_with_gzread_buffered(path: &str) -> io::Result<usize> {
    let gz = GzReader::open(path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, gz);
    let mut buf = [0u8; READ_BUFFER_SIZE];
    let mut total = 0;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        total += n;
    }

    Ok(total)
}

fn bench_reads(c: &mut Criterion) {
    println!("Running bench");
    c.bench_function("stdlib read", |b| {
        b.iter(|| read_with_stdlib(TEST_FILE).unwrap())
    });

    c.bench_function("gzread (raw)", |b| {
        b.iter(|| read_with_gzread(TEST_FILE).unwrap())
    });

    c.bench_function("gzread (BufReader)", |b| {
        b.iter(|| read_with_gzread_buffered(TEST_FILE).unwrap())
    });
}

criterion_group!(benches, bench_reads);
criterion_main!(benches);
