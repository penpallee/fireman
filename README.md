# Fireman

## 프로젝트 목적

Snowman의 동작 원리를 분석해, 디컴파일러를 만든다.

## 코드 작성

### 코드 서식

#### 내부적으로 사용되는 Use 정렬 순서

- std, core (built-in)
- crate, super (inside crate)
- others

위와 같은 그룹으로 나눈 이후, rustfmt의 기본 포맷에 따른다.

### 소스코드 내부 순서

#### 모듈 정의 파일 (mod.rs)

- 서브모듈에 대한 주석
- 서브모듈의 mod
- 서브모듈의 use

위를 반복한다.

타입이나 함수의 선언이 필요할 경우, 서브모듈을 만들어 pub use를 사용한다.

#### 소스코드 파일

- use문 정의
- 소스코드
- 구현 (구현이 작을 경우)
- 구현에 대한 서브모듈의 mod (구현이 많을 경우)
