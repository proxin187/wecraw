use super::*;


pub fn document_content(document: Html) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut content = String::new();

    for text in document.root_element().text() {
        content.extend([text.chars().collect::<Vec<char>>(), vec![' ']].concat());
    }

    Ok(content)
}

pub fn queue_page(
    page: Page,
    queue: Arc<Mutex<Vec<String>>>,
    visited: Arc<Mutex<HashMap<String, ()>>>,
    model: Arc<Mutex<Model>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let link = Selector::parse("a").map_err(|err| err.to_string())?;
    let document = Html::parse_document(&page.content);

    for element in document.select(&link) {
        if let Some(href) = element.attr("href") {
            if let Ok(mut visited) = visited.lock() {
                if visited.insert(href.to_string(), ()).is_none() && !href.contains("?") {
                    let _ = queue.lock().map(|mut queue| queue.push(href.to_string()));
                }
            }
        }
    }

    let content = doc::document_content(document)?;

    let _ = model.lock().map(|mut model| model.insert_document(page.url, content));

    Ok(())
}


