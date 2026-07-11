# Rust CLI Lab

这是我学习 Ken Youens-Clark 的《Command-Line Rust》时编写的练习项目。

本仓库使用：

- Rust stable
- Cargo Workspace
- Clap 4 derive
- GitHub Issues
- Feature Branch
- Pull Request
- GitHub Actions

## 项目进度

- [ ] 01 hello
- [ ] 02 echor
- [ ] 03 catr
- [ ] 04 headr
- [ ] 05 wcr
- [ ] 06 uniqr
- [ ] 07 findr
- [ ] 08 cutr
- [ ] 09 grepr
- [ ] 10 commr
- [ ] 11 tailr
- [ ] 12 fortuner
- [ ] 13 calr
- [ ] 14 lsr

## 本地检查

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
