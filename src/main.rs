use std::fs::{self, DirEntry};

use iced::widget::{Column, Text};

struct Files {
	files: Vec<DirEntry>,
	searchBar: SearchBar,
}

#[derive(Debug, Clone)]
pub enum Message {
	SearchBarUpdate(String),
	SearchValidate,
}

#[derive(Default)]
struct SearchBar {
	content: String,
}

impl Default for Files {
	fn default() -> Self {
		let paths = fs::read_dir("./").unwrap();

		let mut res = Vec::new();

		for path in paths {
			res.push(path.unwrap());
		}
		Self {
			files: res,
			searchBar: SearchBar::default(),
		}
	}
}

impl Files {
	pub fn view(&self) -> Column<'_, Message> {
		// We use a column: a simple vertical layout
		let mut column = Column::new();

		// let input = text_input("Enter a new item...", &self.searchBar.content)
		// 	.padding(10)
		// 	.on_submit(Message::SearchValidate).on_input(Message::SearchBarUpdate);
		// AAAAAAAAAAAAA

		for text in &self.files {
			column = column.push(Text::new(text.file_name().into_string().unwrap()));
		}

		column
	}

	pub fn update(&mut self, message: Message) {
		match message {
			Message::SearchBarUpdate(a) => println!("{a}, {}", self.searchBar.content),
			Message::SearchValidate => println!("{}", self.searchBar.content),
		}
	}
}

fn main() -> iced::Result {
	iced::run("A cool counter", Files::update, Files::view)
}
