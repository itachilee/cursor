use color_eyre::{eyre::Ok, Result};

use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

mod gzip;
mod hitokoto;
mod redis;
mod scrapy;

use hitokoto::{get_hitokoto, Hitokoto};

struct App {
    content: String,
    current_hitokoto: Option<Hitokoto>,
    loading: bool,
}

impl App {
    fn new() -> Self {
        Self {
            content: String::new(),
            current_hitokoto: None,
            loading: false,
        }
    }

    async fn load_next_page(&mut self) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // Here you would call your `scrapy_html` function
        // For demonstration, we just update the content
        let next_page = get_hitokoto().await.unwrap();
        self.content = next_page.hitokoto.clone().unwrap();
        self.current_hitokoto = Some(next_page.to_owned());
        self.loading = false;
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    async fn init_novel() -> Self {
        let novel = get_hitokoto().await.unwrap();
        let content = novel.hitokoto.clone();

        let current_novel = Some(novel);
        Self {
            content: content.unwrap(),
            current_hitokoto: current_novel,
            loading: false,
        }
    }
    fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     color_eyre::install()?;
//     let terminal = ratatui::init();
//     // 初始加载第一页
//     let url = "https://www.bilinovel.com/novel/2978/157901.html";
//     let novels = scrapy_html(url, vec![]).await;
//     ratatui::restore();
//     Ok(())
//     // test_read_redis().await;
// }

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize terminal

    let mut terminal = ratatui::init();

    // Create app state
    let app: Arc<Mutex<App>> = Arc::new(Mutex::new(App::init_novel().await));

    let (tx, mut rx) = mpsc::channel(1);

    let app_clone = Arc::clone(&app);

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match message {
                "load_next_page" => {
                    let mut app = app_clone.lock().await;
                    app.set_loading(true);
                    app.load_next_page().await;
                }
                _ => {}
            }
        }
    });

    run_app(&mut terminal, app, tx).await?;
    ratatui::restore();
    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    tx: mpsc::Sender<&str>,
) -> Result<()> {
    loop {
        let app_ref = app.clone();
        let app = app_ref.lock().await; // 在异步上下文中获取锁
        terminal.draw(|f| {
            ui(f, &app);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Right | KeyCode::Char('d') => {
                    let _ = tx.send("load_next_page").await;
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();
    let block = Block::default().borders(Borders::ALL).title("hitokoto");

    let content = if app.loading {
        "加载中..."
    } else {
        &app.content
    };

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, size);
}
