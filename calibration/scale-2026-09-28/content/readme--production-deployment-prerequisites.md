Production deployment prerequisites


The module's own guardrails (daily message limit per conversation, monthly budget, server-side scope checks) assume the surrounding environment is sane. Three prerequisites below are **not implemented in the module's code on purpose** — they are operational, not application concerns — and were called out by the [security audit](docs/security-audit-commerceagents.md) (findings M1, B1, B6). Apply all three before pointing a real domain at a store running CommerceAgents.

### M1 — Rate-limit `/agent/chat*` by IP

**✅ En place ici, prouvé le 2026-09-15 (MYO-406).** Appliqué dans la config nginx versionnée de ce dépôt : `.ddev/nginx_full/nginx-site.conf` (zone `limit_req_zone` + `location ~ ^/agent/chat`). Preuve par exécution réelle : boucle `curl` de 30 requêtes consécutives sur `/agent/chat` → les 21 premières passent (403 applicatif, en-tête `X-Requested-With` manquant), la 22ᵉ et les suivantes reçoivent **429** de nginx ; navigation FO normale (accueil, sitemap, `/admin/login`) restée en 200 pendant et après le test. Transcript complet en commentaire sur le ticket.

**Risk.** The module limits messages per *conversation* per day, but a client can open a fresh conversation on every request (no login required for the shopping assistant). Without an upstream limit, a simple loop drives unbounded LLM spend and can exhaust the LLM provider's own rate limit for the whole store.

**Recommended value.** Something in the region of 10 requests/minute per IP with a small burst allowance — adjust to real traffic once you have some.

nginx (`limit_req`):

```nginx
# http { } block
limit_req_zone $binary_remote_addr zone=agentchat:10m rate=10r/m;

# server { } block, in front of the PHP upstream
location ~ ^/agent/chat {
    limit_req zone=agentchat burst=20 nodelay;
    limit_req_status 429;

    # ... existing fastcgi_pass / proxy_pass to the app
}
```

Equally valid: a Symfony `RateLimiter` wired at the kernel/firewall level, or a rule on whatever reverse proxy, CDN or WAF already sits in front of the store.

### B1 — Cap the request body size

**✅ En place ici, prouvé le 2026-09-15 (MYO-406).** Même fichier, même `location` : `client_max_body_size 64k;`. Preuve par exécution réelle : POST de 200 015 octets sur `/agent/chat` rejeté avec **413** par nginx (page d'erreur nginx brute, pas le JSON applicatif) ; un POST de taille normale continue d'atteindre PHP (403 applicatif attendu, en-tête manquant, pas 413/429). Transcript complet en commentaire sur le ticket.

**Risk.** `/agent/chat*` validates message length in PHP, after the body has already been read into memory. An oversized POST forces the server to buffer and parse it before that check ever runs.

**Recommended value.** Chat messages are short text — a few KB is enough headroom; 64 KB is a comfortable, still-tight ceiling.

nginx:

```nginx
location ~ ^/agent/chat {
    client_max_body_size 64k;
}
```

PHP (`php.ini`, applies store-wide, not just to this route):

```ini
post_max_size = 1M
upload_max_filesize = 1M
```

### B6 — Strong `kernel.secret` entropy + rotation procedure

**This is the most important of the three.** Every channel connector's settings (mail, Mattermost, Slack — stored centrally as module config values since MYO-300, see **Channels**) and, since the H4 fix, every LLM provider API key saved in the **Providers** tab are encrypted at rest with sodium `secretbox` (`Service/Channel/ChannelSettingsEncryptor` — key derived from Symfony's `kernel.secret`). A default or low-entropy `kernel.secret` (e.g. a fresh Symfony skeleton's placeholder value, never regenerated) makes that encryption decorative: anyone who can read the codebase's default can derive the same key.

**Generate a strong value:**

```bash
php -r "echo bin2hex(random_bytes(32)), PHP_EOL;"
```

Set it as `APP_SECRET` in `.env.local` (never commit this file) and confirm `kernel.secret: '%env(APP_SECRET)%'` in the framework config actually reads from it — do this once, before the store ever has real provider keys or channel settings saved.

**Rotation procedure — and what it breaks.** `kernel.secret` rotation is a one-way operation for anything already encrypted with the old value:

1. Generate a new value with the command above.
2. **Before overwriting `APP_SECRET`**, run `bin/console commerce-agents:rotate-secrets --old-secret=<current value> --new-secret=<new value>` (add `--dry-run` first to preview). This re-encrypts every stored provider API key and every connector's Channels settings in place, in the database, using the old secret to decrypt and the new one to re-encrypt — both secrets are passed explicitly on the command line because the running container is only ever compiled against one of them. A row that fails to decrypt with `--old-secret` (already rotated, corrupted, or a pre-encryption legacy plaintext row) is reported and left untouched rather than silently dropped.
3. Only then update `APP_SECRET` in `.env.local` to the new value and clear the cache (`bin/console cache:clear`; also clear `var/propel/<env>` if Propel's cached DSN needs to pick up an unrelated database change at the same time). Doing this before step 2 would compile the kernel against the new secret while the rows are still encrypted with the old one, and the rotation command would then have neither secret available from `%kernel.secret%` to decrypt with.
4. Confirm the rotation worked: reload a connector's settings in **Configuration › Channels** or run a provider's connection test in **Configuration › Providers**. If a row was reported as skipped in step 2, it was not migrated and must be re-entered by hand (this remains the fallback for anything the command can't decrypt — it is no longer the primary path).
5. Rotate on suspected compromise of the secret or the database at minimum; the module does not enforce or track a fixed rotation schedule.

**Verified by execution, 2026-09-15 (MYO-407):** the procedure above was run end-to-end in an isolated worktree + dedicated database (never against the shared dev database, following the safe-install.sh guardrail) — `commerce-agents:rotate-secrets` did not exist beforehand (the previous revision of this section documented manual re-entry as "the supported path" because no bulk command existed; it was built for this drill and is now the documented default). A synthetic provider API key and a synthetic mail-connector setting were seeded, `kernel.secret` was rotated, and both rows were confirmed to decrypt correctly to their original values under the new secret — and confirmed to *no longer* decrypt under the old one, proving the rotation is real and one-way as described above.

**Entropy audit, 2026-09-15 (MYO-407):** this workspace's `APP_SECRET` was audited (length, distinct-character count, character-class mix, longest contiguous readable run, repeated substrings) without ever recording the value itself. Verdict: **not acceptable for production** — 32 characters but only 13 distinct (lowercase + digits only), including a 9-character contiguous run of plain lowercase letters and a repeated 3-character substring. That profile is consistent with a hand-typed placeholder, not `bin2hex(random_bytes(32))` output (which would show close to the full hex alphabet spread evenly, no long readable run, and no repeats over a 32-character sample beyond coincidence). Any install carrying a secret with this profile should rotate it via the procedure above before going live. Consequently, this workspace's own `APP_SECRET` was rotated too (after `scripts/backup-secrets.sh` + `scripts/backup-database.sh`): no `channel_connector_*_settings` row existed yet on this install, and its one stored provider API key predates the encryption fix (plain text, unaffected by `kernel.secret`), so the rotation had nothing encrypted left to migrate here — it only replaced the weak secret with a freshly generated one.
