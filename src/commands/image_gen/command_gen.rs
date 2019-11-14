use crate::command_framework::{Command, CommandManager};
use crate::commands::image_gen::print_template_features;
use crate::util::image::ImageStorage;

pub fn register_images(command_manager: &mut CommandManager, images: &ImageStorage) {
    for key in images.get_all_keys() {
        command_manager.register_command(Command {
            key: Box::leak(key.clone().into_boxed_str()),
            description: Box::leak(format!("Generates a new {} image", &key).into_boxed_str()),
            help_page: Box::leak(format!("{}", print_template_features(images, &key)).into_boxed_str()),
            category: "Image",
            func: super::image_gen,
        });
    }
}