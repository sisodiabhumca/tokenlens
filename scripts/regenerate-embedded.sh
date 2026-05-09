#!/usr/bin/env bash
# Regenerate crates/tokenlens-core/src/registry/embedded.rs from filters/*.toml.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/crates/tokenlens-core"
python3 - <<'PY'
import os, glob
files = sorted(os.path.basename(p) for p in glob.glob('filters/*.toml'))
out = ['/// Auto-generated. Do not hand-edit.',
       'pub static EMBEDDED_FILTERS: &[(&str, &str)] = &[']
for f in files:
    name = f[:-5]
    out.append(f'    ("{name}", include_str!(concat!("../../filters/", "{f}"))),')
out.append('];')
open('src/registry/embedded.rs','w').write('\n'.join(out)+'\n')
print(f"wrote {len(files)} filters")
PY
