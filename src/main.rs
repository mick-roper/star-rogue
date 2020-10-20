use rltk::{Console, GameState, Rltk, RGB, Point};
use specs::prelude::*;

mod components;
use components::*;

mod player;
use player::{player_input, Player};

mod rect;
use rect::*;

mod map;
use map::*;

mod game_log;
use game_log::*;

mod damage_system;
use damage_system::{DamageSystem, delete_the_dead};

mod map_indexing_system;
use map_indexing_system::*;

mod melee_combat_system;
use melee_combat_system::*;

mod monster_ai_system;
use monster_ai_system::*;

mod vibility_system;
use vibility_system::*;

mod gui;
use gui::{draw_ui};

mod spawner;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        
        let mut map_index = MapIndexingSystem{};
        map_index.run_now(&self.ecs);

        let mut melee_combat = MeleeCombatSystem{};
        melee_combat.run_now(&self.ecs);

        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut new_run_state;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_run_state = *runstate;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_run_state;
        }

        delete_the_dead(&mut self.ecs);
        draw_map(&self.ecs, ctx);

        // draw objects
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.tile_is_visible(pos.x, pos.y) {
                ctx.set(
                    pos.x,
                    pos.y,
                    render.foreground,
                    render.background,
                    render.glyph,
                )
            }
        }

        draw_ui(&self.ecs, ctx);
    }
}

fn main() {
    const MAPWIDTH: i32 = 80;
    const MAPHEIGHT: i32 = 43;

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Star Rogue").build();

    let state = build_state(MAPWIDTH, MAPHEIGHT);

    rltk::main_loop(context, state);
}

fn build_state(width: i32, height: i32) -> State {
    let map = Map::new(width, height);
    let mut gs = State { ecs: World::new() };
    let mut rng = rltk::RandomNumberGenerator::new();
    // register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let (player_x, player_y) = map.get_room(0).centre();

    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // create the player
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // create some enemies
    for i in 1..map.get_room_count() { // skip the first room
        let (x, y) = map.get_room(i).centre();
        spawner::random_monster(&mut gs.ecs, x, y);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog{ entries: vec!["Welcome to Star Rogue!".to_string()] });

    gs
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let wall: u8 = rltk::to_cp437('#');
    let path: u8 = rltk::to_cp437('.');
    let black = RGB::named(rltk::BLACK);

    let (width, height) = map.get_dimensions();

    for x in 0..width {
        for y in 0..height {
            if map.tile_is_revealed(x, y) {
                let tile = map.get_tile(x, y);
                let glyph;
                let mut fg;
                match tile {
                    TileType::Floor => {
                        glyph = path;
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Wall => {
                        glyph = wall;
                        fg = RGB::from_f32(0., 1.0, 0.);
                    }
                };

                if !map.tile_is_visible(x, y) {
                    fg = fg.to_greyscale()
                }
                ctx.set(x, y, fg, black, glyph);
            }
        }
    }
}
