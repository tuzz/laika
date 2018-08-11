mod webgl;
mod webpage;

use self::webpage::Webpage;

pub struct View {

}

impl View {
    pub fn new() -> Self {
        let webpage = Webpage::new("laika");
        let _ = webpage.context;

        View{}
    }
}
