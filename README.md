# FLick - 서버 모니터링 시스템

실시간 서버 모니터링 및 알림 시스템

## 시스템 아키텍처

```mermaid
graph TB
    Client[Flutter Client]
    Server[Rust Server]
    DB[(PostgreSQL + TimescaleDB)]
    WS[WebSocket Server]
    
    Client -->|HTTP| Server
    Client -->|WebSocket| WS
    Server -->|Query| DB
    WS -->|Query| DB
    Server -->|Metrics| DB
```

## 주요 기능

- 실시간 서버 모니터링
- 멀티 플랫폼 지원 (Android/iOS)
- 커스터마이즈 가능한 알림 설정
- 상세한 메트릭스 분석
- 직관적인 대시보드

## 기술 스택

### Frontend
- Flutter
- Provider (상태관리)
- Material 3 디자인
- fl_chart (차트 라이브러리)

### Backend
- Rust
- actix-web
- sqlx
- tokio
- serde

### Database
- PostgreSQL
- TimescaleDB

## 시작하기

### 요구사항

- Rust 1.75.0+
- PostgreSQL 15.0+
- TimescaleDB 2.11.0+
- Flutter 3.19.0+

### 환경 설정

1. 환경 변수 설정
```env
DATABASE_URL=postgresql://user:password@localhost/flick
dKC4IdrUUjbO9CVS+1ETpBPJinRAWrzPn914SB5Vi+g==your-secret-key
RUST_LOG=info
```

2. 데이터베이스 설정
3. 백엔드 서버 실행
4. 프론트엔드 앱 실행

자세한 설치 및 설정 방법은 [배포 가이드](docs/deployment.md)를 참조하세요.

## 프로젝트 구조

```
flick/
├── docs/               # 상세 문서
├── frontend/           # Flutter 앱
├── backend/            # Rust 서버
└── scripts/           # 유틸리티 스크립트
```

## 문서

- [아키텍처 설계](docs/architecture.md)
- [데이터베이스 설계](docs/database.md)
- [API 문서](docs/api.md)
- [보안 설정](docs/security.md)
- [배포 가이드](docs/deployment.md)
- [모니터링 설정](docs/monitoring.md)

## 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.