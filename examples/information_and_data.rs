//! Information and data rates — streaming, gaming, and storage.
//!
//! Run with: `cargo run --example information_and_data`

use rquants::prelude::*;

fn main() {
    println!("==========================================");
    println!("  RQuants Information — Digital Life");
    println!("==========================================\n");

    storage_sizes();
    download_times();
    streaming_math();
}

/// How big is your stuff?
fn storage_sizes() {
    println!("--- Storage Sizes ---\n");

    // A single high-res photo: ~5 MB
    let photo = Information::megabytes(5.0);
    let phone_storage = Information::gigabytes(256.0);
    let photos_fit = phone_storage.to_megabytes() / photo.to_megabytes();
    println!(
        "256 GB phone fits {:.0} photos at {:.0} MB each",
        photos_fit,
        photo.to_megabytes()
    );

    // A 4K movie: ~100 GB on disc
    let movie_4k = Information::gigabytes(100.0);
    println!(
        "4K Blu-ray: {:.0} GB = {:.0} MB = {:.0} Gb",
        movie_4k.to_gigabytes(),
        movie_4k.to_megabytes(),
        movie_4k.to_gigabits()
    );

    // GTA VI estimated install: ~150 GB
    let gta6 = Information::gigabytes(150.0);
    let ssd = Information::terabytes(1.0);
    let games_fit = ssd.to_gigabytes() / gta6.to_gigabytes();
    println!(
        "GTA VI ({:.0} GB): fits {:.0}x on a {:.0} TB SSD",
        gta6.to_gigabytes(),
        games_fit,
        ssd.to_terabytes()
    );

    // All of Wikipedia: ~22 GB (text only)
    let wikipedia = Information::gigabytes(22.0);
    println!(
        "All of Wikipedia (text): {:.0} GB = {:.0} MB — fits on your phone!",
        wikipedia.to_gigabytes(),
        wikipedia.to_megabytes()
    );

    // YouTube uploads per minute: ~500 hours of video = ~750 GB
    let youtube_per_min = Information::gigabytes(750.0);
    let per_hour = youtube_per_min.to_terabytes() * 60.0;
    println!(
        "YouTube upload rate: {:.0} GB/min = {:.1} TB/hour",
        youtube_per_min.to_gigabytes(),
        per_hour
    );

    println!();
}

/// How long to download stuff?
fn download_times() {
    println!("--- Download Times ---\n");

    // Typical home internet: 300 Mbps
    let home_speed = DataRate::megabits_per_second(300.0);

    // Download a game (80 GB)
    let game = Information::gigabytes(80.0);
    let game_time = game / home_speed;
    println!(
        "80 GB game at {:.0} Mbps: {:.0} seconds = {:.1} minutes",
        home_speed.to_megabits_per_second(),
        game_time.to_seconds(),
        game_time.to_minutes()
    );

    // Netflix 4K stream: ~25 Mbps
    let netflix_4k = DataRate::megabits_per_second(25.0);
    let two_hour_movie = netflix_4k * Time::hours(2.0);
    println!(
        "2h Netflix 4K: {:.0} Mbps x 2h = {:.1} GB",
        netflix_4k.to_megabits_per_second(),
        two_hour_movie.to_gigabytes()
    );

    // 5G peak speed: ~10 Gbps
    let five_g = DataRate::gigabits_per_second(10.0);
    let movie_dl = Information::gigabytes(50.0);
    let dl_time = movie_dl / five_g;
    println!(
        "50 GB movie on 5G ({:.0} Gbps): {:.0} seconds",
        five_g.to_gigabits_per_second(),
        dl_time.to_seconds()
    );

    // Spotify song: ~10 MB, on 4G (~50 Mbps)
    let song = Information::megabytes(10.0);
    let lte = DataRate::megabits_per_second(50.0);
    let song_time = song / lte;
    println!(
        "Spotify song ({:.0} MB) on 4G: {:.1} seconds",
        song.to_megabytes(),
        song_time.to_seconds()
    );

    println!();
}

/// Streaming bandwidth math
fn streaming_math() {
    println!("--- Streaming Math ---\n");

    // Twitch stream at 1080p60: ~6 Mbps
    let stream_rate = DataRate::megabits_per_second(6.0);
    let stream_duration = Time::hours(4.0);
    let total_data = stream_rate * stream_duration;
    println!(
        "4h Twitch stream at {:.0} Mbps: {:.1} GB uploaded",
        stream_rate.to_megabits_per_second(),
        total_data.to_gigabytes()
    );

    // Zoom call: ~2.5 Mbps for HD video
    let zoom = DataRate::megabits_per_second(2.5);
    let meeting = Time::hours(1.0);
    let zoom_data = zoom * meeting;
    println!(
        "1h Zoom HD: {:.1} GB — careful on mobile data!",
        zoom_data.to_gigabytes()
    );

    // Monthly data cap: 1 TB. How many hours of 4K Netflix?
    let cap = Information::terabytes(1.0);
    let netflix_rate = DataRate::megabits_per_second(25.0);
    let max_hours = cap / netflix_rate;
    println!(
        "1 TB data cap / {:.0} Mbps 4K = {:.0} hours of Netflix",
        netflix_rate.to_megabits_per_second(),
        max_hours.to_hours()
    );

    println!("\nAll information examples done!");
}
