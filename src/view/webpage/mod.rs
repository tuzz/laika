use super::webgl::WebGLRenderingContext;

use stdweb::web::{document, Element, HtmlElement};
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use stdweb::traits::{IHtmlElement, INode};

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
        Self::resize_canvas(&canvas);

        Self { context }
    }

    fn create_canvas() -> CanvasElement {
        document().create_element("canvas")
            .expect("failed to create element").try_into()
            .expect("failed to convert element to canvas")
    }

    fn resize_canvas(canvas: &CanvasElement) {
        let rectangle = Self::body().get_bounding_client_rect();
        let pixel_ratio = Self::get_device_pixel_ratio();

        let width = rectangle.get_width() as u32 * pixel_ratio;
        let height = rectangle.get_height() as u32 * pixel_ratio;

        canvas.set_width(width);
        canvas.set_height(height);
    }

    fn get_device_pixel_ratio() -> u32 {
        let ratio = js! { return window.devicePixelRatio; };
        ratio.try_into().unwrap_or(1)
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
        Self::body().append_child(element);
    }

    fn body() -> HtmlElement {
        document().body().expect("failed to fetch body")
    }
}
