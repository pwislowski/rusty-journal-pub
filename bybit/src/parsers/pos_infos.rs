use crate::parsers::utils::process_api_val;
use crate::structs::PositionInfo::PositionInfo;
use reqwest::Response;

fn build_position_info(obj: &serde_json::Value) -> PositionInfo {
    PositionInfo {
        symbol: process_api_val(obj, "symbol"),
        avg_price: process_api_val(obj, "avgPrice"),
        entry_price: process_api_val(obj, "entryPrice"),
        created_time: process_api_val(obj, "updatedTime"),
        leverage: process_api_val(obj, "leverage"),
        liq_price: process_api_val(obj, "liqPrice"),
        position_balance: process_api_val(obj, "positionBalance"),
        position_value: process_api_val(obj, "positionValue"),
        side: process_api_val(obj, "side"),
        size_in_qoute: process_api_val(obj, "size"),
        stop_loss: process_api_val(obj, "stopLoss"),
        take_profit: process_api_val(obj, "takeProfit"),
        unrealised_pnl: process_api_val(obj, "unrealisedPnl"),
    }
}

pub async fn parse_response(res: Response) -> Vec<PositionInfo> {
    let mut vec: Vec<PositionInfo> = Vec::new();

    let text = res.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&text).expect("Failed to parse into JSON.");
    let res = json.get("result").unwrap();
    let positions = res.get("list").unwrap().as_array().unwrap();

    for pos in positions {
        let processed = build_position_info(pos);
        vec.push(processed);
    }

    vec
}
