use crate::structs::TradeEntity::TradeEntity;

pub fn parse_trade_entities(to_process: String) -> Vec<TradeEntity> {
    let obj: serde_json::Value = serde_json::from_str(&to_process).unwrap();
    let results = obj.get("results").unwrap().as_array().unwrap();
    let mut v: Vec<TradeEntity> = Vec::new();

    for trade in results {
        let temp: TradeEntity = build_trade_entity(trade);
        v.push(temp);
    }

    v
}

fn build_trade_entity(raw_input: &serde_json::Value) -> TradeEntity {
    let props: &serde_json::Value = raw_input.get("properties").unwrap();

    TradeEntity {
        trade_type: parse_properties(props, "Trade Type"),
        market_structure: parse_properties_items(props, "MS"),
        exit_price: parse_properties(props, "Exit Price"),
        entry_price: parse_properties(props, "Entry Price"),
        win: parse_properties(props, "Win"),
        side: parse_properties(props, "Side"),
        exchange: parse_properties(props, "Exchange"),
        asset: parse_properties(props, "Asset"),
        entry_model: parse_properties(props, "Entry Model"),
        confusion_matrix: parse_properties(props, "Confusion Matrix"),
        used_orderflow: parse_properties(props, "Used Orderflow"),
        confluences: parse_properties_items(props, "Confluences"),
        improvements: parse_properties_items(props, "Improvements"),
        market: parse_properties(props, "Market"),
        entry_date: parse_properties(props, "Entry Date"),
        exit_date: parse_properties(props, "Exit Date"),
        is_open: parse_properties(props, "isOpen"),
        stop_loss: parse_properties(props, "Stop-loss"),
    }
}

fn parse_properties<T>(props: &serde_json::Value, key: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    let json: Option<&serde_json::Value> = props.get(key);
    match json {
        None => panic!("Failed to extract `{}` from the API Response", key),
        Some(val) => {
            let _type = val.get("type").unwrap().to_string().replace("\"", "");
            let easy_parse: Vec<&str> = vec!["number", "checkbox", "boolean"];

            if easy_parse.contains(&_type.as_str()) {
                let target_raw = val.get(_type).unwrap().to_string();
                let target = target_raw.replace("\"", "");
                let processed = target.parse::<T>();

                if let Ok(v) = processed {
                    Some(v)
                } else {
                    None
                }
            } else {
                match _type.as_str() {
                    "select" => handle_type_select::<T>(val, _type),
                    "formula" => parse_properties::<T>(val, _type.as_str()),
                    "date" => handle_type_date::<T>(val, &_type.as_str()),
                    _ => panic!("Failed to parse `{}` of a type `{}`", key, _type),
                }
            }
        }
    }
}

fn handle_type_select<T: std::str::FromStr>(obj: &serde_json::Value, key: String) -> Option<T> {
    let nested_obj = obj.get(key.as_str());

    match nested_obj {
        None => None,
        Some(o) => {
            let val = o.get("name");
            match val {
                None => None,
                Some(k) => {
                    let processed = k.to_string().replace("\"", "").parse::<T>();

                    match processed {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                }
            }
        }
    }
}

fn parse_properties_items<T: std::str::FromStr>(
    props: &serde_json::Value,
    key: &str,
) -> Option<Vec<T>> {
    let json: Option<&serde_json::Value> = props.get(key);
    match json {
        None => panic!("Failed to find `{}` in the obejct", key),
        Some(nested) => {
            let mut v: Vec<T> = Vec::new();
            let arr = nested.get("multi_select").unwrap().as_array().unwrap();

            for o in arr {
                let nnested = o.get("name");
                match nnested {
                    None => {
                        println!("There's no `name` in there");
                        return None;
                    }
                    Some(k) => match k.to_string().parse::<T>() {
                        Ok(val) => v.push(val),
                        Err(_) => return None,
                    },
                }
            }

            Some(v)
        }
    }
}

fn handle_type_date<T: std::str::FromStr>(obj: &serde_json::Value, key: &str) -> Option<T> {
    let nested_obj = obj.get(key);
    match nested_obj {
        None => None,
        Some(o) => {
            let ddate = o.get("start");

            match ddate {
                None => None,
                Some(d) => match d.to_string().parse::<T>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, FixedOffset};

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_date() {
        let rhs = "2023-06-25T18:42:00+00:00";
        let lhs = rhs.parse::<DateTime<FixedOffset>>().unwrap();
        assert_eq!(&lhs.to_rfc3339(), rhs);
    }
}
