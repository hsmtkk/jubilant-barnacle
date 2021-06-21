mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Foundation::SYSTEMTIME,
    Windows::Win32::System::SystemInformation::GetSystemTime,
};

fn main() -> windows::Result<()> {
    let mut t = SYSTEMTIME {
        wYear: 0,
        wMonth: 0,
        wDay: 0,
        wDayOfWeek: 0,
        wHour: 0,
        wMinute: 0,
        wSecond: 0,
        wMilliseconds: 0,
    };
    unsafe {
        GetSystemTime(&mut t);
    }
    dbg!(t);
    Ok(())
}
