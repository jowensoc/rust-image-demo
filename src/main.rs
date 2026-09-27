fn main() {
    create_new_image(90);
    create_new_image(180);
    create_new_image(270);
    create_new_image(310);
}

fn create_new_image(hue_rotate: i32) {
    println!("- Open image");
    let mut img = image::open("input/beach-ball.png").unwrap();

    println!("- Rotate image hue by {}", hue_rotate);
    img = img.huerotate(hue_rotate);

    let save_file_path = format!("output/beach-ball-{}.png", hue_rotate);

    println!("- Save new image as {}", save_file_path);
    img.save_with_format(save_file_path, image::ImageFormat::Png).unwrap();

    println!(" ");
}
