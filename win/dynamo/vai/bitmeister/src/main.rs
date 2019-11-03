

use winapi::um::winuser::{GetDC, OpenClipboard, EmptyClipboard,
    SetClipboardData, CloseClipboard, CF_BITMAP, ReleaseDC};
use winapi::shared::windef::{HWND};
use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, BitBlt, DeleteDC, SRCCOPY,
    BITMAPINFOHEADER, BITMAPINFO, RGBQUAD, DIB_RGB_COLORS, GetDIBits, BI_RGB, DeleteObject};
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::fs;
use std::thread::{sleep};
use std::time::{Duration, Instant};
use std::fmt;
use dynamo::{left_click, move_cursor, cursor_position};

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32
// }


const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;
const TOTAL_CELLS: usize = (WIDTH as usize) * (HEIGHT as usize);
// Windows explorer icon
// const XTARGET_WINDOWS_EXPLORER: i32 = 112;
// const YTARGET_WINDOWS_EXPLORER: i32 = 10;
// const XTARGET: i32 = 112;
// const YTARGET: i32 = 10;
// Windows start button icon
const XTARGET_START_MENU: i32 = 14;
const YTARGET_START_MENU: i32 = 10;
// Brave icon
// const XTARGET: i32 = 211;
// const YTARGET: i32 = 10;
// Skip Ad Full Screen
const XTARGET_SKIP_AD_FULLSCREEN: i32 = 1825;
const YTARGET_SKIP_AD_FULLSCREEN: i32 = 956;
const XTARGET_SKIP_AD_WINDOW: i32 = 1808;
const YTARGET_SKIP_AD_WINDOW: i32 = 864;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cell {
    index: usize,
    avg_light: u8,
}

const AD_FINGERPRINT_FILE: &str = "e:\\t_\\ad_fingerprint.txt";
const START_MENU_FINGERPRINT_FILE: &str = "e:\\t_\\fingerprint_start_menu.txt";

fn save_to_file(cells: Vec<Cell>) {
    let mut table: Vec<Cell> = cells[..50].to_vec();
    table.extend_from_slice(&cells[TOTAL_CELLS - 50..]);
    let s = serde_json::to_string(&table);
    fs::write(AD_FINGERPRINT_FILE, s.unwrap());
}

fn read_from_file(path: &str) -> Vec<Cell> {
    let s = fs::read_to_string(&path)
        .expect("fingerprint file.");
    serde_json::from_str(&s)
        .expect("valid fingerprint json data.")
}

fn screen_pixels(x: i32, y: i32, width: i32, height: i32, 
    copy_to_clipboard: bool) -> Vec<u8> {
    let hwnd: HWND = std::ptr::null_mut();
    let hdc = unsafe { GetDC(hwnd) };
    let hdc_memory = unsafe { CreateCompatibleDC(hdc) };
    let hbmp_target = unsafe { CreateCompatibleBitmap(hdc, WIDTH, HEIGHT) };
    let old_bmp = unsafe { 
        SelectObject(hdc_memory, hbmp_target as *mut winapi::ctypes::c_void) 
        };
    let info_header = BITMAPINFOHEADER { biBitCount: 0, biClrImportant: 0,
        biClrUsed: 0, biCompression: 0, biHeight: 0, biPlanes: 0,
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biSizeImage: 0, biWidth: 0, biXPelsPerMeter: 0, biYPelsPerMeter: 0 };
    let rgb_quad = RGBQUAD { 
        rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 };
    let mut bminfo = BITMAPINFO { 
        bmiHeader: info_header, bmiColors: [rgb_quad] };
    // Most of the time is spent on this BitBlt call. It takes about 13ms.
    // My guess is that Windows synchronizes these calls to their 30 fps loop
    // or something. Anyway, it's best to spend the time calling this just once
    // than to incur these costs on repeated calls like in other APIs.
    unsafe { 
        BitBlt(hdc_memory, 0, 0, WIDTH, HEIGHT, hdc, x, y, SRCCOPY);
        GetDIBits(hdc, hbmp_target, 0, 0, std::ptr::null_mut(), &mut bminfo, 
            DIB_RGB_COLORS);
    }
    let mut pixels = vec![0u8; bminfo.bmiHeader.biSizeImage as usize];
    bminfo.bmiHeader.biCompression = BI_RGB;
    unsafe {
        GetDIBits(hdc, hbmp_target, 0, bminfo.bmiHeader.biHeight as u32,
            pixels.as_mut_ptr() as *mut winapi::ctypes::c_void,
            &mut bminfo, DIB_RGB_COLORS);
        if copy_to_clipboard {
            OpenClipboard(hwnd);
            EmptyClipboard();
            SetClipboardData(CF_BITMAP, 
                hbmp_target as *mut winapi::ctypes::c_void);
            CloseClipboard();
        }
        SelectObject(hdc_memory, old_bmp);
        DeleteObject(hbmp_target as *mut winapi::ctypes::c_void);
        DeleteDC(hdc_memory);
        ReleaseDC(hwnd, hdc);
    }
    pixels
}

fn pixels_to_cells(pixels: &Vec<u8>) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![Cell{index: 0, avg_light: 0};TOTAL_CELLS];
    for i in 0..TOTAL_CELLS {
        let pi = i * 4;
        // In bitmap the order is BGR apparently. Even if it doesn't matter for
        // this algorithm.
        let b = pixels[pi] as i32;
        let g = pixels[pi + 1] as i32;
        let r = pixels[pi + 2] as i32;
        // println!("b + g + r {}", b + g + r);
        let avg = (b + g + r) / 3;
        // println!("avg {}", avg);
        cells[i] = Cell{ index: i, avg_light: avg as u8};
    }
    cells
}

fn rgb_to_html(pixels: &Vec<u8>, cells: &Vec<Cell>) -> String {
    let mut s = String::new();
    for i in 0..TOTAL_CELLS {
        if i % 20 == 0 {
            if i > 0 {
                s.push_str("</tr>\n");
            }
            s.push_str("<tr>");
        }
        let pi = cells[i].index * 4;
        // In bitmap the order is BGR apparently. Even if it doesn't matter for
        // this algorithm.
        let b = pixels[pi] as i32;
        let g = pixels[pi + 1] as i32;
        let r = pixels[pi + 2] as i32;
        s.push_str(&format!(r#"<td style="background-color:rgb({},{},{});width:20;height:20"></td>"#, r, g, b));
    }
    s.push_str("</tr>\n");
    s
}

fn match_fingerprint(table: &Vec<Cell>, cells: &Vec<Cell>) -> bool {
    let mut match_count = 0.0;
    let tier1 = 255 / 20;
    let tier2 = 255 / 5;
    for fpcell in table {
        // println!("{} || {}", cells[10].avg_light, fpcell.avg_light);
        let avg = cells[fpcell.index].avg_light as i32;
        let fp_avg = fpcell.avg_light as i32;
        if fp_avg >= avg - tier1 && fp_avg <= avg + tier1 {
            match_count += 1.1;
        } else if fp_avg >= avg - tier2 && fp_avg <= avg + tier2 {
            match_count += 0.8;
        }
    }
    if match_count > 60.0 {
        println!("match count {}", match_count);
    }
    // println!("match count {}", match_count);
    match_count >= 80.0
}

#[derive(Debug)]
struct Fingerprint {
    name: String,
    fingerprint: Vec<Cell>,
}

const DB: &str = "e:\\t_\\";
const MIC: &str = "e:\\t_\\match_icon_count.txt";

fn main() {
    // println!("XTARGET_START_MENU {} YTARGET_START_MENU {}", XTARGET_START_MENU, YTARGET_START_MENU);
    // let pixels_start_menu = screen_pixels(XTARGET_START_MENU, YTARGET_START_MENU, WIDTH, HEIGHT, false);
    // let mut cells_start_menu = pixels_to_cells(&pixels_start_menu);
    let dur = Duration::from_millis(4000);
    println!("{:?}", dur);
    let mut fingerprints: Vec<Fingerprint> = Vec::new();
    let names = ["fingerprint_skip_ad_fade_window.txt",
        // "fingerprint_skip_ad_fade_window_2.txt",
        // "fingerprint_skip_ad_bright_window.txt"
        ];
    for name in &names {
        let fp = path::join(DB, name);
        fingerprints.push(Fingerprint{name: name.to_string(), 
            fingerprint: read_from_file(&fp)});
    }
    // let mut match_icon_count = fs::read_to_string(&MIC).expect("MIC").parse::<i32>().unwrap();
    // let start_menu_fingerprint = read_from_file(&START_MENU_FINGERPRINT_FILE);
    loop {
        for i in 0..2 {
            let diff = i * 9;

    let pixels = screen_pixels(XTARGET_SKIP_AD_WINDOW + diff, YTARGET_SKIP_AD_WINDOW, WIDTH, HEIGHT, false);
    // let pixels = screen_pixels(XTARGET_SKIP_AD, YTARGET_SKIP_AD, WIDTH, HEIGHT, true);
    let mut cells = pixels_to_cells(&pixels);
    // let mut html = String::new();
    // html.push_str("<html><body><table>");
    // html.push_str(&rgb_to_html(&pixels, &cells));
    // cells.sort_by(|a, b| b.avg_light.partial_cmp(&a.avg_light).unwrap());
    // html.push_str(r#"</tr><tr style="background-color:#e5e5e5;height:20"><td colspan="20"><hr></td>"#);
    // html.push_str(&rgb_to_html(&pixels, &cells));
    // html.push_str("</table></body></html>");
    // match_icon_count += 1;
    // println!("PATH {:?}", path::join(DB, &path::join("match_results", &format!("icon{}.html", match_icon_count))));
    // fs::write(path::join(DB, &path::join("match_results", &format!("icon{}.html", match_icon_count))), html);
    // fs::write(&MIC, match_icon_count.to_string());
    // println!("{}", html);
    // if !match_fingerprint(&start_menu_fingerprint, &cells_start_menu) {
        // println!("WE HAVE A MATCH!");
        for fp in &fingerprints {
           if match_fingerprint(&fp.fingerprint, &cells) {
                println!("SKIP THE AD YEAH: {:?}", &fp.name);
                let cp = cursor_position();
                move_cursor(XTARGET_SKIP_AD_WINDOW, YTARGET_SKIP_AD_WINDOW);
                left_click();
                move_cursor(cp.x, cp.y);
           } 
        }
        }
        sleep(dur);
    }
    // save_to_file(cells);
}
