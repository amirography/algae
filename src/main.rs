use std::{
    ffi::{OsStr, OsString},
    fmt::{Debug, Display},
    fs::{self, read_dir},
};

use anyhow::Result;
use rustix::process;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let startup_handle = startup();
    let tags_handle = tags();
    let bg = "0x".to_owned() + &catppuccin::Flavour::Macchiato.base().hex();
    let border_focused = "0x".to_owned() + &catppuccin::Flavour::Macchiato.flamingo().hex();
    let border_unfocused = "0x".to_owned() + &catppuccin::Flavour::Macchiato.base().hex();
    let border_urgent = "0x".to_owned() + &catppuccin::Flavour::Macchiato.red().hex();

    let handles = vec![
        //
        // theme
        //
        vec!["background-color", &bg],
        vec!["border-color-focused", &border_focused],
        vec!["border-color-unfocused", &border_unfocused],
        vec!["border-color-urgent", &border_urgent],
        vec!["border-width", "5"],
        //
        // inputs
        //
        vec![
            "input",
            "pointer-1739-30385-CUST0001:00_06CB:76B1_Touchpad",
            "drag",
            "enabled",
        ],
        vec![
            "input",
            "pointer-1739-30385-CUST0001:00_06CB:76B1_Touchpad",
            "tap",
            "enabled",
        ],
        vec![
            "input",
            "pointer-1739-30385-CUST0001:00_06CB:76B1_Touchpad",
            "events",
            "enabled",
        ],
        vec![
            "input",
            "pointer-1739-30385-CUST0001:00_06CB:76B1_Touchpad",
            "natural-scroll",
            "enabled",
        ],
        vec![
            "input",
            "pointer-1739-30385-CUST0001:00_06CB:76B1_Touchpad",
            "scroll-method",
            "two-finger",
        ],
        //
        // options
        //
        vec!["set-repeat", "50", "300"],
        // Make certain views start floating
        vec!["float-filter-add", "app-id", "Rofi"],
        vec!["float-filter-add", "app-id", "float"],
        vec!["float-filter-add", "app-id", "popup"],
        vec!["float-filter-add", "app-id", "pinentry-qt"],
        vec!["float-filter-add", "app-id", "pinentry-gtk"],
        vec!["float-filter-add", "title", "Picture-in-Picture"],
        vec!["float-filter-add", "app-id", "launcher"],
        vec!["csd-filter-add", "app-id", "Rofi"],
        vec!["csd-filter-add", "app-id", "launcher"],
        // mouse stuff
        vec!["focus-follows-cursor", "normal"],
        vec!["set-cursor-warp", "on-output-change"],
        // layout related
        vec!["attach-mode", "bottom"], // new window's open at the end of stack instead of on top
        vec!["default-layout", "rivertile"], // default layouting engine
        //
        // keymaps
        vec!["map-pointer", "normal", "Super", "BTN_LEFT", "move-view"],
        //
        vec!["map-pointer", "normal", "Super", "BTN_RIGHT", "resize-view"],
        vec!["map", "normal", "Super", "R", "spawn", "/home/a/river/init"],
        vec!["map", "normal", "Super", "Return", "spawn", "tmux-picker"],
        vec!["map", "normal", "Super", "D", "spawn", "rofi -show"],
        vec!["map", "normal", "Super", "P", "spawn", "rofi-rbw"],
        vec!["map", "normal", "Super", "J", "focus-view", "next"],
        vec!["map", "normal", "Super", "K", "focus-view", "previous"],
        vec!["map", "normal", "Super", "space", "zoom"],
        vec!["map", "normal", "Super", "Q", "close"],
        vec!["map", "normal", "Super", "Period", "focus-output", "next"],
        vec![
            "map",
            "normal",
            "Super",
            "Comma",
            "focus-output",
            "previous",
        ],
        vec![
            "map",
            "normal",
            "Super+Shift",
            "Period",
            "send-to-output",
            "next",
        ],
        vec![
            "map",
            "normal",
            "Super+Shift",
            "Comma",
            "send-to-output",
            "previous",
        ],
        vec![
            "map",
            "normal",
            "Super",
            "H",
            "send-layout-cmd",
            "rivertile",
            "main-ratio -0.05",
        ],
        vec![
            "map",
            "normal",
            "Super",
            "L",
            "send-layout-cmd",
            "rivertile",
            "main-ratio +0.05",
        ],
        vec![
            "map",
            "normal",
            "Super+Alt+Shift",
            "H",
            "resize",
            "horizontal -100",
        ],
        vec![
            "map",
            "normal",
            "Super+Alt+Shift",
            "J",
            "resize",
            "vertical 100",
        ],
        vec![
            "map",
            "normal",
            "Super+Alt+Shift",
            "K",
            "resize",
            "vertical -100",
        ],
        vec![
            "map",
            "normal",
            "Super+Alt+Shift",
            "L",
            "resize",
            "horizontal 100",
        ],
        vec!["map", "normal", "Super+Shift", "F", "toggle-float"],
        vec!["map", "normal", "Super", "F", "toggle-fullscreen"],
        vec![
            "map",
            "normal",
            "Super",
            "Up",
            "send-layout-cmd",
            "rivertile",
            "main-location top",
        ],
        vec![
            "map",
            "normal",
            "Super",
            "Right",
            "send-layout-cmd",
            "rivertile",
            "main-location right",
        ],
        vec![
            "map",
            "normal",
            "Super",
            "Down",
            "send-layout-cmd",
            "rivertile",
            "main-location bottom",
        ],
        vec![
            "map",
            "normal",
            "Super",
            "Left",
            "send-layout-cmd",
            "rivertile",
            "main-location left",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioMedia",
            "spawn",
            "playerctl play-pause",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioPlay",
            "spawn",
            "playerctl play-pause",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioPrev",
            "spawn",
            "playerctl previous",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioNext",
            "spawn",
            "playerctl next",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioRaiseVolume",
            "spawn",
            "pactl set-sink-volume @DEFAULT_SINK@ +5%",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioLowerVolume",
            "spawn",
            "pactl set-sink-volume @DEFAULT_SINK@ -5%",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86AudioMute",
            "spawn",
            "pactl set-sink-mute @DEFAULT_SINK@ toggle",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86MonBrightnessUp",
            "spawn",
            "brightnessctl s 5+",
        ],
        vec![
            "map",
            "normal",
            "None",
            "XF86MonBrightnessDown",
            "spawn",
            "brightnessctl s 5-",
        ],
    ]
    .into_iter()
    .map(riverctl);

    for handle in handles {
        handle.await?;
    }
    tags_handle.await?;
    startup_handle.await?;
    Ok(())
}

async fn tags() -> Result<()> {
    static MAP: &str = "map";
    static NORMAL: &str = "normal";
    static SET_FOCUS: &str = "set-focused-tags";
    static TOGGLE_FOCUS: &str = "toggle-focused-tags";
    static TOGGLE_VIEW: &str = "toggle-view-tags";
    static SET_VIEW: &str = "set-view-tags";

    for i in 1..=9 {
        let numb = format!("{}", i);
        let tag = format!("{}", 1 << (i - 1));

        let que: Vec<Vec<&str>> = vec![
            vec![MAP, NORMAL, "Super", &numb, SET_FOCUS, &tag],
            vec![MAP, NORMAL, "Super+Shift", &numb, SET_VIEW, &tag],
            vec![MAP, NORMAL, "Super+Control", &numb, TOGGLE_FOCUS, &tag],
            vec![MAP, NORMAL, "Super+Shift+Control", &numb, TOGGLE_VIEW, &tag],
        ];

        let mut handles = vec![];
        for command in que.iter() {
            handles.push(riverctl(command.to_vec()));
        }
        for handle in handles {
            handle.await?;
        }
    }

    let all_tags = format!("{}", (1u64 << 32) - 1);
    riverctl(vec![MAP, NORMAL, "Super", "0", SET_FOCUS, &all_tags]).await?;
    riverctl(vec![MAP, NORMAL, "Super+Shift", "0", SET_VIEW, &all_tags]).await?;
    Ok(())
}
async fn startup() -> Result<()> {
    let wall_handle = wall();
    let dbus_handle = start_proc(
        "dbus-update-activation-environment",
        vec![
            "SEATD_SOCK",
            "DISPLAY",
            "WAYLAND_DISPLAY",
            "XDG_SESSION_TYPE",
            "XDG_CURRENT_DESKTOP",
        ],
    );

    let handles = vec![
        restart_proc("waybar", vec![]),
        restart_proc("v2ray", vec!["run", "-c", "~/vmess-new.json"]),
        restart_proc("arti", vec!["proxy"]),
        restart_proc(
            "wl-paste",
            vec!["-t", "text", "--watch", "clipman", "store"],
        ),
    ];
    for handle in handles {
        handle.await?;
    }
    dbus_handle.await?;
    wall_handle.await?;
    start_proc(
        "rivertile",
        vec!["-view-padding", "05", "-outer-padding", "05"],
    )
    .await?;

    Ok(())
}
async fn wall() -> Result<()> {
    // restart_proc("swww", vec!["init"]).await?;
    // start_proc("swww", vec!["img", "~/pretty_12.png"]).await?;
    start_proc("swaybg", vec!["-i", "/home/a/pretty_12.png"]).await?;
    Ok(())
}

async fn riverctl<T: Sized + Display + AsRef<OsStr> + Debug>(args: Vec<T>) -> Result<()> {
    println!("{:#?}", &args);
    Command::new("riverctl")
        .args(args)
        .spawn()?
        .wait_with_output()
        .await?;
    Ok(())
}

async fn start_proc(process_name: &str, args: Vec<&str>) -> Result<()> {
    Command::new(process_name).args(&args).spawn()?;
    Ok(())
}

async fn restart_proc(process_name: &str, args: Vec<&str>) -> Result<()> {
    kill_procs(process_name).await?;
    Command::new(process_name)
        .args(&args)
        .spawn()?
        .wait_with_output()
        .await?;
    Ok(())
}

async fn kill_procs(process_name: &str) -> Result<()> {
    read_dir("/proc")?
        .into_iter()
        .filter(|ent| match ent {
            Ok(file) => match file.file_type() {
                Ok(ty) if is_proc_dir(&file.file_name()) => ty.is_dir(),
                _ => false,
            },
            Err(_) => false,
        })
        .filter_map(|ent| ent.ok())
        .filter(|x| {
            match fs::read_to_string(format!(
                "{:}/cmdline",
                format!("{:?}", x.path()).trim_matches('"')
            )) {
                Ok(ent) => ent.contains(process_name),
                Err(_) => false,
            }
        })
        .for_each(|x| {
            if let Some(name) = x.file_name().to_str() {
                if let Ok(p) = name.parse::<u32>() {
                    kill(p)
                }
            }
        });
    Ok(())
}

fn kill(pid: u32) {
    unsafe {
        _ = process::kill_process(process::Pid::from_raw(pid).unwrap(), process::Signal::Kill);
    }
}
fn is_proc_dir(file_name: &OsString) -> bool {
    match file_name.to_owned().into_string() {
        Ok(name) => !name.chars().any(|x| x.is_alphabetic()),
        _ => false,
    }
}
