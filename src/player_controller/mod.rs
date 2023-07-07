use event_emitter_rs::EventEmitter;

pub mod next_music;
pub mod play_stop;
pub mod prev_music;
pub mod set_random_music;
pub mod set_repeat_music;
pub mod set_repeat_playlist;
pub mod unset_random_music;
pub mod util;

pub fn register_events(event: &mut EventEmitter) {
    event.on("prev_music", move |value: String| {
        println!("prev_music: {}", value);

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let response = prev_music::prev_music().await;
            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("next_music", move |value: String| {
        println!("next_music: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = next_music::next_music().await;
            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("play_stop", move |value: String| {
        println!("play_stop: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = play_stop::play_stop().await;

            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("unset_random_music", move |value: String| {
        println!("unset_random_music: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = unset_random_music::unset_random_music().await;

            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("set_random_music", move |value: String| {
        println!("set_random_music: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = set_random_music::set_random_music().await;

            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("set_repeat_playlist", move |value: String| {
        println!("set_repeat_playlist: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = set_repeat_playlist::set_repeat_playlist().await;

            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
    event.on("set_repeat_music", move |value: String| {
        println!("set_repeat_music: {}", value);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let response = set_repeat_music::set_repeat_music().await;

            match response {
                Ok(res) => match res.text().await {
                    Ok(message) => println!("message: {}", message),
                    Err(err) => println!("parse message err: {}", err),
                },
                Err(err) => println!("err: {}", err),
            }
        });
    });
}
