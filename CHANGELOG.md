# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [0.6.0](https://github.com/justinrubek/lockpad/compare/0.1.0..0.2.0) - 2023-04-01
#### Bug Fixes
- Make tests ignore token expiration date - ([686c267](https://github.com/justinrubek/lockpad/commit/686c267f39500021994def304995f41dfbb2a3c4)) - [@justinrubek](https://github.com/justinrubek)
- Use key from state when logging in - ([577367c](https://github.com/justinrubek/lockpad/commit/577367c62a1ed9dd2cfb28a60906721a0ed24e2f)) - [@justinrubek](https://github.com/justinrubek)
#### Continuous Integration
- publish lockpad-auth to crates.io - ([982535d](https://github.com/justinrubek/lockpad/commit/982535d1093f2ffcf6c8cf2962459bbd7fde9f07)) - [@justinrubek](https://github.com/justinrubek)
#### Documentation
- **(readme)** add running instructions - ([ef027fe](https://github.com/justinrubek/lockpad/commit/ef027fe3e7078a038ab7cda520c8c17e238e60cf)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- API keys - ([c1b06fc](https://github.com/justinrubek/lockpad/commit/c1b06fc2b4fe1d58f16cc9a6f2f50df1d4d90275)) - [@justinrubek](https://github.com/justinrubek)
- ulid sqlx type - ([7cbf755](https://github.com/justinrubek/lockpad/commit/7cbf7552030129e9fb0fdef1c90b672d3ec45193)) - [@justinrubek](https://github.com/justinrubek)
- add sqlx - ([781735c](https://github.com/justinrubek/lockpad/commit/781735c24e91c82a94ea3f0687e78719d630e0f1)) - [@justinrubek](https://github.com/justinrubek)
- application routes now determine the owner_id from a token - ([ff41f98](https://github.com/justinrubek/lockpad/commit/ff41f98f907f128b86d73a8fc37d3ab4e71b2d70)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- removed most calls to tracing::info - ([c069e7a](https://github.com/justinrubek/lockpad/commit/c069e7a15bb7f7651f645e4014b611d9f3531486)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- Use axum::extract::FromRef instead of AsRef - ([dc5b36e](https://github.com/justinrubek/lockpad/commit/dc5b36ef3198cada2177bc0b920b96a543bed945)) - [@justinrubek](https://github.com/justinrubek)
- convert application model to postgres - ([dd5fd08](https://github.com/justinrubek/lockpad/commit/dd5fd08322889ed633f7fee61007e920b512769b)) - [@justinrubek](https://github.com/justinrubek)
- convert user model to postgres - ([330a9e0](https://github.com/justinrubek/lockpad/commit/330a9e0e83cd34d0021688221ce2a0e83aec2981)) - [@justinrubek](https://github.com/justinrubek)

- - -
## [0.3.0](https://github.com/justinrubek/lockpad/compare/0.2.0..0.3.0) - 2024-02-29
#### Build system
- **(cargo)** update axum to 0.7 - ([c9cd7fa](https://github.com/justinrubek/lockpad/commit/c9cd7fa3d9f03b707a092cd061f7d430aa5e52e5)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** cargo update - ([320f90f](https://github.com/justinrubek/lockpad/commit/320f90f342a23dc67d44939b6f805003d1ee1c6f)) - [@justinrubek](https://github.com/justinrubek)
- **(deps)** bump webpki from 0.22.1 to 0.22.4 - ([2aef69d](https://github.com/justinrubek/lockpad/commit/2aef69db1bc82ab9c2ad2943c5bd235e7c7930a9)) - dependabot[bot]
- **(nix)** update nixpkgs, pgx-ulid - ([3c27c12](https://github.com/justinrubek/lockpad/commit/3c27c12cb773a7d3d6ec13ba1d9397b83eda42bb)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update many inputs - ([56b9d16](https://github.com/justinrubek/lockpad/commit/56b9d1694c7e0c7e0cbab5b1b2d7aa3a511a97f3)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update fenix input - ([34a23bb](https://github.com/justinrubek/lockpad/commit/34a23bb65aecdf46fa337b59f06d9c91ee7d9841)) - [@justinrubek](https://github.com/justinrubek)
#### Continuous Integration
- **(github)** remove github release step - ([03a7c27](https://github.com/justinrubek/lockpad/commit/03a7c2774ee87a72d31cf0b2dbc7113c18d6ff33)) - [@justinrubek](https://github.com/justinrubek)
- **(github)** attempt to fixup action - ([8b764c8](https://github.com/justinrubek/lockpad/commit/8b764c8ca7988f2015ef541a00ac7714b74e0a73)) - [@justinrubek](https://github.com/justinrubek)
- **(github/actions)** rename container build - ([216e0fb](https://github.com/justinrubek/lockpad/commit/216e0fbbbbc2465973cec975638b46f56907e310)) - [@justinrubek](https://github.com/justinrubek)
- **(github/actions)** add container build action - ([12b62d6](https://github.com/justinrubek/lockpad/commit/12b62d61f0068043ca466e1773d945624e2e8bc1)) - [@justinrubek](https://github.com/justinrubek)
- switch from docker.io to ghcr.io - ([7c616f4](https://github.com/justinrubek/lockpad/commit/7c616f47cd7bbef564c6b034516f247199084fb1)) - [@justinrubek](https://github.com/justinrubek)
#### Documentation
- **(hacking.md)** outline instructions for using services - ([d7dfcd1](https://github.com/justinrubek/lockpad/commit/d7dfcd18d41b891f159fa8707fafd697b68b2c41)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- **(containers)** provide migrations in lockpad image - ([9894cb3](https://github.com/justinrubek/lockpad/commit/9894cb3c643c7b183f234143b8c22f22a1e76629)) - [@justinrubek](https://github.com/justinrubek)
- **(containers)** lockpad/docker package - ([eab3614](https://github.com/justinrubek/lockpad/commit/eab3614ce092ee4fa325a129759240147a56a974)) - [@justinrubek](https://github.com/justinrubek)
- **(flake-parts)** add ci devShell - ([61f5b21](https://github.com/justinrubek/lockpad/commit/61f5b21830144f6f7c1030180fe25e7fad1b051c)) - [@justinrubek](https://github.com/justinrubek)
- **(http)** add /health route - ([99bc557](https://github.com/justinrubek/lockpad/commit/99bc557310580451b57b0de2a2272a1035edbe57)) - [@justinrubek](https://github.com/justinrubek)
- add /api/authorize JSON route - ([98d2a49](https://github.com/justinrubek/lockpad/commit/98d2a49e2b3f6833281ac2b76c2230d03e535c4b)) - [@justinrubek](https://github.com/justinrubek)
- build container image for postgres - ([2cade95](https://github.com/justinrubek/lockpad/commit/2cade95b05fe35e2e8c6acd802b394948e8fbdf4)) - [@justinrubek](https://github.com/justinrubek)
- add services-flake for process-compose - ([f2fce9a](https://github.com/justinrubek/lockpad/commit/f2fce9aa24b3e1600cd4d1335ae34369d095340c)) - [@justinrubek](https://github.com/justinrubek)
- login form validation - ([f7d1239](https://github.com/justinrubek/lockpad/commit/f7d123905031208085b3b01193efc6b415d3237c)) - [@justinrubek](https://github.com/justinrubek)
- added HtmlPage struct - ([9a0b993](https://github.com/justinrubek/lockpad/commit/9a0b99399f61760472e6e007b18e1c48983462c6)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- remove postgres scripts - ([ec15764](https://github.com/justinrubek/lockpad/commit/ec157641315f6730ee5cbacccee6d5d206e32a7f)) - [@justinrubek](https://github.com/justinrubek)
- update .gitignore - ([52411dd](https://github.com/justinrubek/lockpad/commit/52411dd160fe4de0cdb57182f64ad161765ed371)) - [@justinrubek](https://github.com/justinrubek)
- add cargo metadata - ([0bb7460](https://github.com/justinrubek/lockpad/commit/0bb7460d2d01f32a6b4d8dff5f14dccc95354ee3)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- Removed javascript from form submission - ([182ef56](https://github.com/justinrubek/lockpad/commit/182ef5685048a8a559f493e385af91e681c65fc3)) - [@justinrubek](https://github.com/justinrubek)

- - -


## [0.1.0](https://github.com/justinrubek/lockpad/compare/5507daafaef71a2a89bf33a33277782eacfa1c97..0.1.0) - 2023-04-01
#### Build system
- **(devshell)** add scripts for running a postgres development server - ([fa45d5f](https://github.com/justinrubek/lockpad/commit/fa45d5fb5aab98318a7b04ef40f5534ad449df5c)) - [@justinrubek](https://github.com/justinrubek)
- Add `run-scylla` script to devshell - ([5135eac](https://github.com/justinrubek/lockpad/commit/5135eacf2e4379df42e2a293242c50e629d52048)) - [@justinrubek](https://github.com/justinrubek)
#### Documentation
- **(readme)** add project description - ([c07757f](https://github.com/justinrubek/lockpad/commit/c07757ff678f95a4f4fbc3229aa60e48328199ee)) - [@justinrubek](https://github.com/justinrubek)
- **(readme)** basic readme - ([e4d2239](https://github.com/justinrubek/lockpad/commit/e4d2239dfa04e6851e2949a110b99e48e989adc2)) - [@justinrubek](https://github.com/justinrubek)
- Revised HACKING.md - ([7e8c79c](https://github.com/justinrubek/lockpad/commit/7e8c79c66982052b9690b0ef8117ae7b042cdfec)) - [@justinrubek](https://github.com/justinrubek)
- add scylladb info to HACKING.md - ([202655a](https://github.com/justinrubek/lockpad/commit/202655a5d13b8af1257f8c47a025e7f5d4322f85)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- **(cli)** Add key generation - ([4e91c53](https://github.com/justinrubek/lockpad/commit/4e91c537f38e10fe495032b24aad1736265b92b5)) - [@justinrubek](https://github.com/justinrubek)
- load configuration from environment - ([5bf46c2](https://github.com/justinrubek/lockpad/commit/5bf46c253f27971bf6d026e807c77f5aea11e652)) - [@justinrubek](https://github.com/justinrubek)
- Load development environment variables - ([f2cf133](https://github.com/justinrubek/lockpad/commit/f2cf133e42ab81573aab8af9bae848159d51540a)) - [@justinrubek](https://github.com/justinrubek)
- Example consumer rest apis - ([1620272](https://github.com/justinrubek/lockpad/commit/1620272d42c2d1becea381f6836881a69f83f8a0)) - [@justinrubek](https://github.com/justinrubek)
- Implement jwt and verification - ([e4ae934](https://github.com/justinrubek/lockpad/commit/e4ae93495f552719968f549c0f13600abeee1b06)) - [@justinrubek](https://github.com/justinrubek)
- token generation - ([02ad602](https://github.com/justinrubek/lockpad/commit/02ad602196b1004a6e6b170a6a88e2e254725c68)) - [@justinrubek](https://github.com/justinrubek)
- Added Application entity type - ([1269b24](https://github.com/justinrubek/lockpad/commit/1269b24ed31b95764a669e57dd09a545730abaca)) - [@justinrubek](https://github.com/justinrubek)
- GetItem on User - ([6dfd004](https://github.com/justinrubek/lockpad/commit/6dfd004d504dadb8b277d8c381bb7dba1ee584a3)) - [@justinrubek](https://github.com/justinrubek)
- query trait impl - ([e07b1b7](https://github.com/justinrubek/lockpad/commit/e07b1b74df3b4f3cbe8a63216230226ff6841431)) - [@justinrubek](https://github.com/justinrubek)
- admin wipe table route - ([73caa6e](https://github.com/justinrubek/lockpad/commit/73caa6e5887a542a4269a6df9c45c59b5e9c743d)) - [@justinrubek](https://github.com/justinrubek)
- derives for entities that are unique and also with an owner - ([0c5a070](https://github.com/justinrubek/lockpad/commit/0c5a070232ce47162639a0131469c52597682c96)) - [@justinrubek](https://github.com/justinrubek)
- implement trait for storing object with unique constraint into - ([47bdf4a](https://github.com/justinrubek/lockpad/commit/47bdf4a28777bef89390552a11658408afad7595)) - [@justinrubek](https://github.com/justinrubek)
- password hash verification - ([1cff3aa](https://github.com/justinrubek/lockpad/commit/1cff3aa41fb6a807d41521e8f23654f99166a680)) - [@justinrubek](https://github.com/justinrubek)
- user signup - ([3c19462](https://github.com/justinrubek/lockpad/commit/3c19462f9e5ea273c92427c8d4111a2c6362f557)) - [@justinrubek](https://github.com/justinrubek)
- added axum webserver capable of mimicking a login process - ([9cf6dfa](https://github.com/justinrubek/lockpad/commit/9cf6dfa5d1d3031792db1604ba4f8403b0c91751)) - [@justinrubek](https://github.com/justinrubek)
- Access scylla alternator - ([5c651fc](https://github.com/justinrubek/lockpad/commit/5c651fc24c2c754675679c41310656e446e594a4)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- **(nix)** initialize flake - ([a62603e](https://github.com/justinrubek/lockpad/commit/a62603e2e7dd7ba7bab9aaf9250967c67fbb63af)) - [@justinrubek](https://github.com/justinrubek)
- add bomper configuration - ([a09f6a0](https://github.com/justinrubek/lockpad/commit/a09f6a0f43538b64cb26b47c10cd6d34cdc2f15e)) - [@justinrubek](https://github.com/justinrubek)
- changes from review - ([e4b48c3](https://github.com/justinrubek/lockpad/commit/e4b48c36f2c9f249bbdacda38a1bc1702b1a9059)) - [@justinrubek](https://github.com/justinrubek)
- changes from review - ([0ef20d3](https://github.com/justinrubek/lockpad/commit/0ef20d340dacab4871842584dbc3a1520d543563)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- **(cargo)** Define common manifest values in workspace block - ([3033eff](https://github.com/justinrubek/lockpad/commit/3033eff399886298063e1e350f5eb5913fcd6452)) - [@justinrubek](https://github.com/justinrubek)
- move dynamodb traits above models crate - ([5b0e9d4](https://github.com/justinrubek/lockpad/commit/5b0e9d4ef0d2fbc0aed0bfde6436f9e31ab61231)) - [@justinrubek](https://github.com/justinrubek)
- move handlers into submodules - ([a2e1ea9](https://github.com/justinrubek/lockpad/commit/a2e1ea9cbfba4e25c845b19377a8c139d35f6f74)) - [@justinrubek](https://github.com/justinrubek)


