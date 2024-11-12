# axmount

Filesystem mounting and initialization for embedded systems.

This crate provides functionality to initialize and mount various filesystems in a no_std environment. It supports different filesystem types including ext2, FAT, and custom filesystems, as well as virtual filesystems like devfs and sysfs.

Features

+ Multiple filesystem support (ext2, FAT, custom)
+ Virtual filesystem mounting (devfs, sysfs, ramfs)
+ Block device management
+ Root filesystem initialization

## Examples

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[rt_early_console]: ok!\n";
    early_console::write_bytes(msg.as_bytes());
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
```

## Functions

### `init`

```rust
pub fn init(_cpu_id: usize, _dtb_pa: usize)
```

Initializes the entire filesystem hierarchy.

### `init_filesystems`

```rust
pub fn init_filesystems(
    blk_devs: AxDeviceContainer<AxBlockDevice>,
    _need_fmt: bool
) -> Arc<Ext2Fs>
```

Initializes the main filesystem using available block devices.

### `init_root`

```rust
pub fn init_root() -> Arc<RootDirectory>
```

Returns a reference to the initialized root directory.

### `init_rootfs`

```rust
pub fn init_rootfs(main_fs: Arc<dyn VfsOps>) -> Arc<RootDirectory>
```

Initializes and configures the root filesystem with various mount points.
