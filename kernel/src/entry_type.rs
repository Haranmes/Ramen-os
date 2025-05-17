use limine::memory_map::EntryType;

pub fn entry_type_to_str(typ: EntryType) -> &'static str {
    match typ {
        EntryType::USABLE => "Usable",
        EntryType::RESERVED => "Reserved",
        EntryType::ACPI_RECLAIMABLE => "ACPI Reclaimable",
        EntryType::ACPI_NVS => "ACPI NVS",
        EntryType::BAD_MEMORY => "Bad Memory",
        EntryType::BOOTLOADER_RECLAIMABLE => "Bootloader Reclaimable",
        EntryType::EXECUTABLE_AND_MODULES => "Executable and Modules",
        EntryType::FRAMEBUFFER => "Framebuffer",
        _ => "Unknown",
    }
}
