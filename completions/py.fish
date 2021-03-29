# Wrap the `python` command.
# While wrapping the latest Python version specifically would provide
# the most accurate completions, fish only ships with completions for
# `python` itself.
complete -c py --wraps python

# Statically-known completions.
complete -c py --long-option list --no-files -d "List all known interpreters"
complete -c py --short-option h --long-option help --no-files -d "Display help and exit"

# Dynamic/system-specific completions.
# XXX Major-minor
# XXX Major-only
