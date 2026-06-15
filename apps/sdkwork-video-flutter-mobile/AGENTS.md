# SDKWork Video Flutter Mobile Application Agent Instructions

## SDKWORK Soul

Read `../../../../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../../../../sdkwork-specs/README.md`
- `../../../../sdkwork-specs/SOUL.md`
- `../../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

This is the Flutter Mobile application root for SDKWork Video. It follows `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md` for iOS and Android Flutter packaging.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `sdkwork.app.config.json`: Flutter Mobile application manifest.
- `.sdkwork/`: local workspace metadata, skills, and plugins.
- `specs/`: local component/application specs.
- `sdks/`: SDK workspaces and generator inputs.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` when present.
3. Read local `specs/README.md` and `specs/component.spec.json` when present.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` when relevant.
5. Read `../../../../sdkwork-specs/README.md` and the task-specific root specs.
6. Inspect implementation files only after the relevant dictionary entries are clear.

## Required Specs By Task Type

- Flutter application architecture: `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md`, `APPLICATION_SPEC.md`, `APP_SDK_INTEGRATION_SPEC.md`.
- Flutter UI: `APP_FLUTTER_UI_SPEC.md`, `UI_ARCHITECTURE_SPEC.md`, `FRONTEND_SPEC.md`.
- Cross-client alignment: `APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`.
- Any code change: `CODE_STYLE_SPEC.md`, `NAMING_SPEC.md`, plus only the touched language/framework spec.

## Build, Test, and Verification

Run `flutter pub get`, `flutter analyze`, `flutter test`, `flutter build apk`, `flutter build appbundle`, `flutter build ipa` for development and verification.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the task is explicitly about generated artifacts and the source contract is verified. Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning module, package, or app root. Record the exact verification commands and important outputs before reporting completion.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, or changing generated SDK ownership. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.