use cli_candlestick_chart::{Candle, Chart};
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use serde_json::Value;
use url::Url;
use tokio::sync::mpsc;
use std::env;
use std::process::Command;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <symbol>", args[0]);
        return;
    }
    let symbol = args[1].to_lowercase();
    let interval = "1m";
    let number_of_candles_to_show = 100;
    let (tx, mut rx) = mpsc::channel(100);
    
    let url = format!("wss://stream.binance.com:9443/ws/{}@kline_{}", symbol, interval);
    let (ws_stream, _) = connect_async(Url::parse(&url).unwrap())
        .await
        .expect("Failed to connect");
    
    let (_, mut read) = ws_stream.split();
    println!("📡 Connected to Binance WebSocket for {}...", symbol);
    
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Ok(text) = msg.into_text() {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    if let Some(kline) = json.get("k") {
                        let candle = Candle::new(
                            kline["o"].as_str().unwrap().parse().unwrap(), // Open price
                            kline["h"].as_str().unwrap().parse().unwrap(), // High price
                            kline["l"].as_str().unwrap().parse().unwrap(), // Low price
                            kline["c"].as_str().unwrap().parse().unwrap(), // Close price
                        );
                        tx.send(candle).await.unwrap();
                    }
                }
            }
        }
    });

    let mut candles: Vec<Candle> = Vec::new();
    let mut chart = Chart::new(&candles);
    chart.set_name(symbol.clone());
    chart.set_bull_color(0, 128, 0);
    chart.set_bear_color(255, 107, 153);
    
    while let Some(candle) = rx.recv().await {
        if candles.len() >= number_of_candles_to_show {
            candles.remove(0);
        }
        candles.push(candle);
        
        chart = Chart::new(&candles);
        chart.set_name(symbol.clone());
        chart.set_bull_color(0, 128, 0);
        chart.set_bear_color(255, 107, 153);
        
        Command::new("clear").status().unwrap();
        chart.draw();
    }
}
