// https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html

pub const VIRTIO: u64 = 0x1000_1000;
pub const VIRTIO_MMIO_MAGIC_VALUE: u64 = VIRTIO + 0x000;
pub const VIRTIO_MMIO_VERSION: u64 = VIRTIO + 0x004;
pub const VIRTIO_MMIO_DEVICE_ID: u64 = VIRTIO + 0x008;
pub const VIRTIO_MMIO_VENDOR_ID: u64 = VIRTIO + 0x00C;
pub const VIRTIO_MMIO_DEVICE_FEATURES: u64 = VIRTIO + 0x010;
pub const VIRTIO_MMIO_DRIVER_FEATURES: u64 = VIRTIO + 0x020;
pub const VIRTIO_MMIO_GUEST_PAGE_SIZE: u64 = VIRTIO + 0x028;
pub const VIRTIO_MMIO_QUEUE_SEL: u64 = VIRTIO + 0x030;
pub const VIRTIO_MMIO_STATUS: u64 = VIRTIO + 0x070;
pub const VIRTIO_END: u64 = 0x1000_1FFF;

#[derive(Debug)]
pub struct Virtio {
    mmio_status: u64,
    mmio_driver_features: u64,
    mmio_guest_page_size: u64,
    mmio_queue_sel: u64,
}

impl Virtio {
    pub fn new() -> Virtio {
        Virtio {
            mmio_status: 0,
            mmio_driver_features: 0,
            mmio_guest_page_size: 0,
            mmio_queue_sel: 0,
        }
    }

    pub fn read(&self, addr: u64) -> u64 {
        match addr {
            VIRTIO_MMIO_MAGIC_VALUE => 0x7472_6976,
            VIRTIO_MMIO_VERSION => 0x1,
            VIRTIO_MMIO_DEVICE_ID => 0x2, // Block Device
            VIRTIO_MMIO_VENDOR_ID => 0x554d4551,
            VIRTIO_MMIO_DEVICE_FEATURES => 0,
            _ => panic!("invalid read to virtio address"),
        }
    }

    pub fn write(&mut self, addr: u64, data: u64) {
        match addr {
            VIRTIO_MMIO_STATUS => self.mmio_status = data,
            VIRTIO_MMIO_DRIVER_FEATURES => self.mmio_driver_features = data,
            VIRTIO_MMIO_GUEST_PAGE_SIZE => self.mmio_guest_page_size = data,
            VIRTIO_MMIO_QUEUE_SEL => self.mmio_queue_sel = data,
            _ => panic!("invalid write to virtio address"),
        }
    }
}
