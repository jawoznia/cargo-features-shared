# Cargo-features-shared

```
workspace
|
- (1) dep with feature
|
- (2) module using dep with feature
|
- (3) module using dep without a feature
```

In above example (3) will have feature from (1) enabled even though in `Cargo.toml` it has it added
as dependency without the feature enabled.
