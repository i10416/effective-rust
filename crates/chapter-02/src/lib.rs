//======================== Simple Trait Example ========================
pub trait Additive {
    fn plus(&self, another: &Self) -> Self;
    fn zero() -> Self;
}

pub fn sum<T: Additive>(values: &[T]) -> T {
    values.iter().fold(T::zero(), |a, b| a.plus(b))
}

impl Additive for i32 {
    fn plus(&self, another: &Self) -> Self {
        self + *another
    }
    fn zero() -> Self {
        0
    }
}
impl Additive for f32 {
    fn plus(&self, another: &Self) -> Self {
        self + *another
    }
    fn zero() -> Self {
        0.0
    }
}

pub trait Area {
    fn area(&self) -> i32;
}

pub struct Square {
    pub size: i32,
}

pub struct Rect {
    pub width: i32,
    pub height: i32,
}
pub struct RectInlinable {
    pub width: i32,
    pub height: i32,
}

impl Area for Rect {
    #[inline(never)]
    fn area(&self) -> i32 {
        self.width * self.height
    }
}
impl Area for RectInlinable {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl Area for Square {
    #[inline(never)]
    fn area(&self) -> i32 {
        self.size * self.size
    }
}

pub trait Drawable {
    fn draw(&self);
}

impl Drawable for Rect {
    fn draw(&self) {
        println!("drawing Rect")
    }
}
impl Drawable for Square {
    fn draw(&self) {
        println!("drawing Square")
    }
}

#[derive(Default)]
pub struct Scene {
    drawables: Vec<Box<dyn Drawable>>
}


pub struct StaticScene<T: Drawable> {
    drawables: Vec<T>
}


impl <T: Drawable> StaticScene<T> {
    pub fn new() -> Self {
        Self {
            drawables: vec![]
        }
    }
    pub fn draw(&self) {
        for component in self.drawables.iter() {
            component.draw();
        }
    }
    pub fn add_component(&mut self, component: T) {
        self.drawables.push(component);
    }
}

impl Scene {
    pub fn draw(&self) {
        for component in self.drawables.iter() {
            component.draw();
        }
    }
    pub fn add_component(&mut self, component: Box<dyn Drawable>) {
        self.drawables.push(component);
    }
}

pub fn polymorphism_example_dynamic() {
    let mut scene = Scene::default();
    let rect = Rect {width:10,height:10};
    let square = Square {size: 10};
    scene.add_component(Box::new(rect));
    scene.add_component(Box::new(square));
}

pub fn polymorphism_example_static() {
    let mut scene = StaticScene::new();
    let rect = Rect {width:10,height:10};
    #[allow(unused)]
    let square = Square {size: 10};
    scene.add_component(rect);
    // scene.add_component(Box::new(square)); // compile error
}


//======================== Static/Dynamic Dispatch Example ========================
#[cfg(feature = "static")]
pub fn static_dispatch<T: Area>(t: &T) -> i32 {
    t.area()
}

#[cfg(feature = "dynamic")]
#[no_mangle]
pub fn dynamic_dispatch(t: &dyn Area) -> i32 {
    t.area()
}
