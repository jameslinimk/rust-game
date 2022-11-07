use super::object::IDObject;
use super::objects::assets::load_image;
use super::objects::enemies::enemy::Enemy;
use super::objects::items::guns::GUNS;
use super::objects::objects_enum::Objects;
use super::objects::player::main::Player;
use super::objects::test::TestObj;
use super::room_gen::gen::{load_room, Objects as RoomObjects, ROOMS};
use crate::camera::Camera;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::hex;
use crate::{pub_global_variable, repeat_for_vec, repeat_function, Object};
use macroquad::prelude::{clear_background, rand::ChooseRandom, WHITE};

pub_global_variable!(GAME, _GAME, GameScene);

pub struct GameScene {
    pub player: Player,
    pub objects: Vec<Objects>,
    pub walls: Vec<Rect>,
    pub room: Vec<Vec<RoomObjects>>,
    pub enemies: Vec<Enemy>,
    pub camera: Camera,
}
impl GameScene {
    pub fn new() -> GameScene {
        let rooms = ROOMS.lock().unwrap();
        let rand_room = rooms.choose().unwrap();

        GameScene {
            player: Player::new(),
            objects: vec![Objects::from(TestObj::new())],
            // objects: vec![],
            room: rand_room.to_vec(),
            walls: load_room(rand_room),
            enemies: vec![Enemy::new(200.0, 200.0, 10.0)],
            camera: Camera::new(),
        }
    }

    pub async fn init(&self) {
        load_image("./assets/guns/border.png").await;
        for gun in GUNS {
            load_image(gun.image_file).await;
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        repeat_for_vec!(update, self.enemies, self.objects);
        repeat_function!(update, self.player, self.camera);
    }

    fn draw(&mut self) {
        clear_background(hex("#313639"));

        repeat_function!(draw, self.player);
        repeat_for_vec!(draw, self.objects, self.enemies);
        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
    }
}
