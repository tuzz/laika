use super::webgl::WebGLRenderingContext;

use stdweb::web::{document, Element};
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use stdweb::traits::INode;

pub struct Webpage {
    pub context: WebGLRenderingContext,
}

impl Webpage {
    pub fn new(title: &str) -> Self {
        document().set_title(title);

        let canvas = Self::create_canvas();
        let context = Self::fetch_context(&canvas);
        let style = Self::create_style();

        Self::add_to_page(&canvas);
        Self::add_to_page(&style);

        Self { context }
    }

    fn create_canvas() -> CanvasElement {
        document().create_element("canvas")
            .expect("failed to create element").try_into()
            .expect("failed to convert element to canvas")
    }

    fn fetch_context(canvas: &CanvasElement) -> WebGLRenderingContext {
        canvas.get_context().expect("failed to fetch render context")
    }

    fn create_style() -> Element {
        let style = document().create_element("style").expect("failed to create element");
        let css = "html, body, canvas { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; }";
        let inner = document().create_text_node(css);

        style.append_child(&inner);
        style
    }

    fn add_to_page<T: INode>(element: &T) {
        document().body().expect("failed to fetch body").append_child(element);
    }
}
