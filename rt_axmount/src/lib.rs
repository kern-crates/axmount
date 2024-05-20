#![no_std]

#[macro_use]
extern crate axlog2;
extern crate alloc;
use alloc::sync::Arc;

use core::panic::PanicInfo;
use axtype::{align_up_4k, align_down_4k, phys_to_virt, virt_to_phys};
use driver_block::{ramdisk, BlockDriverOps};
use axdriver::{prelude::*, AxDeviceContainer};

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    axhal::cpu::init_primary(cpu_id);

    axlog2::init();
    axlog2::set_max_level("debug");
    info!("[rt_axmount]: ...");

    let start = align_up_4k(virt_to_phys(_ekernel as usize));
    let end = align_down_4k(axconfig::PHYS_MEMORY_END);
    axalloc::global_init(phys_to_virt(start), end - start);

    let ctx = Arc::new(taskctx::init_sched_info());
    unsafe {
        let ptr = Arc::into_raw(ctx.clone());
        axhal::cpu::set_current_task_ptr(ptr);
    }

    run_queue::init();

    {
        let mut disk = ramdisk::RamDisk::new(0x10000);
        let mut disk = AxDeviceContainer::from_one(disk);

        let main_fs = axmount::init_filesystems(disk, true);
        let root_dir = axmount::init_rootfs(main_fs);
    }

    info!("[rt_axmount]: ok!");
    axhal::misc::terminate();
}

pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}

extern "C" {
    fn _ekernel();
}
