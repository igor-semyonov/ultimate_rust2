use rand::prelude::*;
use rusty_engine::prelude::*;

mod score;
use score::Score;


#[derive(Resource)]
// #[allow(dead_code)]
struct GameState {
    high_score: Score,
    score: Score,
    direction: Vec2,
    feris_index: u32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: Score::new(
                0, "High ",
            ),
            score: Score::default(),
            direction: Vec2::new(
                1.0, 0.0,
            ),
            feris_index: 0,
            spawn_timer: Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            ),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(
        Window {
            title: "Tutorial".to_string(),
            mode: WindowMode::Windowed,
            // resolution: WindowResolution::new(3440.0,
            // 1440.0), width: 2400,
            // height: 1200,
            ..Default::default()
        },
    );

    let player = game.add_sprite(
        "player",
        SpritePreset::RacingCarBlue,
    );
    player.translation = Vec2::new(
        0.0, 0.0,
    );
    // player.rotation = std::f32::consts::PI / 3.0;
    player.scale = 1.3;
    player.collision = true;

    let score = game.add_text(
        "score", "Score: 0",
    );
    score.translation = Vec2::new(
        520.0, 320.0,
    );
    let high_score = game.add_text(
        "high_score",
        "High Score: 0",
    );
    high_score.translation = Vec2::new(
        -520.0, 320.0,
    );

    game.audio_manager
        .play_music(
            MusicPreset::Classy8Bit,
            0.1,
        );
    game.add_logic(game_logic);

    game.run(GameState::default());
}

fn game_logic(
    engine: &mut Engine,
    game_state: &mut GameState,
) {
    if engine
        .keyboard_state
        .just_pressed(KeyCode::Q)
    {
        engine.should_exit = true;
    }

    let score = engine
        .texts
        .get_mut("score")
        .unwrap();
    score
        .translation
        .x = engine
        .window_dimensions
        .x
        / 2.0
        - 80.0;
    let score_y_offset = ((engine.time_since_startup_f64
        * 7.0)
        .sin()
        * 10.0) as f32;
    score
        .translation
        .y = engine
        .window_dimensions
        .y
        / 2.0
        - 30.0
        + score_y_offset;
    let high_score = engine
        .texts
        .get_mut("high_score")
        .unwrap();
    high_score
        .translation
        .x = -engine
        .window_dimensions
        .x
        / 2.0
        + 100.0;
    high_score
        .translation
        .y = engine
        .window_dimensions
        .y
        / 2.0
        - 30.0;

    for event in engine
        .collision_events
        .drain(..)
    {
        #[allow(clippy::single_match)]
        match event.state {
            CollisionState::Begin => {
                if event
                    .pair
                    .one_starts_with("player")
                {
                    for label in event.pair {
                        if label != "player" {
                            engine
                                .sprites
                                .remove(&label);
                        }
                    }
                    game_state.score += 1;
                    let score = engine
                        .texts
                        .get_mut("score")
                        .unwrap();
                    score.value = game_state
                        .score
                        .to_string();

                    if game_state.score
                        > game_state.high_score
                    {
                        game_state
                            .high_score
                            .value = game_state
                            .score
                            .value;
                        let high_score = engine
                            .texts
                            .get_mut("high_score")
                            .unwrap();
                        high_score.value = game_state
                            .high_score
                            .to_string();
                    }
                    engine
                        .audio_manager
                        .play_sfx(
                            SfxPreset::Jingle1,
                            0.5,
                        );
                }
            }
            _ => {}
        };
    }

    // handle movement
    const MOVEMENT_SPEED: f32 = 400.0;
    let mut new_direction = Vec2::new(
        0.0, 0.0,
    );
    let player = engine
        .sprites
        .get_mut("player")
        .unwrap();
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        new_direction.y += MOVEMENT_SPEED;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::R])
    {
        new_direction.y -= MOVEMENT_SPEED;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        new_direction.x -= MOVEMENT_SPEED;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::S])
    {
        new_direction.x += MOVEMENT_SPEED;
    }
    player.translation += new_direction * engine.delta_f32;

    if new_direction
        .abs()
        .max_element()
        != 0.0
    {
        game_state.direction = new_direction;
    }
    player.rotation = f32::atan(
        game_state
            .direction
            .y
            / game_state
                .direction
                .x,
    );
    if game_state
        .direction
        .x
        == -MOVEMENT_SPEED
    {
        player.rotation += std::f32::consts::PI;
    }

    // handle mouse
    if engine
        .mouse_state
        .just_pressed(MouseButton::Left)
    {
        if let Some(mouse_location) = engine
            .mouse_state
            .location()
        {
            let feris_label = format!(
                "feris{}",
                game_state.feris_index
            );
            game_state.feris_index += 1;
            let feris = engine.add_sprite(
                feris_label.clone(),
                "cuddly-feris.png",
            );
            feris.translation = mouse_location;
            feris.scale = 0.4;
            feris.collision = true;
        }
    }

    // spawn new feris's
    if game_state
        .spawn_timer
        .tick(engine.delta)
        .just_finished()
    {
        let feris_label = format!(
            "feris{}",
            game_state.feris_index
        );
        game_state.feris_index += 1;
        let feris = engine.add_sprite(
            feris_label.clone(),
            "cuddly-feris.png",
        );
        feris
            .translation
            .x = thread_rng().gen_range(-550.0..550.0);
        feris
            .translation
            .y = thread_rng().gen_range(-305.0..305.0);
        feris.scale = 0.4;
        feris.collision = true;
        engine
            .audio_manager
            .play_sfx(
                SfxPreset::Minimize1,
                0.5,
            );
    }

    // reset game
    if engine
        .keyboard_state
        .just_pressed(KeyCode::G)
    {
        game_state.score = Score::default();
        let score = engine
            .texts
            .get_mut("score")
            .unwrap();
        score.value = "Score: 0".to_string();
    }
}
