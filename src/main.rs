fn main() {
    generate_filtered_image("mars".to_string(), 90, 1.0, 5);
    generate_filtered_image("neptune".to_string(), 180, 2.5, 5);
    generate_filtered_image("saturn".to_string(), 270, 5.0, 5);
}

fn generate_filtered_image(filter_name: String, hue_rotate_value: i32, sigma_value: f32, threshold_value: i32) {
    println!("Add filter to image");
    println!("- Open image input/newcastle.png");
    let mut img = image::open("input/newcastle.png").unwrap();

    println!("- Rotate hue/colour palette to {} degrees", hue_rotate_value);
    img = img.huerotate(hue_rotate_value);

    println!("- Use Unsharpen. Signma: {}, Threshold: {}", sigma_value, threshold_value);
    img = img.unsharpen(sigma_value, threshold_value);

    let save_file_path = format!("output/newcastle-{0}-filter.png", filter_name);

    println!("- Save new image as {}", save_file_path);
    img.save_with_format(save_file_path, image::ImageFormat::Png).unwrap();

    println!(" ");
}