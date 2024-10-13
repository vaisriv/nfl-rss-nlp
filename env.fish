set -x LIBTORCH $(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
set -x LD_LIBRARY_PATH $LIBTORCH/lib $LD_LIBRARY_PATH
