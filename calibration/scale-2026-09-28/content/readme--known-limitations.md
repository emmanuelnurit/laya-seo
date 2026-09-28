Known limitations


- UI and prompts ship in English, French, Spanish and Italian (`I18n/`) — narrower than the back-office theme's 21-locale set (`templates/backOffice/default-twig/translations/`). **Fallback verified 2026-09-16 (MYO-419):** real HTTP checks (admin session locale forced via `/admin/...?lang=<locale>`, not code reading) on 3 uncovered locales (`de_DE`, `cs_CZ`, `nl_NL`) against the **Agents IA** list screen and the module configuration screen show clean, readable English — no raw translation keys, no blank strings, no errors — while the surrounding back-office chrome stays correctly localized in the target language. One related bug was found and fixed in the same pass: the Slack/Mattermost webhook URL and the e-mail channel's recipient field descriptions were hardcoded French strings that bypassed the translation domain entirely (they showed French even under `en_US`); they now go through `trans(..., CommerceAgents::DOMAIN_NAME)` like the rest of the module.

See `CHANGELOG.md` for the full feature list (V1 and post-V1) and `docs/guide-exploitation.md` (French) for a short day-to-day operator guide.
