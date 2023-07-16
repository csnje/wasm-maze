// An implementation of some maze generating and solving algorithms in WebAssembly.

mod direction;
mod generate;
mod geometry;
mod solve;

use direction::{Direction, DIRECTIONS};
use geometry::row_and_col;

use js_sys::Math::random;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement,
    HtmlOptionElement, HtmlSelectElement,
};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Default number of cells
const DEFAULT_WIDTH: u32 = 20;
const DEFAULT_HEIGHT: u32 = 20;

// Number of pixels in each cell dimension
const CELL_PIXELS: u32 = 20;

// Fill and stroke styles
const BACKGROUND_STYLE: &str = "rgb(255,255,255)";
const CELL_BORDER_STYLE: &str = "rgb(0,0,0)";
const FROM_TO_STYLE: &str = "rgb(255,0,0)";
const SEARCH_STYLE: &str = "rgba(255,127,0,0.5)";
const RESULT_STYLE: &str = "rgb(255,0,0)";

// Stroke widths
const CELL_BORDER_WIDTH: f64 = 2.0;
const SEARCH_LINE_WIDTH: f64 = 2.0;
const RESULT_LINE_WIDTH: f64 = 4.0;

type Dimensions = (usize, usize);

/// Solution details for `Cell`.
#[derive(Clone, Default)]
struct CellSolution {
    from: bool,
    to: bool,
    previous: Option<usize>,
    result: bool,
}

/// A type for a cell in a maze.
#[derive(Clone)]
struct Cell {
    // bit per wall; 0 is removed, 1 is present
    walls: u8,
    // walk index from generator
    walk: Option<usize>,
    // solution details
    solution: CellSolution,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            walls: DIRECTIONS
                .iter()
                .fold(0, |accumulator, direction| accumulator + *direction as u8),
            walk: None,
            solution: CellSolution::default(),
        }
    }
}

impl Cell {
    /// Remove wall.
    fn remove_wall(&mut self, direction: Direction) {
        self.walls &= !(direction as u8);
    }

    /// Weather wall is present.
    fn has_wall(&self, direction: Direction) -> bool {
        self.walls & direction as u8 > 0
    }

    /// Draw into canvas.
    fn draw(&self, dimensions: Dimensions, idx: usize, context: &CanvasRenderingContext2d) {
        // Drawing references:
        // - https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Drawing_shapes
        // - https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Applying_styles_and_colors

        let (row, col) = row_and_col(dimensions, idx);
        let (x, y) = (col * CELL_PIXELS as usize, row * CELL_PIXELS as usize);
        match self.walk {
            Some(_) => {
                context.set_line_width(CELL_BORDER_WIDTH);
                context.set_stroke_style(&JsValue::from_str(CELL_BORDER_STYLE));
                context.begin_path();
                if self.has_wall(Direction::First) {
                    context.move_to(x as f64, y as f64);
                    context.line_to((x + CELL_PIXELS as usize) as f64, y as f64);
                }
                if self.has_wall(Direction::Second) {
                    context.move_to((x + CELL_PIXELS as usize) as f64, y as f64);
                    context.line_to(
                        (x + CELL_PIXELS as usize) as f64,
                        (y + CELL_PIXELS as usize) as f64,
                    );
                }
                if self.has_wall(Direction::Third) {
                    context.move_to(
                        (x + CELL_PIXELS as usize) as f64,
                        (y + CELL_PIXELS as usize) as f64,
                    );
                    context.line_to(x as f64, (y + CELL_PIXELS as usize) as f64);
                }
                if self.has_wall(Direction::Forth) {
                    context.move_to(x as f64, (y + CELL_PIXELS as usize) as f64);
                    context.line_to(x as f64, y as f64);
                }
                context.stroke();

                if self.solution.from {
                    context.set_fill_style(&JsValue::from_str(FROM_TO_STYLE));
                    context.begin_path();
                    let _ = context.arc(
                        x as f64 + CELL_PIXELS as f64 / 2.0,
                        y as f64 + CELL_PIXELS as f64 / 2.0,
                        CELL_PIXELS as f64 * 0.4,
                        0.0,
                        std::f64::consts::TAU,
                    );
                    context.fill();
                }

                if self.solution.to {
                    context.set_line_width(CELL_PIXELS as f64 * 0.1);
                    context.set_stroke_style(&JsValue::from_str(FROM_TO_STYLE));
                    context.begin_path();
                    let _ = context.arc(
                        x as f64 + CELL_PIXELS as f64 / 2.0,
                        y as f64 + CELL_PIXELS as f64 / 2.0,
                        CELL_PIXELS as f64 * 0.3,
                        0.0,
                        std::f64::consts::TAU,
                    );
                    context.stroke();
                }

                if let Some(previous) = self.solution.previous {
                    let (prev_row, prev_col) = row_and_col(dimensions, previous);
                    let (prev_x, prev_y) = (
                        prev_col * CELL_PIXELS as usize,
                        prev_row * CELL_PIXELS as usize,
                    );

                    context.set_line_width(match self.solution.result {
                        true => RESULT_LINE_WIDTH,
                        false => SEARCH_LINE_WIDTH,
                    });
                    context.set_stroke_style(&JsValue::from_str(match self.solution.result {
                        true => RESULT_STYLE,
                        false => SEARCH_STYLE,
                    }));
                    context.begin_path();
                    context.move_to(
                        prev_x as f64 + CELL_PIXELS as f64 / 2.0,
                        prev_y as f64 + CELL_PIXELS as f64 / 2.0,
                    );
                    context.line_to(
                        x as f64 + CELL_PIXELS as f64 / 2.0,
                        y as f64 + CELL_PIXELS as f64 / 2.0,
                    );
                    context.stroke();
                }
            }
            None => {
                context.set_fill_style(&JsValue::from_str(CELL_BORDER_STYLE));
                context.fill_rect(x as f64, y as f64, CELL_PIXELS as f64, CELL_PIXELS as f64);
            }
        }
    }
}

/// A type indicating the phase of the application.
#[derive(Clone)]
enum Phase {
    Generate,
    Solve,
    Complete,
}

fn window() -> web_sys::Window {
    web_sys::window().expect("should have window")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register request animation frame callback");
}

/// Entry point of the application.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = window().document().expect("should have document");
    let body = document.body().ok_or("should have document body")?;

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(DEFAULT_WIDTH * CELL_PIXELS);
    canvas.set_height(DEFAULT_HEIGHT * CELL_PIXELS);
    body.append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    context.set_line_cap("round");

    let div = document.create_element("div")?;
    div.set_text_content(Some("Generator"));
    body.append_child(&div)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let input_width = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    input_width.set_type("number");
    input_width.set_min("2");
    input_width.set_value(DEFAULT_WIDTH.to_string().as_str());
    div.append_child(&input_width)?;

    let label = document.create_element("label")?;
    label.set_text_content(Some("width"));
    div.append_child(&label)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let input_height = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    input_height.set_type("number");
    input_height.set_min("2");
    input_height.set_value(DEFAULT_HEIGHT.to_string().as_str());
    div.append_child(&input_height)?;

    let label = document.create_element("label")?;
    label.set_text_content(Some("height"));
    div.append_child(&label)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let select_generator = document
        .create_element("select")?
        .dyn_into::<HtmlSelectElement>()?;
    div.append_child(&select_generator)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let button_generator = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    button_generator.set_text_content(Some("Generate"));
    div.append_child(&button_generator)?;

    let div = document.create_element("div")?;
    div.set_text_content(Some("Solver"));
    body.append_child(&div)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let select_solver = document
        .create_element("select")?
        .dyn_into::<HtmlSelectElement>()?;
    div.append_child(&select_solver)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let input_from_to = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    input_from_to.set_type("checkbox");
    div.append_child(&input_from_to)?;

    let label = document.create_element("label")?;
    label.set_text_content(Some("with new locations"));
    div.append_child(&label)?;

    let div = document.create_element("div")?;
    body.append_child(&div)?;

    let button_solver = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    button_solver.set_text_content(Some("Solve"));
    button_solver.set_disabled(true);
    div.append_child(&button_solver)?;

    // setup generators
    let mut generators: BTreeMap<String, fn() -> Box<dyn generate::Generator>> = BTreeMap::new();
    generators.insert("Wilson's algorithm".to_string(), || {
        Box::<generate::Wilson>::default()
    });
    generators.insert(
        "Randomised depth first search algorithm".to_string(),
        || Box::<generate::RandomisedDepthFirstSearch>::default(),
    );
    for name in generators.keys() {
        let option = document
            .create_element("option")?
            .dyn_into::<HtmlOptionElement>()?;
        option.set_value(name);
        option.set_text_content(Some(name));
        select_generator.append_child(&option)?;
    }
    let generator = generators.get(&select_generator.value()).unwrap()();

    // setup solvers
    let mut solvers: BTreeMap<String, fn() -> Box<dyn solve::Solver>> = BTreeMap::new();
    solvers.insert(
        "A* algorithm (using Taxicab distance heuristic)".to_string(),
        || Box::<solve::AStarSearch<solve::TaxicabDistance>>::default(),
    );
    solvers.insert(
        "Dijkstra's algorithm (A* algorithm without heuristic)".to_string(),
        || Box::<solve::AStarSearch<solve::Zero>>::default(),
    );
    solvers.insert(
        "Randomised depth first search algorithm".to_string(),
        || Box::<solve::RandomisedDepthFirstSearch>::default(),
    );
    solvers.insert("Wall follower (left turn)".to_string(), || {
        Box::<solve::WallFollowerSearch<solve::Left>>::default()
    });
    solvers.insert("Wall follower (right turn)".to_string(), || {
        Box::<solve::WallFollowerSearch<solve::Right>>::default()
    });
    for name in solvers.keys() {
        let option = document
            .create_element("option")?
            .dyn_into::<HtmlOptionElement>()?;
        option.set_value(name);
        option.set_text_content(Some(name));
        select_solver.append_child(&option)?;
    }
    let solver = solvers.get(&select_solver.value()).unwrap()();

    let context = Box::new(RefCell::new(context));
    let select_solver = Arc::new(RefCell::new(select_solver));
    let button_solver = Arc::new(RefCell::new(button_solver));
    let generator = Arc::new(RefCell::new(generator));
    let solvers = Arc::new(RefCell::new(solvers));
    let solver = Arc::new(RefCell::new(solver));

    // program phase; also used for synchronisation
    let phase = Arc::new(Mutex::new(Phase::Generate));

    // maze dimensions
    let dimensions = Arc::new(RefCell::new((
        DEFAULT_WIDTH as usize,
        DEFAULT_HEIGHT as usize,
    )));

    // maze cells
    let cells = {
        let dimensions = dimensions.borrow();
        Arc::new(RefCell::new(vec![
            Cell::default();
            dimensions.0 * dimensions.1
        ]))
    };

    // generate button behaviour
    {
        let context = context.clone();
        let button_solver = button_solver.clone();
        let generator = generator.clone();
        let phase = phase.clone();
        let dimensions = dimensions.clone();
        let cells = cells.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            let mut phase = phase.lock().unwrap();
            button_solver.borrow().set_disabled(true);
            let mut dimensions = dimensions.borrow_mut();
            *dimensions = (
                input_width.value().parse().unwrap_or(dimensions.0).max(2),
                input_height.value().parse().unwrap_or(dimensions.1).max(2),
            );
            input_width.set_value(dimensions.0.to_string().as_str());
            input_height.set_value(dimensions.1.to_string().as_str());
            {
                let context = context.borrow();
                let canvas = context.canvas().unwrap();
                canvas.set_width(dimensions.0 as u32 * CELL_PIXELS);
                canvas.set_height(dimensions.1 as u32 * CELL_PIXELS);
                context.set_line_cap("round");
            }
            *cells.borrow_mut() = vec![Cell::default(); dimensions.0 * dimensions.1];
            *generator.borrow_mut() = generators.get(&select_generator.value()).unwrap()();
            *phase = Phase::Generate;
        });
        button_generator
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // from and to cell indexes
    let (from, to) = (Arc::new(RefCell::new(0)), Arc::new(RefCell::new(0)));

    // solve button behaviour
    {
        let select_solver = select_solver.clone();
        let solvers = solvers.clone();
        let solver = solver.clone();
        let phase = phase.clone();
        let cells = cells.clone();
        let (from, to) = (from.clone(), to.clone());
        let closure = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            let mut phase = phase.lock().unwrap();
            let mut cells = cells.borrow_mut();
            for cell in &mut *cells {
                cell.solution = CellSolution::default();
            }
            let (mut from, mut to) = (from.borrow_mut(), to.borrow_mut());
            if input_from_to.checked() {
                (*from, *to) = (
                    (random() * cells.len() as f64) as usize,
                    (random() * cells.len() as f64) as usize,
                );
                while *from == *to {
                    *to = (random() * cells.len() as f64) as usize;
                }
            }
            (cells[*from].solution.from, cells[*to].solution.to) = (true, true);
            *solver.borrow_mut() = solvers
                .borrow()
                .get(&select_solver.borrow().value())
                .unwrap()();
            *phase = Phase::Solve;
        });
        button_solver
            .borrow()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // visualisation
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let mut phase = phase.lock().unwrap();
        if match *phase {
            Phase::Generate => {
                let mut cells = cells.borrow_mut();
                if !generator
                    .borrow_mut()
                    .step(*dimensions.borrow(), &mut cells)
                {
                    let (mut from, mut to) = (from.borrow_mut(), to.borrow_mut());
                    (*from, *to) = (
                        (random() * cells.len() as f64) as usize,
                        (random() * cells.len() as f64) as usize,
                    );
                    while *from == *to {
                        *to = (random() * cells.len() as f64) as usize;
                    }
                    (cells[*from].solution.from, cells[*to].solution.to) = (true, true);
                    *solver.borrow_mut() = solvers
                        .borrow()
                        .get(&select_solver.borrow().value())
                        .unwrap()();
                    button_solver.borrow().set_disabled(false);
                    *phase = Phase::Solve;
                }
                true
            }
            Phase::Solve => {
                let (mut cells, from, to) = (cells.borrow_mut(), from.borrow(), to.borrow());
                if !solver
                    .borrow_mut()
                    .step(*dimensions.borrow(), &mut cells, *from, *to)
                {
                    *phase = Phase::Complete;
                }
                true
            }
            Phase::Complete => false,
        } {
            let context = context.borrow();
            let canvas = context.canvas().unwrap();
            context.set_fill_style(&JsValue::from_str(BACKGROUND_STYLE));
            context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            for (idx, cell) in cells.borrow().iter().enumerate() {
                cell.draw(*dimensions.borrow(), idx, &context);
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
