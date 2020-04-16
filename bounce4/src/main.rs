use pancurses::{Window, Input, initscr, endwin};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(window: &Window) -> Game {
        let (max_y, max_x) = window.get_max_yx();
        let frame = Frame {
            width: (max_x - 4) as u32,
            height: (max_y - 4) as u32,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Game { frame, ball }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

fn main() -> Result<(), String> {
    let window = initscr();

    window.timeout(33);
    let mut game = Game::new(&window);

    loop {
        window.clear();
        window.border('|', '|', '-', '-', '+', '+', '+', '+');
        window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
        window.mv(0, 0);
        window.refresh();

        match window.getch() {
            Some(Input::Character('q')) => {
                endwin();
                println!("Thanks for wathcing!");
                return Ok(());
            },
            Some(Input::KeyResize) => {
                game = Game::new(&window);
            },
            _ => game.step()
        };
    }
}
