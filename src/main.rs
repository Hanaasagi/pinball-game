#![feature(iterator_step_by)]
#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate lazy_static;

use stdweb::web;
use std::sync::Mutex;
use std::f32::consts::PI;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const WINNER_SCORE: i32 = 5;

struct Game {
    context: stdweb::Value,
    ball_x: f32,
    ball_y: f32,
    ball_r: f32,
    ball_vx: f32,
    ball_vy: f32,
    panel_w: f32,
    panel_h: f32,
    panel_1y: f32,
    panel_2y: f32,
    player1_score: i32,
    player2_score: i32,
    is_end: bool,

}

impl Game {

    fn new() -> Game {
        let canvas = web::document().get_element_by_id("viewport").unwrap();
        let panel_w = 10.0;
        let panel_h = 100.0;
        let game = Game {
            context: js!( return @{&canvas}.getContext("2d"); ),
            ball_x: WIDTH / 2.0,
            ball_y: HEIGHT / 2.0,
            ball_r: 10.0,
            ball_vx: 10.0,
            ball_vy: 2.0,
            panel_w: panel_w,
            panel_h: panel_h,
            panel_1y: (HEIGHT - panel_h) / 2.0,
            panel_2y: (HEIGHT - panel_h) / 2.0,
            player1_score: 0,
            player2_score: 0,
            is_end: false,
        };
        return game;
    }

    fn fill_text(&self, text: &str,
                 x: f32, y: f32, font: &str, style: &str) {
        js!(
            @{&self.context}.fillStyle = @{style};
            @{&self.context}.font = @{font};
            @{&self.context}.textAlign = "center";
            @{&self.context}.textBaseline = "middle";
            @{&self.context}.fillText(@{text}, @{x}, @{y});
        );
    }

    fn fill_circle(&self, x: f32, y: f32, r: f32, style: &str) {
        js!(
            @{&self.context}.fillStyle = @{style};
            @{&self.context}.beginPath();
            @{&self.context}.arc(@{x}, @{y}, @{r}, 0.0, @{PI} * 2);
            @{&self.context}.fill();
        );
    }

    fn fill_rect(&self, x: f32, y: f32, w: f32, h: f32, style: &str) {
        js!(
            @{&self.context}.fillStyle = @{style};
            @{&self.context}.fillRect(@{x}, @{y}, @{w}, @{h});
        );
    }

    fn draw_net(&self) {
        for i in (0..HEIGHT as i32).step_by(40) {
            self.fill_rect(WIDTH/2.0-1.0, i as f32+10.0, 2.0, 20.0, "white")
        }
    }
    fn animate(&mut self){
        self.fill_rect(0.0, 0.0, WIDTH, HEIGHT, "black");
        if self.is_end {
            if self.player1_score >= WINNER_SCORE {
                self.fill_text("You win", WIDTH / 2.0, HEIGHT / 2.0, "40px DejaVu Sans Mono", "white");
            } else {
                self.fill_text("You lose", WIDTH / 2.0, HEIGHT / 2.0, "40px DejaVu Sans Mono", "white");
            }
            self.fill_text("click to play again", WIDTH / 2.0, HEIGHT / 3.0 * 2.0, "32px DejaVu Sans Mono", "white");
            return;
        }
        self.draw_net();
        if (self.panel_2y + self.panel_h / 2.0) < (self.ball_y - 40.0) {
            self.panel_2y += 5.0;
        } else if (self.panel_2y + self.panel_h / 2.0) > (self.ball_y + 40.0) {
            self.panel_2y -= 5.0;
        }
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;
        if self.ball_x - self.ball_r - self.panel_w < 0.0 {
            if (self.ball_y > self.panel_1y) && (self.ball_y < (self.panel_1y + self.panel_h)) {
                self.ball_vx = -self.ball_vx;
                self.ball_vy = (self.ball_y - (self.panel_1y + self.panel_h / 2.0)) * 0.3;
            } else {
                self.player2_score += 1;
                if self.player1_score >= WINNER_SCORE || self.player2_score >= WINNER_SCORE {
                    self.is_end = true;
                }
                self.ball_vx = -self.ball_vx;
                self.ball_x = WIDTH / 2.0;
                self.ball_y = HEIGHT / 2.0;
            }
        }
        if self.ball_x + self.ball_r + self.panel_w > WIDTH {
            if (self.ball_y > self.panel_2y) && (self.ball_y < self.panel_2y + self.panel_h) {
                self.ball_vx = -self.ball_vx;
                self.ball_vy = (self.ball_y - (self.panel_2y + self.panel_h / 2.0)) * 0.3;
            } else {
                self.player1_score += 1;
                if self.player1_score >= WINNER_SCORE || self.player2_score >= WINNER_SCORE {
                    self.is_end = true;
                }
                self.ball_vx = -self.ball_vx;
                self.ball_x = WIDTH / 2.0;
                self.ball_y = HEIGHT / 2.0;
            }
        }

        if (self.ball_y + self.ball_r < 0.0) || (self.ball_y - self.ball_r > HEIGHT) {
            self.ball_vy = -self.ball_vy;
        }

        self.fill_rect(1.0, self.panel_1y, self.panel_w, self.panel_h, "white");
        self.fill_rect(WIDTH - self.panel_w - 1.0, self.panel_2y, self.panel_w, self.panel_h, "white");
        self.fill_circle(self.ball_x, self.ball_y, self.ball_r, "white");
        self.fill_text(&self.player1_score.to_string(), 100.0, 100.0, "40px DejaVu Sans Mono", "white");
        self.fill_text(&self.player2_score.to_string(), WIDTH - 100.0, 100.0, "40px DejaVu Sans Mono", "white");
    }

}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

fn game_loop() {
    {
        let mut game = GAME.lock().unwrap();
        game.animate()
    }

    web::window().request_animation_frame(move |_| {
        game_loop();
    });
}

fn on_move(y: f64) {
    let mut game = GAME.lock().unwrap();
    game.panel_1y = y as f32;
}

fn on_click() {
    let mut game = GAME.lock().unwrap();
    if game.is_end {
        game.player1_score = 0;
        game.player2_score = 0;
        game.is_end = false;
        game.animate();
    }
}

fn main() {
    stdweb::initialize();

    let canvas = web::document().get_element_by_id("viewport").unwrap();
    let game = GAME.lock().unwrap();
    js!(
        Module.exports.deviceClickCallback = @{on_click};
        Module.exports.deviceMoveCallback = @{on_move};
        @{&canvas}.width = 800;
        @{&canvas}.height = 600;
        @{&canvas}.addEventListener("click", function() {
            Module.exports.deviceClickCallback();
        });
        @{&canvas}.addEventListener("mousemove", function (e) {
            Module.exports.deviceMoveCallback(e.clientY - @{&canvas}.getBoundingClientRect().top - @{game.panel_h} / 2)
        });
    );
    web::window().request_animation_frame( |_| {
        game_loop();
    });

    stdweb::event_loop();
}
