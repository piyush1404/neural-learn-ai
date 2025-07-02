use base64::{engine::general_purpose::STANDARD, Engine};
use dioxus::prelude::*;
use opencv::{
    core::{Mat, Rect, Scalar, Vector},
    imgcodecs::{imencode, imread, IMREAD_COLOR},
    imgproc,
    prelude::*,
};
use rfd::FileDialog;
use opencv::core::Size;

const RENDERED_IMG_WIDTH: i32 = 500; // same as width in img tag

#[derive(Clone)]
struct ImageData {
    original_base64: String,
    cropped_base64: String,
    cropped_vec: Vec<u8>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    crop_width: i32,
    crop_height: i32
}

#[derive(Clone, Default)]
struct RectState {
    start_x: i32,
    start_y: i32,
    current_x: i32,
    current_y: i32,
    is_drawing: bool
}


#[component]
pub fn ImageRoi() -> Element {
    let image_data = use_signal(|| None as Option<ImageData>);
    let mat_data = use_signal(|| None as Option<Mat>);
    let rect_state = use_signal(RectState::default);
    let mut color_mode = use_signal(|| "grayscale".to_string());


    let load_image = {
        let mut image_data = image_data.clone();
        let mut mat_data = mat_data.clone();
        let mut rect_state = rect_state.clone();
        move |_| {
            if let Some(file) = FileDialog::new().add_filter("Image", &["jpg", "png"]).pick_file() {
                if let Ok(mat) = imread(file.to_str().unwrap(), IMREAD_COLOR) {
                    let base64 = mat_to_base64(&mat).unwrap();
                    image_data.set(Some(ImageData {
                        original_base64: base64,
                        cropped_base64: "".into(),
                        cropped_vec: vec![],
                        x1: 0,
                        y1: 0,
                        x2: 0,
                        y2: 0,
                        crop_width: 0,      // or: x2 - x1
                        crop_height: 0, // or: y2 - y1
                    }));
                    mat_data.set(Some(mat));
                    rect_state.set(RectState::default());
                }
            }
        }
    };

    let onmousedown = {
        let mut rect_state = rect_state.clone();
        move |evt: Event<MouseData>| {
            let coords = evt.data.element_coordinates();
            let x = coords.x as i32;
            let y = coords.y as i32;
            rect_state.set(RectState {
                is_drawing: true,
                start_x: x,
                start_y: y,
                current_x: x,
                current_y: y,
            });
        }
    };

    let onmousemove = {
        let mut rect_state = rect_state.clone();
        move |evt: Event<MouseData>| {
            if rect_state.read().is_drawing {
                let coords = evt.data.element_coordinates();
                let x = coords.x as i32;
                let y = coords.y as i32;
                let prev = rect_state.read().clone();
                rect_state.set(RectState {
                    is_drawing: true,
                    start_x: prev.start_x,
                    start_y: prev.start_y,
                    current_x: x,
                    current_y: y,
                });
            }
        }
    };

    let onmouseup = {
        let mut rect_state = rect_state.clone();
        let mat_data = mat_data.clone();
        let mut image_data = image_data.clone();
        move |_| {
            let r = rect_state.read().clone();

            if let Some(mat) = &*mat_data.read() {
                let orig_width = mat.cols();
                let orig_height = mat.rows();

                let rendered_img_width = RENDERED_IMG_WIDTH as f64;
                let rendered_img_height = (orig_height as f64 / orig_width as f64 * RENDERED_IMG_WIDTH as f64).round();

                // Calculate scale from rendered image to original image
                let scale_x = orig_width as f64 / rendered_img_width;
                let scale_y = orig_height as f64 / rendered_img_height;

                // Map from rendered to original image coordinates
                let x1 = (r.start_x.min(r.current_x) as f64 * scale_x) as i32;
                let y1 = (r.start_y.min(r.current_y) as f64 * scale_y) as i32;
                let x2 = (r.start_x.max(r.current_x) as f64 * scale_x) as i32;
                let y2 = (r.start_y.max(r.current_y) as f64 * scale_y) as i32;

                let width = x2 - x1;
                let height = y2 - y1;

                if width <= 1 || height <= 1 {
                    println!("Invalid crop area");
                    rect_state.set(RectState::default());
                    return;
                }

                let mut cloned = mat.clone();

                let rect = Rect::new(x1, y1, width, height);
                let _ = imgproc::rectangle(
                    &mut cloned,
                    rect,
                    Scalar::new(0.0, 255.0, 0.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    0,
                );
                if let Ok(cropped_ref) = mat.roi(rect) {
                    if let Ok(cropped) = cropped_ref.try_clone() {
                        let is_grayscale = color_mode.read().as_str() == "grayscale";

                        let mut processed = Mat::default();
                        if is_grayscale {
                            imgproc::cvt_color(
                                &cropped,
                                &mut processed,
                                imgproc::COLOR_BGR2GRAY,
                                0,
                            ).expect("Grayscale conversion failed");
                        } else {
                            processed = cropped.clone(); // RGB (BGR actually in OpenCV)
                        }

                        let target_size = if is_grayscale {
                            Size::new(16, 16) // 256 bytes (1 channel)
                        } else {
                            Size::new(9, 9) // 243 bytes (3 channels)
                        };

                        let mut resized = Mat::default();
                        imgproc::resize(
                            &processed,
                            &mut resized,
                            target_size,
                            0.0,
                            0.0,
                            imgproc::INTER_AREA,
                        ).expect("Resize failed");

                        let cropped_vec: Vec<u8> = if is_grayscale {
                            // Grayscale: Directly get 256 bytes from 16x16
                            resized.data_bytes().unwrap().to_vec()
                        } else {
                            // RGB: Resize to 9x9 and split into R, G, B channels
                            let mut resized_rgb = Mat::default();
                            imgproc::resize(
                                &processed,
                                &mut resized_rgb,
                                Size::new(9, 9),
                                0.0,
                                0.0,
                                imgproc::INTER_AREA,
                            ).expect("Resize failed");

                            let mut channels = opencv::core::Vector::<Mat>::new();
                            opencv::core::split(&resized_rgb, &mut channels).expect("Split failed");

                            let r = channels.get(2).unwrap(); // OpenCV uses BGR format
                            let g = channels.get(1).unwrap();
                            let b = channels.get(0).unwrap();

                            let r_data = r.data_bytes().unwrap();
                            let g_data = g.data_bytes().unwrap();
                            let b_data = b.data_bytes().unwrap();

                            println!("🎨 R Channel (9x9):");
                            for i in 0..9 {
                                for j in 0..9 {
                                    print!("{:3} ", r_data[i * 9 + j]);
                                }
                                println!();
                            }

                            println!("🎨 G Channel (9x9):");
                            for i in 0..9 {
                                for j in 0..9 {
                                    print!("{:3} ", g_data[i * 9 + j]);
                                }
                                println!();
                            }

                            println!("🎨 B Channel (9x9):");
                            for i in 0..9 {
                                for j in 0..9 {
                                    print!("{:3} ", b_data[i * 9 + j]);
                                }
                                println!();
                            }

                            println!("🔗 Combined RGB (9x9):");
                            for i in 0..9 {
                                for j in 0..9 {
                                    let idx = i * 9 + j;
                                    print!("[{:3},{:3},{:3}] ", r_data[idx], g_data[idx], b_data[idx]);
                                }
                                println!();
                            }

                            // Interleave RGB data into final 9x9*3 = 243 byte vec: [R,G,B,R,G,B,...]
                            let mut rgb_combined = Vec::with_capacity(243);
                            for i in 0..(9 * 9) {
                                rgb_combined.push(r_data[i]);
                                rgb_combined.push(g_data[i]);
                                rgb_combined.push(b_data[i]);
                            }
                            rgb_combined
                        };
                        if is_grayscale {
                            // ✅ Add this here to print all 256 grayscale values
                            for row in 0..resized.rows() {
                                for col in 0..resized.cols() {
                                    let val = resized.at_2d::<u8>(row, col).unwrap();
                                    print!("{:3} ", val);
                                }
                                println!();
                            }
                        }
                        println!("✅ Final cropped_vec size: {}", cropped_vec.len());

                        // Optional base64 encoding for preview
                        let cropped_b64 = mat_to_base64(&resized).unwrap_or_default();
                        let updated_original_b64 = mat_to_base64(&cloned).unwrap_or_default();
                        image_data.set(Some(ImageData {
                            original_base64: updated_original_b64,
                            cropped_base64: cropped_b64,
                            cropped_vec,
                            x1,
                            y1,
                            x2,
                            y2,
                            crop_width: width,
                            crop_height: height
                        }));
                    }
                }


            }

            rect_state.set(RectState::default());
        }
    };

    // Compute rectangle for drawing (must be outside rsx!)
    let (rendered_img_width, rendered_img_height) = if let Some(mat) = mat_data.read().as_ref() {
        let orig_width = mat.cols();
        let orig_height = mat.rows();
        let width = RENDERED_IMG_WIDTH;
        let height = ((orig_height as f64 / orig_width as f64) * width as f64).round() as i32;
        (width, height)
    } else {
        (RENDERED_IMG_WIDTH, RENDERED_IMG_WIDTH)
    };

    // Rectangle for overlay
    let is_drawing = rect_state.read().is_drawing;
    let (x, y, w, h) = if is_drawing {
        (
            rect_state.read().start_x.min(rect_state.read().current_x),
            rect_state.read().start_y.min(rect_state.read().current_y),
            (rect_state.read().start_x - rect_state.read().current_x).abs(),
            (rect_state.read().start_y - rect_state.read().current_y).abs(),
        )
    } else {
        (0, 0, 0, 0)
    };

    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif;",
            h1 { "🖼️ Rust + Dioxus Image Cropper" }
            
            button {
                onclick: load_image,
                style: "margin-bottom: 20px; padding: 10px 20px;",
                "Upload Image"
            }

            label { "Color Mode: " }
            select {
                onchange: move |evt| {
                    color_mode.set(evt.value().to_string());
                },
                option { value: "grayscale", selected: true, "Grayscale" }
                option { value: "rgb", "RGB" }
            }

            if let Some(img) = image_data.read().as_ref() {
                div { style: "display: flex; gap: 30px;",
                    div {
                        h3 { "Original Image" }
                        div {
                            style: "position: relative; width: {rendered_img_width}px; height: {rendered_img_height}px;",
                            img {
                                src: "data:image/jpeg;base64,{img.original_base64}",
                                style: "width: 100%; height: 100%; border: 1px solid gray; user-select: none; cursor: crosshair;",
                                onmousedown: onmousedown,
                                onmousemove: onmousemove,
                                onmouseup: onmouseup,
                                draggable: false
                            }
                            if is_drawing {
                                div {
                                    style: "position: absolute; left: {x}px; top: {y}px; width: {w}px; height: {h}px;
                                            border: 2px dashed red; pointer-events: none;"
                                }
                            }
                        }
                    }

                    div {
                        h3 { "📐 Cropped Image" }
                        if !img.cropped_base64.is_empty() {
                            img {
                                style: "max-width: 300px; border: 1px solid black;",
                                src: "data:image/jpeg;base64,{img.cropped_base64}"
                            }
                            p { "Coordinates:" }
                            ul {
                                li { "x1: {img.x1}" }
                                li { "y1: {img.y1}" }
                                li { "x2: {img.x2}" }
                                li { "y2: {img.y2}" }
                                li { "Width: {img.crop_width}" }
                                li { "Height: {img.crop_height}" }
                            }
                            // p { "Cropped Vec<u8>:" }
                            pre { "{color_mode.read().as_str()}" }
                            div {
                                if color_mode.read().as_str() == "grayscale" { h3 { "Cropped Image Preview (16x16 Grayscale)" } } else { h3 { "Cropped Image Preview (9x9 RGB)" } }
                                img {
                                    src: "data:image/png;base64,{img.cropped_base64}",
                                    style: "image-rendering: pixelated; width:128px; height:128px; border:1px solid black;"
                                }
                                p { "Raw bytes: {img.cropped_vec.len()} bytes" }
                            }
                        } else {
                            p { "⬅️ Draw rectangle on image to crop." }
                        }
                    }                    
                }                
            }
            p {
                "Mouse rect: ({rect_state.read().start_x}, {rect_state.read().start_y}) to ({rect_state.read().current_x}, {rect_state.read().current_y})"
            }
            div {
                class: "bg-testcolor text-white p-4 rounded",
                "✅ Tailwind CSS is WORKING"
            }
        }
    }
}
fn mat_to_base64(mat: &Mat) -> opencv::Result<String> {
    let mut buf = Vector::new();
    imencode(".jpg", mat, &mut buf, &Vector::new())?;
    Ok(STANDARD.encode(&buf.to_vec()))
}
