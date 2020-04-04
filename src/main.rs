use scraper::{Html, Selector};

fn main() {
	// Based heavily on the scraper crate readme
	let html = r#"
		<!DOCTYPE html>
		<html>
			<head>
				<meta charset="utf-8">
				<title>Hello, world!</title>
			</head>
				<body>
				<main>
					<div class="foo">Bar</div>
					<div class="foo">Baz</div>
					<div class="green">Eggs</div>
				</main>
			</body>
		</html>
	"#;

	let document = Html::parse_document(html);
	let title_sel = Selector::parse("title").unwrap();
	let foo_sel = Selector::parse(".foo").unwrap();
	for title in document.select(&title_sel) {
		println!(
			"Document title: {}",
			title.text()
				.map(String::from)
				.collect::<String>()
		);
	}
	println!("Items of class foo in the document:");
	for foo in document.select(&foo_sel) {
		println!(
			"\t{}", foo.text()
				.map(String::from)
				.collect::<String>()
		);
	}
}
