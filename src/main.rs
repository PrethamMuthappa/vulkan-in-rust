use vulkano::device::DeviceOwned;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::device::{Device,DeviceCreateInfo,QueueCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use vulkano::memory::allocator::{MemoryAllocatePreference, StandardMemoryAllocator};
use vulkano::memory::allocator::{AllocationCreateInfo,MemoryUsage};
use vulkano::buffer::{Buffer,BufferCreateInfo,BufferUsage};
use vulkano::command_buffer::allocator::{StandardCommandBufferAlloc, StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder,CommandBufferUsage,CopyBufferInfo};
use vulkano::swapchain::FullScreenExclusive::Default as OtherDefault;
use vulkano::sync::{self,GpuFuture};
use vulkano::buffer::BufferContents;
use core::default::Default;
use std::error::Error;


fn main() {
// we need to include the library
let library=VulkanLibrary::new().expect("no vulkan library");
    //create an instance
    let instance=Instance::new(library,InstanceCreateInfo::default())
        .expect("failed to create instance");

//create a physical device

let physicaldevice=instance.enumerate_physical_devices().expect("device not created").next().expect("no devices");

  // create a quue to check the device index
 let queue_family_index=physicaldevice
     .queue_family_properties()
     .iter()
     .enumerate()
     .position(|(_queue_family_index, queue_family_properties)| {
     queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
 })
     .expect("coudnt find graphic quue") as u32;

//now dreate device

    let (device,mut queues)=Device::new(
        physicaldevice,DeviceCreateInfo {
            queue_create_infos:vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
        .expect("failed to create a device");

    let queue=queues.next().unwrap();

    //now device and queue is both available

    //cretae a memory alloc using stand memory alloc lib
    let memory_allocator=StandardMemoryAllocator::new_default(device.clone());


   let source_content:Vec<i32> = (0..64).collect();

    let source=Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage:BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage:MemoryUsage::Upload,
            ..Default::default()
        },
        source_content,
    )
        .unwrap();

    let dest_content:Vec<i32>=(0..64).map(|_| 0).collect();
    let destination=Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
          usage:BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage:MemoryUsage::Download,
            ..Default::default()
        },
        dest_content,
    )
        .unwrap();

    //creatig a new command buffer, but for than we cant use our normal allocator

     let commandbufferalloc=StandardCommandBufferAllocator::new(device.clone(),StandardCommandBufferAllocatorCreateInfo::default(),);

     let mut builder=AutoCommandBufferBuilder::primary(
         &commandbufferalloc,
         queue_family_index,
         CommandBufferUsage::OneTimeSubmit,
     )
         .unwrap();
    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(),destination.clone()))
        .unwrap();

    let commandbuffer=builder.build().unwrap();

    let future=sync::now(device.clone())
        .then_execute(queue.clone(),commandbuffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();



    let srccontent=source.read().unwrap();
    let destinationcontent=destination.read().unwrap();
    assert_eq!(&*srccontent,&*destinationcontent);

    println!("everything succeeded");


}
