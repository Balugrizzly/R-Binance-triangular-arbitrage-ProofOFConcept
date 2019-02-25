extern crate binance;

use binance::api::*;
use binance::market::*;

use std::time::Duration;
use std::thread;

fn main() {
    println!("Started");
    let market: Market = Binance::new(None, None);

    for _ in 0..10000 {
        // Best price/qty on the order book for ALL symbols
        match market.get_all_book_tickers() {
            Ok(book) => test2(book),
            Err(e) => println!("Error: {}", e),
        }

        thread::sleep(Duration::from_millis(10000))
    }

    fn test2(book: binance::model::BookTickers) {
        match book {
            binance::model::BookTickers::AllBookTickers(book) => for page in book.iter() {
                // println!("Page: {:?}", page.symbol);

                // we asume we have bought eth (1eth in the example case)
                // eth buy price for now we just asume a whole + the fee we have to pay
                let btc_threshold_amount = book[0].ask_price + (book[0].ask_price * 0.001);
                let ethamount: f64 = 1.0;

                // simulating buying another random coin
                if page.symbol.ends_with("ETH") {

                    // simulating a buy of that coin
                    // * 0.001 is the fee again this time we pay it in eth
                    let coin_amount = (ethamount * 0.001) / page.ask_price;
                    // getting the coin name we just bought
                    let coin_name = page.symbol.replace("ETH", "");

                    // simulating a trade in of the above coin for btc
                     for mirror_page in book.iter() {
                         if mirror_page.symbol == format!("{}{}",&coin_name, "BTC"){
                             // * 0.001 is the fee again this time we pay it in $coin_name
                             let final_btc_amount = (coin_amount * 0.001) * mirror_page.bid_price;

                             if final_btc_amount > btc_threshold_amount {
                                 println!("Coin: {:?}", mirror_page.symbol);
                                 println!("Profit: {:?}", final_btc_amount - btc_threshold_amount);
                                 // then do the trade

                             }
                         }
                    }
                }
            },
        }
    }

    //
    // // Best price/qty on the order book for ONE symbol
    // match market.get_book_ticker("BNBETH") {
    //     Ok(answer) => println!(
    //         "Bid Price: {}, Ask Price: {}",
    //         answer.bid_price, answer.ask_price
    //     ),
    //     Err(e) => println!("Error: {}", e),
    // }
    //
    // // 24hr ticker price change statistics
    // match market.get_24h_price_stats("BNBETH") {
    //     Ok(answer) => println!(
    //         "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
    //         answer.open_price, answer.high_price, answer.low_price
    //     ),
    //     Err(e) => println!("Error: {}", e),
    // }
    //
    // // last 10 5min klines (candlesticks) for a symbol:
    // match market.get_klines("BNBETH", "5m", 10, None, None) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }
}
