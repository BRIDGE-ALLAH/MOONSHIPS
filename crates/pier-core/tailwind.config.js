/**
 * Tailwind config for the MOONSHIPS admin panel (Pier-compatible runtime crate).
 *
 * Tailwind v3 — matches the runtime that `cdn.tailwindcss.com` used, so all
 * existing template classes and `dark:` variants compile unchanged. Build the
 * static stylesheet with:
 *
 *   npx tailwindcss@3 -c tailwind.config.js -i assets/css/tailwind.css -o assets/static/tailwind.css --minify
 *
 * The output `assets/static/tailwind.css` is committed and embedded via
 * rust_embed, so `cargo build` needs no Node/CLI. Re-run after adding new
 * Tailwind classes to templates. See the repository's UI build documentation.
 */
module.exports = {
  content: ['./assets/templates/**/*.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        moonships: { 50: '#eff6ff', 500: '#3b82f6', 600: '#2563eb', 700: '#1d4ed8' },
      },
    },
  },
  safelist: [],
};
