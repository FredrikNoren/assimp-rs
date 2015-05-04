use std::ffi::CString;
use std::ptr;

use ffi::*;

pub struct LogStream {
    raw: AiLogStream,
    attached: bool
}

impl LogStream {
    pub fn file(filename: &str) -> Option<LogStream> {
        let cstr = CString::new(filename).unwrap().as_ptr();
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::File, cstr) };
        if stream.callback.is_some() {
            Some(LogStream { raw: stream, attached: false })
        } else {
            None
        }
    }

    pub fn stdout() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::StdOut, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    pub fn stderr() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::StdErr, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    #[cfg(windows)]
    pub fn debug() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::Debugger, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    pub fn attached(&self) -> bool { self.attached }

    pub fn attach(&mut self) {
        if !self.attached { unsafe { aiAttachLogStream(&self.raw) } }
    }

    pub fn detach(&mut self) {
        if self.attached { unsafe { aiDetachLogStream(&self.raw); } }
    }

    pub fn set_verbose_logging(state: bool) {
        unsafe { aiEnableVerboseLogging(if state { AI_TRUE } else { AI_FALSE }) }
    }
}

impl Drop for LogStream {
    fn drop(&mut self) {
        self.detach()
    }
}
