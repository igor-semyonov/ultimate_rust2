use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    loss: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 10,
            loss: false,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let mut game_state = GameState::default();

    let player = game.add_sprite(
        "player",
        SpritePreset::RacingCarBlue,
    );
    player.translation = Vec2::new(
        -500.0, 0.0,
    );
    player.layer = 10.0;
    player.collision = true;

    for i in 0..10 {
        let roadline = game.add_sprite(
            format!(
                "roadline{}",
                i
            ),
            SpritePreset::RacingBarrierWhite,
        );
        roadline.scale = 0.1;
        roadline
            .translation
            .x = -600.0 + 150.0 * i as f32;
        roadline.layer = 0.0;
    }

    // health message
    let health_message = game.add_text(
        "health_message",
        format!(
            "Health: {}",
            &game_state.health_amount
        ),
    );
    health_message
        .translation
        .x = -360.0;
    health_message
        .translation
        .y = 310.0;
    health_message.font_size = 128.0;
    health_message.layer = 0.0;

    game.audio_manager
        .play_music(
            MusicPreset::WhimsicalPopsicle,
            0.2,
        );

    // Create obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_presets
        .into_iter()
        .enumerate()
    {
        let obstacle = game.add_sprite(
            format!(
                "obstacle{}",
                i
            ),
            preset,
        );
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle
            .translation
            .x = thread_rng().gen_range(800.0..1600.0);
        obstacle
            .translation
            .y = thread_rng().gen_range(-300.0..300.0);
    }

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(
    engine: &mut Engine,
    game_state: &mut GameState,
) {
    if game_state.loss {
        return;
    }

    let mut direction = 0.0;
    const MOVEMENT_SPEED: f32 = 700.0;
    const ROAD_SPEED: f32 = 400.0;
    if engine
        .keyboard_state
        .pressed(KeyCode::W)
    {
        direction += 1.0;
    }
    if engine
        .keyboard_state
        .pressed(KeyCode::R)
    {
        direction -= 1.0;
    }
    let player = engine
        .sprites
        .get_mut("player")
        .unwrap();
    player.rotation = direction * 0.15;
    player
        .translation
        .y += direction * engine.delta_f32 * MOVEMENT_SPEED;
    if player
        .translation
        .y
        >= 320.0
        || player
            .translation
            .y
            <= -320.0
    {
        game_state.health_amount = 0;
        engine
            .texts
            .get_mut("health_message")
            .unwrap()
            .value = format!(
            "Health: {}",
            &game_state.health_amount
        );
    }

    // move road left
    for sprite in engine
        .sprites
        .values_mut()
    {
        if sprite
            .label
            .starts_with("roadline")
        {
            sprite
                .translation
                .x -= ROAD_SPEED * engine.delta_f32;
            if sprite
                .translation
                .x
                < -675.0
            {
                sprite
                    .translation
                    .x += 1500.0
            };
        } else if sprite
            .label
            .starts_with("obstacle")
        {
            sprite
                .translation
                .x -= ROAD_SPEED * engine.delta_f32;
            if sprite
                .translation
                .x
                < -800.0
            {
                sprite
                    .translation
                    .x =
                    thread_rng().gen_range(800.0..1600.0);
                sprite
                    .translation
                    .y =
                    thread_rng().gen_range(-300.0..300.0);
            };
        };
    }

    // handle collisions
    let mut health_message = engine
        .texts
        .get_mut("health_message")
        .unwrap();
    for event in engine
        .collision_events
        .drain(..)
    {
        if !event
            .pair
            .either_contains("player")
        // || event
        //     .state
        //     .is_end()()
        {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!(
                "Health: {}",
                &game_state.health_amount
            );
            engine
                .audio_manager
                .play_sfx(
                    SfxPreset::Impact3,
                    0.5,
                );
        }
    }

    // loss condition
    if game_state.health_amount <= 0 {
        game_state.loss = true;
        let game_over = engine.add_text("game_over", "Game Over!");
        game_over.font_size = 256.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 1.0);
    }
}
