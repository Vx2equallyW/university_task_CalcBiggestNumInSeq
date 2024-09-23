use iced::{alignment, Alignment, Element, Padding, Sandbox, Settings, Length};
use iced::widget::{container, text, column, Column, button, text_input, row, scrollable, Row};

struct mainWindow {
    values_history: Vec<f32>,
    input_value: String,
    value: f32
}

#[derive(Debug, Clone)]
enum Message {
    InputValue(String),
    Submitted,
}

impl Sandbox for mainWindow {
    type Message = Message;
    /* App initialization */
    fn new() -> mainWindow {
        Self {
            values_history: vec![],
            input_value: String::default(),
            value: 0.0
        }
    }

    /* App title */
    fn title(&self) -> String {
        String::from("Task 15 Variant")
    }

    fn update(&mut self, message: Self::Message) {
        //Update the state of your app
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                self.value = self.input_value.parse().unwrap_or(0.0);
                let mut top_add: f32 = 0.0;
                let mut bottom_add: f32 = 1.0;
                let mut result: f32 = 1.0;

                while result < self.value {
                    result += (1.0 + top_add) / (1.0 + bottom_add);
                    top_add += 1.0;
                    bottom_add += 1.0;
                }

                if self.values_history.len() >= 6 {
                    self.values_history.clear();
                }
                self.values_history.push(result);

                self.input_value = String::default();
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            column!(
                values_history_list(&self.values_history),
                text("Hello world"),
                row!(
                    text_input("Input number here", &self.input_value)
                    .on_input(|value| Message::InputValue(value))
                    .on_submit(Message::Submitted),

                    button("submit")
                    .on_press(Message::Submitted)
                )
                .spacing(30)
                .padding(Padding::from(30))
            )
            .align_items(Alignment::Center)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }
}

fn values_history_list(items: &Vec<f32>) -> Element<'static, Message> {
    let mut col = Column::new()
    .spacing(20)
    .align_items(Alignment::Center)
    .width(Length::Fill);
    for item in items {
        col = col.push(text(item));
    }

    container(
        col
    )
    .height(250.0)
    .into()
}

fn main() -> iced::Result {
    mainWindow::run(Settings::default())
}
