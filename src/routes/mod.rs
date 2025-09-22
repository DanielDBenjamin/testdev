pub mod about;
pub mod home;
pub mod timetable;
pub mod statistics;
pub mod login;
pub mod register;
pub mod error;
pub mod classes;
pub mod new_class;
pub mod new_module;

pub use about::About;

pub use home::HomePage;
pub use timetable::Timetable;
pub use statistics::Statistics;
pub use login::Login;
pub use register::Register;
pub use error::Error;
pub use classes::ClassesPage;
pub use new_class::NewClass;
pub use new_module::NewModule;
