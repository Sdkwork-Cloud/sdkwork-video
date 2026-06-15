# SDKWork Video Flutter Mobile Application

## Purpose
Flutter mobile application root for SDKWork Video.

## Architecture
This application root follows `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md` for iOS and Android Flutter packaging.

## Directory Structure
```
apps/sdkwork-video-flutter-mobile/
  .sdkwork/           # Workspace metadata and skills
  config/             # Configuration templates by runtime target
  docs/               # Architecture notes and runbooks
  lib/                # Root shell entry and composition boundary
  packages/           # Reusable Dart/Flutter packages
  scripts/            # Build, validation, generation utilities
  sdks/               # SDK workspaces and generator inputs
  specs/              # Local component/application specs
  test/               # Application-level integration tests
```

## Package Taxonomy
- `sdkwork_video_flutter_mobile_core/` - Core runtime, SDK factories, token manager
- `sdkwork_video_flutter_mobile_commons/` - Shared Flutter UI/runtime primitives
- `sdkwork_video_flutter_mobile_shell/` - App shell, route assembly, navigation
- `sdkwork_video_flutter_mobile_<capability>/` - App/user domain features
- `sdkwork_video_flutter_mobile_console_<capability>/` - User-facing management console
- `sdkwork_video_flutter_mobile_admin_<capability>/` - Internal admin operations
- `sdkwork_video_flutter_mobile_host/` - Platform adapters

## Related Specs
- `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md`
- `APP_FLUTTER_UI_SPEC.md`
- `APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`
- `APP_SDK_INTEGRATION_SPEC.md`
- `CONFIG_SPEC.md`

## Commands
```bash
flutter pub get
flutter analyze
flutter test
flutter build apk
flutter build appbundle
flutter build ipa
```