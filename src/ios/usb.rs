use core::mem::MaybeUninit;

use alloc::{boxed::Box, vec::Vec};

use crate::Buf32;

/// USB System marker
///
/// Keep this around otherwise `USB_Deinitialize` is called
pub struct Usb(());

/// USB Device
///
/// Keep this around otherwise `USB_CloseDevice` is called
#[derive(Clone)]
pub struct Device<'a> {
    usb: &'a Usb,
    fd: i32,
}

/// USB device descriptor
///
/// Keep a hold of thise otherwise `USB_FreeDescriptors` is called
pub struct DeviceDescriptor(pub crate::ffi::usb_devdesc);

/// USB generic descriptor
pub struct Descriptor(Box<[u8]>);

/// USB HID descriptor
pub struct HidDescriptor(pub crate::ffi::usb_hiddesc);

/// USB device description
pub struct Description(pub crate::ffi::usb_devdesc);

/// USB device entry
///
/// This is returned from `Usb::get_devices`
pub struct DeviceEntry(pub crate::ffi::usb_device_entry);

impl Usb {
    /// Initialize Usb systems.
    pub fn new() -> Result<Self, i32> {
        let result = unsafe { crate::ffi::USB_Initialize() };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(Self(()))
    }
    /// Open new USB device
    pub fn open_device(&self, device_id: i32, vid: u16, pid: u16) -> Result<Device<'_>, i32> {
        let mut fd = i32::MAX;
        let result = unsafe { crate::ffi::USB_OpenDevice(device_id, vid, pid, &mut fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(Device { usb: self, fd: fd })
    }

    /// Get currently connected usb devices in the device class
    pub fn get_devices(&self, interface_class: u8) -> Result<Box<[DeviceEntry]>, i32> {
        let mut devices = Vec::with_capacity(8);
        let mut response = Buf32::new(core::mem::size_of::<crate::ffi::usb_device_entry>() * 8);
        let mut device_count = 0u8;
        let result = unsafe {
            crate::ffi::USB_GetDeviceList(
                response.as_mut_ptr().cast(),
                8,
                interface_class,
                &mut device_count,
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }

        for buf in response
            .as_slice()
            .chunks_exact(core::mem::size_of::<crate::ffi::usb_device_entry>())
            .take(device_count.into())
        {
            // Cast underyling bytes back into struct
            devices.push(DeviceEntry(unsafe {
                buf.as_ptr().cast::<crate::ffi::usb_device_entry>().read()
            }))
        }
        Ok(devices.into_boxed_slice())
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
    /// Get associated usb device descriptor
    pub fn get_device_descriptor(&self) -> Result<DeviceDescriptor, i32> {
        let mut descriptor = MaybeUninit::<crate::ffi::usb_devdesc>::uninit();
        let result = unsafe { crate::ffi::USB_GetDescriptors(self.fd, descriptor.as_mut_ptr()) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(DeviceDescriptor(unsafe { descriptor.assume_init() }))
    }

    /// Get generic descriptor
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

    /// Get HID descriptor
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

    /// Get report descriptor size.
    ///
    /// Recommend just using `get_report_descriptor`
    pub fn get_report_descriptor_size(&self, interface: u8) -> Result<i32, i32> {
        let result = unsafe { crate::ffi::USB_GetReportDescriptorSize(self.fd, interface) };
        if result < i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(result)
    }

    /// Get report descriptor
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

    /// Get associated usb device description
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
    // TODO:
    // Needs more thought and less generics
    //
    // pub fn device_removal_notify_async<Func, Args, Ret>(&self, func: Func, args: Args) -> Result<(), i32> where Func: FnOnce(Args) -> Ret {
    //     struct Userdata<Func, Args, Ret> where Func: FnOnce(Args) -> Ret {
    //         func: Func,
    //         args: Args,
    //         ret: (MaybeUninit<Ret>, bool)
    //     }
    //     let data = Userdata { func, args, ret: (MaybeUninit::uninit(), false) };
    //     unsafe extern "C" fn thunk<Func, Args, Ret>(_fd: i32, userdata: *mut core::ffi::c_void) -> i32 where Func: FnOnce(Args) -> Ret {
    //         let Userdata { func, args, mut ret } = unsafe { userdata.cast::<Userdata<Func, Args, Ret>>().read() };
    //         ret.0.write(func(args));
    //         ret.1 = true;
    //         return crate::ffi::USB_OK as i32;
    //
    //     }
    //
    //     let result = unsafe { crate::ffi::USB_DeviceRemovalNotifyAsync(self.fd, Some(thunk::<Func, Args, Ret>), Box::into_raw(Box::new(data)).cast()) };
    //     if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
    //         return Err(result);
    //     }
    //     Ok(())
    // }

    /// Suspend associated usb device
    pub fn suspend(&self) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_SuspendDevice(self.fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    /// Resume associated usb device
    pub fn resume(&self) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_ResumeDevice(self.fd) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    /// Read isochronous message from endpoint writing the data back into the packet buffers
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
            buffer.as_mut_slice()[..length]
                .copy_from_slice(&response.as_slice()[cursor..cursor + length]);
            cursor += length;
        }
        Ok(())
    }

    /// Write isochronous message using the data from packet_buffers
    pub fn write_ios_msg(&mut self, endpoint: u8, packet_buffers: &[&Buf32]) -> Result<Buf32, i32> {
        let mut packet_sizes = Vec::with_capacity(packet_buffers.len());
        let mut w_len = 0usize;
        for buffer in packet_buffers {
            packet_sizes.push(u16::try_from(buffer.len()).unwrap());
            w_len += buffer.len();
        }

        let mut response = Buf32::new(w_len);
        let result = unsafe {
            crate::ffi::USB_WriteIsoMsg(
                self.fd,
                endpoint,
                u8::try_from(packet_buffers.len()).unwrap(),
                packet_sizes.as_ptr().cast_mut(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(response)
    }

    /// Read interrupt message writing data into buffer
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

    /// Write interrupt message to endpoint using data from buffer
    pub fn write_intr_msg(&mut self, endpoint: u8, buffer: &[u8]) -> Result<Buf32, i32> {
        let mut response = Buf32::new(buffer.len());
        response.copy_from_slice(buffer);
        let result = unsafe {
            crate::ffi::USB_WriteIntrMsg(
                self.fd,
                endpoint,
                u16::try_from(buffer.len()).unwrap(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(response)
    }

    /// Read bulk message writing data into buffer
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

    /// Write bulk message to endpoint using data from buffer
    pub fn write_blk_msg(&mut self, endpoint: u8, buffer: &[u8]) -> Result<Buf32, i32> {
        let mut response = Buf32::new(buffer.len());
        response.copy_from_slice(buffer);
        let result = unsafe {
            crate::ffi::USB_WriteBlkMsg(
                self.fd,
                endpoint,
                u16::try_from(buffer.len()).unwrap(),
                response.as_mut_ptr().cast(),
            )
        };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(response)
    }

    /// Read ctrl message writing data into buffer
    pub fn read_ctrl_msg(
        &self,
        request_type: u8,
        request: u8,
        w_value: u16,
        w_index: u16,
        buffer: &mut [u8],
    ) -> Result<(), i32> {
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

    /// Write bulk message to endpoint using data from buffer
    pub fn write_ctrl_msg(
        &mut self,
        request_type: u8,
        request: u8,
        w_value: u16,
        w_index: u16,
        buffer: &[u8],
    ) -> Result<Buf32, i32> {
        let mut response = Buf32::new(buffer.len());
        response.copy_from_slice(buffer);
        let result = unsafe {
            crate::ffi::USB_WriteCtrlMsg(
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
        Ok(response)
    }

    /// Get USB configuration
    pub fn get_configuration(&self) -> Result<u8, i32> {
        let mut configuration: MaybeUninit<u8> = MaybeUninit::uninit();
        let result =
            unsafe { crate::ffi::USB_GetConfiguration(self.fd, configuration.as_mut_ptr().cast()) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(unsafe { configuration.assume_init() })
    }

    /// Set USB configuration
    pub fn set_configuration(&mut self, conf: u8) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_SetConfiguration(self.fd, conf) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    /// Clear halt on endpoint
    pub fn clear_halt(&mut self, endpoint: u8) -> Result<(), i32> {
        let result = unsafe { crate::ffi::USB_ClearHalt(self.fd, endpoint) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    /// Set alternative setting on interface
    pub fn set_alt_interface(&mut self, interface: u8, alt_setting: u8) -> Result<(), i32> {
        let result =
            unsafe { crate::ffi::USB_SetAlternativeInterface(self.fd, interface, alt_setting) };
        if result != i32::try_from(crate::ffi::USB_OK).unwrap() {
            return Err(result);
        }
        Ok(())
    }

    /// Get associated string descriptor and string
    pub fn get_ascii_string(
        &self,
        index: u8,
        w_lang_id: u16,
        buffer: &mut [u8],
    ) -> Result<(), i32> {
        let mut response = Buf32::new(buffer.len());
        let result = unsafe {
            crate::ffi::USB_GetAsciiString(
                self.fd,
                index,
                w_lang_id,
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
