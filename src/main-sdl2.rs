
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

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < std::time::Duration::from_secs(2) {
        for event in event_pump.poll_iter() {
            dbg!(event);
        }
    }

    Ok(())
}
