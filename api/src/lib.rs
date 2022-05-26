use chrono::{DateTime, Utc};
use serde::*;

// JSON の{"user_agent": "xxx", "response_time": 0, "timestamp": "yyyy-MM-dd+HH:mm:ss"} に対応
// 戻り値で使うログは timestamp が Option ではない
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

// クエリパラメータの ?from=yyyy-MM-dd+HH:mm:ss&util=yyyy-MM-dd+HH:mm:ss に対応
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

pub mod csv {
    pub mod get {
        use crate::DateTimeRange;

        pub type Query = DateTimeRange;
        // get はファイルを返すので Response型の定義がない
    }

    pub mod post {
        use serde::*;

        // CSV ファイルを受け付けるのでリクエストデータはない
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        // 受領したログの数を返す
        pub struct Response(pub usize);
    }
}

pub mod logs {
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde::*;

        pub type Query = DateTimeRange;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        // 保存しているログをすべて返す
        pub struct Response(pub Vec<Log>);
    }

    pub mod post {
        use chrono::{DateTime, Utc};
        use serde::*;

        // 説明した通りのデータを受け付ける
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<DateTime<Utc>>,
        }
        // Accepted を返すので Response データ型の定義はない
    }
}
