use winapi::{
    shared::{windef::HWND, minwindef::{DWORD, FALSE}}, 
    um::{winbase::QueryFullProcessImageNameW, processthreadsapi::OpenProcess, winnt::{PROCESS_QUERY_INFORMATION}, handleapi::CloseHandle},
    um::winuser::{GetWindowTextLengthW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId}
};

use crate::err::Error;

pub struct Window {
    handle: HWND
}

impl Window {
    pub fn foreground_window() -> Result<Self, Error> {
        let handle = unsafe { GetForegroundWindow() };
        if handle.is_null() {
            return Err(Error::Fetch);
        }

        Ok(Self { handle })
    }

    pub fn title(&self) -> Result<String, Error> {
        let len = unsafe { GetWindowTextLengthW(self.handle) } + 1;

        let mut v = vec![0; len as usize];
        let result_len = unsafe { GetWindowTextW(self.handle, v.as_mut_ptr(), len) };
        if result_len == 0 {
            return Err(Error::Fetch);
        }

        String::from_utf16(&v[..result_len as usize]).map_err(|_| Error::Fetch)
    }

    pub fn process_id(&self) -> Result<u32, Error> {
        unsafe {
            let mut pid: DWORD = std::mem::zeroed();
            if GetWindowThreadProcessId(self.handle, &mut pid) == 0 {
                return Err(Error::Fetch);
            }
            Ok(pid)
        }
    }

    pub fn process_name(&self) -> Result<String, Error> {
        let pid = self.process_id()?;
        let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, pid) };
        if handle.is_null() {
            return Err(Error::Fetch);
        }

        let mut v = vec![0; 256];
        let mut len = 256;
        if unsafe { QueryFullProcessImageNameW(handle, 0, v.as_mut_ptr(), &mut len) } == 0 {
            return Err(Error::Fetch);
        }

        unsafe { CloseHandle(handle); }

        String::from_utf16(&v[..len as usize]).map_err(|_| Error::Fetch)
    }
}