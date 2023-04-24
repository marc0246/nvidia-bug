use ash::vk;
use std::thread;

fn main() {
    unsafe {
        let entry = ash::Entry::load().unwrap();
        let instance = entry
            .create_instance(&vk::InstanceCreateInfo::default(), None)
            .unwrap();
        let physical_device = instance.enumerate_physical_devices().unwrap()[0];
        let device = instance
            .create_device(
                physical_device,
                &vk::DeviceCreateInfo::builder().queue_create_infos(&[vk::DeviceQueueCreateInfo {
                    queue_family_index: 0,
                    queue_count: 1,
                    ..Default::default()
                }]),
                None,
            )
            .unwrap();

        thread::spawn(move || {
            let descriptor_pool = device
                .create_descriptor_pool(
                    &vk::DescriptorPoolCreateInfo::builder()
                        .max_sets(1)
                        .pool_sizes(&[vk::DescriptorPoolSize {
                            ty: vk::DescriptorType::UNIFORM_BUFFER,
                            descriptor_count: 1,
                        }]),
                    None,
                )
                .unwrap();
            device.destroy_descriptor_pool(descriptor_pool, None);
            device.destroy_device(None);
            instance.destroy_instance(None);
            drop(entry);
        });
    }
}
