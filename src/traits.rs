use std::fmt::{self};

pub fn traits_main(){
    let mut drill: Drill = Default::default();
    let mut saw: Saw = Default::default();

    // First, I can call apply() on each tool directly:
    drill.apply("wood");
    saw.apply("wood");

    // Next, I can call a method that accepts a single instance of PowerTool to turn them on:
    turn_on_tool(&mut drill);
    turn_on_tool(&mut saw);

    // I can call a method that acts on an array of PowerTool. &[&dyn PowerTool] is a slice of trait objects, where each object points to a type that implements PowerTool via dynamic dispatch.
    let material = "oak";
    let tools: &[&dyn PowerTool] = &[&drill, &saw];
    work_on_project(&tools, material);

    // Finally, I can call a method that accepts a single instance of PowerTool to turn them off, using trait bound syntax:
    turn_off_tool(&mut drill);
    turn_off_tool(&mut saw);
}

// static dispatch (caller type is known at compile time). The compiler will actually create multiple versions of this method in the binary for each type.
// &impl means accept any type that implements our trait
fn turn_on_tool(tool: &mut impl PowerTool) {
    tool.power_on();
}

fn turn_off_tool<T: PowerTool>(tool: &mut T) {
    tool.power_off();
}

fn work_on_project(tools: &[&dyn PowerTool], material: &str) { // dynamic displatch (caller types are known at runtime, hence the dyn keyword)
    for (_i, tool) in tools.iter().enumerate() {
        tool.apply(material);
    }
}

trait PowerTool: fmt::Display {
    fn apply(&self, material: &str) {
        println!("You attempted to use a {self} on some {material} and didn't know how! Your finger has been chopped off!");
    }

    fn power_on(&mut self);

    fn power_off(&mut self);
}

// Derive uses a "procedural macro" which is a method you can implement (in a seperate crate), where you return the default implementation as a string!
// The compiler will take that string and then compile it as a function. Not recommended - meta-programming.
#[derive(Default)]
struct Drill {
    powered_on: bool
}

struct Saw {
    powered_on: bool
}

impl PowerTool for Drill {
    fn apply(&self, material: &str) {
        println!("You drilled a hole in some {material}");
    }

    fn power_on(&mut self) {
        self.powered_on = true;
    }

    fn power_off(&mut self) {
        self.powered_on = false;
    }
}

impl fmt::Display for Drill{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Drill")
    }
}

impl PowerTool for Saw {
    fn power_on(&mut self) {
        self.powered_on = true;
    }

    fn power_off(&mut self) {
        self.powered_on = false;
    }
}

impl fmt::Display for Saw{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Saw")
    }
}

// Implement an external trait!
impl Default for Saw {
    fn default() -> Self {
        Saw{ powered_on: false}
    }
}