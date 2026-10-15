#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mode=${1:-full}
if [[ $# -gt 0 ]]; then shift; fi
case "$mode" in quick|audit|full) ;; *) echo 'Invalid check mode' >&2; exit 2 ;; esac
scope=${1:-worktree}
case "$scope" in worktree|--staged|--push) ;; *) echo 'Expected --staged or --push' >&2; exit 2 ;; esac
compose=(docker compose --env-file /dev/null -p mybrigitte-checks -f docker-compose.checks.yml)
built=false
ensure_image() {
    if [[ $built == false ]]; then
        docker compose version >/dev/null
        "${compose[@]}" build --quiet checks
        built=true
    fi
}
# Documentation-only changes require no container or application tooling.
needs_checks() {
    local file
    while IFS= read -r -d '' file; do
        case "$file" in docs/*|.github/workflows/README.md) continue ;; esac
        if [[ $file != */* && $file == *.md ]]; then continue; fi
        return 0
    done < "$1"
    return 1
}
if [[ $scope == worktree ]]; then
    ensure_image
    git ls-files --cached --others --exclude-standard --deduplicate -z |
        while IFS= read -r -d '' file; do
            case "$file" in .env|.env.*|*/.env|*/.env.*) continue ;; esac
            if [[ -f "$file" || -L "$file" ]]; then printf '%s\0' "$file"; fi
        done |
        tar --null --verbatim-files-from -T - -cf - |
        "${compose[@]}" run --rm --no-deps -T checks "$mode"
    exit
fi
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
run_snapshot() {
    local revision=$1 paths=$2 snapshot
    if ! needs_checks "$paths"; then
        echo 'No code checks required for this diff.'
        return
    fi
    snapshot=$(mktemp -d "$scratch/snapshot.XXXXXX")
    # Read the index/commit tree, never unstaged or untracked application files.
    git archive --format=tar "$revision" -- . ':(exclude).env' ':(exclude).env.*' ':(exclude)**/.env' ':(exclude)**/.env.*' |
        tar -xf - -C "$snapshot"
    mkdir -p "$snapshot/.checks-runner"
    # Current checker tooling can also validate older commits/tags which lack it.
    cp scripts/check.py scripts/ci-plan.py "$snapshot/.checks-runner/"
    cp "$paths" "$snapshot/.checks-runner/paths"
    ensure_image
    tar -C "$snapshot" -cf - . |
        "${compose[@]}" run --rm --no-deps -T checks "$mode" --changed-paths .checks-runner/paths
}
if [[ $scope == --staged ]]; then
    # Git may supply an alternate index for `commit -a` / partial path commits.
    if [[ -n ${CHECKS_INDEX_FILE:-} ]]; then export GIT_INDEX_FILE=$CHECKS_INDEX_FILE; fi
    git diff --cached --name-only --no-renames -z > "$scratch/paths"
    tree=$(git write-tree)
    run_snapshot "$tree" "$scratch/paths"
else
    remote=${2:-origin}
    records=()
    # Consume Git's ref list before Docker uses stdin for the source archive.
    while IFS= read -r record; do records+=("$record"); done
    for record in "${records[@]}"; do
        read -r local_ref local_sha _remote_ref remote_sha extra <<< "$record"
        if [[ -n ${extra:-} || ! $local_sha =~ ^[0-9a-fA-F]{40}$ || ! $remote_sha =~ ^[0-9a-fA-F]{40}$ ]]; then
            echo 'Invalid pre-push ref record' >&2; exit 2
        fi
        if [[ $local_sha == 0000000000000000000000000000000000000000 ]]; then continue; fi
        base=$remote_sha
        if [[ $base == 0000000000000000000000000000000000000000 ]]; then
            base=''
            for branch in dev main; do
                if base=$(git merge-base "$local_sha" "refs/remotes/$remote/$branch" 2>/dev/null); then break; fi
                base=''
            done
        elif ! git cat-file -e "$base^{commit}" 2>/dev/null; then
            git fetch --no-tags --depth=1 "$remote" "$base" >/dev/null 2>&1 || true
        fi
        if [[ $local_ref == refs/tags/* || -z $base ]] || ! git diff --name-only --no-renames -z "$base" "$local_sha" > "$scratch/paths"; then
            printf 'unknown-base\0' > "$scratch/paths"
        fi
        echo "Checking outgoing ref $local_ref"
        run_snapshot "$local_sha" "$scratch/paths"
    done
fi
