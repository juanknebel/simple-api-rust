use crate::model::{
  message_service::{MessageService, MessageServiceImpl},
  user_service::{UserService, UserServiceImpl},
};

pub trait App: Sync + Send {
  fn user_service(&self) -> Box<dyn UserService>;
  fn message_service(&self) -> Box<dyn MessageService>;
}

pub struct MainApp;

impl App for MainApp {
  fn user_service(&self) -> Box<dyn UserService> {
    Box::new(UserServiceImpl)
  }

  fn message_service(&self) -> Box<dyn MessageService> {
    Box::new(MessageServiceImpl)
  }
}

pub struct TestApp;

impl App for TestApp {
  fn user_service(&self) -> Box<dyn UserService> {
    Box::new(UserServiceImpl)
  }

  fn message_service(&self) -> Box<dyn MessageService> {
    Box::new(MessageServiceImpl)
  }
}
