mod game {
    pub struct GameState {
        frame_count: usize,
        ycoord: usize,
        xcoord: usize,
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                frame_count: 0,
                ycoord: 0,
                xcoord: 3,
            }
        }
    }

    pub fn game_loop(mut state: RefMut<GameState>) {
        let ctx = get_context();

        state.frame_count += 1;
        if state.frame_count == 48 {
            state.frame_count = 0;
            state.ycoord += 1;
        }

        clear_board(&ctx);
        draw_piece(&ctx, Tetronimo::T, 
                Rotation::Up, 
                state.xcoord, state.ycoord);
    }

    pub fn move_piece(event: &KeyboardEvent, mut state: RefMut<GameState>) {
        match event.key().as_ref() {
            "ArrowLeft" => if state.xcoord > 0 { state.xcoord -= 1; }
            "ArrowRight" => if state.xcoord < 7 { state.xcoord += 1; }
            "ArrowUp" => state.rotation.increment(),
            "Control" => state.rotation.decrement(),
            _ => {},
        }
    }

    fn draw_piece(ctx: CanvasRenderingContext2d,
            shape: Tetronimo, rot: Rotation,
            xcoord: usize, ycoord: usize) {

        // Get Piece
        let piece = PIECES[shape as usize]
            [rot as usize];
        ctx.set_fill_style(&JsValue::from(format!(
                        "hsl({}, 50%, 50%)", 45*(shape as usize)
                        )));

        // Set aside piece dimensions
        const PIECE_WIDTH: usize = 4;
        const SQUARE_SIZE: f64 = 10.0;

        // Loop over the piece array and draw each square
        for i in 0..piece.len() {
            let spot_value = piece[i];

            if spot_value == 1 {
                let xcoord = i % PIECE_WIDTH;
                let ycoord = i / PIECE_WIDTH;
                ctx.fill_rect(xcoord as f64 * SQUARE_SIZE,
                        ycoord as f64 * SQUARE_SIZE,
                        SQUARE_SIZE, SQUARE_SIZE);

            }
        }
    }

    fn clear_board(ctx: &CanvasRenderingContext2d) {
        const BOARD_WIDTH: usize = 100;
        const BOARD_HEIGHT: usize = 200;

        ctx.set_fill_style(&JsValue::from("black"));
        ctx.fill_rect(0.0, 0.0, BOARD_WIDTH as f64,
                BOARD_HEIGHT as f64);
    }
}

mod pieces {
#[derive(Debug, Copy, Clone)]
    enum Tetronimo {
        L,
        O,
        S,
        I,
        J,
        T,
    }

#[derive(Debug, Copy, Clone)]
    enum Rotation {
        Up,
        Left,
        Down,
        Right,
    }

    impl Rotation {
        pub fn increment(&mut self) {
            *self = match *self {
                Rotation::Up => Rotation::Left,
                Rotation::Left => Rotation::Down,
                Rotation::Down => Rotation::Right,
                Rotation::Right => Rotation::Up,
            }
        }

        pub fn decrement(&mut self) {
            *self = match *self {
                Rotation::Up => Rotation::Right,
                Rotation::Right => Rotation::Down,
                Rotation::Down => Rotation::Left,
                Rotation::Left => Rotation::Up,
            }
        }
    }

    pub const PIECES: [[[u8; 16]; 4]; 7] = [
    [// Z
        [1, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 0, 1, 0,
        0, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        1, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 0]
    ], [// L
        [0, 0, 1, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        1, 0, 0, 0,
        0, 0, 0, 0],
        [1, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ], [// O
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
    ], [// S
        [0, 1, 1, 0,
        1, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 1, 0,
        0, 0, 1, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        0, 1, 1, 0,
        1, 1, 0, 0,
        0, 0, 0, 0],
        [1, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ], [// I
        [0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 0, 1, 0,
        0, 0, 1, 0,
        0, 0, 1, 0,
        0, 0, 1, 0],
        [0, 0, 0, 0,
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0],
    ], [// J
        [1, 0, 0, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 1, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        0, 0, 1, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 0, 0,
        1, 1, 0, 0,
        0, 0, 0, 0],
    ], [// T
        [0, 1, 0, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        0, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 0, 0, 0,
        1, 1, 1, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
        [0, 1, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0],
    ]];
}
