use vulkano::device::DeviceOwned;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::device::{Device,DeviceCreateInfo,QueueCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use core::default::Default;


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

    println!("{:?}",queue);

}