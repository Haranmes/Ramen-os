#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::arch::asm;
use core::panic;
use limine::memory_map::{Entry, EntryType};
use limine::BaseRevision;
use limine::request::{FramebufferRequest, MemoryMapRequest, RequestsEndMarker, RequestsStartMarker};

mod psf_font;
mod makros;
mod utils;
mod entry_type;
mod log;
mod serial;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
/// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();


#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());

    #[cfg(test_mode)]
    test_main();

    // Parse Memory map from limine
    if let Some(memmap_response) = MEMMAP_REQUEST.get_response() {
        for entry in memmap_response.entries() {
            let entry_type_str : &'static str =  entry_type::entry_type_to_str(entry.entry_type);
            serial_println!("Base: {:#x}, Length: {:#x}, Type: {}",
                entry.base,
                entry.length,
                entry_type_str
            );
            
            info!(
                "Base: {:#x}, Length: {:#x}, Type: {}",
                entry.base,
                entry.length,
                entry_type_str
            );
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    error!("{}", _info);
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}



#[repr(u32)]
#[allow(dead_code)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    unsafe {
        let port = 0xf4;
        core::arch::asm!("out dx, al", in("dx") port, in("al") exit_code as u8);
        core::hint::unreachable_unchecked();
    }
}




