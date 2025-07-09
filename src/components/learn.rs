use dioxus::prelude::*;
use opencv::{
    core::{Mat, Rect, Scalar, Vector},
    imgcodecs::{imencode, imread, IMREAD_COLOR},
    imgproc,
    prelude::*,
};
use opencv::core::AlgorithmHint;
use rfd::FileDialog;
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::views::project_details::AppState;
const RENDERED_IMG_SIZE: i32 = 228;

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
    crop_height: i32,
}

#[derive(Clone, Default)]
struct RectState {
    start_x: i32,
    start_y: i32,
    current_x: i32,
    current_y: i32,
    is_drawing: bool,
}

#[component]
pub fn Learn(app_state: AppState) -> Element {
    let mut shared = app_state.shared;
    println!("state: {:?}", shared().algorithm);
    println!("state: {:?}", shared().clear_clicked);
    // let clear_clicked = shared().clear_clicked;
    let mut image_data = use_signal(|| None as Option<ImageData>);
    use_effect(move || {
        if(shared().clear_clicked) {
            println!("Clearing image data");
            image_data.set(None);
            shared.write().clear_clicked = false;
        }
    });

    let mat_data = use_signal(|| None as Option<Mat>);
    let rect_state = use_signal(RectState::default);
    let mut color_mode = shared().algorithm;

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
                        crop_width: 0,
                        crop_height: 0,
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
        let mut color_mode = color_mode.clone();
        move |_| {
            let r = rect_state.read().clone();

            if let Some(mat) = &*mat_data.read() {
                let orig_width = mat.cols();
                let orig_height = mat.rows();

                let scale_x = orig_width as f64 / RENDERED_IMG_SIZE as f64;
                let scale_y = orig_height as f64 / RENDERED_IMG_SIZE as f64;

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
                        let is_grayscale = color_mode == "grayscale";

                        let mut processed = Mat::default();
                        if is_grayscale {
                            imgproc::cvt_color(  &cropped,
                                &mut processed,
                                imgproc::COLOR_BGR2GRAY,
                                0,
                                AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
                        } else {
                            processed = cropped.clone();
                        }

                        let target_size = if is_grayscale {
                            opencv::core::Size::new(16, 16)
                        } else {
                            opencv::core::Size::new(9, 9)
                        };

                        let mut resized = Mat::default();
                        imgproc::resize(&processed, &mut resized, target_size, 0.0, 0.0, imgproc::INTER_AREA).unwrap();

                        let cropped_vec: Vec<u8> = if is_grayscale {
                            resized.data_bytes().unwrap().to_vec()
                        } else {
                            let mut channels = opencv::core::Vector::<Mat>::new();
                            opencv::core::split(&resized, &mut channels).unwrap();
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
                            let mut combined = Vec::with_capacity(243);
                            for i in 0..(9 * 9) {
                                combined.push(channels.get(2).unwrap().data_bytes().unwrap()[i]);
                                combined.push(channels.get(1).unwrap().data_bytes().unwrap()[i]);
                                combined.push(channels.get(0).unwrap().data_bytes().unwrap()[i]);
                            }
                            combined
                        };
                        // ✅ Add this here to print all 256 grayscale values
                        if color_mode == "grayscale" {
                            for row in 0..resized.rows() {
                                for col in 0..resized.cols() {
                                    let val = resized.at_2d::<u8>(row, col).unwrap();
                                    print!("{:3} ", val);
                                }
                                println!();
                            }
                        }
                        
                        println!("✅ Final cropped_vec size: {}", cropped_vec.len());
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
                            crop_height: height,
                        }));
                    }
                }
            }

            rect_state.set(RectState::default());
        }
    };

 // Compute the overlay rectangle once, *outside* of rsx!
 let (overlay_x, overlay_y, overlay_w, overlay_h) = if rect_state.read().is_drawing {
     let r = rect_state.read();
     let x = r.start_x.min(r.current_x);
     let y = r.start_y.min(r.current_y);
     let w = (r.current_x - r.start_x).abs();
     let h = (r.current_y - r.start_y).abs();
     (x, y, w, h)
 } else {
     (0, 0, 0, 0)
 };
 let grayscale_matrix: Option<VNode> = if color_mode == "grayscale" {
    if let Some(img) = image_data.read().as_ref() {
        if img.cropped_vec.len() == 256 {
            let mut cells: Vec<VNode> = Vec::new();

            for row in 0..16 {
                for col in 0..16 {
                    let idx = row * 16 + col;
                    let val = img.cropped_vec[idx];
                    let bg = format!("background-color: rgb({0},{0},{0});", val);
                    cells.push(rsx! {
                        div {
                            class: "w-4 h-4 flex items-center justify-center text-[7px] text-white",
                            style: "{bg}",
                            "{val}"
                        }
                    }.unwrap());
                }
            }

            Some(rsx! {
                div {
                    class: "mt-4",
                    h3 { "🧊 Grayscale Matrix (16×16)" }
                    div {
                        style: "display: grid; grid-template-columns: repeat(16, 1fr); gap: 1px;",
                        // ✅ The critical fix:
                        {cells.into_iter()}
                    }
                }
            }.unwrap())
        } else {
            None
        }
    } else {
        None
    }
} else {
    None
};




    rsx! {
        div {
            class:"flex h-1/2 mx-6 mt-5",
            div {
                class:"w-3/4 border-t border-b border-l border-[#BEBEBE] bg-[#FFFFFF] rounded-tl-[10px] rounded-bl-[10px]",
                div {
                    class:"flex p-2",
                    span {
                        class:"flex justify-start items-center font-normal text-sm text-[#404040]",
                        "Learn"
                    }
                    span {
                        class:"w-full flex justify-end items-center gap-2",
                        div {
                            class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",

                            div {
                                class: "w-4 h-4 bg-[#CACACA] flex items-center justify-center",
                                span {
                                    class:"h-[15px] w-[15px] flex items-center justify-center",
                                    svg {
                                        width: "12",
                                        height: "12",
                                        view_box: "0 0 12 12",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            d: "M11.5313 5.53115C11.8272 5.23147 11.9931 4.82728 11.9931 4.40615C11.9931 3.98501 11.8272 3.58083 11.5313 3.28115L8.7301 0.468649C8.42592 0.181466 8.02343 0.0214844 7.6051 0.0214844C7.18677 0.0214844 6.78428 0.181466 6.4801 0.468649L0.480097 6.46865C0.184218 6.76833 0.0183105 7.17251 0.0183105 7.59365C0.0183105 8.01478 0.184218 8.41897 0.480097 8.71865L3.28135 11.5199C3.58103 11.8158 3.98521 11.9817 4.40635 11.9817C4.82748 11.9817 5.23166 11.8158 5.53135 11.5199L11.5313 5.53115ZM7.03135 1.03115C7.18095 0.882757 7.38313 0.799492 7.59385 0.799492C7.80456 0.799492 8.00674 0.882757 8.15635 1.03115L10.9576 3.8324C11.106 3.982 11.1893 4.18418 11.1893 4.3949C11.1893 4.60562 11.106 4.8078 10.9576 4.9574L6.8326 9.0824L2.9026 5.16365L7.03135 1.03115ZM4.9651 10.9649C4.81197 11.1064 4.61111 11.185 4.4026 11.185C4.19408 11.185 3.99323 11.1064 3.8401 10.9649L1.0351 8.16365C0.886706 8.01405 0.803441 7.81187 0.803441 7.60115C0.803441 7.39043 0.886706 7.18825 1.0351 7.03865L2.33635 5.7374L6.2701 9.66365L4.9651 10.9649Z",
                                            fill: "white"
                                        }
                                        path {
                                            d: "M11.5988 11.2011H7.20001C7.14431 11.1957 7.08809 11.202 7.03498 11.2196C6.98186 11.2372 6.93302 11.2657 6.89158 11.3034C6.85014 11.341 6.81703 11.3868 6.79437 11.438C6.77171 11.4892 6.76001 11.5445 6.76001 11.6005C6.76001 11.6564 6.77171 11.7118 6.79437 11.7629C6.81703 11.8141 6.85014 11.86 6.89158 11.8976C6.93302 11.9352 6.98186 11.9637 7.03498 11.9813C7.08809 11.9989 7.14431 12.0052 7.20001 11.9998H11.5988C11.6545 12.0052 11.7107 11.9989 11.7638 11.9813C11.8169 11.9637 11.8657 11.9352 11.9072 11.8976C11.9486 11.86 11.9817 11.8141 12.0044 11.7629C12.027 11.7118 12.0388 11.6564 12.0388 11.6005C12.0388 11.5445 12.027 11.4892 12.0044 11.438C11.9817 11.3868 11.9486 11.341 11.9072 11.3034C11.8657 11.2657 11.8169 11.2372 11.7638 11.2196C11.7107 11.202 11.6545 11.1957 11.5988 11.2011Z",
                                            fill: "white"
                                        }
                                    }

                                }
                            }

                            button {
                                onclick: load_image,
                                class: "ml-2 outline-none border-none bg-transparent font-normal text-[11px] text-[#151515]",
                                "Upload"
                            }
                        }
                        div {
                            class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",

                            div {
                                class: "w-4 h-4 bg-[#CACACA] flex items-center justify-center",
                                span {
                                    class:"h-[15px] w-[15px] flex items-center justify-center",
                                    svg {
                                        width: "12",
                                        height: "12",
                                        view_box: "0 0 12 12",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            d: "M11.5313 5.53115C11.8272 5.23147 11.9931 4.82728 11.9931 4.40615C11.9931 3.98501 11.8272 3.58083 11.5313 3.28115L8.7301 0.468649C8.42592 0.181466 8.02343 0.0214844 7.6051 0.0214844C7.18677 0.0214844 6.78428 0.181466 6.4801 0.468649L0.480097 6.46865C0.184218 6.76833 0.0183105 7.17251 0.0183105 7.59365C0.0183105 8.01478 0.184218 8.41897 0.480097 8.71865L3.28135 11.5199C3.58103 11.8158 3.98521 11.9817 4.40635 11.9817C4.82748 11.9817 5.23166 11.8158 5.53135 11.5199L11.5313 5.53115ZM7.03135 1.03115C7.18095 0.882757 7.38313 0.799492 7.59385 0.799492C7.80456 0.799492 8.00674 0.882757 8.15635 1.03115L10.9576 3.8324C11.106 3.982 11.1893 4.18418 11.1893 4.3949C11.1893 4.60562 11.106 4.8078 10.9576 4.9574L6.8326 9.0824L2.9026 5.16365L7.03135 1.03115ZM4.9651 10.9649C4.81197 11.1064 4.61111 11.185 4.4026 11.185C4.19408 11.185 3.99323 11.1064 3.8401 10.9649L1.0351 8.16365C0.886706 8.01405 0.803441 7.81187 0.803441 7.60115C0.803441 7.39043 0.886706 7.18825 1.0351 7.03865L2.33635 5.7374L6.2701 9.66365L4.9651 10.9649Z",
                                            fill: "white"
                                        }
                                        path {
                                            d: "M11.5988 11.2011H7.20001C7.14431 11.1957 7.08809 11.202 7.03498 11.2196C6.98186 11.2372 6.93302 11.2657 6.89158 11.3034C6.85014 11.341 6.81703 11.3868 6.79437 11.438C6.77171 11.4892 6.76001 11.5445 6.76001 11.6005C6.76001 11.6564 6.77171 11.7118 6.79437 11.7629C6.81703 11.8141 6.85014 11.86 6.89158 11.8976C6.93302 11.9352 6.98186 11.9637 7.03498 11.9813C7.08809 11.9989 7.14431 12.0052 7.20001 11.9998H11.5988C11.6545 12.0052 11.7107 11.9989 11.7638 11.9813C11.8169 11.9637 11.8657 11.9352 11.9072 11.8976C11.9486 11.86 11.9817 11.8141 12.0044 11.7629C12.027 11.7118 12.0388 11.6564 12.0388 11.6005C12.0388 11.5445 12.027 11.4892 12.0044 11.438C11.9817 11.3868 11.9486 11.341 11.9072 11.3034C11.8657 11.2657 11.8169 11.2372 11.7638 11.2196C11.7107 11.202 11.6545 11.1957 11.5988 11.2011Z",
                                            fill: "white"
                                        }
                                    }

                                }
                            }

                            button {
                                class: "ml-2 outline-none border-none bg-transparent font-normal text-[11px] text-[#151515]",
                                "Learn"
                            }
                        }
                        div {
                            class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",

                            div {
                                class: "w-4 h-4 bg-[#CACACA] flex items-center justify-center",
                                span {
                                    class:"h-[15px] w-[15px] flex items-center justify-center",
                                    svg {
                                        width: "12",
                                        height: "12",
                                        view_box: "0 0 12 12",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            d: "M11.5313 5.53115C11.8272 5.23147 11.9931 4.82728 11.9931 4.40615C11.9931 3.98501 11.8272 3.58083 11.5313 3.28115L8.7301 0.468649C8.42592 0.181466 8.02343 0.0214844 7.6051 0.0214844C7.18677 0.0214844 6.78428 0.181466 6.4801 0.468649L0.480097 6.46865C0.184218 6.76833 0.0183105 7.17251 0.0183105 7.59365C0.0183105 8.01478 0.184218 8.41897 0.480097 8.71865L3.28135 11.5199C3.58103 11.8158 3.98521 11.9817 4.40635 11.9817C4.82748 11.9817 5.23166 11.8158 5.53135 11.5199L11.5313 5.53115ZM7.03135 1.03115C7.18095 0.882757 7.38313 0.799492 7.59385 0.799492C7.80456 0.799492 8.00674 0.882757 8.15635 1.03115L10.9576 3.8324C11.106 3.982 11.1893 4.18418 11.1893 4.3949C11.1893 4.60562 11.106 4.8078 10.9576 4.9574L6.8326 9.0824L2.9026 5.16365L7.03135 1.03115ZM4.9651 10.9649C4.81197 11.1064 4.61111 11.185 4.4026 11.185C4.19408 11.185 3.99323 11.1064 3.8401 10.9649L1.0351 8.16365C0.886706 8.01405 0.803441 7.81187 0.803441 7.60115C0.803441 7.39043 0.886706 7.18825 1.0351 7.03865L2.33635 5.7374L6.2701 9.66365L4.9651 10.9649Z",
                                            fill: "white"
                                        }
                                        path {
                                            d: "M11.5988 11.2011H7.20001C7.14431 11.1957 7.08809 11.202 7.03498 11.2196C6.98186 11.2372 6.93302 11.2657 6.89158 11.3034C6.85014 11.341 6.81703 11.3868 6.79437 11.438C6.77171 11.4892 6.76001 11.5445 6.76001 11.6005C6.76001 11.6564 6.77171 11.7118 6.79437 11.7629C6.81703 11.8141 6.85014 11.86 6.89158 11.8976C6.93302 11.9352 6.98186 11.9637 7.03498 11.9813C7.08809 11.9989 7.14431 12.0052 7.20001 11.9998H11.5988C11.6545 12.0052 11.7107 11.9989 11.7638 11.9813C11.8169 11.9637 11.8657 11.9352 11.9072 11.8976C11.9486 11.86 11.9817 11.8141 12.0044 11.7629C12.027 11.7118 12.0388 11.6564 12.0388 11.6005C12.0388 11.5445 12.027 11.4892 12.0044 11.438C11.9817 11.3868 11.9486 11.341 11.9072 11.3034C11.8657 11.2657 11.8169 11.2372 11.7638 11.2196C11.7107 11.202 11.6545 11.1957 11.5988 11.2011Z",
                                            fill: "white"
                                        }
                                    }

                                }
                            }

                            button {
                                class: "ml-2 outline-none border-none bg-transparent font-normal text-[11px] text-[#151515]",
                                "Reset"
                            }
                        }
                    }
                }

                hr { class: "border-t-[0.5px] border-[#BEBEBE] my-1 w-full" }

                if let Some(img) = image_data.read().as_ref() {
    div {
        // ─────────────── Left column: Original Image ───────────────
        div {
            // class:"flex",
            class: "flex justify-center items-center",
            // h3 { "Original Image" }
            // this wrapper both centers the image and acts as position:relative
            div {
                class: "flex justify-center items-center",
                style: format!(
                    "position: relative; width: {}px; height: {}px;",
                    RENDERED_IMG_SIZE, RENDERED_IMG_SIZE
                ),
                // the image itself
                img {
                    src: "data:image/jpeg;base64,{img.original_base64}",
                    style: "width: {RENDERED_IMG_SIZE}px; height: {RENDERED_IMG_SIZE}px; user-select: none; cursor: crosshair;",
                    onmousedown: onmousedown,
                    onmousemove: onmousemove,
                    onmouseup: onmouseup,
                    draggable: false
                }
                // red-dashed preview while dragging
                                     // Now use the precomputed overlay_x, _y, _w, _h
                     if rect_state.read().is_drawing {
                         div {
                             style: format!(
                                 "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;\
                                 border: 2px dashed red; pointer-events: none;",
                                 overlay_x, overlay_y, overlay_w, overlay_h
                             )
                         }
                     }
                     
            }
            
        }
        // div { 
        //     {grayscale_matrix}
        //  }

        // ───────────── Right column: Cropped Image ─────────────
        // div {
        //     h3 { "📐 Cropped Image" }
        //     if !img.cropped_base64.is_empty() {
        //         img {
        //             style: "max-width: 300px; border: 1px solid black;",
        //             src: "data:image/jpeg;base64,{img.cropped_base64}"
        //         }
        //         ul {
        //             li { "x1: {img.x1}" }
        //             li { "y1: {img.y1}" }
        //             li { "x2: {img.x2}" }
        //             li { "y2: {img.y2}" }
        //             li { "Width: {img.crop_width}" }
        //             li { "Height: {img.crop_height}" }
        //         }
        //         p { "Raw bytes: {img.cropped_vec.len()} bytes" }
        //     } else {
        //         p { "⬅️ Draw a rectangle on the image to crop." }
        //     }
        // }
    }
}

                // div {
                //     class:"flex justify-center items-center gap-2 bg-white px-4 py-2 border-[#BEBEBE] rounded-lg",
                //     img {
                //         class:"w-[228px] h-[188px]",
                //         src:"https://images.pexels.com/photos/414612/pexels-photo-414612.jpeg?cs=srgb&dl=pexels-souvenirpixels-414612.jpg&fm=jpg",
                //         alt:"Preview Image"
                //     }
                // }

                div {
                    class: "flex justify-center items-center gap-2 bg-white px-4 py-2 border-[#BEBEBE] rounded-lg",

                    button {
                        class: "w-6 h-6 mr-2 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        // onclick: go_prev,
                        // disabled: "{current_page() == 0}",
                        svg {
                            width: "22",
                            height: "22",
                            view_box: "0 0 22 22",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",

                            rect {
                                x: "0.5",
                                y: "0.5",
                                width: "20.4737",
                                height: "20.4737",
                                rx: "10.2368",
                                fill: "#F7F7F7",
                                stroke: "#EDEDED"
                            }

                            path {
                                d: "M16.5806 12.0215C16.4646 12.0215 16.3533 12.0676 16.2713 12.1496C16.1892 12.2317 16.1431 12.343 16.1431 12.459V13.953C16.1431 14.3052 16.0032 14.6429 15.7542 14.892C15.5052 15.141 15.1675 15.2809 14.8153 15.2809H7.20281C6.85065 15.2809 6.51292 15.141 6.26391 14.892C6.01489 14.6429 5.875 14.3052 5.875 13.953V12.459C5.875 12.343 5.82891 12.2317 5.74686 12.1496C5.66481 12.0676 5.55353 12.0215 5.4375 12.0215C5.32147 12.0215 5.21019 12.0676 5.12814 12.1496C5.04609 12.2317 5 12.343 5 12.459V13.953C5.00058 14.5371 5.23285 15.097 5.64583 15.51C6.05881 15.923 6.61877 16.1553 7.20281 16.1559H14.8153C15.3994 16.1553 15.9593 15.923 16.3723 15.51C16.7853 15.097 17.0175 14.5371 17.0181 13.953V12.459C17.0181 12.343 16.972 12.2317 16.89 12.1496C16.8079 12.0676 16.6967 12.0215 16.5806 12.0215Z",
                                fill: "#0387D9"
                            }

                            path {
                                d: "M8.82144 8.2362L10.5714 6.4862V12.9831C10.5714 13.0991 10.6175 13.2104 10.6996 13.2924C10.7816 13.3745 10.8929 13.4206 11.0089 13.4206C11.125 13.4206 11.2362 13.3745 11.3183 13.2924C11.4003 13.2104 11.4464 13.0991 11.4464 12.9831V6.49276L13.1964 8.24276C13.2371 8.28341 13.2853 8.31565 13.3385 8.33765C13.3916 8.35965 13.4485 8.37097 13.506 8.37097C13.5635 8.37097 13.6204 8.35965 13.6735 8.33765C13.7266 8.31565 13.7749 8.28341 13.8155 8.24276C13.8561 8.20211 13.8884 8.15385 13.9104 8.10074C13.9324 8.04763 13.9437 7.99071 13.9437 7.93323C13.9437 7.87574 13.9324 7.81882 13.9104 7.76571C13.8884 7.7126 13.8561 7.66434 13.8155 7.6237L11.3196 5.12776C11.2588 5.06877 11.1832 5.02721 11.1008 5.00745C11.03 4.99441 10.9571 4.99853 10.8882 5.01947C10.8193 5.0404 10.7565 5.07753 10.7049 5.12776L8.20894 7.61713C8.13948 7.70071 8.10361 7.80715 8.10832 7.91572C8.11303 8.02429 8.15798 8.12722 8.23441 8.20447C8.31084 8.28172 8.41329 8.32776 8.5218 8.33362C8.63031 8.33949 8.73712 8.30476 8.82144 8.2362Z",
                                fill: "#0387D9"
                            }
                        }


                    }

                    button {
                        class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        // onclick: move |_| current_page.set(0),
                        // disabled: "{current_page() == 0}",
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                                fill: "#CACACA",
                            }
                        }

                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                                fill: "#CACACA",
                            }
                        }
                    }

                    button {
                        class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        // onclick: go_prev,
                        // disabled: "{current_page() == 0}",
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                                fill: "#CACACA",
                            }
                        }
                    }

                    button {
                        class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        // onclick: go_next,
                        // disabled: "{current_page() + 1 >= total_pages}",
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                                fill: "#CACACA",
                            }
                        }

                    }

                    button {
                        class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        // onclick/: move |_| current_page.set(total_pages.saturating_sub(1)),
                        // disabled: "{current_page() + 1 >= total_pages}",
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                                fill: "#CACACA",
                            }
                        }
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                                fill: "#CACACA",
                            }
                        }
                    }

                    span {
                        class: "ml-2 bg-[#F7F7F7] text-[#555555] text-xs font-normal border border-[#EDEDED] rounded-[50px] px-[10px] py-[5px]",
                        "01/02"
                    }
                }

            }
            div {
                class: "w-1/3 flex flex-col border-[0.5px] border-[#BEBEBE] bg-[#FAFAFA] rounded-tr-[10px] rounded-br-[10px]",
                // Header
                div {
                    class: "flex justify-between items-center px-[14px] py-[10px]",
                    span { class: "font-normal text-sm text-[#404040]", "Learn Properties" },
                    button {
                        class: "w-6 h-6 bg-white text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                        svg {
                            width: "6",
                            height: "10",
                            view_box: "0 0 6 10",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M0.698294 9.21005C0.570811 9.2108 0.445996 9.17356 0.339761 9.10308C0.233526 9.03261 0.150685 8.9321 0.101799 8.81436C0.0529129 8.69662 0.0401973 8.56699 0.0652733 8.44199C0.0903492 8.317 0.152081 8.2023 0.242597 8.11253L3.64428 4.71727L0.242597 1.322C0.137449 1.19922 0.0825047 1.04128 0.088744 0.87975C0.0949833 0.718217 0.161946 0.564988 0.276252 0.450682C0.390558 0.336376 0.543787 0.269413 0.705319 0.263174C0.866852 0.256935 1.02479 0.311879 1.14757 0.417027L4.99853 4.26799C5.11807 4.38824 5.18517 4.55091 5.18517 4.72048C5.18517 4.89004 5.11807 5.05271 4.99853 5.17296L1.14757 9.02392C1.02803 9.1425 0.86667 9.20934 0.698294 9.21005Z",
                                fill: "#0387D9",
                            }
                        }
                    }
                }
    
                // Mode & Category
                div {
                    class: "px-[14px] py-[10px] flex gap-10",
                    // Learning Mode
                    div {
                        class: "flex px-2",
                        span {
                            class: "flex flex-col gap-[6px]",
                            label { class: "font-normal text-[10px] text-[#8F8F8F]", "Learning Mode" },
                            select {
                                class: "
                                   
                                    px-3
                                    pr-7
                                    text-[11px]
                                    font-normal
                                    text-[#555555]
                                    rounded
                                    border border-[0.5px] border-[#BEBEBE]
                                    appearance-none
                                    bg-[url(\"data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E\")]
                                    bg-no-repeat
                                    bg-[position:right_0.75rem_center]
                                    bg-[length:10px_6px]
                                ",
                                option { value: "", disabled: true, selected: true, "Options" }
                                option { value: "web", "Web" }
                                option { value: "mobile", "Mobile" }
                                option { value: "desktop", "Desktop" }
                            }
                        }
                    }
    
                    // Categories
                    div {
                        class: "flex",
                        span {
                            class: "flex flex-col justify-end gap-[6px]",
                            label { class: "font-normal text-[10px] text-[#8F8F8F]", "Categories" },
                            div {
                                class: "flex items-center gap-2",
                                select {
                                    class: "
                                       
                                        px-3
                                        pr-7
                                        text-[11px]
                                        font-normal
                                        text-[#555555]
                                        rounded
                                        border border-[0.5px] border-[#BEBEBE]
                                        appearance-none
                                        bg-[url(\"data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E\")]
                                        bg-no-repeat
                                        bg-[position:right_0.75rem_center]
                                        bg-[length:10px_6px]
                                    ",
                                    option { value: "", disabled: true, selected: true, "Options" }
                                    option { value: "web", "Web" }
                                    option { value: "mobile", "Mobile" }
                                    option { value: "desktop", "Desktop" }
                                },
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "19",
                                    height: "19",
                                    view_box: "0 0 19 19",
                                    fill: "none",
                                    rect {
                                        x: "0.5",
                                        y: "0.5",
                                        width: "18",
                                        height: "18",
                                        rx: "3.5",
                                        fill: "white",
                                        stroke: "#0387D9"
                                    }
                                    path {
                                        fill_rule: "evenodd",
                                        clip_rule: "evenodd",
                                        d: "M4 9.49998C4 9.87956 4.3079 10.1875 4.68748 10.1875H14.3125C14.6922 10.1875 15 9.87956 15 9.49998C15 9.1204 14.6922 8.8125 14.3125 8.8125H4.68763C4.3079 8.8125 4 9.1204 4 9.49998Z",
                                        fill: "#0387D9"
                                    }
                                    path {
                                        fill_rule: "evenodd",
                                        clip_rule: "evenodd",
                                        d: "M9.50002 15C9.8796 15 10.1875 14.6921 10.1875 14.3125V4.68748C10.1875 4.30776 9.8796 4 9.50002 4C9.12044 4 8.81254 4.30776 8.81254 4.68748V14.3124C8.81254 14.6921 9.12044 15 9.50002 15Z",
                                        fill: "#0387D9"
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "px-[14px] py-[10px] flex",
                    div {
                        class: "flex flex-col px-2 gap-[6px]",
                        label { class: "font-normal text-[10px] text-[#8F8F8F]", "Step XY (px)" },
                        span {
                            class: "flex gap-1",
                            input {
                                r#type: "number",
                                min: "16", max: "256", value: "16",
                                class: "p-1 w-[53px] border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131] appearance-none"
                            },
                            input {
                                r#type: "number",
                                min: "16", max: "256", value: "16",
                                class: "p-1 w-[53px] border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131] appearance-none"
                            }
                        }
                    }
    
                    div {
                        class: "w-full mt-4 flex justify-end items-center p-2 gap-2",
                        label { class: "text-sm text-[#313131]", "Full Image" }
                        input { type: "checkbox", class: "w-4 h-4 accent-[#0387D9] border border-[#0387D9] rounded" }
                    }
                }
    
                hr { class: "border-t-[0.5px] border-[#DADADA] my-1 w-full" }
    
                button {
                    class: "mx-4 my-2 p-[6px] flex justify-center items-center gap-2 bg-[#0387D9] text-white text-xs font-medium rounded-2xl",
                    "Select Full Image"
                }
            }
         }
    }
}
fn mat_to_base64(mat: &Mat) -> opencv::Result<String> {
    let mut buf = Vector::new();
    imencode(".jpg", mat, &mut buf, &Vector::new())?;
    Ok(STANDARD.encode(&buf.to_vec()))
}