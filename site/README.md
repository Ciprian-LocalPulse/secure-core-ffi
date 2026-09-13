# Secure Core FFI launch page

This is a dependency-free static site. It can be deployed directly to Vercel from the repository root.

## Vercel project settings

- Framework preset: **Other**
- Root directory: repository root
- Build command: none
- Output directory: none
- Install command: none

The root `vercel.json` rewrites requests to `site/`. For automatic deployment through a separate CI workflow, configure `VERCEL_TOKEN`, `VERCEL_ORG_ID`, and `VERCEL_PROJECT_ID` as repository secrets.
