use glfw::{Action, Context, Key, WindowHint, fail_on_errors};

fn main() {
    let WINDOW_WIDTH: i32 = 1500;
    let WINDOW_HEIGHT: i32 = 800;

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(WindowHint::Resizable(false));
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
            "Mine",
            glfw::WindowMode::Windowed,
        )
        .unwrap();
    window.set_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    window.set_key_polling(true);

    let (width, height) = window.get_size();
    println!("Window Initiated with size: {}x{}", width, height);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
        window.swap_buffers();
    }
}
