use std::sync::Arc;

use super::Relation;

/// 코드 블럭의 연결 데이터를 관리하는 구조체
#[derive(Debug)]
pub struct Relations {
    /// 내부 데이터
    data: Vec<Relation>,
}

impl Relations {
    /// 연결을 관리하는 구조체를 생성한다.
    ///
    /// ### Returns
    /// - `Arc<Self>`: 연결 데이터를 관리하는 구조체
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }
}
