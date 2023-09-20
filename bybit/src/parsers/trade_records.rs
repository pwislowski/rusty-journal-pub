use super::utils::process_api_val;
use crate::structs::TradeRecord::TradeRecord;

pub fn build_trade_records(arr: &Vec<serde_json::Value>) -> Vec<TradeRecord> {
    let mut v: Vec<TradeRecord> = Vec::new();

    for obj in arr {
        let temp: TradeRecord = parse_json_to_trade_record(obj);

        v.push(temp);
    }

    v
}

fn parse_json_to_trade_record(obj: &serde_json::Value) -> TradeRecord {
    TradeRecord {
        symbol: process_api_val(obj, "symbol"),
        exec_fee: process_api_val(obj, "execFee"),
        exec_id: process_api_val(obj, "execId"),
        exec_price: process_api_val(obj, "execPrice"),
        exec_qty: process_api_val(obj, "execQty"),
        exec_type: process_api_val(obj, "execType"),
        fee_rate: process_api_val(obj, "feeRate"),
        last_liquidity_ind: process_api_val(obj, "lastLiquidityInd"),
        leaves_qty: process_api_val(obj, "leavesQty"),
        order_id: process_api_val(obj, "orderId"),
        order_link_id: process_api_val(obj, "orderLinkId"),
        order_price: process_api_val(obj, "orderPrice"),
        order_qty: process_api_val(obj, "orderQty"),
        order_type: process_api_val(obj, "orderType"),
        stop_order_type: process_api_val(obj, "stopOrderType"),
        side: process_api_val(obj, "side"),
        exec_time: process_api_val(obj, "execTime"),
        closed_size: process_api_val(obj, "closedSize"),
        iv: process_api_val(obj, "iv"),
        block_trade_id: process_api_val(obj, "blockTradeId"),
        mark_price: process_api_val(obj, "markPrice"),
        mark_iv: process_api_val(obj, "markIv"),
        underlying_price: process_api_val(obj, "underlyingPrice"),
        index_price: process_api_val(obj, "indexPrice"),
        is_maker: process_api_val(obj, "isMaker"),
    }
}
