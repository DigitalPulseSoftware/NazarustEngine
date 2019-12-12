#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Pressed,
    Released,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowEvent {
    Resized,
    Moved,
    CloseRequested,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    Moved,
    Button {
        state: State,
        mouse_button: MouseButton,
    },
    Scroll,
}
// https://docs.rs/winit/0.20.0-alpha5/winit/event/enum.VirtualKeyCode.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyEvent {
    Key1 { state: State },
    Key2 { state: State },
    Key3 { state: State },
    Key4 { state: State },
    Key5 { state: State },
    Key6 { state: State },
    Key7 { state: State },
    Key8 { state: State },
    Key9 { state: State },
    Key0 { state: State },
    A { state: State },
    B { state: State },
    C { state: State },
    D { state: State },
    E { state: State },
    F { state: State },
    G { state: State },
    H { state: State },
    I { state: State },
    J { state: State },
    K { state: State },
    L { state: State },
    M { state: State },
    N { state: State },
    O { state: State },
    P { state: State },
    Q { state: State },
    R { state: State },
    S { state: State },
    T { state: State },
    U { state: State },
    V { state: State },
    W { state: State },
    X { state: State },
    Y { state: State },
    Z { state: State },
    Escape { state: State },
    F1 { state: State },
    F2 { state: State },
    F3 { state: State },
    F4 { state: State },
    F5 { state: State },
    F6 { state: State },
    F7 { state: State },
    F8 { state: State },
    F9 { state: State },
    F10 { state: State },
    F11 { state: State },
    F12 { state: State },
    F13 { state: State },
    F14 { state: State },
    F15 { state: State },
    F16 { state: State },
    F17 { state: State },
    F18 { state: State },
    F19 { state: State },
    F20 { state: State },
    F21 { state: State },
    F22 { state: State },
    F23 { state: State },
    F24 { state: State },
    Snapshot { state: State },
    // Equivalent to Scroll variant in winit
    ScrollLock { state: State },
    Pause { state: State },
    Insert { state: State },
    Home { state: State },
    Delete { state: State },
    End { state: State },
    PageDown { state: State },
    PageUp { state: State },
    Left { state: State },
    Up { state: State },
    Right { state: State },
    Down { state: State },
    Back { state: State },
    // Equivalent to Return variant in winit
    Enter { state: State },
    Space { state: State },
    Compose { state: State },
    Caret { state: State },
    Numlock { state: State },
    Numpad0 { state: State },
    Numpad1 { state: State },
    Numpad2 { state: State },
    Numpad3 { state: State },
    Numpad4 { state: State },
    Numpad5 { state: State },
    Numpad6 { state: State },
    Numpad7 { state: State },
    Numpad8 { state: State },
    Numpad9 { state: State },
    AbntC1 { state: State },
    AbntC2 { state: State },
    Add { state: State },
    Apostrophe { state: State },
    Apps { state: State },
    At { state: State },
    Ax { state: State },
    Backslash { state: State },
    Calculator { state: State },
    Capital { state: State },
    Colon { state: State },
    Comma { state: State },
    Convert { state: State },
    Decimal { state: State },
    Divide { state: State },
    Equals { state: State },
    Grave { state: State },
    Kana { state: State },
    Kanji { state: State },
    LAlt { state: State },
    LBracket { state: State },
    LControl { state: State },
    LShift { state: State },
    LWin { state: State },
    Mail { state: State },
    MediaSelect { state: State },
    MediaStop { state: State },
    Minus { state: State },
    Multiply { state: State },
    Mute { state: State },
    MyComputer { state: State },
    NavigateForward { state: State },
    NavigateBackward { state: State },
    NextTrack { state: State },
    NoConvert { state: State },
    NumpadComma { state: State },
    NumpadEnter { state: State },
    NumpadEquals { state: State },
    OEM102 { state: State },
    Period { state: State },
    PlayPause { state: State },
    Power { state: State },
    PrevTrack { state: State },
    RAlt { state: State },
    RBracket { state: State },
    RControl { state: State },
    RShift { state: State },
    RWin { state: State },
    Semicolon { state: State },
    Slash { state: State },
    Sleep { state: State },
    Stop { state: State },
    Subtract { state: State },
    Sysrq { state: State },
    Tab { state: State },
    Underline { state: State },
    Unlabeled { state: State },
    VolumeDown { state: State },
    VolumeUp { state: State },
    Wake { state: State },
    WebBack { state: State },
    WebFavorites { state: State },
    WebForward { state: State },
    WebHome { state: State },
    WebRefresh { state: State },
    WebSearch { state: State },
    WebStop { state: State },
    Yen { state: State },
    Copy { state: State },
    Paste { state: State },
    Cut { state: State },
}
