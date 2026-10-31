use core::mem::MaybeUninit;

use alloc::{boxed::Box, vec::Vec};

use crate::Buf32;

pub struct Usb(());

pub struct Device<'a> {
    usb: &'a Usb,
    fd: i32,
}

pub struct DeviceDescriptor(pub crate::ffi::usb_devdesc);
pub struct Descriptor(Box<[u8]>);
pub struct HidDescriptor(pub crate::ffi::usb_hiddesc);
pub struct Description(pub crate::ffi::usb_devdesc);

impl Usb {
    pub fn new() -> Result<Self, i32> {
        let result = unsafe { crate::ffi::USB_Initialize() };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(Self(()))
    }

    pub fn open_device(&self, device_id: i32, vid: u16, pid: u16) -> Result<Device<'_>, i32> {
        let mut fd = i32::MAX;
        let result = unsafe { crate::ffi::USB_OpenDevice(device_id, vid, pid, &mut fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(Device { usb: self, fd: fd })
    }
}

impl Drop for Usb {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::USB_Deinitialize();
        }
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::USB_CloseDevice(&mut self.fd);
        }
    }
}

impl Drop for DeviceDescriptor {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::USB_FreeDescriptors(&mut self.0);
        }
    }
}

impl<'a> Device<'a> {
    pub fn get_device_descriptor(&self) -> Result<DeviceDescriptor, i32> {
        let mut descriptor = MaybeUninit::<crate::ffi::usb_devdesc>::uninit();
        let result = unsafe { crate::ffi::USB_GetDescriptors(self.fd, descriptor.as_mut_ptr()) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(DeviceDescriptor(unsafe { descriptor.assume_init() }))
    }

    pub fn get_generic_descriptor(
        &self,
        kind: u8,
        index: u8,
        interface: u8,
        size: usize,
    ) -> Result<Descriptor, i32> {
        let mut descriptor = Vec::<u8>::with_capacity(size);
        let ptr = descriptor.spare_capacity_mut();
        let result = unsafe {
            crate::ffi::USB_GetGenericDescriptor(
                self.fd,
                kind,
                index,
                interface,
                ptr.as_mut_ptr().cast(),
                u32::try_from(ptr.len()).unwrap(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        let len = ptr.len();
        unsafe {
            descriptor.set_len(len);
        }
        Ok(Descriptor(descriptor.into_boxed_slice()))
    }

    pub fn get_hid_descriptor(&self, interface: u8) -> Result<HidDescriptor, i32> {
        let hid_descriptor_size: u32 = crate::ffi::USB_DT_HID_SIZE;
        let mut hid_descriptor = MaybeUninit::<crate::ffi::usb_hiddesc>::uninit();
        let result = unsafe {
            crate::ffi::USB_GetHIDDescriptor(
                self.fd,
                interface,
                hid_descriptor.as_mut_ptr(),
                hid_descriptor_size,
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(HidDescriptor(unsafe { hid_descriptor.assume_init() }))
    }

    pub fn get_report_descriptor_size(&self, interface: u8) -> Result<i32, i32> {
        let result = unsafe { crate::ffi::USB_GetReportDescriptorSize(self.fd, interface) };
        if result < i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(result)
    }

    pub fn get_report_descriptor(&self, interface: u8) -> Result<Descriptor, i32> {
        let mut descriptor = Vec::with_capacity(
            usize::try_from(self.get_report_descriptor_size(interface)?).unwrap(),
        );
        let ptr = descriptor.spare_capacity_mut();
        let result = unsafe {
            crate::ffi::USB_GetReportDescriptor(
                self.fd,
                interface,
                ptr.as_mut_ptr().cast(),
                u16::try_from(ptr.len()).unwrap(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        let len = ptr.len();
        unsafe { descriptor.set_len(len) }
        Ok(Descriptor(descriptor.into_boxed_slice()))
    }

    pub fn get_device_description(&self) -> Result<Description, i32> {
        let mut description = MaybeUninit::<crate::ffi::usb_devdesc>::uninit();
        let result = unsafe {
            crate::ffi::USB_GetDeviceDescription(self.fd, description.as_mut_ptr().cast())
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(Description(unsafe { description.assume_init() }))
    }

    //TODO:
    // crate::ffi::USB_DeviceRemovalNotifyAsync
    // crate::ffi::USB_DeviceChangeNotifyAsync

    pub fn suspend(&self) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_SuspendDevice(self.fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    pub fn resume(&self) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_ResumeDevice(self.fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    pub fn read_iso_msg(&self, endpoint: u8, packet_buffers: &mut [&mut Buf32]) -> Result<(), i32> {
        let mut packet_sizes = Vec::with_capacity(packet_buffers.len());
        let mut w_len = 0usize;
        for buffer in packet_buffers.iter() {
            packet_sizes.push(u16::try_from(buffer.len()).unwrap());
            w_len += buffer.len();
        }
        let mut packet_sizes = packet_sizes.into_boxed_slice();

        let mut response = Buf32::new(w_len);
        let packets_len = packet_buffers.len();
        let result = unsafe {
            crate::ffi::USB_ReadIsoMsg(
                self.fd,
                endpoint,
                u8::try_from(packets_len).unwrap(),
                packet_sizes.as_mut_ptr(),
                response.as_mut_ptr().cast(),
            )
        };

        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        
        let mut cursor = 0;
        for (i, buffer) in packet_buffers.iter_mut().enumerate() {
            let length = usize::from(packet_sizes[i]);
            buffer.as_mut_slice()[..length].copy_from_slice(&response.as_slice()[cursor..cursor+length]);
            cursor += length;
        }
        Ok(())
    }

    pub fn read_intr_msg(&self, endpoint: u8, buffer: &mut [u8]) -> Result<(), i32> {
        let mut response = Buf32::new(buffer.len());
        let result = unsafe {
            crate::ffi::USB_ReadIntrMsg(
                self.fd,
                endpoint,
                u16::try_from(response.len()).unwrap(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        buffer.copy_from_slice(response.as_slice());
        Ok(())
    }

    pub fn read_blk_msg(&self, endpoint: u8, buffer: &mut [u8]) -> Result<(), i32> {
        let mut response = Buf32::new(buffer.len());
        let result = unsafe {
            crate::ffi::USB_ReadBlkMsg(
                self.fd,
                endpoint,
                u16::try_from(response.len()).unwrap(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        buffer.copy_from_slice(response.as_slice());
        Ok(())
    }

    pub fn read_ctrl_msg(&self, request_type: u8, request: u8, w_value: u16, w_index: u16, buffer: &mut [u8]) -> Result<(), i32> {
        let mut response = Buf32::new(buffer.len());
        let result = unsafe {
            crate::ffi::USB_ReadCtrlMsg(
                self.fd,
                request_type,
                request,
                w_value,
                w_index,
                u16::try_from(response.len()).unwrap(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        buffer.copy_from_slice(response.as_slice());
        Ok(())
    }

}
