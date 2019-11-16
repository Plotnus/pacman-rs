
fn main() -> std::result::Result<(), std::string::String> {
    let cache_line_size = sdl2::cpuinfo::cpu_cache_line_size();
    let num_cpus = sdl2::cpuinfo::cpu_count();
    dbg!(cache_line_size);
    dbg!(num_cpus);

    Ok(())
}
