
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

    let target_fps = 60.0;
    let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / target_fps);

    'mainloop: loop {
        let frame_start_time = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {println!("Event::Quit"); break 'mainloop},
                sdl2::event::Event::AppTerminating {..} => {dbg!(event);},
                sdl2::event::Event::Window { win_event, ..} => {dbg!(win_event);},
                sdl2::event::Event::KeyDown{ keycode: Some(keycode), .. } => {dbg!(keycode);},
                _ => {}
            }
        }

        // idle
        if frame_start_time.elapsed() > target_frame_duration {
            let elapsed_time = frame_start_time.elapsed().as_secs_f64();
            let target_time = target_frame_duration.as_secs_f64();
            println!("WARNING: OVER TIME BUDGET BY {:.2}%)" ,(elapsed_time / target_time) - 1.0);
        } else {
            // waste time
            // reason added: this is much more accurate than a call to `std::thread::sleep`
            // a goal of this engine is a high level of precision on per frame updates
            while frame_start_time.elapsed() < target_frame_duration {
            }
        }

        println!("FPS: {:.2}", 1.0 / frame_start_time.elapsed().as_secs_f64());
        dbg!(frame_start_time.elapsed().as_micros());
        
    }

    Ok(())
}
