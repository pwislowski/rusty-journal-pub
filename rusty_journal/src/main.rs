use std::{cmp, thread, time};

type BybitClient = bybit::Client;
type NotionClient = notion::Client;

#[tokio::main]
async fn main() {
    loop {
        let fifteen_seconds = time::Duration::from_secs(15);
        bybit_flow().await;
        thread::sleep(fifteen_seconds);
    }
}

#[allow(dead_code)]
async fn bybit_flow() {
    // create the logs to cache sent notion requests
    let _ = logger::create_directory();
    let bybit_auth = bybit::fetch_credentials();
    let bybit_client: BybitClient = BybitClient::new(bybit_auth);

    let notion_auth = notion::fetch_credentials();
    let notion_client: NotionClient = NotionClient::new(notion_auth);

    let positions = bybit_client.fetch_current_positions().await;

    for pos in positions {
        let log_file = logger::read_logfile("Bybit", &pos.symbol);

        match log_file {
            Ok(log) => {
                let reading = pos.from_string(&log);

                if reading.avg_price == pos.avg_price {
                    continue;
                }

                let serialized = pos.to_json();
                let _ = logger::create_logfile("Bybit", &pos.symbol, &serialized);

                let transactions = bybit_client.fetch_transaction_log(&pos.symbol).await;
                let latest_tran = transactions
                    .into_iter()
                    .filter(|x| &x.exec_type == "Trade")
                    .fold(u64::MIN, |max, x| cmp::max(max, x.exec_time));

                let notion_object = pos.build_new_page_object(Some(latest_tran));
                match notion_client.create_new_page(notion_object).await {
                    Ok(_) => {
                        let timestamp =
                            bybit::parsers::utils::convert_ts_milis_to_datetime(latest_tran);
                        let success_print = format!(
                            "*Successfully created an entry:*
Pair: {}
Entry Price: {}
Entry Date: {}
--------------------------------",
                            pos.symbol,
                            pos.entry_price,
                            timestamp.to_rfc3339(),
                        );
                        println!("{}", success_print);
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
            Err(_) => {
                // create logfile
                let serialized = pos.to_json();
                let _ = logger::create_logfile("Bybit", &pos.symbol, &serialized);

                let transactions = bybit_client.fetch_transaction_log(&pos.symbol).await;
                let latest_tran = transactions
                    .into_iter()
                    .filter(|x| &x.exec_type == "Trade")
                    .fold(u64::MIN, |max, x| cmp::max(max, x.exec_time));

                let notion_object = pos.build_new_page_object(Some(latest_tran));
                match notion_client.create_new_page(notion_object).await {
                    Ok(_) => {
                        let timestamp =
                            bybit::parsers::utils::convert_ts_milis_to_datetime(latest_tran);
                        let success_print = format!(
                            "*Successfully created an entry:*
Pair: {}
Entry Price: {}
Entry Date: {}
--------------------------------",
                            pos.symbol,
                            pos.entry_price,
                            timestamp.to_rfc3339(),
                        );
                        println!("{}", success_print);
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
        };
    }
}
