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
  let msg = (if ($message | is-empty) { $tag } else { $message })

  if $dry_run {
    print $"Planned tag: ($tag)"
    print $"Would run: git tag -a ($tag) -m '($msg)'"
    print $"Would run: git push origin ($tag)"
    return
  }

  git tag -a $tag -m $msg
  git push origin $tag
  print $"Created and pushed tag ($tag)"
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
