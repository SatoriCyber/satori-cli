
# Provide auto-complete for databases for run commands
elif ((comp_len == 5)); then 
    opts=$(list_databases "$prev")
    local IFS=$'\n'
    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
    return 0
