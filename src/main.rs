use vulkano::device::DeviceOwned;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::device::{Device,DeviceCreateInfo,QueueCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use vulkano::memory::allocator::{MemoryAllocatePreference, StandardMemoryAllocator};
use vulkano::memory::allocator::{AllocationCreateInfo,MemoryUsage};
use vulkano::image::{ImageDimensions,StorageImage};
use vulkano::format::{ClearColorValue, Format};
use vulkano::buffer::{Buffer,BufferCreateInfo,BufferUsage};
use vulkano::command_buffer::allocator::{StandardCommandBufferAlloc, StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyBufferInfo};
use vulkano::command_buffer::CopyImageToBufferInfo;
use vulkano::swapchain::FullScreenExclusive::Default as OtherDefault;
use vulkano::sync::{self,GpuFuture};
use vulkano::buffer::BufferContents;
use core::default::Default;
use std::error::Error;
use image::{ImageBuffer,Rgba};



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


    //creatig a new command buffer, but for than we cant use our normal allocator

     let commandbufferalloc=StandardCommandBufferAllocator::new(device.clone(),StandardCommandBufferAllocatorCreateInfo::default(),);



       let image =StorageImage::new(
           &memory_allocator,
           ImageDimensions::Dim2d {
               width:1024,
               height:1024,
               array_layers:1,
           },
           Format::R8G8B8A8_UNORM,
           Some(queue.queue_family_index()),

       )
           .unwrap();

       let buf=Buffer::from_iter(
           &memory_allocator,
           BufferCreateInfo{
               usage:BufferUsage::TRANSFER_DST,
               ..Default::default()
           },
           AllocationCreateInfo {
               usage:MemoryUsage::Download,
               ..Default::default()
           },
           (0..1024 * 1024 * 4).map(|_| 0u8),

       )
           .expect("failed");

    let mut builder=AutoCommandBufferBuilder::primary(
        &commandbufferalloc,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
        .unwrap();

    builder.clear_color_image(ClearColorImageInfo {
        clear_value:ClearColorValue::Float([0.0,0.0,1.0,1.0]),
        ..ClearColorImageInfo::image(image.clone())
    })
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buf.clone(),
        ))
        .unwrap();
let commandbuffer=builder.build().unwrap();
    let future=sync::now(device.clone())
        .then_execute(queue.clone(),commandbuffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let buffercontent=buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffercontent[..]).unwrap();
    image.save("image.png").unwrap();

    println!("everything is done");
}