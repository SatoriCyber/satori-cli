
# Provide auto-complete for datastores for run commands
comp_len=${#COMP_WORDS[@]}
if ((comp_len ==4)); then 
    opts=$(list_datastores | sed 's/.*/"&"/')
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
