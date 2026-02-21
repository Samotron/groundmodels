#!/usr/bin/env nu

def main [
  --prefix: string = "v" # Tag prefix (default v)
  --message: string # Optional tag message; defaults to generated tag
  --dry-run # Print commands without executing
] {
  let dirty = (^git status --porcelain | str trim)
  if ($dirty | is-empty) == false {
    error make {
      msg: "Working tree is dirty. Commit or stash changes before tagging."
    }
  }

  print "Running tests..."
  ^cargo test

  let today = (date now | format date "%Y.%m.%d")
  let tag = (next_tag $today $prefix)
  let version = (tag_to_version $tag)
  let msg = (if ($message | is-empty) { $tag } else { $message })

  if $dry_run {
    print $"Planned version: ($version)"
    print $"Planned tag: ($tag)"
    print $"Would update Cargo.toml version to ($version)"
    print $"Would run: git add Cargo.toml Cargo.lock"
    print $"Would run: git commit -m 'chore: bump version to ($version)'"
    print $"Would run: git tag -a ($tag) -m '($msg)'"
    print $"Would run: git push origin main"
    print $"Would run: git push origin ($tag)"
    return
  }

  print $"Updating version to ($version)..."
  update_cargo_version $version

  print "Committing version bump..."
  ^git add Cargo.toml Cargo.lock
  ^git commit -m $"chore: bump version to ($version)"

  print $"Creating tag ($tag)..."
  ^git tag -a $tag -m $msg

  print "Pushing commit and tag..."
  ^git push origin main
  ^git push origin $tag

  print $"Released ($tag)"
}

def next_tag [datever: string, prefix: string] {
  let base = $"($prefix)($datever)"
  let existing = (git tag -l $"($base).*" | lines)
  if ($existing | length) == 0 {
    return $"($base).01"
  }

  let max_suffix = (
    $existing
    | each {|t| $t | str replace $"($base)." "" }
    | where {|s| $s | str length | $in == 2 }
    | each {|s| $s | into int }
    | math max
  )

  let next = ($max_suffix + 1)
  let padded = ("00" + ($next | into string) | str substring (-2)..)
  $"($base).($padded)"
}

def tag_to_version [tag: string] {
  let parts = ($tag | str replace "v" "" | split row ".")
  let year = ($parts | get 0 | into int | into string)
  let month = ($parts | get 1 | into int | into string)
  let day = ($parts | get 2 | into int | into string)
  let suffix = ($parts | get 3 | into int | into string)
  $"($year).($month).($day).($suffix)"
}

def update_cargo_version [version: string] {
  let content = (open -r Cargo.toml)
  let updated = ($content | str replace -r 'version = ".*"' $"version = \"($version)\"")
  $updated | save -f Cargo.toml
  ^cargo build --locked
}
