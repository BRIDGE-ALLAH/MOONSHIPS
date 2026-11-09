# MOONSHIPS — Engineering TODO & Product Roadmap

> **Status:** Draft backlog hasil static audit tahap pertama + product brainstorm.  
> **Target branch:** `rebrand/moonships-full-brand`  
> **Prioritas:** P0 = release blocker; P1 = security/reliability wajib sebelum production; P2 = hardening/quality; P3 = product expansion.  
> **Catatan audit:** Temuan di bawah adalah hasil static review pada jalur kritis, bukan klaim bahwa seluruh 368 file telah diaudit baris demi baris. Setiap task wajib diverifikasi ulang terhadap HEAD terbaru sebelum implementasi. Jangan menandai selesai hanya karena kode sudah ditulis; bukti test dan review harus tersedia.

---

## 0. Aturan kerja dan Definition of Done

### Global engineering rules
- [ ] **G-01 — Compatibility & migration safety**
  - **SPEC**
    - Pertahankan kompatibilitas runtime dan format data Pier yang masih dipakai oleh instalasi existing, kecuali ada migration plan eksplisit.
    - Semua perubahan schema harus additive/backward-compatible terlebih dahulu; destructive migration harus melalui fase deprecation dan backup.
    - Jangan menghapus identifier, environment variable, path, service label, atau API field legacy tanpa inventory pemakaian dan migration.
  - **RULE**
    - Dilarang melakukan destructive migration otomatis saat startup.
    - Dilarang menimpa konfigurasi pengguna tanpa backup dan konfirmasi.
    - Semua migrasi harus idempotent: aman dijalankan ulang.
    - Simpan AGPL/license notices dan atribusi upstream yang diwajibkan.
  - **DESIGN**
    - Buat migration module terpisah dengan version marker, preflight, backup, apply, verify, dan rollback guidance.
    - Catat perubahan kompatibilitas di `docs/migrations/`.
  - **ACCEPTANCE CRITERIA**
    - [ ] Upgrade dari versi sebelum perubahan berhasil pada fixture database/config yang representatif.
    - [ ] Menjalankan migrasi dua kali tidak merusak data.
    - [ ] Backup dibuat dan dapat diverifikasi sebelum operasi destruktif.
    - [ ] Test membuktikan konfigurasi/service lama tetap terbaca.
    - [ ] Dokumentasi menjelaskan upgrade, rollback, dan breaking changes.

- [ ] **G-02 — Secure-by-default, observability, dan test evidence**
  - **SPEC**
    - Setiap endpoint sensitif, proses privileged, deployment, backup, dan perubahan credential harus punya kontrol akses, audit trail, error handling, dan test.
    - Log harus membantu diagnosis tanpa membocorkan secret, token, password, private key, atau payload sensitif.
  - **RULE**
    - Default harus deny untuk akses yang tidak terverifikasi.
    - Tidak boleh menganggap secret kosong sebagai validasi yang berhasil.
    - Tidak boleh mencatat secret mentah ke log atau error response.
    - Semua task wajib menyertakan unit/integration test sesuai risiko.
  - **DESIGN**
    - Gunakan shared authorization policy, typed error, structured audit event, dan redaction helper.
    - Standarkan correlation/request ID pada alur API dan background job.
  - **ACCEPTANCE CRITERIA**
    - [ ] Negative tests membuktikan unauthorized/underprivileged request ditolak.
    - [ ] Log dan response diuji tidak mengandung secret.
    - [ ] Test failure path, timeout, retry, dan partial failure tersedia.
    - [ ] Dokumentasi operasional diperbarui.

---

# 1. P0 — Release blockers

- [ ] **P0-01 — Perbaiki Docker build context dan workspace manifests**
  - **SPEC**
    - Audit Dockerfile terhadap root workspace `Cargo.toml`, seluruh workspace member, file yang di-`COPY`, build script, asset runtime, dan direktori template.
    - Audit awal menemukan indikasi `COPY templates/ templates/` merujuk direktori root yang tidak terlihat pada tree, serta tahap manifest copy belum mencakup semua workspace member (`pier-net-helper`, `pier-tests`).
    - Hasil akhir harus menghasilkan image reproducible dari clean checkout, bukan hanya dari working tree developer.
  - **RULE**
    - Jangan menghapus workspace member dari manifest hanya untuk membuat Docker build lolos.
    - Jangan memakai file lokal yang tidak committed sebagai dependency build.
    - Pin base image/toolchain secara eksplisit dan gunakan build stage non-root bila memungkinkan.
    - Jangan memasukkan secret build-time ke image layer.
  - **DESIGN**
    - Buat Docker build stage yang menyalin root manifest/lockfile dan manifest semua crate sebelum source untuk dependency caching.
    - Salin asset runtime hanya dari path yang benar-benar ada; jika asset optional, buat conditional build/copy yang eksplisit.
    - Tambahkan `.dockerignore` yang aman dan dokumentasikan build args yang didukung.
  - **ACCEPTANCE CRITERIA**
    - [ ] `docker build` berhasil dari clean checkout pada CI.
    - [ ] Build menggunakan `Cargo.lock` dan mode locked.
    - [ ] Image dapat start pada smoke test dan endpoint health/readiness merespons.
    - [ ] Binary/helper/asset yang dibutuhkan tersedia di image pada path yang diharapkan.
    - [ ] Tidak ada credential atau file lokal developer di image.
    - [ ] Build log dan ukuran image direview; tidak ada dependency workspace yang terlewat.

- [ ] **P0-02 — Jadikan CI sebagai gerbang wajib sebelum release**
  - **SPEC**
    - Workflow release tidak boleh publish artifact/tag/image jika validasi CI untuk commit yang sama belum sukses.
    - Audit awal menemukan release workflow berjalan pada push ke `main` tanpa dependency eksplisit pada workflow CI. Branch protection belum diverifikasi dan harus dicek terpisah.
  - **RULE**
    - Release hanya dari protected branch dan commit yang sudah lolos checks.
    - Jangan gunakan `continue-on-error` pada security/build/test gate wajib.
    - Hindari kondisi race: CI untuk commit lama tidak boleh mengesahkan commit baru.
  - **DESIGN**
    - Konfigurasikan branch protection/ruleset untuk mewajibkan PR dan status checks.
    - Buat release workflow memakai `workflow_run` dengan validasi branch + head SHA, atau mekanisme dependency yang menjamin checks milik commit yang sama.
    - Pisahkan build/test, package, sign, dan publish agar publish hanya terjadi setelah gate.
  - **ACCEPTANCE CRITERIA**
    - [ ] Simulasikan CI gagal: tidak ada release/tag/image publish.
    - [ ] Simulasikan CI sukses untuk SHA yang sama: release dapat berjalan.
    - [ ] CI dari SHA berbeda tidak dapat membuka gate.
    - [ ] Branch protection/ruleset terdokumentasi dan diverifikasi di GitHub.
    - [ ] Release log menyimpan commit SHA dan hasil checks.

---

# 2. P1 — Security & reliability wajib sebelum production

- [ ] **P1-01 — Wajibkan Global Admin/Owner untuk mengubah security settings**
  - **AREA**
    - `crates/pier-core/src/api/security.rs`
    - `crates/pier-core/src/api/mod.rs`
  - **SPEC**
    - Endpoint `/security/settings` yang mengubah `security.require_password_on_delete` harus dibatasi ke Global Admin/Owner.
    - Hak akses harus diperiksa di server untuk setiap request, bukan mengandalkan visibility UI.
    - Perubahan setting harus dapat diaudit.
  - **RULE**
    - Project Editor, Viewer, dan user authenticated biasa tidak boleh mengubah global security policy.
    - Unknown role, missing identity, atau gagal memuat role harus fail closed.
    - Jangan mengubah behavior default secara diam-diam; pertahankan nilai existing saat upgrade.
  - **DESIGN**
    - Tambahkan AuthUser/context ke handler dan gunakan policy/helper authorization global yang konsisten.
    - Tempatkan route pada router yang menerapkan `require_global_admin`, atau lakukan guard eksplisit yang setara.
    - Tulis audit event berisi actor ID, key setting, old/new value, timestamp, request ID; jangan simpan token.
  - **ACCEPTANCE CRITERIA**
    - [ ] Global Admin/Owner dapat mengubah setting.
    - [ ] Project Admin/Editor/Viewer dan unauthenticated ditolak (403/401 sesuai kontrak).
    - [ ] Test mencakup role tidak dikenal dan kegagalan lookup role.
    - [ ] Audit event tercipta untuk perubahan sukses dan tidak untuk perubahan gagal.
    - [ ] UI menampilkan error permission yang jelas tanpa menjadi satu-satunya kontrol.
    - [ ] API docs dan security settings docs diperbarui.

- [ ] **P1-02 — Batasi DB Browser dengan least-privilege credentials dan database allowlist**
  - **AREA**
    - `crates/pier-core/src/api/db_browser.rs`
  - **SPEC**
    - Query DB Browser harus dibatasi pada database/schema yang memang dimiliki resource/service yang dipilih.
    - Jangan gunakan cluster superuser/root credential untuk operasi SQL interaktif pengguna.
    - `RunRequest.database` dari client harus divalidasi terhadap resource binding server-side.
  - **RULE**
    - Client tidak boleh menentukan host, username, password, atau target database arbitrary.
    - Project Editor tidak otomatis mendapat hak superuser pada seluruh instance/container.
    - Query destructive/DDL harus mengikuti permission yang eksplisit; Viewer hanya read-only.
    - Kegagalan validasi target harus ditolak sebelum membuka koneksi.
  - **DESIGN**
    - Saat provisioning DB, buat credential khusus database/service dengan grant minimum.
    - Simpan mapping resource ID → engine, host, database/schema allowlist, credential reference.
    - Resolve target sepenuhnya dari server-side resource ID; abaikan/reject nama database yang tidak ada di allowlist.
    - Pisahkan capability `db.read`, `db.write`, dan `db.admin`; terapkan read-only transaction/session untuk Viewer.
    - Rencanakan migrasi credential lama dari root ke credential scoped dengan fallback yang terkontrol dan masa transisi.
  - **ACCEPTANCE CRITERIA**
    - [ ] Editor pada Service A tidak dapat membaca/menulis database Service B.
    - [ ] Request dengan nama database arbitrary ditolak.
    - [ ] Viewer tidak dapat menjalankan INSERT/UPDATE/DELETE/DDL.
    - [ ] Credential runtime bukan postgres/root untuk operasi normal.
    - [ ] Test untuk PostgreSQL dan MySQL/MariaDB mencakup allowlist, role, dan privilege.
    - [ ] Credential lama dimigrasikan atau ditandai dengan instruksi remediation yang jelas.
    - [ ] Audit log mencatat actor, resource, jenis operasi, dan status tanpa menyimpan SQL sensitif secara default.

- [ ] **P1-03 — Wajibkan verifikasi signature untuk semua webhook event**
  - **AREA**
    - `crates/pier-core/src/api/webhooks.rs`
  - **SPEC**
    - Semua event webhook yang dapat mengubah state atau memicu deploy wajib diverifikasi sebelum payload diproses.
    - Service tanpa webhook secret tidak boleh menerima event sebagai trusted.
    - Event GitHub/GitLab harus memakai algoritma/header signature yang sesuai provider.
  - **RULE**
    - Secret kosong/missing = reject, bukan bypass.
    - Verifikasi dilakukan sebelum parsing/side effect yang mengubah state.
    - Jangan log secret atau signature mentah.
    - Tolak signature malformed, body mismatch, unknown provider, dan replay delivery ID bila mekanisme tersedia.
  - **DESIGN**
    - Ambil raw request body, verifikasi HMAC/signature menggunakan constant-time comparison, baru parse payload.
    - Terapkan verifikasi pada push, installation, ping, dan event lain; tidak boleh ada jalur event khusus yang melewati guard.
    - Simpan webhook secret terenkripsi dan sediakan rotasi secret dengan overlap terbatas.
    - Simpan delivery ID + timestamp secara idempotent untuk mencegah replay/duplikasi.
    - Sediakan UI/API untuk generate, rotate, test, dan revoke secret.
  - **ACCEPTANCE CRITERIA**
    - [ ] Webhook tanpa secret ditolak.
    - [ ] Signature valid diterima untuk event yang didukung.
    - [ ] Signature invalid/missing/malformed ditolak sebelum side effect.
    - [ ] Installation event tidak dapat mengubah mapping tanpa signature valid.
    - [ ] Replay delivery ID ditangani idempotently/ditolak sesuai desain.
    - [ ] Test mencakup GitHub dan GitLab serta payload kosong/oversized.
    - [ ] Dokumentasi migrasi secret lama tersedia; deploy tidak diam-diam menjadi unauthenticated.

- [ ] **P1-04 — Verifikasi integritas binary privileged helper sebelum instalasi**
  - **AREA**
    - `scripts/install.sh` dan mekanisme bootstrap/helper release
  - **SPEC**
    - Setiap helper binary yang diunduh dan dijalankan dengan privilege root harus diverifikasi integritas dan asalnya.
    - Instalasi gagal tertutup jika checksum/signature tidak tersedia atau tidak cocok.
  - **RULE**
    - Dilarang menjalankan binary hasil download sebelum verifikasi.
    - Jangan menulis langsung ke path final sebelum validasi.
    - Jangan menerima checksum dari channel yang sama tanpa autentikasi tambahan sebagai satu-satunya trust root.
  - **DESIGN**
    - Download ke file temporary dengan permission ketat.
    - Verifikasi SHA-256 dari manifest release yang dipercaya; idealnya validasi signature/provenance (mis. Sigstore/cosign) dengan identity/issuer yang dipin.
    - Setelah lolos, install atomically, set owner/mode yang benar, lalu lakukan version/self-test.
    - Sediakan cache dan mekanisme retry yang tidak menurunkan standar verifikasi.
  - **ACCEPTANCE CRITERIA**
    - [ ] Checksum mismatch menyebabkan instalasi gagal dan file temporary dihapus.
    - [ ] Manifest/checksum hilang menyebabkan fail closed.
    - [ ] Binary hanya dipindahkan ke path final setelah validasi.
    - [ ] Test shell mencakup download gagal, file kosong, checksum salah, permission salah, dan release tidak tersedia.
    - [ ] Prosedur rotasi signing identity/key terdokumentasi.

- [ ] **P1-05 — Enkripsi credential backup yang saat ini tersimpan plaintext di SQLite**
  - **AREA**
    - `crates/pier-core/src/backup/executor.rs`
    - tabel `database_credentials` dan key management
  - **SPEC**
    - Password database yang disimpan untuk backup tidak boleh tersimpan plaintext pada database SQLite.
    - Enkripsi harus tetap memungkinkan scheduled backup berjalan setelah restart tanpa meminta password manual setiap kali.
  - **RULE**
    - Jangan hardcode encryption key di source code atau image.
    - Jangan log plaintext credential.
    - Kehilangan key harus menghasilkan error recovery yang eksplisit; jangan diam-diam menganggap credential kosong.
    - Jangan menghapus credential lama sebelum proses re-encryption terverifikasi.
  - **DESIGN**
    - Gunakan envelope encryption: data key per record atau per scope, dibungkus master key dari file secret dengan permission ketat atau external secret manager.
    - Tambahkan versi format ciphertext dan key ID untuk rotasi.
    - Buat migrator yang membaca record plaintext, mengenkripsi, menulis atomically, lalu memverifikasi decrypt sebelum menghapus plaintext.
    - Sediakan command/status untuk mendeteksi record legacy dan menyelesaikan migrasi.
  - **ACCEPTANCE CRITERIA**
    - [ ] SQLite baru tidak menyimpan password plaintext.
    - [ ] Migrasi database existing mengubah semua record dan diverifikasi.
    - [ ] Backup tetap berjalan setelah restart dengan key yang sama.
    - [ ] Wrong/missing key menghasilkan error aman dan actionable.
    - [ ] Rotasi key dapat dilakukan tanpa kehilangan kemampuan decrypt.
    - [ ] Test memastikan plaintext tidak muncul di DB dump, log, atau API response.

---

# 3. P2 — Hardening, correctness, operability

- [ ] **P2-01 — Kill seluruh process tree saat shell task cancel/timeout**
  - **AREA:** `crates/pier-agent/src/shell.rs`
  - **SPEC**
    - Cancel dan timeout harus menghentikan shell beserta seluruh descendant process yang dibuat oleh task tersebut.
  - **RULE**
    - Jangan hanya mengirim SIGTERM ke PID bash jika child process masih dapat hidup.
    - Jangan membunuh proses di luar process group milik task.
    - Berikan grace period, lalu SIGKILL jika proses tidak berhenti.
  - **DESIGN**
    - Spawn command pada process group/session terpisah.
    - Simpan PGID dan kirim SIGTERM ke process group; tunggu grace period; kirim SIGKILL jika perlu.
    - Pastikan stdout/stderr reader dan child handle ditutup/di-await dengan benar.
  - **ACCEPTANCE CRITERIA**
    - [ ] Test command dengan child process membuktikan semua descendant berhenti saat cancel.
    - [ ] Test timeout membuktikan tidak ada orphan process.
    - [ ] Task normal tetap mengembalikan exit code/stdout/stderr.
    - [ ] Tidak ada proses pengguna lain yang ikut terbunuh.
    - [ ] Test berjalan stabil pada platform yang didukung; behavior platform-specific didokumentasikan.

- [ ] **P2-02 — Perluas CI test dari satu binary ke seluruh workspace**
  - **AREA:** `.github/workflows/ci.yml`
  - **SPEC**
    - CI harus menjalankan format, lint, check, test seluruh workspace, integration test kritis, dan Docker build.
  - **RULE**
    - Gunakan `--locked` untuk menjaga lockfile.
    - Test penting tidak boleh di-skip tanpa alasan dan issue yang tercatat.
    - Pisahkan test yang butuh privileged environment dengan runner/job yang aman.
  - **DESIGN**
    - Pipeline minimum: shellcheck/syntax, cargo fmt --check, cargo clippy, cargo check --workspace --locked, cargo test --workspace --locked.
    - Tambahkan integration tests untuk auth/RBAC, webhook signature, DB isolation, backup migration, dan shell cancellation.
    - Tambahkan Docker build smoke test dan artifact retention untuk debugging.
    - Cache dependency dengan key yang menyertakan Cargo.lock dan toolchain.
  - **ACCEPTANCE CRITERIA**
    - [ ] Semua crate workspace terdeteksi dan dites.
    - [ ] Test failure menyebabkan PR check merah.
    - [ ] CI berjalan pada PR dan push ke main.
    - [ ] Docker image build diuji dari clean checkout.
    - [ ] Test privileged menggunakan fixture/container terisolasi, bukan host runner.
    - [ ] Durasi CI dicatat dan tidak mengorbankan coverage demi kecepatan.

- [ ] **P2-03 — Kurangi blocking SQLite mutex di async runtime**
  - **AREA:** `SharedState`, handlers async, dan akses `rusqlite::Connection`
  - **SPEC**
    - Operasi SQLite blocking tidak boleh menahan Tokio worker thread saat query lambat/contended.
    - Akses database perlu memiliki batas concurrency dan pola transaksi yang konsisten.
  - **RULE**
    - Jangan memegang mutex selama network I/O atau operasi filesystem yang lama.
    - Jangan memindahkan connection rusqlite antar thread jika melanggar thread-safety contract.
    - Jangan mengubah isolation/transaction semantics tanpa regression test.
  - **DESIGN**
    - Petakan semua callsite lock dan ukur contention sebelum refactor.
    - Pilih salah satu: dedicated DB worker/actor, `spawn_blocking` dengan ownership connection yang valid, atau pool SQLite yang terkontrol.
    - Pisahkan read/write path bila membantu concurrency; tetapkan busy timeout dan WAL policy secara eksplisit.
    - Tambahkan metrics untuk query duration, queue wait, lock wait, dan busy errors.
  - **ACCEPTANCE CRITERIA**
    - [ ] Tidak ada blocking DB call berat langsung pada async executor.
    - [ ] Tidak ada mutex held selama await/network I/O.
    - [ ] Concurrent API load test tidak mengalami starvation/deadlock.
    - [ ] Data consistency dan transaction rollback test tetap lulus.
    - [ ] Benchmark sebelum/sesudah tersedia untuk workload representatif.

- [ ] **P2-04 — Bangun backup integrity check dan restore drill otomatis**
  - **AREA:** backup executor, scheduler, UI/status, docs
  - **SPEC**
    - Status "backup sukses" harus membuktikan artifact lengkap dan dapat dibaca; keberadaan file saja tidak cukup.
    - Restore harus diuji secara berkala pada lingkungan terisolasi.
  - **RULE**
    - Jangan restore test ke production database/volume.
    - Jangan mengirim credential atau data backup ke log.
    - Retention cleanup tidak boleh menghapus satu-satunya backup valid.
  - **DESIGN**
    - Tambahkan manifest berisi format version, timestamp, resource identity, ukuran, checksum, dan komponen backup.
    - Setelah backup, lakukan checksum/readability validation.
    - Jalankan restore drill pada temporary container/namespace/network terisolasi, verifikasi schema/data marker, lalu destroy environment.
    - Tampilkan last verified backup, last restore drill, duration, dan error reason.
  - **ACCEPTANCE CRITERIA**
    - [ ] Artifact korup terdeteksi sebelum dinyatakan valid.
    - [ ] Restore drill sukses pada fixture PostgreSQL dan MySQL/MariaDB yang didukung.
    - [ ] Temporary environment selalu dibersihkan meskipun test gagal.
    - [ ] Retention policy mempertahankan jumlah backup valid minimum.
    - [ ] Alert muncul jika backup gagal atau restore drill melewati SLA.
    - [ ] Runbook disaster recovery diuji oleh operator lain.

- [ ] **P2-05 — Tambahkan audit log terstruktur untuk aksi sensitif**
  - **SPEC**
    - Catat login/security setting, credential rotation, DB access, deployment, webhook configuration, backup/restore, user/role changes, dan privileged operation.
  - **RULE**
    - Audit log append-only dari perspektif aplikasi; user biasa tidak dapat mengubah/menghapus.
    - Jangan simpan secret, token, private key, atau query parameter sensitif.
    - Pisahkan audit event dari debug log.
  - **DESIGN**
    - Definisikan schema event: actor, action, target, timestamp UTC, outcome, source IP bila tersedia, request/correlation ID, metadata yang sudah direduksi.
    - Sediakan filter UI/API dan export.
    - Terapkan retention dan opsi forward ke syslog/SIEM.
  - **ACCEPTANCE CRITERIA**
    - [ ] Setiap aksi sensitif yang tercantum menghasilkan event.
    - [ ] Event gagal juga tercatat dengan outcome yang tepat.
    - [ ] Tidak ada secret dalam event.
    - [ ] Filter berdasarkan actor, target, action, dan rentang waktu berfungsi.
    - [ ] Akses audit log mengikuti role khusus.

- [ ] **P2-06 — Tambahkan API token dengan scope dan expiry**
  - **SPEC**
    - Integrasi automation tidak harus memakai session/global admin credential.
    - Token harus dapat dibatasi berdasarkan scope, resource, expiry, dan revocation.
  - **RULE**
    - Token baru hanya menampilkan plaintext sekali saat dibuat.
    - Token disimpan sebagai hash yang aman, bukan plaintext.
    - Default scope minimum dan expiry wajib atau default terbatas.
  - **DESIGN**
    - Scope contoh: `service:read`, `service:deploy`, `backup:read`, `backup:restore`, `db:read`.
    - Ikat token ke owner + resource allowlist; dukung revoke dan last-used metadata.
    - Middleware memvalidasi scope per endpoint dan menolak privilege escalation.
  - **ACCEPTANCE CRITERIA**
    - [ ] Token tanpa scope yang diperlukan ditolak.
    - [ ] Token expired/revoked langsung tidak dapat digunakan.
    - [ ] Token tidak dapat mengakses resource di luar allowlist.
    - [ ] Plaintext tidak tersimpan di DB/log.
    - [ ] UI mendukung create, copy-once, rotate, revoke, dan expiry display.

---

# 4. P3 — Product features & differentiation

- [ ] **P3-01 — Smart Deployment: health gate, canary/blue-green, auto rollback**
  - **SPEC**
    - Deployment harus dapat menunggu health check dan otomatis rollback bila versi baru gagal memenuhi policy.
    - Sediakan progressive rollout untuk service yang mendukung multi-instance.
  - **RULE**
    - Auto rollback hanya jika ada previous known-good deployment dan rollback procedure aman.
    - Database migration irreversible harus memblokir auto rollback kecuali ada compatibility contract.
    - User dapat memilih manual approval untuk production.
  - **DESIGN**
    - Model deployment state: queued → preparing → deploying → health-checking → promoted/rolled-back/failed.
    - Health gate mencakup container health, HTTP/TCP probe, startup grace period, consecutive failures, dan timeout.
    - Canary rollout bertahap dengan traffic weight dan pause/approval gates.
    - Simpan deployment manifest dan previous image/config reference.
  - **ACCEPTANCE CRITERIA**
    - [ ] Health check gagal menyebabkan rollout berhenti.
    - [ ] Auto rollback mengembalikan image/config sebelumnya tanpa kehilangan persistent data.
    - [ ] Manual approval dan maintenance window berfungsi.
    - [ ] Timeline menunjukkan setiap fase dan alasan rollback.
    - [ ] Test mencakup deploy sukses, probe timeout, crash loop, dan rollback failure.

- [ ] **P3-02 — Security Center: image scan, SBOM, secrets, posture score**
  - **SPEC**
    - Berikan satu tempat untuk melihat vulnerability, image provenance, exposed ports, insecure configuration, dan secret leak signals.
  - **RULE**
    - Score harus explainable; jangan menyebut sistem "secure" hanya berdasarkan skor.
    - Secret scanning tidak boleh mengirim secret mentah ke pihak ketiga tanpa consent.
    - Finding harus memiliki severity, evidence, remediation, dan last-scanned timestamp.
  - **DESIGN**
    - Generate SBOM per image/build.
    - Integrasikan scanner modular untuk CVE, image config, dependency, dan repository secret patterns.
    - Map finding ke service/deployment dan tampilkan trend.
    - Tambahkan exception/accepted-risk dengan owner, alasan, expiry.
  - **ACCEPTANCE CRITERIA**
    - [ ] SBOM dapat diunduh dan terikat ke image digest.
    - [ ] Finding memiliki evidence dan remediation guidance.
    - [ ] Exception memiliki expiry dan audit trail.
    - [ ] Scan dapat dijalankan manual dan terjadwal.
    - [ ] False-positive workflow tersedia tanpa menghapus evidence.

- [ ] **P3-03 — Fleet Manager untuk multi-server**
  - **SPEC**
    - Kelola banyak host/agent dari satu dashboard: health, capacity, version, maintenance, dan deployment target.
  - **RULE**
    - Host identity harus unik dan terverifikasi.
    - Aksi massal harus preview target dan mendukung staged rollout.
    - Host offline tidak boleh dianggap berhasil menerima perubahan.
  - **DESIGN**
    - Host registry dengan enrollment token sekali pakai, agent version, capabilities, heartbeat, labels, dan region.
    - Group host berdasarkan environment/team/label.
    - Support drain/maintenance mode dan rolling agent upgrade.
  - **ACCEPTANCE CRITERIA**
    - [ ] Host online/offline/stale dibedakan.
    - [ ] Enrollment/revoke aman dan dapat diaudit.
    - [ ] Upgrade agent bertahap dan dapat dihentikan.
    - [ ] Aksi massal menampilkan preview dan hasil per host.
    - [ ] Capacity view menampilkan CPU/RAM/disk dan timestamp data.

- [ ] **P3-04 — GitOps dan configuration drift detection**
  - **SPEC**
    - Service configuration dapat disinkronkan dari Git dan perbedaan antara desired state dengan live state dapat dideteksi.
  - **RULE**
    - Jangan overwrite live config tanpa preview/diff dan policy.
    - Secret harus menggunakan reference/secret manager, bukan plaintext dalam Git.
    - Reconcile loop harus idempotent dan memiliki rate limit.
  - **DESIGN**
    - Definisikan schema manifest versioned untuk service, env references, ports, volumes, networks, healthchecks, dan deploy policy.
    - Pull-based reconciliation dengan status drift: in-sync, drifted, blocked, error.
    - Sediakan preview diff, approve, sync, dan rollback commit.
  - **ACCEPTANCE CRITERIA**
    - [ ] Manifest tervalidasi sebelum diterapkan.
    - [ ] Drift terdeteksi dan menunjukkan field yang berbeda.
    - [ ] Sync membutuhkan policy/approval yang sesuai environment.
    - [ ] Secret value tidak pernah ditulis ke Git diff/log.
    - [ ] Reconcile berulang tidak membuat perubahan berulang yang tidak perlu.

- [ ] **P3-05 — Resource Intelligence dan capacity forecasting**
  - **SPEC**
    - Tampilkan histori CPU/RAM/disk/network, tren penggunaan, anomali, dan estimasi kapan kapasitas akan habis.
  - **RULE**
    - Tampilkan rentang waktu, sampling interval, dan kualitas/kelengkapan data.
    - Forecast harus diberi label estimasi, bukan kepastian.
    - Hindari telemetry berlebihan yang membebani agent.
  - **DESIGN**
    - Agent mengirim metrics teragregasi dengan retention/downsampling.
    - Dashboard per host/service dan alert threshold yang dapat diatur.
    - Forecast sederhana berbasis trend + confidence band sebelum model kompleks.
  - **ACCEPTANCE CRITERIA**
    - [ ] Grafik histori dapat difilter berdasarkan host/service/rentang waktu.
    - [ ] Alert disk pressure dan capacity threshold dapat diuji.
    - [ ] Forecast menampilkan horizon dan confidence/uncertainty.
    - [ ] Sampling dan retention dapat dikonfigurasi.
    - [ ] Overhead agent diukur dan berada dalam budget yang disepakati.

- [ ] **P3-06 — Disaster Recovery Wizard**
  - **SPEC**
    - Operator dapat memulihkan server/service dari backup melalui langkah terpandu, dengan validasi preflight dan recovery report.
  - **RULE**
    - Restore production membutuhkan konfirmasi eksplisit dan permission tinggi.
    - Tampilkan target, data yang akan ditimpa, timestamp backup, dan risiko sebelum eksekusi.
    - Tidak boleh menganggap restore selesai sebelum post-restore verification.
  - **DESIGN**
    - Wizard: pilih backup → verify integrity → pilih target → preflight capacity/compatibility → review impact → approve → restore → verify → report.
    - Sediakan restore ke environment sementara untuk rehearsal.
  - **ACCEPTANCE CRITERIA**
    - [ ] Preflight mendeteksi backup corrupt, versi tidak kompatibel, dan kapasitas kurang.
    - [ ] User melihat impact summary sebelum confirm.
    - [ ] Restore rehearsal tidak menyentuh production.
    - [ ] Report memuat langkah, durasi, hasil verifikasi, dan error.
    - [ ] Aksi restore masuk audit log.

- [ ] **P3-07 — Diagnostics bundle dengan redaction**
  - **SPEC**
    - Sediakan bundle diagnostik untuk mempercepat troubleshooting tanpa membocorkan credential atau data pribadi.
  - **RULE**
    - Redaction wajib dilakukan sebelum archive dibuat.
    - User dapat preview daftar file dan kategori data yang akan disertakan.
    - Jangan menyertakan database dump atau file volume secara default.
  - **DESIGN**
    - Kumpulkan version/build info, sanitized config, service status, recent redacted logs, agent health, dan relevant system metrics.
    - Gunakan allowlist file; redaction patterns untuk token, password, private key, DSN, email/IP opsional.
    - Beri manifest isi bundle dan checksum.
  - **ACCEPTANCE CRITERIA**
    - [ ] Test fixtures berisi dummy secrets dan memastikan semuanya ter-redact.
    - [ ] User dapat memilih kategori diagnostik.
    - [ ] Bundle tidak berisi raw DB/volume data secara default.
    - [ ] Manifest dan checksum tersedia.
    - [ ] Ukuran bundle dibatasi dan proses dapat dibatalkan.

- [ ] **P3-08 — Passkeys/WebAuthn untuk akun admin**
  - **SPEC**
    - Tambahkan authentication factor phishing-resistant untuk admin dan opsi kebijakan organisasi.
  - **RULE**
    - Recovery path harus aman dan tidak melemahkan MFA.
    - Credential WebAuthn terikat ke user dan relying-party ID yang benar.
    - Perubahan factor/recovery harus diaudit.
  - **DESIGN**
    - Registration/authentication ceremony sesuai WebAuthn.
    - Support multiple authenticators, revoke, recovery codes, dan policy enforcement per role.
  - **ACCEPTANCE CRITERIA**
    - [ ] Registration dan login diuji pada browser/platform yang didukung.
    - [ ] Origin/RP ID mismatch ditolak.
    - [ ] Revoke authenticator langsung berlaku.
    - [ ] Recovery flow diuji dan diaudit.
    - [ ] Admin dapat diwajibkan memakai passkey/MFA sesuai policy.

- [ ] **P3-09 — Plugin/integration SDK**
  - **SPEC**
    - Sediakan extension point untuk integrasi notification, monitoring, deployment hooks, dan secret providers.
  - **RULE**
    - Plugin berjalan dengan capability minimum dan tidak otomatis mewarisi admin privilege.
    - Plugin third-party harus memiliki manifest, version compatibility, permission declaration, dan lifecycle.
    - Network access dan secret access harus eksplisit.
  - **DESIGN**
    - Mulai dari webhook/event integration yang terisolasi sebelum arbitrary code execution.
    - Definisikan plugin manifest, capability scopes, event schema version, timeout, retry, dan signature.
    - Sediakan sandbox/worker boundary bila nantinya mendukung executable plugins.
  - **ACCEPTANCE CRITERIA**
    - [ ] Plugin tanpa capability tidak dapat mengakses resource.
    - [ ] Event delivery idempotent dengan retry/backoff.
    - [ ] Plugin dapat disable/revoke tanpa restart seluruh platform jika memungkinkan.
    - [ ] Compatibility check dan audit trail tersedia.
    - [ ] Threat model ditinjau sebelum executable plugin runtime dirilis.

---

# 5. Cross-cutting UX & operations

- [ ] **UX-01 — Permission matrix dan role clarity**
  - **SPEC:** Dokumentasikan capability untuk Global Owner/Admin, Project Admin/Editor/Viewer, service-scoped token, dan agent.
  - **RULE:** UI dan API harus memakai policy yang sama; UI hide button bukan authorization.
  - **DESIGN:** Buat permission matrix sebagai sumber kebenaran, lalu gunakan test table-driven.
  - **ACCEPTANCE CRITERIA:** [ ] Semua endpoint sensitif terpetakan. [ ] Setiap role diuji allow/deny. [ ] Dokumentasi dan UI konsisten.

- [ ] **OPS-01 — Operational runbooks**
  - **SPEC:** Runbook untuk upgrade, rollback, webhook failure, DB credential rotation, backup/restore, helper install, dan agent offline.
  - **RULE:** Langkah harus dapat dilakukan operator lain tanpa asumsi pengetahuan penulis.
  - **DESIGN:** Setiap runbook memuat symptom, impact, diagnosis, safe mitigation, recovery, verification, escalation.
  - **ACCEPTANCE CRITERIA:** [ ] Minimal satu simulasi tabletop per skenario kritis. [ ] Perintah destructive diberi warning dan precondition. [ ] Runbook memiliki owner dan tanggal review.

- [ ] **OPS-02 — Metrics, alerting, and SLO baseline**
  - **SPEC:** Ukur API availability/latency, agent heartbeat, deployment success, backup success, restore verification, queue delay, dan DB contention.
  - **RULE:** Alert harus actionable dan memiliki severity/owner; hindari alert tanpa runbook.
  - **DESIGN:** Tentukan SLI/SLO awal berdasarkan baseline pengukuran, bukan angka arbitrer.
  - **ACCEPTANCE CRITERIA:** [ ] Dashboard operasional tersedia. [ ] Alert diuji dengan fault injection. [ ] Setiap alert kritis mengarah ke runbook.

---

# 6. Urutan implementasi yang disarankan

| Wave | Isi | Exit gate |
|---|---|---|
| **Wave 0 — Build/release integrity** | P0-01 Docker build, P0-02 CI release gate | Clean build dan release gate terbukti |
| **Wave 1 — Access & webhook security** | P1-01 security settings RBAC, P1-02 DB isolation, P1-03 webhook signature | Negative authorization tests lulus |
| **Wave 2 — Supply chain & secrets** | P1-04 helper verification, P1-05 credential encryption | No unverified privileged binary; no plaintext DB credentials |
| **Wave 3 — Runtime & recovery** | P2-01 process tree, P2-02 CI coverage, P2-03 SQLite, P2-04 restore drill, P2-05 audit | Integration tests + restore evidence |
| **Wave 4 — Platform APIs** | P2-06 scoped tokens, UX-01 permission matrix, OPS runbooks/metrics | API contract dan operational readiness |
| **Wave 5 — Product expansion** | P3-01 sampai P3-09 | Feature-by-feature RFC, threat model, staged rollout |

## 7. Release readiness checklist

- [ ] Semua P0 selesai dan diverifikasi pada clean checkout.
- [ ] Semua P1 selesai atau ada risk acceptance tertulis dari owner (tidak disarankan untuk production).
- [ ] CI wajib lulus pada commit SHA yang akan dirilis.
- [ ] Docker image build dan startup smoke test lulus.
- [ ] Migration test dari versi existing lulus; backup dan rollback plan tersedia.
- [ ] Webhook signature, RBAC, DB isolation, secret handling, dan helper integrity diuji.
- [ ] Backup integrity check dan restore drill memiliki bukti terbaru.
- [ ] Release notes, upgrade guide, rollback guide, dan known issues tersedia.
- [ ] Tidak ada secret, credential, atau private key pada source, image, artifact, atau logs.
- [ ] Security review dan approval release tercatat.

## 8. Task completion template

Gunakan template ini untuk setiap task baru agar implementasi dapat direview:

- [ ] **TASK-ID — Judul**
  - **SPEC:** problem, scope, non-goals, compatibility, expected behavior.
  - **RULE:** authorization, security constraints, invariants, failure behavior.
  - **DESIGN:** affected modules, API/schema changes, data flow, migration, observability.
  - **ACCEPTANCE CRITERIA:** testable positive/negative cases, performance/operational evidence, docs.
  - **TEST PLAN:** unit, integration, regression, migration, failure injection.
  - **ROLLBACK:** trigger, safe rollback steps, data compatibility.
  - **EVIDENCE:** CI run, test output, review link, screenshots/log references (redacted).

---

**Maintainer note:** Kerjakan P0/P1 lebih dahulu. P3 adalah backlog ide, bukan komitmen release. Jangan menggabungkan banyak perubahan security besar dalam satu commit; buat perubahan kecil, terukur, dan reviewable. Tidak ada task dalam dokumen ini yang mengizinkan deployment atau perubahan pada production server secara otomatis.
