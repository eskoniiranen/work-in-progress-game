// src/main.rs

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BattleTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(tick_timer)
        .add_system(character_attack)
        .add_system(boss_attack)
        .add_system(apply_damage)
        .run();
}

// === Components ===

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Boss;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Skill {
    damage: i32,
    range: f32,
    cooldown: Timer,
}

#[derive(Component)]
struct Target(Entity);

// === Resources ===

#[derive(Resource)]
struct BattleTimer(Timer);

struct DamageEvent {
    target: Entity,
    amount: i32,
}

// === Setup ===

fn setup(mut commands: Commands) {
    let boss = commands
        .spawn((
            Boss,
            Health(100),
            Position(Vec2::new(5.0, 5.0)),
        ))
        .id();

    commands.spawn((
        Character,
        Health(50),
        Position(Vec2::new(0.0, 0.0)),
        Skill {
            damage: 10,
            range: 6.0,
            cooldown: Timer::from_seconds(2.0, TimerMode::Once),
        },
        Target(boss),
    ));
}

// === Systems ===

fn tick_timer(time: Res<Time>, mut timer: ResMut<BattleTimer>) {
    timer.0.tick(time.delta());
}

fn character_attack(
    time: Res<Time>,
    timer: Res<BattleTimer>,
    mut commands: Commands,
    mut query: Query<(&mut Skill, &Position, &Target), With<Character>>,
    boss_pos: Query<&Position, With<Boss>>,
) {
    if !timer.0.just_finished() {
        return;
    }

    for (mut skill, pos, target) in &mut query {
        skill.cooldown.tick(time.delta());

        if skill.cooldown.finished() {
            if let Ok(target_pos) = boss_pos.get(target.0) {
                if pos.0.distance(target_pos.0) <= skill.range {
                    println!("Character attacks boss for {} damage", skill.damage);
                    commands.spawn().insert(DamageEvent {
                        target: target.0,
                        amount: skill.damage,
                    });
                    skill.cooldown.reset();
                }
            }
        }
    }
}

fn boss_attack(
    timer: Res<BattleTimer>,
    mut commands: Commands,
    query: Query<Entity, With<Character>>,
) {
    if !timer.0.just_finished() {
        return;
    }

    for entity in &query {
        println!("Boss hits character for 5 damage");
        commands.spawn().insert(DamageEvent {
            target: entity,
            amount: 5,
        });
    }
}

fn apply_damage(
    mut commands: Commands,
    mut events: Query<(Entity, &DamageEvent)>,
    mut health_query: Query<&mut Health>,
) {
    for (event_entity, event) in &mut events {
        if let Ok(mut hp) = health_query.get_mut(event.target) {
            hp.0 -= event.amount;
            println!("Entity {:?} takes {} damage, {} HP left", event.target, event.amount, hp.0);
        }
        commands.entity(event_entity).despawn();
    }
}
