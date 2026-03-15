# miro 배포 구현 메모 (miro-release-feat)

## 구현 상태: 완료

계획 문서: `docs/miro-release-plan.md`

---

## 구현된 파일

### `.github/workflows/release.yml`

`v*` 태그 push 시 자동 실행되는 4단계 파이프라인.

| job | 역할 |
|-----|------|
| `build` | aarch64 / x86_64 크로스 컴파일 + tar.gz 아카이브 |
| `release` | GitHub Release 생성 + 바이너리 + checksums.txt 업로드 |
| `update-tap` | `dambyworld/homebrew-tap` Formula/miro.rb 자동 갱신 |
| `install-test` | 클린 VM에서 brew install → 버전/아키텍처/서브커맨드 검증 |

**필요한 GitHub Secrets:**

| Secret | 설명 |
|--------|------|
| `GITHUB_TOKEN` | 자동 제공 (release 생성용) |
| `TAP_GITHUB_TOKEN` | `dambyworld/homebrew-tap` 쓰기 권한 PAT (수동 등록 필요) |

`TAP_GITHUB_TOKEN` 등록 방법:
```
GitHub → dambyworld/miro → Settings → Secrets and variables → Actions → New repository secret
Name: TAP_GITHUB_TOKEN
Value: <Ruska-Zone 쓰기 권한 PAT>
```

---

### `scripts/release.sh`

GitHub Actions 없이 로컬에서 수동 릴리스할 때 사용하는 sh 스크립트.

```sh
# 버전 자동 추출 (Cargo.toml 기준)
./scripts/release.sh

# 버전 직접 지정
./scripts/release.sh 0.1.0
```

수행 작업:
1. 두 타겟 크로스 컴파일
2. tar.gz 아카이브 생성
3. SHA256 체크섬 계산
4. `gh release create` 실행
5. Tap 포뮬라 수동 업데이트 안내 출력
6. 임시 파일 정리

> **주의**: 로컬 brew/포뮬라에는 일절 영향 없음.

---

### `dambyworld/homebrew-tap` — `Formula/miro.rb`

- 저장소: https://github.com/dambyworld/homebrew-tap
- Apple Silicon(aarch64) / Intel(x86_64) 분기 처리
- `sha256` 플레이스홀더(`<aarch64-sha256>`, `<x86_64-sha256>`)는 첫 릴리스 시 `update-tap` job이 자동 교체

---

## 릴리스 절차 (운영)

```sh
# 1. Cargo.toml 버전 bump
#    version = "0.1.0" → "0.2.0"

# 2. 변경사항 커밋
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.2.0"

# 3. 태그 생성 및 push → Actions 자동 실행
git tag v0.2.0
git push origin main --tags
```

---

## 검증 항목 (install-test job)

- [x] `brew install dambyworld/tap/miro` 오류 없이 완료
- [x] `miro --version` 이 `Cargo.toml` 버전과 일치
- [x] 바이너리 아키텍처 확인 (`file $(which miro)`)
- [x] 서브커맨드(`list`, `themes`) 오류 없이 응답
- [x] `brew upgrade miro` 정상 동작 (no-op)
- [x] `brew uninstall` 후 재설치 정상 동작

---

## 설치 명령 (최종 사용자)

```bash
# 신규 설치
brew install dambyworld/tap/miro

# 업그레이드
brew upgrade dambyworld/tap/miro
```

---

## 참고

- Tap 저장소: https://github.com/dambyworld/homebrew-tap
- miro 저장소: https://github.com/dambyworld/miro
- 계획 문서: `docs/miro-release-plan.md`
