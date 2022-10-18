# Fireman

해당 프로그램은 개발 진행중입니다.

## 프로젝트 목적

Snowman의 동작 원리를 분석해, 디컴파일러를 만든다.

## 목표

### 프로그램 구조

- [ ] TODO...
- [ ] 오브젝트

### 기능 목표

- [ ] 어셈블리 파싱
  - [ ] 가상주소 -> 파일오프셋 매칭 함수 작성
  - [ ] x86_64
    - [ ] PE파일을 불러와 어셈블리 파싱
    - [ ] ELF파일을 불러와 어셈블리 파싱
  - [ ] ARM
    - [ ] MACH파일을 불러와 어셈블리 파싱
- [ ] 어셈블리 패턴
  - [ ] Call을 기반으로 함수 설정
    - [ ] xAnalyzer 분석 및 적용
      - [ ] 함수 패턴 파싱 로직 분석
      - [ ] 함수 패턴 파싱 로직 적용
      - [ ] 함수 인자 파싱 로직 분석
      - [ ] 함수 패턴 학습 로직 분석
  - [ ] Jmp기반 반복문 설정
    - [ ] TODO...
  - [ ] Jmp기반 함수 설정 및 심층분석 작성
    - [ ] TODO...
- [ ] 학습
  - [ ] 패턴 파싱된 파일을 기반으로 패턴 학습 작성
  - [ ] 패턴 파일 파싱 프로그램 작성
- [ ] LIB
  - [ ] 프로그램을 사용할 수 있는 LIB 제공 및 함수 제공
  - [ ] 헤더파일 생성 적용
- [ ] CLI
  - [ ] snowman의 nocode와 같이, 모든 프로그램에 대해 디컴파일 기능 작성
  - [ ] 프로그램 인자를 기반으로 여러 디컴팡리 옵션 작성
    - [ ] 부분 디컴파일 기능 작성
    - [ ] 분석 단계 설정 기능 작성
- [ ] GUI
  - [ ] eframe기반 디컴파일러 작성

### 개발 목표

- [ ] 프로그램 디스어셈블 기능
- [ ] 함수 범위 지정 기능
- [ ] 함수 인자 파싱 기능
- [ ] call과 jmp를 기반으로 함수 시작지점 지정 기능
- [ ] jmp를 기반으로 반복문 지정 기능
- [ ] 주로 사용되는 패턴에 대해 분석 기능

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