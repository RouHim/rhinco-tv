## [2.11.4](https://github.com/RouHim/rhinco-tv/compare/v2.11.3...v2.11.4) (2026-08-30)


### Bug Fixes

* **ci:** repair release workflow triggers and permissions to fix update loop ([9c552de](https://github.com/RouHim/rhinco-tv/commit/9c552de59c3df438adab2692904ee3ec513708da))

## [2.11.3](https://github.com/RouHim/rhinco-tv/compare/v2.11.2...v2.11.3) (2026-08-30)


### Bug Fixes

* **updater:** ensure update targets displayed version to prevent loop ([20621fb](https://github.com/RouHim/rhinco-tv/commit/20621fb48b08150c4cbe3aa1780531dc31d3d5da))

## [2.11.2](https://github.com/RouHim/rhinco-tv/compare/v2.11.1...v2.11.2) (2026-03-16)


### Bug Fixes

* **ci:** build release binaries after version bump to fix update loop ([e888c92](https://github.com/RouHim/rhinco-tv/commit/e888c92d58768c37d247cd1e32d03cfe24c8b11b))

## [2.11.1](https://github.com/RouHim/rhinco-tv/compare/v2.11.0...v2.11.1) (2026-02-26)


### Bug Fixes

* **ui:** center save flow modals consistently ([decee9d](https://github.com/RouHim/rhinco-tv/commit/decee9d66e268d5c283ade6b0bee5c7be2dd9e27))
* **ui:** replace save emoji with FontAwesome floppy_disk icon ([0e3606f](https://github.com/RouHim/rhinco-tv/commit/0e3606f3b4050eb499bf8728a7c8a91459f33752))

# [2.11.0](https://github.com/RouHim/rhinco-tv/compare/v2.10.0...v2.11.0) (2026-02-26)


### Bug Fixes

* **toast:** position notification at top-center instead of bottom-center ([628b308](https://github.com/RouHim/rhinco-tv/commit/628b308b6cb5241ba1cb4dc5f1e6199347e70638))
* **toast:** render as small bottom-center overlay instead of fullscreen ([2c9759c](https://github.com/RouHim/rhinco-tv/commit/2c9759cfeedb9b41c11e1ed8b2710aae038dd9e2))
* **ui:** add left/right navigation to save path config modal ([91848b2](https://github.com/RouHim/rhinco-tv/commit/91848b2c7d65f399fceb46e22bc6d76141cc0b90))
* **ui:** remove leftover manual_path/editing_manual from save path modal ([fff3eee](https://github.com/RouHim/rhinco-tv/commit/fff3eee9e6573defe8dc1cfb6a2bd8bba70dfcb6))


### Features

* **save-detect:** add Heroic save path extraction ([9006922](https://github.com/RouHim/rhinco-tv/commit/900692210076f7b6b7ad10358f80078f738b9342))
* **save-detect:** add ludusavi manifest parser with AppID and fuzzy name lookup ([5d5af2a](https://github.com/RouHim/rhinco-tv/commit/5d5af2ab25f1be330d48bed2ca2cff6e422e8d88))
* **save-detect:** add save path detector orchestrating all 4 sources ([17c2530](https://github.com/RouHim/rhinco-tv/commit/17c2530ef61bd280426f308b39e3889658bb96f8))
* **save-detect:** add Steam cloud remotestorage log parser ([861daff](https://github.com/RouHim/rhinco-tv/commit/861daff0f2a12a899516704be28e2dbccdd1bc64))
* **ui:** wire auto-detection into save path modal, remove manual input ([d131428](https://github.com/RouHim/rhinco-tv/commit/d131428dbe2ede92dd9f13f4d8a7ee0d76fc2778))

# [2.10.0](https://github.com/RouHim/rhinco-tv/compare/v2.9.2...v2.10.0) (2026-02-26)


### Bug Fixes

* **ui:** make Configure Save Paths always visible and fix Save Sync Settings ([b7e3453](https://github.com/RouHim/rhinco-tv/commit/b7e3453ffc1e92e4929f18dc0921546cee1b1344))
* **ui:** pass ui_scale to all modal render functions ([02a3115](https://github.com/RouHim/rhinco-tv/commit/02a31159c9c772e8e7c65bfcf7cd9bcd9f329e1e))


### Features

* **ui:** add blocking progress modal for ludusavi operations ([ef3f4e6](https://github.com/RouHim/rhinco-tv/commit/ef3f4e637bf80234ee20ca1ecbd392e12654820e))
* **ui:** add icon badge overlay on game tile for backup status ([46fa731](https://github.com/RouHim/rhinco-tv/commit/46fa7310bb010429868627ec00e384503b39f40f))
* **ui:** add loading feedback during wine prefix scanning ([316f1f7](https://github.com/RouHim/rhinco-tv/commit/316f1f79acabe80cb673246e7b3352d5c082b5f7))
* **ui:** add ModalState variants for restore confirm, progress, and scanning ([9c2f815](https://github.com/RouHim/rhinco-tv/commit/9c2f815b3170ace6e042fbda309db949951c679a))
* **ui:** add restore confirmation dialog with gamepad navigation ([ddeffae](https://github.com/RouHim/rhinco-tv/commit/ddeffae2e63c05387a491a1a20357cf673b0f0a0))

## [2.9.2](https://github.com/RouHim/rhinco-tv/compare/v2.9.1...v2.9.2) (2026-02-15)


### Bug Fixes

* **ludusavi:** parse stdout on non-zero exit to capture unknown games ([66df350](https://github.com/RouHim/rhinco-tv/commit/66df35075e3e217e1f6d3812c24f7d68d904c024))

## [2.9.1](https://github.com/RouHim/rhinco-tv/compare/v2.9.0...v2.9.1) (2026-02-13)

# [2.9.0](https://github.com/RouHim/rhinco-tv/compare/v2.8.0...v2.9.0) (2026-02-13)


### Bug Fixes

* **games:** cross-reference Heroic installed.json for reliable game detection ([2b661a3](https://github.com/RouHim/rhinco-tv/commit/2b661a367f1ec89f58b20943a380c4f13f9c568c))
* **system-info:** detect all compatibility tools via marker files instead of name matching ([e0ee13d](https://github.com/RouHim/rhinco-tv/commit/e0ee13d7e6e7bd56524b9c611967c0b2811d5aa0))
* **ui:** use gear icon for settings tile ([d0cd628](https://github.com/RouHim/rhinco-tv/commit/d0cd628c8eed123a234bfe737294c3b8a5adc68e))


### Features

* **games:** add Zoom Platform library scanning for Heroic ([2b4f8bc](https://github.com/RouHim/rhinco-tv/commit/2b4f8bc0e68d4df4b1e39bf8ff05d18e181b9f9d))

# [2.8.0](https://github.com/RouHim/rhinco-tv/compare/v2.7.0...v2.8.0) (2026-02-04)


### Features

* **autostart:** add autostart module with enable/disable functions ([bc044c1](https://github.com/RouHim/rhinco-tv/commit/bc044c15779dd664e42d2d386f1b6bbe33ce85e2))
* **settings:** add message handlers for Settings modal ([5b866d4](https://github.com/RouHim/rhinco-tv/commit/5b866d4ca10fc9696328c50de8de117e43b9f70d))
* **settings:** add messages and action types for Settings modal ([5c4f305](https://github.com/RouHim/rhinco-tv/commit/5c4f3054c3a518472a071e3abf882673cd3a064a))
* **settings:** add ModalState::Settings variant with state management ([b5f6e41](https://github.com/RouHim/rhinco-tv/commit/b5f6e41e1f8646a0c536978352b7349188e287c4))
* **settings:** add Settings item to System menu ([1dc757b](https://github.com/RouHim/rhinco-tv/commit/1dc757b827231f2f2b18acea1fb25761abfc8ea2))
* **settings:** create settings modal UI renderer ([8634afa](https://github.com/RouHim/rhinco-tv/commit/8634afa3a7c7dad79c8417bbdaf2eff68a49a6ab))
* **settings:** implement navigation and message handlers ([f92c0de](https://github.com/RouHim/rhinco-tv/commit/f92c0de7c246f665286818727c3f5e1295564f0e))
* **ui:** add autostart toggle to system info modal ([a22732d](https://github.com/RouHim/rhinco-tv/commit/a22732d1d5cb2d30539f08771d2a6bdb814eb465))

# [2.7.0](https://github.com/RouHim/rhinco-tv/compare/v2.6.0...v2.7.0) (2026-02-01)


### Features

* **ludusavi:** add CLI wrapper module ([375c8da](https://github.com/RouHim/rhinco-tv/commit/375c8da115ec7306342f805a8c6795e366a1c5ea))
* **ludusavi:** complete Wave 3 - operation handling and backup badges ([bb793b4](https://github.com/RouHim/rhinco-tv/commit/bb793b42052e3fa6ff4a7e0f227df46eb93a568a))
* **ludusavi:** implement Wave 2 - UI integration and auto-backup ([74b3b5b](https://github.com/RouHim/rhinco-tv/commit/74b3b5b0e00d49dab5d4be49f70abb5963661d80))
* **storage:** add auto_backup and auto_cloud_sync settings ([19ffea3](https://github.com/RouHim/rhinco-tv/commit/19ffea343a71f76a8bbca2c88b5af0f20b882c6c))
* **ui:** add toast notification system ([21435a6](https://github.com/RouHim/rhinco-tv/commit/21435a6d77baa33adee173e4884d4a19fbfc3af8))

# [2.6.0](https://github.com/RouHim/rhinco-tv/compare/v2.5.0...v2.6.0) (2026-01-27)


### Features

* **ui:** add animated modal overlay fade-in with instant dismiss ([16a48e9](https://github.com/RouHim/rhinco-tv/commit/16a48e9f4d73f035c7c94c1ad9b52a24ecec702d))
* **ui:** add iced_anim dependency for UI animations ([78b59d6](https://github.com/RouHim/rhinco-tv/commit/78b59d6171d820d45d0af53bd9b0276977a79cf8))
* **ui:** animate category title color on selection change ([e5b8931](https://github.com/RouHim/rhinco-tv/commit/e5b8931de922705191a131d4feec0682cd3c0413))
* **ui:** animate context menu selection highlight ([aee753e](https://github.com/RouHim/rhinco-tv/commit/aee753ed5ce4aeda0586c55be667eb3c871b6f93))
* **ui:** animate item selection border and shadow glow ([c37453d](https://github.com/RouHim/rhinco-tv/commit/c37453d5e2d59725a0ab651d5b0cbd9877a727f3))

# [2.5.0](https://github.com/RouHim/rhinco-tv/compare/v2.4.1...v2.5.0) (2026-01-26)


### Features

* **snes9x:** add SNES game scanning support and update game sources ([f7a7cfe](https://github.com/RouHim/rhinco-tv/commit/f7a7cfe4d604e1ec18d4d72b76ca244ed53a6c21))

## [2.4.1](https://github.com/RouHim/rhinco-tv/compare/v2.4.0...v2.4.1) (2026-01-26)

# [2.4.0](https://github.com/RouHim/rhinco-tv/compare/v2.3.0...v2.4.0) (2026-01-26)


### Features

* **n64:** replace gopher64 with mupen64plus ([afa90ae](https://github.com/RouHim/rhinco-tv/commit/afa90ae34565d3feab52f193e77a1cd0609935b9))

# [2.3.0](https://github.com/RouHim/rhinco-tv/compare/v2.2.0...v2.3.0) (2026-01-25)


### Features

* **auth:** implement in-app sudo authentication ([d546890](https://github.com/RouHim/rhinco-tv/commit/d5468905b687bb4775fa5cbb83d7f91231b9e5be))
* **system:** implement sleep inhibition manager ([f312280](https://github.com/RouHim/rhinco-tv/commit/f31228075b66246beb8a6bc5537729d9ace7826a))
* **ui:** integrate sleep inhibition into launcher lifecycle ([258af25](https://github.com/RouHim/rhinco-tv/commit/258af250686b87b8180c690e45f51ef93a028f10))

# [2.2.0](https://github.com/RouHim/rhinco-tv/compare/v2.1.0...v2.2.0) (2026-01-24)


### Bug Fixes

* **ui:** adjust system icon size to prevent overflow ([218bec5](https://github.com/RouHim/rhinco-tv/commit/218bec564557fa9cb9574cf213c654ed5c6332ef))
* **ui:** ensure consistent scaling in status bar components ([de32fbb](https://github.com/RouHim/rhinco-tv/commit/de32fbb1a8e4dfd33ca415a35a902984681b71b4))


### Features

* **ui:** add main view vertical scrolling with controller/keyboard navigation ([97235d8](https://github.com/RouHim/rhinco-tv/commit/97235d82afb062ad6b18badb029d815435d804ff))

# [2.1.0](https://github.com/RouHim/rhinco-tv/compare/v2.0.3...v2.1.0) (2026-01-21)


### Bug Fixes

* **updater:** refactor update logic and add interactive UI ([2f3d12a](https://github.com/RouHim/rhinco-tv/commit/2f3d12aa0a06506c94b0223d29298dc84444a8e0))


### Features

* **ui:** add canvas background ([fc9f973](https://github.com/RouHim/rhinco-tv/commit/fc9f97338a17aefbbd3fa2c053a56be47d8a4aed))
* **ui:** show system battery status ([fbbdc0f](https://github.com/RouHim/rhinco-tv/commit/fbbdc0f8d4c4918e8ea6e33596952ff69255f5f2))

## [2.0.3](https://github.com/RouHim/rhinco-tv/compare/v2.0.2...v2.0.3) (2026-01-20)


### Bug Fixes

* **ui:** adjust main view padding to prevent content from being obscured by status bar and controls hint ([c47f28d](https://github.com/RouHim/rhinco-tv/commit/c47f28d23668a9be45a9a42e39519d5238d9ffc0))

## [2.0.2](https://github.com/RouHim/rhinco-tv/compare/v2.0.1...v2.0.2) (2026-01-20)

## [2.0.1](https://github.com/RouHim/rhinco-tv/compare/v2.0.0...v2.0.1) (2026-01-20)


### Bug Fixes

* **ui:** hide system update when unsupported ([6c93b19](https://github.com/RouHim/rhinco-tv/commit/6c93b195d030304d099abe9d8cec2b7c3d857b9f))

# [2.0.0](https://github.com/RouHim/rhinco-tv/compare/v1.2.0...v2.0.0) (2026-01-18)


### Bug Fixes

* **ui:** handle missing app launches gracefully ([f004ef5](https://github.com/RouHim/rhinco-tv/commit/f004ef5093234e3f7a65b5f37a644c8b8c1b4d2a))


### Features

* add Tux TV icon, update banner branding, and improve README ([11febd6](https://github.com/RouHim/rhinco-tv/commit/11febd63850df2e61fcf43f698622eb47b28bae2))
* rename application from Linux TV Launcher to RhincoTV ([24f6a28](https://github.com/RouHim/rhinco-tv/commit/24f6a281c0310d3da60e6b819235e7bf35c43083))
* replace Tux with whale shark in icon and banner ([1af11e7](https://github.com/RouHim/rhinco-tv/commit/1af11e7363de70f72082c71eaa0633546801cc1b)), closes [#0d1a2a](https://github.com/RouHim/rhinco-tv/issues/0d1a2a) [#1a2a3a](https://github.com/RouHim/rhinco-tv/issues/1a2a3a) [#5577aa](https://github.com/RouHim/rhinco-tv/issues/5577aa)


### BREAKING CHANGES

* Config directory changed from ~/.config/com/linux-tv-launcher to ~/.config/com/rhinco-tv

# [1.2.0](https://github.com/RouHim/rhinco-tv/compare/v1.1.0...v1.2.0) (2026-01-18)


### Features

* **games:** replace simple64 scan with gopher64 ([7585f92](https://github.com/RouHim/rhinco-tv/commit/7585f92ff42105535951ecc38d4b2925519c5e0b))

# [1.1.0](https://github.com/RouHim/rhinco-tv/compare/v1.0.1...v1.1.0) (2026-01-18)


### Features

* **games:** add Simple64 N64 emulator support ([0fb079c](https://github.com/RouHim/rhinco-tv/commit/0fb079cc49300988b6f6a87b1eda6591cbfbbf14))

## [1.0.1](https://github.com/RouHim/rhinco-tv/compare/v1.0.0...v1.0.1) (2026-01-17)


### Bug Fixes

* **ci:** package binaries as tar.gz and disable wayland to fix panic ([bc23277](https://github.com/RouHim/rhinco-tv/commit/bc23277d54c27a5e0085413f71bcca0178ef8d5b))
* **ci:** switch to gnu build with cross and system deps ([4d2ffb0](https://github.com/RouHim/rhinco-tv/commit/4d2ffb01d46dd3c0e43b9fecd8be1f2b4f50b595))
* **ci:** use native arm64 runner and remove cross config ([26d3cb9](https://github.com/RouHim/rhinco-tv/commit/26d3cb9dfaba37440bc6c0b4836c529c5f1927a3))

# 1.0.0 (2026-01-17)


### Bug Fixes

* add Cross.toml for musl cross-compilation dependencies ([b6dc96b](https://github.com/RouHim/rhinco-tv/commit/b6dc96b61dad24ac320ee5e21a5b076bc27a8555))
* address code review feedback ([0ffdbce](https://github.com/RouHim/rhinco-tv/commit/0ffdbce8469eea24325fe0ec46942d32e0b31879))
* battery icon alignment and build error in gamepad interval ([d74d52d](https://github.com/RouHim/rhinco-tv/commit/d74d52d3573d480ecfdcabed7a92339b1619e416))
* **ci:** add missing @semantic-release/exec dependency ([1f1d029](https://github.com/RouHim/rhinco-tv/commit/1f1d0298adb1c515db7fbe585f65a5d28a9249ea))
* **ci:** compile eudev with -fPIC to fix linker error ([9cc47f3](https://github.com/RouHim/rhinco-tv/commit/9cc47f3a1e4122023bc2cc59b0032579b614bc12))
* **ci:** install system dependencies for build and cross-compile ([893015f](https://github.com/RouHim/rhinco-tv/commit/893015f22aa65bf85487a6709f0cc595ef4ef153))
* **ci:** update workflow configuration and semantic-release setup ([7a3c830](https://github.com/RouHim/rhinco-tv/commit/7a3c830a22803fe77afeaf09df754f115fa362b8))
* cleanup unreachable code and improve error handling ([64138ef](https://github.com/RouHim/rhinco-tv/commit/64138ef7f341760346944166da83745676e1518b))
* correct YAML indentation in CI workflow ([3e41a82](https://github.com/RouHim/rhinco-tv/commit/3e41a82623136a66e2bd9884030ac6c76fbc2b94))
* **gamepad:** fix memory leak and inverted y-axis mapping ([87b5fb6](https://github.com/RouHim/rhinco-tv/commit/87b5fb6ef4fa366edec787a3b30948c4ab2bb293))
* improve GPU detection to list all installed GPUs using lspci ([7a87b0a](https://github.com/RouHim/rhinco-tv/commit/7a87b0ae5207206e61ff4173ddb6fd03ae8e0604))
* improve keyboard detection logic for controllers ([56bfc75](https://github.com/RouHim/rhinco-tv/commit/56bfc759721408fed72abbf2715cddc2f9a8b583))
* include 'Display' class in GPU detection to find secondary GPUs ([a1da455](https://github.com/RouHim/rhinco-tv/commit/a1da455c77f1b8e4f4cc53d5b37fc75634d3a318))
* install libudev-dev on ARM64 runner to resolve build failure ([d3d6493](https://github.com/RouHim/rhinco-tv/commit/d3d649313aa1f31cc587a0cc28c42156ebfc37dd))
* install pkg-config for musl builds ([90381fb](https://github.com/RouHim/rhinco-tv/commit/90381fb10750f9ff5b9b72ce43f3454ce9546c2f))
* multi-controller axis interference and unknown battery visibility ([20a30a6](https://github.com/RouHim/rhinco-tv/commit/20a30a61f8ed921e436080575c70cb5b2ff5d115))
* resolve focus manager polling issues ([93a9b02](https://github.com/RouHim/rhinco-tv/commit/93a9b02f137b400fd2241dd527c3d4688b97cb11))
* **ui:** remove duplicate update check and blocking IO in startup ([1e7654a](https://github.com/RouHim/rhinco-tv/commit/1e7654af014408a2e789ef713b0c5c20581c2667))


### Features

* Add controller bindings help modal with persistent hint ([196d82c](https://github.com/RouHim/rhinco-tv/commit/196d82cba6470ba28224309e16b232dbe704a484))
* add disk usage and ZRAM info to system info modal ([05f1bf8](https://github.com/RouHim/rhinco-tv/commit/05f1bf87258f66f23d58687b3f40518adf089b6b))
* add GPU numbering for multiple GPUs ([b26b96c](https://github.com/RouHim/rhinco-tv/commit/b26b96cf26a294888bb8ece3fba6561d35c8a718))
* add launch history keys ([dead1c7](https://github.com/RouHim/rhinco-tv/commit/dead1c7221b5c8fe9932c5fe8238b230512c682e))
* add more search paths for Proton GE detection ([c01e4c7](https://github.com/RouHim/rhinco-tv/commit/c01e4c7bfad347ba1730d646a7cacd0fbb077af7))
* Add Sansation font as static resource ([3c7ef28](https://github.com/RouHim/rhinco-tv/commit/3c7ef28e10eef3c0a42ac35eb88f16c9e883dc5d))
* Add shoulder button tab navigation with LB/RB and LT/RT ([12e6ea1](https://github.com/RouHim/rhinco-tv/commit/12e6ea1c30cab11978f4d6cfa8d2a1674fba7aff))
* add System Info modal with gaming-relevant details ([ce93017](https://github.com/RouHim/rhinco-tv/commit/ce930176d1318bd962977cb4604b9d95b819c79f))
* app removal, ui polish and parallel scanning ([fa92c86](https://github.com/RouHim/rhinco-tv/commit/fa92c86cff8caeae971411558f18a787d64dd28c))
* Block keyboard/gamepad input while game is running ([29ac8ac](https://github.com/RouHim/rhinco-tv/commit/29ac8ac30c475dfce3ee6e079c305b57d7d57e6d))
* cascading image lookup with Heroic + SteamGridDB + SearXNG ([771a0c4](https://github.com/RouHim/rhinco-tv/commit/771a0c444a39fd379089fe2ee95bd1c6c47c5ee5))
* Center tabs horizontally on screen ([9688f30](https://github.com/RouHim/rhinco-tv/commit/9688f30709fcbe5123679fc53d1b1d39ae34fa84))
* implement 'Add App' picker with XDG scanning and smart scrolling ([07932e8](https://github.com/RouHim/rhinco-tv/commit/07932e8ecd5d9964bca73f6322e6b51e50559a60))
* implement context menu and quit shortcut ([8cb0679](https://github.com/RouHim/rhinco-tv/commit/8cb06790c28ce1f059d233d087a678a5bdec7c4f))
* implement held button repeats for faster gamepad navigation ([724a0dc](https://github.com/RouHim/rhinco-tv/commit/724a0dc60fe396baedb6f794d8d8b4f15c4acc74))
* improve keyboard vs gamepad detection heuristic ([a45105d](https://github.com/RouHim/rhinco-tv/commit/a45105d39af97f0a74cc9fab69a955f6f9899234))
* improve Proton detection with version file reading ([8be1fec](https://github.com/RouHim/rhinco-tv/commit/8be1fec79ba7a7836d6d0a6725fec9d42c06b4d8))
* integrate Iced 0.14 Grid and SteamGridDB artwork ([afa2fad](https://github.com/RouHim/rhinco-tv/commit/afa2fad394ed4eb074bf490b1d4c9cf005d28815))
* migrate to musl-based static builds using rust-musl-cross ([a1e005b](https://github.com/RouHim/rhinco-tv/commit/a1e005b7d4daf208c3f79bc52f697eda1af9a05c))
* Replace SVG system icons with FontAwesome via iced_fonts ([3ab8958](https://github.com/RouHim/rhinco-tv/commit/3ab8958f7691be286eed7963cb634c1ddd995052))
* secure SteamGridDB API key and refactor config logic ([2863a58](https://github.com/RouHim/rhinco-tv/commit/2863a587144925b2413c769e1bf27be7bf41925e))
* show controller name on tooltip ([249ff7a](https://github.com/RouHim/rhinco-tv/commit/249ff7a88f04a0b2832ef17b2974cadf2f1a3293))
* show keyboard icon if controller name contains 'keyboard' ([30f745e](https://github.com/RouHim/rhinco-tv/commit/30f745ed7622ec5664b30c0d69896519dd08a9d7))
* show versions for Wine, Proton, and Proton GE in system info ([145e10c](https://github.com/RouHim/rhinco-tv/commit/145e10c45c4295b7a86667b2f6462ff7ef80aa87))
* update input and ui ([d4e0679](https://github.com/RouHim/rhinco-tv/commit/d4e0679745e16513dee51aba5403d87eccf91406))
* upgrade ureq from 2.12 to 3.1.4 ([88c36b6](https://github.com/RouHim/rhinco-tv/commit/88c36b6afab36f828524a5260b83d207d26b2b43))


### Performance Improvements

* add sccache for faster compilation ([9195dd9](https://github.com/RouHim/rhinco-tv/commit/9195dd9b137569f34084d1b9ee17e5b2d5f8cda9))
