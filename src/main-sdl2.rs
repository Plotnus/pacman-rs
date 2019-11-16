
fn main() -> std::result::Result<(), std::string::String> {
    let cache_line_size = sdl2::cpuinfo::cpu_cache_line_size();
    let num_cpus = sdl2::cpuinfo::cpu_count();
    dbg!(cache_line_size);
    dbg!(num_cpus);

    let sdl_context = sdl2::init()?;

    let mut event_pump = sdl_context.event_pump()?;

    let video_subsystem = sdl_context.video()?;
    let _window = video_subsystem.window("fun times", 1270, 720)
        .position_centered()
        .build()
        .unwrap();

    'mainloop: loop {

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {println!("Event::Quit"); break 'mainloop},
                sdl2::event::Event::AppTerminating {..} => {dbg!(event);},
                sdl2::event::Event::Window { win_event, ..} => {dbg!(win_event);},
                sdl2::event::Event::KeyDown{ keycode: Some(keycode), .. } => {dbg!(keycode);},
                _ => {}
            }
        }
    }

    Ok(())
}
