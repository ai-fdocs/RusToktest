# Phase 3: Admin Auth & User Security (Standard Flow, No SSO)

This document outlines the Phase 3 scope for **standard** multi-language admin authentication
and user security flows. It intentionally excludes SSO/OIDC/SAML to keep the first iteration
simple and production-ready.

## Goals

- Ship a production-grade login/register experience in the admin panel.
- Provide user profile management with password change and session management.
- Ensure full RU/EN localization coverage for UI, emails, and validation.
- Keep flows consistent with multi-tenant access patterns.
- Prioritize **multi-language UX** and **parallel flow support** (registration, invites,
  password reset, and profile/security actions can be developed and shipped independently).

## In Scope (MVP)

### 1) Authentication

- **Login page**
  - Tenant slug + email + password.
  - Clear error messages for invalid credentials and missing fields.
  - Remember language choice (persisted client-side).
  - Parallelizable: can ship while registration and reset are still in progress.
- **Password reset**
  - Request reset email.
  - Reset link with token + new password.
  - Token expiration handling.
  - Parallelizable with registration and invites.
- **Email verification**
  - Verify email after registration (or optional soft-verify for internal users).
  - Resend verification email action.
  - Parallelizable with password reset.

### 2) Registration

- **Sign-up form**
  - Email + password + tenant slug.
  - Optional name field.
  - Password strength hints.
  - Parallelizable with login, password reset, and profile.
- **Invite-based onboarding**
  - Accept invitation links with role pre-selected.
  - Expired invitation handling.
  - Can be delivered separately from open registration.

### 3) User Profile & Security

- **Profile page**
  - Update name, avatar, timezone, preferred language.
  - Separate user-facing language preference from admin default language.
- **Change password**
  - Requires current password.
  - Show password policy hints.
  - Parallelizable with session list.
- **Active sessions**
  - List recent sessions (device, IP, last active).
  - “Sign out all” action.
  - Parallelizable with login history.
- **Login history**
  - Successful/failed logins with timestamps and IPs.
  - Use localized date/time formatting.

## Localization (RU/EN)

- All auth/profile UI strings are localized.
- Email templates are localized: verify email, reset password, invite.
- Locale selection persists across sessions.
- Ensure validation errors are localized and context-aware (field + error type).

## Data & Audit

- Track audit events for:
  - Logins (success/failure).
  - Password changes.
  - Session invalidations.
  - Email verification changes.
  - Invite accepted/expired events.

## UX Notes

- Keep forms minimal and mobile-friendly.
- Use inline validation with precise messages.
- Use clear empty states for sessions/log history.
- Prefer UX patterns that allow teams to develop features in parallel:
  - shared auth UI components,
  - isolated endpoints per flow,
  - independent feature flags.

## Out of Scope (Phase 3)

- SSO (OIDC/SAML).
- Passwordless magic links.
- 2FA / TOTP (planned for future phase).
