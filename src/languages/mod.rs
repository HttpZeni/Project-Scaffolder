pub mod cpp;
pub mod python;
pub mod rust;

pub trait Language {
    fn extention(&self) -> String;
    fn main_file(&self) -> String;
    fn gitignore(&self) -> String;
    fn readme(&self) -> String;
}
