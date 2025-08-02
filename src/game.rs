use crossterm::event::{self, Event, KeyCode};
use crate::game::snake::{ Snake, QLearningSnake };
use crate::game::snake::Direction;
use crate::game::map::Map;
use std::process::exit;

mod snake;
mod map;

pub struct Game {
    ai: QLearningSnake,
    apples: usize,
    map: Map,
    snake: Snake,
}

impl Game {
    pub fn start_game(width: usize, height: usize) -> Self {
        let ai = QLearningSnake::new();
        let mut map = Map::new(width, height);
        let snake = Snake::new(width / 2, height / 2);

        let apples = 2;
        for _ in 0..apples {
            map.place_food(&[(width / 2, height / 2)]);
        }

        Game { ai, apples, map, snake }
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_snake(&mut self) -> &mut Snake {
        &mut self.snake
    }

    fn update(&mut self) {
        let state = self.ai.encode_state(&self.snake, self.food_pos, self.map.get_size());
    
        let action = self.ai.decide(state);
        self.snake.set_dir(action); // меняем направление
        self.ai.remember_action(action); // нужно для обучения
    
        // 3. Двигаем змейку
        let prev_head = self.snake.head_position();
        self.snake.update(self.map.get_size());
        let new_head = self.snake.head_position();
    
        // 4. Вычисляем награду
        let mut reward = -0.1; // штраф за шаг (чтобы не стояла)
    
        if new_head == self.food_pos {
            self.snake.grow();
            self.map.place_food();
            reward = 10.0;
        } else if self.snake.is_collision(new_head) {
            reward = -100.0;
            self.reset(); // или закончить эпизод
        }
    
        // 5. Обновляем Q-таблицу
        let new_state = self.ai.encode_state(&self.snake, self.food_pos, self.map.get_size());
        self.ai.learn(new_state, reward);
    }
    
 
    // pub fn update(&mut self) {
    //     let snake_coords = self.snake.get_body();
    //     self.map.apply_cells(snake_coords, Cell::Snake);
    //     Game::handle_input(&mut self.snake);
    //     self.snake.update(self.get_map().get_size());

    //     if self.map.count_food() < self.apples {
    //         self.map.place_food();
    //         self.snake.grow();
    //     }
        
    //     if self.is_game_over() {
    //         println!("Игра окончена!");
    //         exit(0);
    //     }
    // }

    fn is_game_over(&self) -> bool {
        let head = self.snake.head_position();
        if !self.map.is_inside(head.0, head.1) || self.snake.is_collision(head) {
            return true;
        }
        false
    }

    fn handle_input(snake: &mut Snake) {
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => snake.set_dir(Direction::Up),
                    KeyCode::Down => snake.set_dir(Direction::Down),
                    KeyCode::Left => snake.set_dir(Direction::Left),
                    KeyCode::Right => snake.set_dir(Direction::Right),
                    KeyCode::Char('q') => exit(0),
                    _ => {}
                }
            }
        }
    }
}