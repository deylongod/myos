pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga_buffer;

pub use gdt::init as gdt_init;
pub use interrupts::init_idt as interrupts_init_idt;

