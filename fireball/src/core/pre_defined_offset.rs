/// 파일 내부에 지정되어있는 데이터에 대한 구조체가 들어있는 모듈
#[derive(Debug)]
pub(crate) struct PreDefinedOffset {
    /// 파일에 대한 오프셋
    pub(crate) offset: usize,
    /// 파일 내부에 지정되어있는 이름
    pub(crate) name: String,
}
