
use winapi::um::winuser::{GetForegroundWindow};
use winapi::shared::windef::{HWND};
use winapi::shared::d3d9::{IDirect3DSurface9, D3DADAPTER_DEFAULT, D3D_SDK_VERSION,
    Direct3DCreate9, D3DCREATE_SOFTWARE_VERTEXPROCESSING, IDirect3DDevice9,
    IDirect3D9};
use winapi::shared::d3d9types::{D3DDISPLAYMODE, D3DPOOL_SYSTEMMEM, D3DPRESENT_PARAMETERS, D3DMULTISAMPLE_NONE,
    D3DSWAPEFFECT_DISCARD, D3DPRESENTFLAG_LOCKABLE_BACKBUFFER, D3DPRESENT_RATE_DEFAULT, D3DDEVTYPE_REF, D3DDEVTYPE_HAL,
    D3DSURFACE_DESC, D3DLOCKED_RECT, D3DLOCK_READONLY, D3DFMT_A8R8G8B8};
use winapi::shared::d3d9caps::{D3DPRESENT_INTERVAL_DEFAULT};
use std::thread::{sleep};
use std::time::{Duration, Instant};
use std::fs;
use std::format;
use serde_json::json;
use serde::{Serialize, Deserialize};
use dynamo::{left_click, move_cursor, cursor_position};
use murmur3::murmur3_32;
use regex::Regex;
use aliasopts::{AliasOpts, CT};
use std::env;


fn hash_32(s: &str) -> String {
    let v = murmur3_32(&mut s.as_bytes(), 0);
    format!("{:x}", v)
}

const D3D_OK: i32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cell {
    index: usize,
    avg_light: u8,
}

struct ScreenMeister {
    // ip = interface pointer.
    ip_d3d: *mut IDirect3D9,
    ip_device: *mut IDirect3DDevice9,
    ip_surface: *mut IDirect3DSurface9,
    locked_rect: D3DLOCKED_RECT
}

impl ScreenMeister {

    fn new() -> ScreenMeister {
        ScreenMeister {
            ip_d3d: std::ptr::null_mut(),
            ip_device: std::ptr::null_mut(),
            ip_surface: std::ptr::null_mut(),
            locked_rect: D3DLOCKED_RECT { 
                Pitch: 0, pBits: std::ptr::null_mut() }
        }
    }

    fn setup(&mut self) {
        self.ip_d3d = unsafe { Direct3DCreate9(D3D_SDK_VERSION) };
        let d3d = unsafe { self.ip_d3d.as_ref().unwrap() };
        let mut display_mode = D3DDISPLAYMODE { Width: 0, Height: 0,
            RefreshRate: 0, Format: 0 };
        unsafe { d3d.GetAdapterDisplayMode(D3DADAPTER_DEFAULT,
            &mut display_mode) };
        let fg_whnd = unsafe { GetForegroundWindow() };
        let mut present_parameters = D3DPRESENT_PARAMETERS{
            BackBufferWidth: display_mode.Width,
            BackBufferHeight: display_mode.Height,
            BackBufferFormat: display_mode.Format,
            BackBufferCount: 1,
            MultiSampleType: D3DMULTISAMPLE_NONE,
            MultiSampleQuality: 0,
            SwapEffect: D3DSWAPEFFECT_DISCARD,
            hDeviceWindow: fg_whnd,
            Windowed: 1, // true
            EnableAutoDepthStencil: 0, // false
            AutoDepthStencilFormat: 0,
            Flags: D3DPRESENTFLAG_LOCKABLE_BACKBUFFER,
            FullScreen_RefreshRateInHz: D3DPRESENT_RATE_DEFAULT,
            PresentationInterval: D3DPRESENT_INTERVAL_DEFAULT,
            };
        // Somehow this D3D device seems to leak memory so we can't have it in
        // a loop.
        if D3D_OK != unsafe { d3d.CreateDevice(D3DADAPTER_DEFAULT,
            // D3DDEVTYPE_REF,
            D3DDEVTYPE_HAL,
            fg_whnd, D3DCREATE_SOFTWARE_VERTEXPROCESSING,
            &mut present_parameters, &mut self.ip_device) } {
            panic!("Unable to create D3D device.");
        }
        let device = unsafe { self.ip_device.as_ref().unwrap() };
        unsafe { device.CreateOffscreenPlainSurface(display_mode.Width, 
            display_mode.Height, D3DFMT_A8R8G8B8, D3DPOOL_SYSTEMMEM, 
            &mut self.ip_surface, std::ptr::null_mut()) };
    }

    fn grab_surface(&mut self) {
        // A significant amount of time is spent in this call.
        unsafe { self.ip_device.as_ref().unwrap().GetFrontBufferData(
            D3DADAPTER_DEFAULT, self.ip_surface) };
    }

    fn lock_rect(&mut self) {
        unsafe { self.ip_surface.as_ref().unwrap().LockRect(
            &mut self.locked_rect, std::ptr::null(), D3DLOCK_READONLY) };
    }

    fn unlock_rect(&self) {
        unsafe { self.ip_surface.as_ref().unwrap().UnlockRect() };
    }

    fn list_cells(&self, x: i32, y: i32, width: u32, height: u32) -> Vec<Cell> {
        let ptr_image_data: *const u32 = self.locked_rect.pBits as *const u32;
        let mut list = vec![Cell{index: 0, avg_light: 0}; 
            (width * height) as usize];
        let xoffset = x;
        let yoffset = y;
        let twidth = width as i32;
        let theight = height as i32;
        let mut ci = 0;
        for i in yoffset..yoffset + theight {
            for j in xoffset..xoffset + twidth {
                let pi: isize = 
                    (i * (self.locked_rect.Pitch / 4) + j) as isize;
                let they = unsafe { 
                    let aaaa = *ptr_image_data.offset(pi);
                    std::mem::transmute::<u32, [u8; 4]>(aaaa)
                    };
                let b = they[0] as i32;
                let g = they[1] as i32;
                let r = they[2] as i32;
                let n = (r + g + b) / 3;
                list[ci] = Cell{index: ci, avg_light: n as u8};
                ci += 1;
            }
        }     
        list
    }

    fn fingerprint(&self, x: i32, y: i32, width: u32, height: u32) -> String {
        let mut list = self.list_cells(x, y, width, height);
        list.sort_by(|a, b| b.avg_light.partial_cmp(&a.avg_light).unwrap());
        let mut table: Vec<Cell> = list[..50].to_vec();
        table.extend_from_slice(&list[list.len() - 50..]);
        serde_json::to_string(&table).unwrap()
    }

    fn write_fingerprint(&self, path: &str, x: i32, y: i32, width: u32, 
        height: u32) {
        let s = self.fingerprint(x, y, width, height);
        fs::write(&path, s);
    }

    fn write(&self, path: &str, x: i32, y: i32, width: u32, height: u32) {
        let ptr_image_data: *const u32 = self.locked_rect.pBits as *const u32;
        let mut html = String::new();
        html.push_str("<html><body><table border=\"0\" cellspacing=\"0\" \
            cellpadding=\"0\">");
        let xoffset = x;
        let yoffset = y;
        let twidth = width as i32;
        let theight = height as i32;
        for i in yoffset..yoffset + theight {
            html.push_str("<tr>");
            for j in xoffset..xoffset + twidth {
                let pi: isize = 
                    (i * (self.locked_rect.Pitch / 4) + j) as isize;
                unsafe {
                    let aaaa = *ptr_image_data.offset(pi);
                    let they = std::mem::transmute::<u32, [u8; 4]>(aaaa);
                    let b = they[0];
                    let g = they[1];
                    let r = they[2];
                    let a = they[3];
                    html.push_str(&format!("<td style=\"width:1px;height:1px;\
                        background-color:rgba({},{},{},{})\"></td>", 
                        r, g, b, a));
                    // if a != 0 || r != 0 || g != 0 || b != 0 {
                    //     println!("pi {}", pi);
                    //     println!("r {}", r);
                    //     println!("g {}", g);
                    //     println!("b {}", b);
                    //     println!("a {}", a);
                    // }
                    // println!("==================");
                }
            }
            html.push_str("</tr>");
        }     
        html.push_str("</table></body></html>");
        fs::write(&path, &html);
    }

    fn store_fingerprint(&self, path: &str, version: &str, x: i32, y: i32,
        width: u32, height: u32) {
        let s = self.fingerprint(x, y, width, height);
        let hash = hash_32(&s);
        let pattern = format!("^fp.+?{}\\.txt$", hash);
        let re = Regex::new(&pattern).unwrap();
        let mut lister = |name: &str, path: &str| -> bool {
            if re.is_match(name) {
                return true;
            }
            false
        };
        if !search_files::find_filename(&path, false, &mut lister) {
            // Found a fingerprint file with this hash so just exit the function
            println!("wait what");
           return; 
        }
        println!("pattern {}", pattern);
        let fp_path = path::join(path, 
            &format!("fp_{}_x{}_y{}_w{}_h{}_mc0_{}.txt",
            version, x, y, width, height, hash));
        println!("fp_path {}", fp_path);
        fs::write(&fp_path, s);
        println!("did it? {}", path);
    }

    fn release(&mut self) {
        unsafe { 
            self.ip_surface.as_ref().unwrap().Release();
            self.ip_device.as_ref().unwrap().Release();
            self.ip_d3d.as_ref().unwrap().Release();
        };
        self.ip_surface = std::ptr::null_mut();
        self.ip_device = std::ptr::null_mut();
        self.ip_d3d = std::ptr::null_mut();
    }

}

// #[derive(Debug, Clone)]
#[derive(Debug)]
struct FingerprintItem {
    path: String,
    filename: String,
    mask: String,
    match_count: u32,
    x: i32,
    y: i32,
    table: Vec<Cell>
}

#[derive(Debug)]
struct FingerprintMatcher {
    items: Vec<FingerprintItem>
}

impl FingerprintMatcher {

    fn new() -> FingerprintMatcher {
        FingerprintMatcher {
            items: Vec::new()
        }
    }

    fn load_all(&mut self, path: &str) {
        let pattern = 
            "^(fp_[^_]+_x)(-?\\d+)_y(-?\\d+)(_w\\d+_h\\d+)_mc(\\d+)(_.+?)\
            \\.txt$";
        let re = Regex::new(&pattern).unwrap();
        let mut lister = |name: &str, path: &str| {
            if let Some(mm) = re.captures(name) {
                let mask = format!("{}{}_y{}{}_mc{{mc}}{}.txt", 
                    mm.get(1).unwrap().as_str(),
                    mm.get(2).unwrap().as_str(),
                    mm.get(3).unwrap().as_str(),
                    mm.get(4).unwrap().as_str(),
                    mm.get(6).unwrap().as_str());
                let x = mm.get(2).unwrap().as_str().parse::<i32>()
                    .expect("Expected x value of type i32.");
                let y = mm.get(3).unwrap().as_str().parse::<i32>()
                    .expect("Expected y value of type i32.");
                let mc = mm.get(5).unwrap().as_str().parse::<u32>()
                    .expect("Expected fingerprint file match count u32.");
                let fp = path::join(path, name);
                let s = fs::read_to_string(&fp)
                    .expect("Expected fingerprint file.");
                let table = serde_json::from_str(&s)
                    .expect("Expected valid fingerprint json data.");
                let h = FingerprintItem{
                    path: path.to_string(),
                    filename: name.to_string(),
                    mask: mask,
                    match_count: mc,
                    x: x,
                    y: y,
                    table: table
                    };
                // println!("hhh {:?}", h);
                self.items.push(h);
                // println!("mask {}", mask);
            }
        };
        search_files::list_filenames(&path, &mut lister);
        self.items.sort_by(|a, b| 
            b.match_count.partial_cmp(&a.match_count).unwrap()
            );
        // for item in &self.items {
        //     println!("{:?}\n==================================", item);
        // }
        println!("Loaded {} fingerprint items.", self.items.len());
    }

    fn do_match(&mut self, sm: &ScreenMeister) -> Option<usize> {
        let tier1 = 255 / 20; // 5%
        let tier2 = 255 / 5; // 20%
        let mut list: Vec<Cell> = Vec::new();
        let mut xlist: i32 = -1;
        let mut ylist: i32 = -1;
        for i in 0..self.items.len() {
            let item = &mut self.items[i];
            if item.x != xlist || item.y != ylist {
                list = sm.list_cells(item.x, item.y, 20, 20);
                xlist = item.x;
                ylist = item.y;
            }
            let mut match_count = 0.0;
            for fpcell in &item.table {
                let avg = list[fpcell.index].avg_light as i32;
                let fp_avg = fpcell.avg_light as i32;
                if fp_avg >= avg - tier1 && fp_avg <= avg + tier1 {
                    match_count += 1.1;
                } else if fp_avg >= avg - tier2 && fp_avg <= avg + tier2 {
                    match_count += 0.8;
                }
            }
            if match_count >= 80.0 {
                println!("match count {}", match_count);
                item.match_count += 1;
                return Some(i);
            }
        }
        None
    }

    fn update_item(&mut self, index: usize) {
        let item = &mut self.items[index];
        let new_filename = item.mask
            .replace("{mc}", &item.match_count.to_string());
        println!("new_filename {}", new_filename);
        fs::rename(path::join(&item.path, &item.filename),
            path::join(&item.path, &new_filename));
        item.filename = new_filename;
    }

    fn match_click(&mut self, sm: &ScreenMeister, x: i32, y: i32) -> bool {
        if let Some(index) = self.do_match(&sm) {
            self.update_item(index);
            let cp = cursor_position();
            move_cursor(x, y);
            left_click();
            move_cursor(cp.x, cp.y);
            return true;
        }
        false
    }

}

const XTARGET_SKIP_AD_FULLSCREEN: i32 = 1825;
const YTARGET_SKIP_AD_FULLSCREEN: i32 = 956;
const XTARGET_SKIP_AD_WINDOW: i32 = 1808;
const YTARGET_SKIP_AD_WINDOW: i32 = 864;

fn run(path: &str, label: &str) {
    let find_dur = Duration::from_millis(3000);
    let confirm_dur = Duration::from_millis(1000);
    let cooldown_dur = Duration::from_millis(120000);
    let mut fm = FingerprintMatcher::new();
    let mut confirm = false;
    let mut repeat_count = 0;
    fm.load_all(path);
    let mut sm = ScreenMeister::new();
    sm.setup();
    loop {
        confirm = false;
        sm.grab_surface();
        sm.lock_rect();
        if let Some(_) = fm.do_match(&sm) {
            println!("*Found match*");
            confirm = true;
        }
        sm.unlock_rect();

        if confirm {
            println!("confirming...");
            sleep(confirm_dur);
            // let starttime = Instant::now();
            sm.grab_surface();
            // println!("Elapsed 1: {}", starttime.elapsed().as_millis());
            sm.lock_rect();
            // println!("Elapsed 2: {}", starttime.elapsed().as_millis());
            // sm.write("e:\\t_\\paris.html", 112, 10, 20, 20);
            // sm.write("e:\\t_\\paris.html", 112, 150, 20, 20);
            // let cells = sm.list_cells(112, 10, 20, 20);
            // let cells = sm.list_cells(XTARGET_SKIP_AD_FULLSCREEN, YTARGET_SKIP_AD_FULLSCREEN, 20, 20);
            // sm.store_fingerprint(path, &label, 112, 10, 20, 20);
            // println!("match {:?}", fm.do_match(&cells));
            // fm.match_click(&cells, XTARGET_SKIP_AD_WINDOW, YTARGET_SKIP_AD_WINDOW);
            // fm.match_click(&cells, 112, 10);
            if fm.match_click(&sm, XTARGET_SKIP_AD_FULLSCREEN,
                YTARGET_SKIP_AD_FULLSCREEN) {
                repeat_count += 1;
                println!("*Confirmed match*");
            } else {
                repeat_count = 0;
            }
            // println!("novo {:?}", fm);
            sm.unlock_rect();
            // println!("Elapsed 3: {}", starttime.elapsed().as_millis());
        } else {
            repeat_count = 0;
        }

        if repeat_count > 1 {
            repeat_count = 0;
            println!("*cooldown*");
            sleep(cooldown_dur);
        } else {
            sleep(find_dur);
        }
    }
    sm.release();
}

fn grab_at(path: &str, label: &str, x: i32, y: i32) {
    let mut sm = ScreenMeister::new();
    sm.setup();
    sm.grab_surface();
    sm.lock_rect();
    sm.store_fingerprint(path, &label, x, y, 20, 20);
    sm.unlock_rect();
    sm.release();
}

fn write_at(path: &str, label: &str, x: i32, y: i32) {
    let mut sm = ScreenMeister::new();
    sm.setup();
    sm.grab_surface();
    sm.lock_rect();
    sm.write(&path::join(path, "sample.html"), x, y, 20, 20);
    sm.unlock_rect();
    sm.release();
}

fn print_help() {
    println!(
r#"Program to simulate clicks given a fingerprint.

Usage: vazando <--path c:\path\to\db\> [option]
    --grab_at  <x,y>  - Grabs a fingerprint at the x,y position and saves it
                to the database.
    --write_at  <x,y> - Writes an html file showing the screen at the x,y
                position for testing purposes.
    --path            - Path to the directory used as a database.
    --label           - Fingerprint version label. Default is "a".
    --run             - Starts the looping.
    --help            - Shows this help.
"#);
}

fn main() {
    let mut opts = AliasOpts::new();
    opts.add("--grab_at", CT::CTString)
        .add("--write_at", CT::CTString)
        .add("--path", CT::CTString)
        .add("--label", CT::CTString)
        .add("--run", CT::Flag)
        .add("--help", CT::Flag)
        .parse_args();
    if opts.is_empty() || opts.got("--help") {
        print_help();
        return;
    }
    let path = opts.get("--path").expect("Expected --path value.");
    let db_path = path::join(&path, "fullscreen");
    let mut label = String::new();
    label.push('a');
    if let Some(alabel) = opts.get("--label") {
        label = alabel;
    }
    if let Some(s) = opts.get("--grab_at") {
        let re = Regex::new("^(-?\\d+),(-?\\d+)$").unwrap();
        let m = re.captures(&s)
            .expect("Expected --grab_at x,y of types i32,i32.");
        let x = m.get(1).unwrap().as_str().parse::<i32>()
            .expect("Expected --grab_at x value to be of type i32.");
        let y = m.get(2).unwrap().as_str().parse::<i32>()
            .expect("Expected --grab_at y value to be of type i32.");
        grab_at(&db_path, &label, x, y);
    }
    if let Some(s) = opts.get("--write_at") {
        let re = Regex::new("^(-?\\d+),(-?\\d+)$").unwrap();
        let m = re.captures(&s)
            .expect("Expected --write_at x,y of types i32,i32.");
        let x = m.get(1).unwrap().as_str().parse::<i32>()
            .expect("Expected --write_at x value to be of type i32.");
        let y = m.get(2).unwrap().as_str().parse::<i32>()
            .expect("Expected --write_at y value to be of type i32.");
        write_at(&db_path, &label, x, y);
    }
    if let Some(s) = opts.get("--run") {
        run(&db_path, &label);
    }
}

