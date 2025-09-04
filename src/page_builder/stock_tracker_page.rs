use yew::prelude::*;
 
use crate::utils::image_utils::image_carousel_builder;
 
use crate::utils::mark_down_utils;
 
 
pub fn create_stock_tracker_markdown() -> Html {
     
    let markdown = r#"
# Stock Tracker

A custom **Google Sheets**-based portfolio tracker designed to provide a quick overview of stocks by making HTML requests, fetching the latest stock details, and calculating important financial ratios. This tool also aggregates recent news and displays insider buys/sells, offering a comprehensive view for quick decision-making.

## Features
**Real-time Data Retrieval:** Automatically pulls stock prices, financial ratios, and key metrics from the web.
**Custom Calculations:** Calculates essential financial ratios like P/E ratio, market cap, and dividend yield.
**News Updates:** Displays the latest news relevant to the stock market and individual tickers.
**Insider Trading Alerts:** Monitors and displays recent insider buying and selling activities.
**User-Friendly:** A clean and intuitive interface within **Google Sheets** for easy navigation and data analysis.

Perfect for both casual investors and those looking for a quick snapshot of stock performance!

    "# ; 
    mark_down_utils::create_markdown(markdown) 
 
}

 
fn stock_tracker_image_showcase() -> Html {
let images = vec![
    "stock_tracker/firo_tracker_dashboard.png".to_string(),
    "stock_tracker/dcf_showcase.png".to_string(),
   "stock_tracker/10_yr_forcast.png".to_string(),
   "stock_tracker/Cash_Flow_Statement.png".to_string(),
   "stock_tracker/Ratios.png".to_string(),
   "stock_tracker/automatic_changing_show.png".to_string(),
   "stock_tracker/code_import_style.png".to_string(),
   "stock_tracker/Full_Dashboard.png".to_string(),
   "stock_tracker/Ratios_Table.png".to_string(),
   "stock_tracker/BalanceSheet.png".to_string(),
   "stock_tracker/dcf_chart.png".to_string(),
   "stock_tracker/Income_Statement.png".to_string(),
   "stock_tracker/Book_Value.png".to_string(),
   "stock_tracker/Insider_data.png".to_string(),
];

    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}
 
pub fn stock_tracker_page() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            {create_stock_tracker_markdown()} 
            {stock_tracker_image_showcase()}
        </div>
    }
}
