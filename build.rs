// https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/Win32/System/SystemInformation/fn.GetSystemTime.html
fn main() {
    windows::build! {
        Windows::Win32::System::SystemInformation::GetSystemTime,
        Windows::Win32::Foundation::SYSTEMTIME,
    };
}