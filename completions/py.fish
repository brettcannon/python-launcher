# Wrap the `python` command.
# While wrapping the latest Python version specifically would provide
# the most accurate completions, fish only ships with completions for
# `python` itself.
complete -c py --wraps python

# Statically-known completions.
complete -c py --long-option list --no-files -d "List all known interpreters"
complete -c py --short-option h --long-option help --no-files -d "Display help and exit"

# Dynamic/system-specific completions.
set -l seen_major_versions
py --list | while read -d " │ " -l padded_version padded_path
    # Complete on the `major.minor` version.
    set -l full_version (string trim $padded_version)
    set -l executable_path (string trim $padded_path)
    complete -c py --old-option $full_version -d "Launch $executable_path"
    # Complete on the major version.
    # Assume that `py --list` emits a sorted list of versions, so the
    # first instance of any major version is the one that will be used.
    set -l major_version (string split --fields 1 . $full_version)
    if not contains $major_version $seen_major_versions
        # Must use `--old-option` in case the major version ever goes multi-digit.
        complete -c py --old-option $major_version -d "Launch $executable_path"
        set --append seen_major_versions $major_version
    end
end
