#![no_std]
#![no_main]

extern crate alloc;

use deylon666::println;
use deylon666::task::Task;
use deylon666::task::executor::Executor;
use deylon666::allocator;
use deylon666::kernel::memory::{self, BootInfoFrameAllocator};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    deylon666::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(async_task_1()));
    executor.spawn(Task::new(async_task_2()));
    executor.spawn(Task::new(example_task()));
    executor.run();
}

async fn async_number() -> u32 {
    println!("Выполняем асинхронную операцию");
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

async fn async_task_1() {
    use deylon666::task::yield_now;
    for i in 0..5 {
        println!("Task 1: {}", i);
        for _ in 0..1000 {
            yield_now().await;
        }
    }
}

async fn async_task_2() {
    use deylon666::task::yield_now;
    for i in 0..5 {
        println!("Task 2: {}", i);
        for _ in 0..1000 {
            yield_now().await;
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    deylon666::hlt_loop();
}
