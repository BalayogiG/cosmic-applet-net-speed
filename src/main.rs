mod net;
mod window;

use window::Window;

fn main() -> cosmic::iced::Result {
    let env = env_logger::Env::default()
        .filter_or("COSMIC_NET_SPEED_LOG", "warn")
        .write_style_or("COSMIC_NET_SPEED_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    cosmic::applet::run::<Window>(())
}
