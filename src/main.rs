fn main() {
    //rotate_images();

    add_filter();
}

fn rotate_images() {
    println!("Rotate image hue by 90, 180, 270 and 310 degrees");
    rotate(90);
    rotate(180);
    rotate(270);
    rotate(310);
}

fn rotate(hue_rotate: i32) {
    println!("- Open image");
    let mut img = image::open("input/beach-ball.png").unwrap();

    println!("- Rotate image hue by {}", hue_rotate);
    img = img.huerotate(hue_rotate);

    let save_file_path = format!("output/hue-rotate/beach-ball-{}.png", hue_rotate);

    println!("- Save new image as {}", save_file_path);
    img.save_with_format(save_file_path, image::ImageFormat::Png).unwrap();

    println!(" ");
}

fn add_filter() {
    println!("Add filter to image");
    println!("- Open image");
    let mut img = image::open("input/newcastle.png").unwrap();

    println!("- Add filter");
    img = img.filter3x3(&[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]);

    let save_file_path = "output/filter/newcastle-filter.png";

    println!("- Save new image as {}", save_file_path);
    img.save_with_format(save_file_path, image::ImageFormat::Png).unwrap();

    println!(" ");
}