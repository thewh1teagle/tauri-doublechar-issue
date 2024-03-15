use std::str::FromStr;
use tauri::{LogicalPosition, LogicalSize, Url, WebviewBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {

      // Create main window
      let width = 800.;
      let height = 600.;
      let window = tauri::window::WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .build()?;


      // Create webviews  

      let webview = WebviewBuilder::new("twitter", tauri::WebviewUrl::External(Url::from_str("https://twitter.com/")
        .unwrap()))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.109 Safari/537.36")
        ;
      window.add_child(webview, LogicalPosition::new(width / 2., height / 2.), LogicalSize::new(width / 2., height / 2.)).unwrap();
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
