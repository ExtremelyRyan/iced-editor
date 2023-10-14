// https://www.youtube.com/watch?v=gcBJ7cPSALo
use iced::{Element, Sandbox, Settings, widget::{text, text_editor, container}};


// enter debug view with F12

pub fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor{
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

impl Sandbox for Editor {
    // events or user interactions that our app handles.
    type Message = Message;

    // Here is where you should return the initial state of your app.
    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    // fn theme(&self) -> Theme {
        
    // }

    fn title(&self) -> String {
        String::from("Iced Editor")
    }

    // This is where you define your update logic. 
    // All the messages, produced by user interactions, will be handled by this method.
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) =>  {
                self.content.edit(action);
            }
        }
    }

    // These widgets can produce (element / widget) messages based on user interaction.
    fn view(&self) -> Element<'_, Message> {
        // text("hello iced!").into();
        let input = text_editor(&self.content).on_edit(Message::Edit);

        container(input).padding(15).into()
    }
}