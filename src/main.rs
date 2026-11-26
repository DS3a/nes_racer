use macroquad::prelude::*;


mod racer;
mod track;
mod renderer;


use renderer::window_conf;


fn add_racer(racers_: &mut Vec<racer::Racer>, racer: racer::Racer) {
    racers_.push(racer);    
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut racers: Vec<racer::Racer> = Vec::new();
    add_racer(&mut racers, racer::Racer::new(1u32));
    add_racer(&mut racers, racer::Racer::new(2u32));
    // instanciate a track


    println!("Hello, world!");
    loop {
        racers[1].propagate(0.4);
        renderer::draw_background();
        renderer::draw_kart(&racers[0]);
        renderer::draw_kart(&racers[1]);
        draw_text(&format!("{} FPS", get_fps()), 10.0, 20.0, 20.0, WHITE);
        next_frame().await;
    }

}
