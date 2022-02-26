// https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html

pub const VIRTIO: u64 = 0x1000_1000;
pub const VIRTIO_MMIO_MAGIC_VALUE: u64 = VIRTIO + 0x000;
pub const VIRTIO_MMIO_VERSION: u64 = VIRTIO + 0x004;
pub const VIRTIO_MMIO_DEVICE_ID: u64 = VIRTIO + 0x008;
pub const VIRTIO_MMIO_VENDOR_ID: u64 = VIRTIO + 0x00C;
pub const VIRTIO_END: u64 = 0x1000_1FFF;

#[derive(Debug)]
pub struct Virtio {}

impl Virtio {
    pub fn new() -> Virtio {
        Virtio {}
    }

    pub fn read(&self, addr: u64) -> u64 {
        match addr {
            VIRTIO_MMIO_MAGIC_VALUE => 0x7472_6976,
            VIRTIO_MMIO_VERSION => 0x1,
            VIRTIO_MMIO_DEVICE_ID => 0x2,
            VIRTIO_MMIO_VENDOR_ID => 0x554d4551,
            _ => panic!("invalid read to virtio address"),
        }
    }

    pub fn write(&mut self, addr: u64, data: u64) {}
}
