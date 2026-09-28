Installation


> **⚠️ Production warning.** Never expose `/agent/chat*` on the public internet without an upstream rate-limit. The module's own daily-message limit is per conversation, not per IP — a bare `curl` loop that opens a new conversation on every request bypasses it entirely and burns LLM budget. See [Production deployment prerequisites](#production-deployment-prerequisites) below before going live.

> **⚠️ Comment is a hard prerequisite (MYO-379).** `Hook\Admin\ReviewsTabHook` reads the core `Comment` module's back-office presenter to add the product "Reviews" tab. Activate `Comment` **before** `CommerceAgents` — on a fresh `bin/install`, `Comment` is not among the modules the core installer activates, so this is always a manual step. If Comment ends up inactive anyway, the container still compiles (the dependency is wired with `nullOnInvalid()`) and the tab simply doesn't render — but the "Customer reviews replies" configurable agent preset stays unavailable too (see `ModuleAvailability`).

```bash
# from the Thelia project root
php Thelia module:activate Comment

git clone https://github.com/emmanuelnurit/CommerceAgents.git local/modules/CommerceAgents
php Thelia module:refresh
php Thelia module:activate CommerceAgents
php Thelia cache:clear
```

Activation creates the module's tables (see **Database** below) and seeds the model catalog. Upgrades run the SQL files in `Config/update/` and re-seed the catalog without touching rows edited by hand.

Then open **Modules › CommerceAgents › Configure** in the back office and set a provider key. The front widget and both assistants stay silent until a key is configured.

> **⚠️ `module:refresh` is not just an install step — run it on every release too (MYO-461/MYO-463).** `php Thelia module:refresh` is what makes Thelia re-read `Config/module.xml` and update the `module.version` row in the database. A `git pull`/deploy that updates the module's code **without** also running `module:refresh` leaves the back office reporting the *old* version indefinitely — nothing else re-reads `Config/module.xml` on its own. This bit a real deployment: the BO showed `0.3.8` while the code was already at `0.4.2`, three releases (0.4.0/0.4.1/0.4.2) after the last `module:refresh`. Treat `module:refresh` as a mandatory step of **every** deploy that updates this module's code, not a one-time install action:
>
> ```bash
> git -C local/modules/CommerceAgents pull   # or however the deploy updates the module's code
> php Thelia module:refresh                  # mandatory on every release, not just first install
> php Thelia cache:clear
> ```
>
> `scripts/check-module-db-drift.sh` (cron filet, MYO-463) detects when this step was skipped by comparing `Config/module.xml` to the live `module.version` row and alerting via the same notification path as `check-version-drift.sh` — but it only catches the drift after the fact. Running `module:refresh` as part of the deploy procedure itself is what actually prevents it.
