use std::cmp;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement};
use gloo_console::log;
use derivative::Derivative;
use crate::node::Node;

#[derive(Derivative, Eq, Clone, Copy, Debug)]
#[derivative(PartialEq, Hash)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

pub struct Universe {
    node_ref: NodeRef,
    size: usize,
    cell_size: usize,
    cells: Node,
    cell_vec: Vec<Cell>,
}

pub enum Msg {
    Init,
    Render(bool),
    Mutate(MouseEvent),
}

impl Component for Universe {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Init);
        let vec_init = vec![Cell::Dead; 1024];
        let node = Node::new(vec_init.clone());
        Universe {
            node_ref: NodeRef::default(),
            size: 16,
            cell_size: 50,
            cells: node,
            cell_vec: vec_init,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init => {
                self.init(ctx);
                false
            }
            Msg::Render(is_mut) => {
                self.render(ctx, is_mut);
                false
            }
            Msg::Mutate(event) => {
                self.mutate(event, ctx);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Render(false));
        let mutate = ctx.link().callback(|event: MouseEvent| Msg::Mutate(event));
        html! {
            <div>
                <canvas id="world" 
                    width={(self.size * self.cell_size + 100).to_string()}
                    height={(self.size * self.cell_size + 100).to_string()}
                    ref={self.node_ref.clone()}
                    onclick={mutate}></canvas>
                <button {onclick}>{ "Click" }</button>
            </div>
        }
    }
}

impl Universe {
    fn init(&mut self, ctx: &Context<Self>) {
        let canvas: HtmlCanvasElement = self.node_ref.cast().unwrap();
        let canvas_ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        canvas_ctx.begin_path();
        let size: u32 = self.size as u32;
        let cell_size: u32 = self.cell_size as u32;

        for i in 0..=size {
            canvas_ctx.move_to((i * (cell_size + 1) + 1).into(), 0.0);
            canvas_ctx.line_to((i * (cell_size + 1) + 1).into(), ((cell_size + 1) * size + 1).into());
        }

        for i in 0..=size {
            canvas_ctx.move_to(0.0, (i * (cell_size + 1) + 1).into());
            canvas_ctx.line_to(((cell_size + 1) * size + 1).into(), (i * (cell_size + 1) + 1).into());
        }
        canvas_ctx.stroke();

        ctx.link().send_message(Msg::Render(true));
    }

    fn render(&mut self, ctx: &Context<Self>, is_mut: bool) {
        let canvas: HtmlCanvasElement = self.node_ref.cast().unwrap();
        let canvas_ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        if !is_mut {
            self.cells = Node::new(self.cell_vec.clone());
            self.cell_vec = self.cells.evolve().clone(); 
        }
        canvas_ctx.begin_path();
        let size = self.size;
        let cell_size = self.cell_size;
        for r in (size / 2)..(3 * size / 2) {
            for c in (size / 2)..(3 * size / 2) {
                if self.cell_vec[c + r * size * 2] == Cell::Alive {
                    canvas_ctx.set_fill_style(&JsValue::from("#000000"));
                } else {
                    canvas_ctx.set_fill_style(&JsValue::from("#FFFFFF"));
                }

                canvas_ctx.fill_rect(
                    (((c - size / 2) as u32) * ((cell_size as u32) + 1) + 3).into(),
                    (((r - size / 2) as u32) * ((cell_size as u32) + 1) + 3).into(),
                    (cell_size as u32 - 6).into(),
                    (cell_size as u32 - 6).into()
                );
            }
        }
        canvas_ctx.stroke();
        log!("rendered");
    }

    fn mutate(&mut self, event: MouseEvent, ctx: &Context<Self>) {
        let canvas: Element = self.node_ref.cast().unwrap();
        let bounding_rect = canvas.get_bounding_client_rect();
        let xscale = (self.size * self.cell_size + 100) as f64 / bounding_rect.width();
        let yscale = (self.size * self.cell_size + 100) as f64 / bounding_rect.height();
        let canvas_left = (event.client_x() as f64 - bounding_rect.left()) * xscale;
        let canvas_top = (event.client_y() as f64 - bounding_rect.top()) * yscale;
        let r = cmp::min((canvas_top / (self.cell_size + 1) as f64).floor() as usize, self.size - 1);
        let c = cmp::min((canvas_left / (self.cell_size + 1) as f64).floor() as usize, self.size - 1);
        self.cell_vec[(self.size / 2 + c) + 2 * self.size * (self.size / 2 + r)] = match self.cell_vec[(self.size / 2 + c) + 2 * self.size * (self.size / 2 + r)] {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
        ctx.link().send_message(Msg::Render(true));
    }

}

