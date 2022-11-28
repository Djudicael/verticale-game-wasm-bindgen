use web_sys::CanvasRenderingContext2d;

const GRAVITY: f64 = 0.5;

#[derive(Clone, Copy)]
pub struct Player {
    pub position: Position,
    pub velocity: Velocity,
    pub height: f64,
}

impl Player {
    pub fn new(position: Position) -> Self {
        let velocity = Velocity { x: 0., y: 1. };
        Self {
            position,
            velocity,
            height: 100.,
        }
    }

    fn draw(&self, context: &mut CanvasRenderingContext2d) {
        context.set_fill_style(&"red".into());
        context.fill_rect(self.position.x, self.position.y, 100., self.height);
    }

    pub fn update(
        &mut self,
        context: &mut CanvasRenderingContext2d,
        canvas: &web_sys::HtmlCanvasElement,
    ) {
        self.draw(context);
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        if self.position.y + self.height + self.velocity.y < canvas.height() as f64 {
            self.velocity.y += GRAVITY;
        } else {
            self.velocity.y = 0.;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
